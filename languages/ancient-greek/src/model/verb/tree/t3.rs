use flexion::Path;

/// Words ending -ορεω.
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2965);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4557);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5962);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7027);
											match s {
												[] => true,
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
									super::push(out, 5963);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7028);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7678);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9552(s, out),
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
									super::push(out, 5964);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7029);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7679);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9553(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4558);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5965);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7030);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1550);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2966);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4559);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1551);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2967);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4560);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5967);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7031);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7680);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9554(s, out),
														['τ', s @ ..] => super::t4::n9555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1553);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2970);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 4562);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5968);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7032);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1554);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2971);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4563);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1555);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2972);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2973);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4564);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5969);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7033);
											s.is_empty()
										}
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
					super::push(out, 2974);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4565);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5970);
									match s {
										[] => true,
										['π', s @ ..] => super::t4::n7971(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t4::n3088(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρρεω.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1556);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2977);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4569);
							match s {
								[] => true,
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
					super::push(out, 2978);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4570);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5974);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1557);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τρεω.
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1558);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4572);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4573);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5975);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 7039);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρεω.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['ξ', s @ ..] => {
			super::push(out, 1560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2982);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2983);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4574);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5976);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7040);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2984);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 4575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1562);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2985);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4576);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5977);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωρεω.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2986);
					match s {
						[] => true,
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
			super::push(out, 1565);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2987);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4577);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5978);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7041);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7686);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9561(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1566);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2988);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4578);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5979);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7042);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7687);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9562(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2990);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4579);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5980);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 7043);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7688);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9563(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 7689);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9564(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2991);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4580);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 5981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7044);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 5982);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2992);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4581);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5983);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7045);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7690);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9565(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4582);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5984);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 5985);
									match s {
										[] => true,
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
							super::push(out, 4583);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2993);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4584);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5987);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7046);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word περιπατεω.
pub(crate) fn n8986(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επικρατεω.
pub(crate) fn n8987(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -λινοστατεω.
pub(crate) fn n8988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7691);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αποστατεω.
pub(crate) fn n8989(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word λογοθετεω.
pub(crate) fn n8990(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word τοποθετεω.
pub(crate) fn n8991(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -πηρετεω.
pub(crate) fn n4899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4595);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 5998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ξ', 'ε', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ητεω.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 1578);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3006);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1579);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3007);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4598);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5999);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7054);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιτεω.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3008);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 3009);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4599);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3010);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4600);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 4601);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -κτεω.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3011);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4602);
							match s {
								[] => true,
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
					super::push(out, 3012);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4603);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6002);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7055);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7692);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1582);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3013);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4604);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6003);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7056);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7693);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9567(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -ντεω.
pub(crate) fn n637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 637);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1583);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3015);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4606);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6004);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7057);
											s.is_empty()
										}
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
					super::push(out, 3016);
					match s {
						[] => true,
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
			super::push(out, 1584);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 3017);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4607);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6005);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7058);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7694);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9568(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οτεω.
pub(crate) fn n638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 638);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1585);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3018);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4608);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6006);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7059);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7695);
													match s {
														[] => true,
														['ι', s @ ..] => super::t4::n9569(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1586);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρτεω.
pub(crate) fn n639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 639);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1587);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3020);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4609);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στεω.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1588);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3021);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4610);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6008);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7060);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7696);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9570(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1589);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3022);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4611);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6009);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1590);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3023);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4612);
							match s {
								[] => true,
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
					super::push(out, 3024);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4613);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6010);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7061);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7697);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9571(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1591);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4614);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6011);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7062);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1592);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3026);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4615);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6012);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7063);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7698);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9572(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1593);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3027);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4616);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτεω.
pub(crate) fn n641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 641);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3028);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4617);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6014);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 7064);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7699);
													match s {
														[] => true,
														['τ', s @ ..] => super::t4::n9573(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αλλιγραφεω.
pub(crate) fn n9002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7700);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ευκογραφεω.
pub(crate) fn n9003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7701);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 8007);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γαλογραφεω.
pub(crate) fn n9004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7702);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 8008);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 8156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οιπογραφεω.
pub(crate) fn n9005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7703);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 8009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ικρογραφεω.
pub(crate) fn n9006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7704);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 8010);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ματογραφεω.
pub(crate) fn n9007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7705);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 8011);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 8157);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 8214);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ορραφεω.
pub(crate) fn n4929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4621);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 6022);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 7072);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 7706);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 8012);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 8158);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οφεω.
pub(crate) fn n646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1597);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3032);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4622);
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 6023);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7073);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7707);
													match s {
														[] => true,
														['ο', s @ ..] => super::t4::n9581(s, out),
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
									super::push(out, 6024);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7074);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7708);
													match s {
														[] => true,
														['κ', s @ ..] => super::t4::n9582(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1598);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3033);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4623);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6025);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 7075);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word λογομαχεω.
pub(crate) fn n9012(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word φωνομαχεω.
pub(crate) fn n9013(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -υνναυμαχεω.
pub(crate) fn n9014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7709);
	match s {
		[] => true,
		['ξ', s @ ..] => {
			super::push(out, 8015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 8016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αρχεω.
pub(crate) fn n1632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1610);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 3045);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4633);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6031);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3046);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4634);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6032);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7082);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7710);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 8017);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9869(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3047);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4635);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6033);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 7083);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 7711);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 8018);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9870(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3048);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4636);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6034);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υχεω.
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3049);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4637);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6035);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7084);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7712);
													match s {
														[] => true,
														['λ', s @ ..] => super::t4::n9587(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1612);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3051);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4639);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6036);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3052);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1613);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3053);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4641);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6037);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7085);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7713);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9588(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σκυλοδεψεω.
pub(crate) fn n9020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7714);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λωττοδεψεω.
pub(crate) fn n9021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7715);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 8020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word διαβιβαζω.
pub(crate) fn n9022(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -καταβιβαζω.
pub(crate) fn n9023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7716);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιβιβαζω.
pub(crate) fn n9024(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εισβιβαζω.
pub(crate) fn n9025(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ιαζω.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 665);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1625);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 1626);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3063);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4652);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6048);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1627);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4653);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6049);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 7093);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7717);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9592(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1628);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
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
			super::push(out, 1629);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4655);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6050);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7094);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3069);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4656);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6051);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1631);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3070);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4657);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6052);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1633);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3072);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4658);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6053);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7095);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7718);
													match s {
														[] => true,
														['λ', s @ ..] => super::t4::n9593(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4659);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 4660);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6054);
									match s {
										[] => true,
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
							super::push(out, 4661);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6055);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1634);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3074);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4662);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6056);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3075);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4663);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6057);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3076);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4664);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6058);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7096);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7719);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9594(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1635);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3077);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t4::n6666(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => super::t4::n1658(s, out),
		_ => false,
	}
}

/// Words ending -γκαζω.
pub(crate) fn n1659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1637);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3080);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4667);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6061);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7097);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 7098);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7720);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 8023);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9875(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ικαζω.
pub(crate) fn n1660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1638);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3081);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 3082);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 4669);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6062);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7099);
									match s {
										[] => true,
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
							super::push(out, 6063);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7100);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7721);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρκαζω.
pub(crate) fn n1661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1639);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3084);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 4671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λαζω.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 667);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1641);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
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
			super::push(out, 1642);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1643);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3090);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μαζω.
pub(crate) fn n668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 668);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3092);
					match s {
						[] => true,
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
			super::push(out, 1647);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3093);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4674);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6064);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7101);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7722);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9597(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3094);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4675);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6065);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3095);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4676);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6066);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7102);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 7103);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7723);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9598(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3096);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1648);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1649);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3098);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 6067);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7104);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1650);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3099);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 4680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1651);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3100);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ναζω.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 669);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1652);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3101);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1654);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4683);
							match s {
								[] => true,
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
					super::push(out, 3103);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4684);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1655);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4685);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3105);
					match s {
						[] => true,
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
			super::push(out, 1656);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4686);
							match s {
								[] => true,
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
					super::push(out, 3107);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4687);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξαζω.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 670);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1657);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οαζω.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 671);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -παζω.
pub(crate) fn n672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 672);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1659);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3111);
					match s {
						[] => true,
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
			super::push(out, 1661);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4690);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4691);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6070);
									match s {
										[] => true,
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
							super::push(out, 4692);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7105);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6072);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7106);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4693);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6073);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ραζω.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1662);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1663);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3114);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1664);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1665);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3116);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4695);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6074);
									s.is_empty()
								}
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
			super::push(out, 1666);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3117);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1667);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3118);
					match s {
						[] => true,
						['τ', s @ ..] => {
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
		['φ', s @ ..] => {
			super::push(out, 1668);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3119);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4698);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6075);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7107);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3120);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4699);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6076);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταζω.
pub(crate) fn n674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6077);
									s.is_empty()
								}
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
			super::push(out, 1670);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3123);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1671);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3124);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 4702);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1672);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4703);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6078);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 7109);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7724);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 7110);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1673);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3126);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 4704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1674);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3127);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4705);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 4706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1675);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3129);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 4708);
							match s {
								[] => true,
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
					super::push(out, 3130);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4709);
							match s {
								[] => true,
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
					super::push(out, 3131);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6081);
									s.is_empty()
								}
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
			super::push(out, 1676);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3132);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υαζω.
pub(crate) fn n675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3134);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 6082);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7111);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 7725);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9599(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 7112);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7726);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9600(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 7113);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7727);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9601(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 7114);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7728);
													match s {
														[] => true,
														['κ', s @ ..] => super::t4::n9602(s, out),
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
									super::push(out, 6083);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7115);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7729);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9603(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3135);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1678);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαζω.
pub(crate) fn n676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χαζω.
pub(crate) fn n677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3138);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1681);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3140);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωαζω.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1683);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word τιττυβιζω.
pub(crate) fn n9046(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word λαμπαδιζω.
pub(crate) fn n9047(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -οδιζω.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1702);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 4727);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6090);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7119);
									match s {
										[] => true,
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
							super::push(out, 6091);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7120);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7730);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 8029);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9882(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4728);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4729);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θιζω.
pub(crate) fn n689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3161);
					match s {
						[] => true,
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
			super::push(out, 1704);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3163);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4730);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1705);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3164);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4731);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6093);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7121);
											match s {
												[] => true,
												['α', '\u{1}', s @ ..] => {
													super::push(out, 7731);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1706);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
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
			super::push(out, 1707);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3166);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4732);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6094);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7122);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κιζω.
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3167);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4733);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6095);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3168);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4734);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6096);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3169);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4735);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6097);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7123);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4736);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6098);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1709);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4738);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6099);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7124);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3171);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4739);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6100);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 7125);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4740);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 4741);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6101);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7126);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4742);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6102);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6103);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 7127);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4743);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6104);
									match s {
										[] => true,
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
							super::push(out, 4744);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6105);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7128);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3173);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4745);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6106);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7129);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 7130);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7732);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9605(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ξ', s @ ..] => super::t4::n8111(s, out),
										['ρ', s @ ..] => super::t4::n8112(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t4::n1733(s, out),
		['υ', s @ ..] => super::t4::n1734(s, out),
		_ => false,
	}
}

/// Words ending -λιζω.
pub(crate) fn n691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1712);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3176);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4747);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6108);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 7133);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3178);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4748);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6109);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7134);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7735);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9608(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3179);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4749);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6110);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3181);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4751);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6111);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7135);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7736);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9609(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4752);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6112);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1714);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4753);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6113);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7136);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7737);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9610(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3183);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6114);
									s.is_empty()
								}
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
			super::push(out, 1715);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1716);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3185);
					match s {
						[] => true,
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
			super::push(out, 1717);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
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
			super::push(out, 1718);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3187);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4756);
							match s {
								[] => true,
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
					super::push(out, 3188);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μιζω.
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 692);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1719);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4758);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6115);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7137);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1720);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3190);
					match s {
						[] => true,
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
			super::push(out, 1721);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 3191);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4759);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6116);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1722);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 4760);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6117);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7138);
											match s {
												[] => true,
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
									super::push(out, 6118);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7139);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7738);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9611(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4761);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6119);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7140);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7739);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9612(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3194);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4762);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6120);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1723);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νιζω.
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 693);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1724);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1725);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3196);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4764);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6121);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3197);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4765);
							match s {
								[] => true,
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
					super::push(out, 3198);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4766);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3199);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6123);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 6124);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7141);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4768);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6125);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1726);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 3201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3202);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 4771);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1728);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3203);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4772);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6127);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1729);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1730);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3205);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4773);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6128);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7142);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3206);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4774);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6129);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3207);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4775);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 6130);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7143);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7740);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9613(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3208);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4776);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6131);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 7144);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7741);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9614(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1731);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3209);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4777);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3210);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 4778);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6133);
									match s {
										[] => true,
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
							super::push(out, 4779);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6134);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t4::n1755(s, out),
		_ => false,
	}
}

/// Words ending -θροιζω.
pub(crate) fn n3336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3212);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιζω.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 695);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1734);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 3213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1735);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3214);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4783);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6136);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7146);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1736);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4784);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4785);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6137);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7147);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1737);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3216);
					match s {
						[] => true,
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
			super::push(out, 1738);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3217);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4786);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6138);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3218);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριζω.
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 696);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1740);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3219);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4787);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6139);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3220);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4788);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6140);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3221);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1741);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 4791);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6142);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4793);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6143);
									match s {
										[] => true,
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
							super::push(out, 4794);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6144);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7148);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4795);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6145);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7149);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1743);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3227);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4796);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6146);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7150);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7742);
													match s {
														[] => true,
														['χ', s @ ..] => super::t4::n9615(s, out),
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
									super::push(out, 6147);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7151);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7743);
													s.is_empty()
												}
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
							super::push(out, 4797);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6148);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6149);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7152);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7744);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9616(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1744);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3228);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4798);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6150);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7153);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7745);
													match s {
														[] => true,
														['μ', s @ ..] => super::t4::n9617(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3230);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4799);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6151);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t4::n3355(s, out),
				['φ', s @ ..] => super::t4::n3356(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t4::n3357(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t4::n1770(s, out),
		['φ', s @ ..] => super::t4::n1771(s, out),
		['ω', s @ ..] => super::t4::n1772(s, out),
		_ => false,
	}
}

/// The word σιζω.
pub(crate) fn n1773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1750);
	s.is_empty()
}

/// Words ending -τιζω.
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 698);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3241);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4808);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6157);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7156);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 7747);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9619(s, out),
														['α', s @ ..] => super::t4::n9620(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 7157);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7748);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9621(s, out),
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
									super::push(out, 6158);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7158);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4809);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6159);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7159);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7749);
													match s {
														[] => true,
														['γ', s @ ..] => super::t4::n9622(s, out),
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
									super::push(out, 6160);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7160);
											match s {
												[] => true,
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
									super::push(out, 6161);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7161);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4810);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6162);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7162);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7750);
													match s {
														[] => true,
														['τ', s @ ..] => super::t4::n9623(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3242);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4811);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6163);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7163);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1752);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3244);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4813);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6164);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7164);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3245);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4814);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6165);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t4::n1776(s, out),
		['ι', s @ ..] => {
			super::push(out, 1754);
			match s {
				[] => true,
				['α', s @ ..] => super::t4::n3371(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t4::n1778(s, out),
		['ν', s @ ..] => super::t4::n1779(s, out),
		['ο', s @ ..] => super::t4::n1780(s, out),
		['π', s @ ..] => super::t4::n1781(s, out),
		['ρ', s @ ..] => super::t4::n1782(s, out),
		['σ', s @ ..] => super::t4::n1783(s, out),
		['ω', s @ ..] => super::t4::n1784(s, out),
		_ => false,
	}
}

/// Words ending -φιζω.
pub(crate) fn n699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 699);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1762);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 3258);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4825);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6171);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7168);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1763);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3259);
					match s {
						[] => true,
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
			super::push(out, 1764);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3260);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 6172);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7169);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7753);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9626(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χιζω.
pub(crate) fn n700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1765);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3261);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4827);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6173);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1766);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3262);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6174);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 6175);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7170);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7754);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9627(s, out),
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
									super::push(out, 6176);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7171);
											match s {
												[] => true,
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
									super::push(out, 6177);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7172);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7755);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9628(s, out),
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
									super::push(out, 6178);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7173);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7756);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9629(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3263);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3264);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4829);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6179);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7174);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωιζω.
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αναδιδασκω.
pub(crate) fn n9106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7759);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word γενειασκω.
pub(crate) fn n9107(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ποδιδρασκω.
pub(crate) fn n9108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7760);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8039);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αταγηρασκω.
pub(crate) fn n9109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7761);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 8040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νησκω.
pub(crate) fn n1891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1862);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 3333);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 4873);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6207);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7186);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7762);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4874);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6208);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7187);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3334);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4875);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 7188);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7763);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 8041);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9894(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ισκω.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1863);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3335);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 4876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6210);
									s.is_empty()
								}
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
			super::push(out, 1864);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3336);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1865);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3337);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4878);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6211);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1866);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3338);
					match s {
						[] => true,
						['γ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3339);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4879);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3340);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4880);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6213);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3341);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6214);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 6215);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7189);
											match s {
												[] => true,
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
									super::push(out, 6216);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7190);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θυσκω.
pub(crate) fn n1898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1869);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3342);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσκω.
pub(crate) fn n777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 776);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1870);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3343);
					match s {
						[] => true,
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
			super::push(out, 1871);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3344);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4883);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6217);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 7191);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7764);
													match s {
														[] => true,
														['δ', s @ ..] => super::t4::n9635(s, out),
														_ => false,
													}
												}
												['ν', s @ ..] => {
													super::push(out, 7765);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9636(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 7766);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9637(s, out),
														['ε', s @ ..] => super::t4::n9638(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 7192);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7767);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9639(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3345);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4884);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6218);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7193);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7768);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9640(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1872);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3346);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4885);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4886);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6220);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7194);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3348);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4887);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6221);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word καταδιωκω.
pub(crate) fn n9121(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word παραβαλλω.
pub(crate) fn n9124(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταβαλλω.
pub(crate) fn n9125(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word μεταβαλλω.
pub(crate) fn n9126(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word περιβαλλω.
pub(crate) fn n9127(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αμφιβαλλω.
pub(crate) fn n9128(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -κβαλλω.
pub(crate) fn n3508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3357);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μβαλλω.
pub(crate) fn n3509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3358);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 4902);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 6233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οβαλλω.
pub(crate) fn n3510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3359);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4903);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6234);
					match s {
						[] => true,
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
			super::push(out, 4904);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρβαλλω.
pub(crate) fn n3511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3360);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4905);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6236);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 7204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σβαλλω.
pub(crate) fn n3512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3361);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4906);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 6237);
					match s {
						[] => true,
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
			super::push(out, 4907);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 6238);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δαλλω.
pub(crate) fn n1912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1880);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3363);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4908);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θαλλω.
pub(crate) fn n1913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1881);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3364);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 3365);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4909);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6240);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7206);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ιαλλω.
pub(crate) fn n3517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3366);
	s.is_empty()
}

/// Words ending -καλλω.
pub(crate) fn n1915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1883);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 3367);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -παλλω.
pub(crate) fn n1916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1884);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3368);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3369);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4910);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6241);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταλλω.
pub(crate) fn n1917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1885);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3371);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4911);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6242);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαλλω.
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1886);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 3372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψαλλω.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1887);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ελλω.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3373);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6243);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 6244);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 6245);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ε', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 6246);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7207);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7769);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9641(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3374);
					match s {
						[] => true,
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
			super::push(out, 1890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3375);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4913);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6247);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3376);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4914);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3377);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4915);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6249);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3378);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4916);
							match s {
								[] => true,
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
					super::push(out, 3379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4917);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6250);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7208);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4918);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6251);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7209);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4919);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1892);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιλλω.
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 785);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1893);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3381);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4920);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6253);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1894);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3382);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4921);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6254);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υλλω.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 786);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3384);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επαναλαμβανω.
pub(crate) fn n9901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8172);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word παραλαμβανω.
pub(crate) fn n9902(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταλαμβανω.
pub(crate) fn n9903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8173);
	s.is_empty()
}

/// Words ending -οκαταλαμβανω.
pub(crate) fn n9904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8174);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 8221);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 8234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word μεταλαμβανω.
pub(crate) fn n9905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8175);
	s.is_empty()
}

/// The word προσλαμβανω.
pub(crate) fn n9906(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αφαμαρτανω.
pub(crate) fn n9653(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αναβλαστανω.
pub(crate) fn n9654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8053);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεταλαγχανω.
pub(crate) fn n9655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8054);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παρατυγχανω.
pub(crate) fn n9656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8055);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιτυγχανω.
pub(crate) fn n9657(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -περιτυγχανω.
pub(crate) fn n9658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8056);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word συντυγχανω.
pub(crate) fn n9659(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αποτυγχανω.
pub(crate) fn n9660(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προστυγχανω.
pub(crate) fn n9661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8057);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -βαινω.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1923);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3410);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3411);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4946);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4947);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6279);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 7230);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7789);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 8058);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9912(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4948);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6280);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7231);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4949);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6281);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7232);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3412);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4950);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6282);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 4951);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6283);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7234);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3413);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6284);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6285);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7235);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3414);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6286);
							match s {
								[] => true,
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
					super::push(out, 4954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4955);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 6287);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7236);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 6288);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3415);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4956);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6289);
							match s {
								[] => true,
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
					super::push(out, 4957);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6290);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3416);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4958);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6291);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7237);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3417);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4960);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6293);
							match s {
								[] => true,
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
					super::push(out, 4961);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6294);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7238);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ερδαινω.
pub(crate) fn n5337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4963);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 6296);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εαινω.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1925);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3420);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4964);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6297);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 7239);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θαινω.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1926);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 3421);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6298);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6299);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7240);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιαινω.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1927);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3422);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 3423);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4966);
					match s {
						[] => true,
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
			super::push(out, 3424);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 4967);
					match s {
						[] => true,
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
			super::push(out, 3425);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3426);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4969);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -καινω.
pub(crate) fn n1961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1928);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3427);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3428);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4970);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6300);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 7241);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7790);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 8059);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9913(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λαινω.
pub(crate) fn n1962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3429);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μαινω.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3430);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 4972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 3431);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4973);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6302);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7242);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3432);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6303);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6304);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7243);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7791);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3433);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4977);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3434);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4978);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3435);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3436);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ξαινω.
pub(crate) fn n3596(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -παινω.
pub(crate) fn n1965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1932);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3437);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4981);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6308);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 7244);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3438);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 4982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ραινω.
pub(crate) fn n1966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3439);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 3440);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3441);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4985);
					match s {
						[] => true,
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
			super::push(out, 3442);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 4986);
					match s {
						[] => true,
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
			super::push(out, 3443);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4987);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6311);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7245);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3445);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4988);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3446);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4990);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3447);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σαινω.
pub(crate) fn n1967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1934);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3448);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3449);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4992);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαινω.
pub(crate) fn n1968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1935);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3450);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3451);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4993);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3452);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4994);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6316);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3453);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4995);
					match s {
						[] => true,
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
			super::push(out, 3454);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4996);
					match s {
						[] => true,
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
			super::push(out, 3455);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4997);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6317);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χαινω.
pub(crate) fn n1969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1936);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3457);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥαινω.
pub(crate) fn n1970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1937);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εινω.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 805);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1938);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3459);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3461);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3462);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5001);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6318);
									match s {
										[] => true,
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
							super::push(out, 5002);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6319);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7246);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5003);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5004);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6320);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7247);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7792);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9665(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5006);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6321);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7248);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3464);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5007);
							match s {
								[] => true,
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
					super::push(out, 3465);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5008);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6322);
									match s {
										[] => true,
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
							super::push(out, 5009);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λινω.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 806);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1941);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3467);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5010);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6324);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7249);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πινω.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 807);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1943);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3468);
					match s {
						[] => true,
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
			super::push(out, 1945);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3469);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1946);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3470);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρινω.
pub(crate) fn n809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 808);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3471);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5013);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6325);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 5014);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6326);
									match s {
										[] => true,
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
							super::push(out, 5015);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6327);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7251);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3472);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5016);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6328);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τινω.
pub(crate) fn n810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 809);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1948);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1949);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3473);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word περιτεμνω.
pub(crate) fn n9175(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κατευθυνω.
pub(crate) fn n9176(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -καταπλυνω.
pub(crate) fn n8333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μυνω.
pub(crate) fn n820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1972);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3497);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5036);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξυνω.
pub(crate) fn n821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3498);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3499);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5037);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6340);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυνω.
pub(crate) fn n822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3500);
					match s {
						[] => true,
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
			super::push(out, 1975);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 3501);
					match s {
						[] => true,
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
			super::push(out, 1976);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5038);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 5039);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6341);
									match s {
										[] => true,
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
							super::push(out, 5040);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6342);
									match s {
										[] => true,
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
							super::push(out, 5041);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6343);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7256);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1977);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -συνω.
pub(crate) fn n823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3504);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5042);
							match s {
								[] => true,
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
					super::push(out, 3505);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τυνω.
pub(crate) fn n824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1979);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3506);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5044);
							match s {
								[] => true,
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
					super::push(out, 3507);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1980);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3508);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5046);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 6344);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7257);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φυνω.
pub(crate) fn n825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1981);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυνω.
pub(crate) fn n826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1982);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3511);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1984);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3512);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word στοιχειοω.
pub(crate) fn n9182(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word τεκμηριοω.
pub(crate) fn n9183(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αλλοτριοω.
pub(crate) fn n9184(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word παστιλλοω.
pub(crate) fn n9185(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -κατασκηνοω.
pub(crate) fn n9186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7793);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ανακαινοω.
pub(crate) fn n9187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7794);
	s.is_empty()
}

/// Words ending -ταταρταροω.
pub(crate) fn n9188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7795);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8060);
			match s {
				[] => true,
				['κ', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ελευθεροω.
pub(crate) fn n9189(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -πελευθεροω.
pub(crate) fn n9190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7796);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word συμπληροω.
pub(crate) fn n9191(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -καταπετροω.
pub(crate) fn n9192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7797);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word απολυτροω.
pub(crate) fn n9193(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αυροω.
pub(crate) fn n2099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2065);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3581);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5097);
					match s {
						[] => true,
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
			super::push(out, 3582);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 5098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κυροω.
pub(crate) fn n2101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2067);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πυροω.
pub(crate) fn n2102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2068);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεταμορφοω.
pub(crate) fn n9194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7798);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word παραλειπω.
pub(crate) fn n9199(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταλειπω.
pub(crate) fn n9200(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word περιθαλπω.
pub(crate) fn n9201(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word παραπεμπω.
pub(crate) fn n9202(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word μεταπεμπω.
pub(crate) fn n9203(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -υμπροπεμπω.
pub(crate) fn n9204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7800);
	match s {
		[] => true,
		['ξ', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 8062);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word προσπεμπω.
pub(crate) fn n9205(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αντεγειρω.
pub(crate) fn n8389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7285);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θειρω.
pub(crate) fn n2167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 3655);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 5154);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6403);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κειρω.
pub(crate) fn n2168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2129);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πειρω.
pub(crate) fn n2169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2130);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3656);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 3657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 5155);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6404);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 7287);
									match s {
										[] => true,
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
							super::push(out, 6405);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7288);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7801);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τειρω.
pub(crate) fn n2170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2131);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συναλλασσω.
pub(crate) fn n9210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7802);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word απαλλασσω.
pub(crate) fn n9211(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word υπαλλασσω.
pub(crate) fn n9212(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -παραλλασσω.
pub(crate) fn n9213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7803);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ανταλλασσω.
pub(crate) fn n9214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7804);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πλασσω.
pub(crate) fn n3855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3664);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υλασσω.
pub(crate) fn n3856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3665);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 5161);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μασσω.
pub(crate) fn n2180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2139);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3666);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3667);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 3668);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5163);
					match s {
						[] => true,
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
			super::push(out, 3669);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5164);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 6412);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νασσω.
pub(crate) fn n2181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2140);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3670);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3671);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3672);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 5166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 6413);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 7294);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7805);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 6414);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7295);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7806);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πασσω.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2141);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3673);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3674);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5167);
					match s {
						[] => true,
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
			super::push(out, 3675);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5168);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6415);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρασσω.
pub(crate) fn n2183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3676);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5169);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 5171);
					match s {
						[] => true,
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
			super::push(out, 3677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 3678);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5174);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 5175);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6417);
							match s {
								[] => true,
								['δ', s @ ..] => {
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
				['μ', s @ ..] => {
					super::push(out, 5176);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6418);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 7297);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3680);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τασσω.
pub(crate) fn n2184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2143);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3681);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3682);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5177);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6419);
							match s {
								[] => true,
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
					super::push(out, 5178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5179);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6421);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7298);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5180);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6422);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7299);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3683);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5181);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6423);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3684);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 5182);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6424);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3685);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5183);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 3686);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5184);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6426);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7300);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φασσω.
pub(crate) fn n2185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2144);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3687);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5185);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6427);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 6428);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥασσω.
pub(crate) fn n2186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2145);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3688);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εσσω.
pub(crate) fn n927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 926);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 2146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2147);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3691);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ησσω.
pub(crate) fn n928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 927);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2149);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5188);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6429);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7301);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7807);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9680(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5189);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6430);
									match s {
										[] => true,
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
							super::push(out, 5190);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6431);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7302);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2150);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 3694);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2151);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5192);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6432);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7303);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7808);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9681(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ισσω.
pub(crate) fn n929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 928);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2152);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3696);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3697);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2153);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3701);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5196);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6434);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3702);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5197);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6435);
									match s {
										[] => true,
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
							super::push(out, 5198);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2154);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 3703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υσσω.
pub(crate) fn n930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 929);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2156);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3705);
					match s {
						[] => true,
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
			super::push(out, 2157);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2158);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3707);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5201);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6436);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7304);
											s.is_empty()
										}
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
					super::push(out, 3708);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5202);
							match s {
								[] => true,
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
					super::push(out, 3709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5203);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6437);
									s.is_empty()
								}
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
			super::push(out, 2160);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5205);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6438);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7305);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 5206);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6439);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7306);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2161);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5207);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5208);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωσσω.
pub(crate) fn n931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 930);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2162);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2163);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
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
			super::push(out, 2164);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3714);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5210);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6440);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7307);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ανασκαπτω.
pub(crate) fn n9229(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -επισυναπτω.
pub(crate) fn n9230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7810);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λεπτω.
pub(crate) fn n2220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2179);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3735);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 5226);
					match s {
						[] => true,
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
			super::push(out, 3736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5227);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6451);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7312);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηπτω.
pub(crate) fn n937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιπτω.
pub(crate) fn n938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 2182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3739);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3740);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5228);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6452);
									match s {
										[] => true,
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
							super::push(out, 5229);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6453);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7313);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3741);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5230);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6454);
									match s {
										[] => true,
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
							super::push(out, 5231);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6455);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7314);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3742);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5232);
							match s {
								[] => true,
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
					super::push(out, 3743);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5233);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5234);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 6456);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7315);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 6457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3744);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5235);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6458);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7316);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2183);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3745);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5236);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6459);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3746);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5237);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6460);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7317);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 5238);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6461);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7318);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μπτω.
pub(crate) fn n939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2185);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5239);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6462);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7319);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3748);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οπτω.
pub(crate) fn n940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3749);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3750);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5241);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6463);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3751);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5242);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6464);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3752);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5243);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6465);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7320);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3753);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5244);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6466);
									match s {
										[] => true,
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
							super::push(out, 5245);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6467);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρπτω.
pub(crate) fn n941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 940);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2187);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υπτω.
pub(crate) fn n942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 941);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 2188);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2190);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3756);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 6468);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7321);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7811);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9683(s, out),
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
									super::push(out, 6469);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 7322);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7812);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9684(s, out),
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
									super::push(out, 6470);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7323);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7813);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9685(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2191);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3759);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5247);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6471);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7324);
											match s {
												[] => true,
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
									super::push(out, 6472);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7325);
											match s {
												[] => true,
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
									super::push(out, 6473);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7326);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7814);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9686(s, out),
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
									super::push(out, 6474);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7327);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7815);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9687(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5248);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6475);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6476);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7328);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => super::t4::n5684(s, out),
						['κ', s @ ..] => {
							super::push(out, 5250);
							match s {
								[] => true,
								['ε', s @ ..] => super::t4::n7260(s, out),
								_ => false,
							}
						}
						['ο', s @ ..] => super::t4::n5686(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t4::n2233(s, out),
		_ => false,
	}
}

/// Words ending -ωπτω.
pub(crate) fn n943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 942);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2193);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3761);
					match s {
						[] => true,
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
			super::push(out, 2194);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word διακυβευω.
pub(crate) fn n9255(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word λαμπαδευω.
pub(crate) fn n9256(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ρατοπεδευω.
pub(crate) fn n9257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7820);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 8064);
			match s {
				[] => true,
				['σ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 8222);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 8235);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 8240);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 8241);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n10078(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επιτηδευω.
pub(crate) fn n8477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7342);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιδευω.
pub(crate) fn n2270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2229);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3794);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5279);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6500);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 7343);
									match s {
										[] => true,
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
							super::push(out, 6501);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 7344);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7821);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3795);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5280);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θευω.
pub(crate) fn n965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 964);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2230);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιευω.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 2231);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2232);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3798);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5282);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2233);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5283);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6503);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7345);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κευω.
pub(crate) fn n967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 966);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2234);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5284);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6504);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2235);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λευω.
pub(crate) fn n968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 967);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3802);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5285);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6505);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5286);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6506);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7346);
											match s {
												[] => true,
												['α', 'ι', 'δ', '\u{1}', s @ ..] => {
													super::push(out, 7822);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'τ', 'α', 'π', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2237);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3805);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2238);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3806);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5288);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6507);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2239);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3808);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5290);
							match s {
								[] => true,
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
					super::push(out, 3809);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5291);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 6508);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7347);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7823);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9692(s, out),
														['ο', s @ ..] => super::t4::n9693(s, out),
														['τ', s @ ..] => super::t4::n9694(s, out),
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
									super::push(out, 6509);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7348);
											match s {
												[] => true,
												['ξ', s @ ..] => {
													super::push(out, 7824);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9695(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 7825);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9696(s, out),
														_ => false,
													}
												}
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
							super::push(out, 5292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2241);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 3810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μευω.
pub(crate) fn n969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 968);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3811);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 5293);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6510);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7349);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2243);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3812);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5294);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6511);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7350);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7826);
													match s {
														[] => true,
														['β', s @ ..] => super::t4::n9697(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νευω.
pub(crate) fn n970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 969);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2244);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 2245);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3813);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5295);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6512);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7351);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3814);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5296);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7352);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5297);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3815);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5298);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6515);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2246);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2247);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3817);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5300);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6516);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3818);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5301);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6517);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2248);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3819);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5302);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6518);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7353);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2249);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3820);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5303);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6519);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 7354);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 5304);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6520);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7355);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 7827);
													match s {
														[] => true,
														['ι', s @ ..] => super::t4::n9698(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3821);
					match s {
						[] => true,
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
			super::push(out, 2250);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3822);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2251);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3823);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5306);
							match s {
								[] => true,
								['ι', s @ ..] => super::t4::n7312(s, out),
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
			super::push(out, 2252);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t4::n5752(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξευω.
pub(crate) fn n971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 970);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2253);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3825);
					match s {
						[] => true,
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
			super::push(out, 2254);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πευω.
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 971);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2255);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3827);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5308);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6522);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2256);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5309);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2257);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

