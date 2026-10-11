use flexion::Path;

/// Words ending -скать.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1668);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5620);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13042);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13043);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13044);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21727);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13045);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13046);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 30128);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 36022);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t12::n76432(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 13047);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21728);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13048);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13049);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21729);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 21730);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30129);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36023);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76433(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13051);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21731);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1670);
			match s {
				[] => true,
				['л', s @ ..] => super::t12::n6499(s, out),
				['р', s @ ..] => {
					super::push(out, 5624);
					match s {
						[] => true,
						['т', s @ ..] => super::t12::n16967(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t12::n1765(s, out),
		['о', s @ ..] => super::t12::n1766(s, out),
		['р', s @ ..] => super::t12::n1767(s, out),
		['у', s @ ..] => super::t12::n1768(s, out),
		['ы', s @ ..] => super::t12::n1769(s, out),
		['ю', s @ ..] => super::t12::n1770(s, out),
		['я', s @ ..] => super::t12::n1771(s, out),
		_ => false,
	}
}

/// Words ending -ткать.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1678);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5651);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13086);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13087);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1681);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укать.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1682);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1683);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5655);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5656);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5658);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5659);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13091);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 13092);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 21770);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1685);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13093);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5662);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5663);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 13095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5664);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1687);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5666);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 13096);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5667);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1689);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5669);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13098);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1690);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1691);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5670);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21771);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21772);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5671);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13099);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13100);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13101);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13102);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13103);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13104);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 21773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30149);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13106);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21774);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5674);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13107);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t12::n6559(s, out),
				['а', s @ ..] => super::t12::n6560(s, out),
				['о', s @ ..] => super::t12::n6561(s, out),
				['у', s @ ..] => super::t12::n6562(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t12::n1792(s, out),
		['ш', s @ ..] => super::t12::n1793(s, out),
		['я', s @ ..] => super::t12::n1794(s, out),
		_ => false,
	}
}

/// Words ending -цкать.
pub(crate) fn n324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5685);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1698);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 5686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чкать.
pub(crate) fn n325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 321);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13113);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13114);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13115);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13117);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13118);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21778);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1700);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13119);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13120);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкать.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13121);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13122);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13123);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21779);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13124);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5688);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13125);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30152);
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1702);
			match s {
				[] => true,
				['в', s @ ..] => {
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
		['у', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13126);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21780);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5690);
					match s {
						[] => true,
						['у', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1704);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5691);
					match s {
						[] => true,
						['ю', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыкать.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1706);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13127);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5693);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5694);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5695);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 21781);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30153);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5696);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5697);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 13128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5698);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5699);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1709);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5700);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13129);
							match s {
								[] => true,
								['б', 'а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5701);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13130);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5702);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21782);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30154);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1710);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 1711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5703);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5704);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 21783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30155);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30156);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30157);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36030);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76439(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 36031);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t12::n76440(s, out),
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
									super::push(out, 21784);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30158);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30159);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36032);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76441(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 36033);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76442(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30160);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 21785);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30161);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t12::n1811(s, out),
		['н', s @ ..] => super::t12::n1812(s, out),
		['р', s @ ..] => {
			super::push(out, 1715);
			match s {
				[] => true,
				['а', s @ ..] => super::t12::n6609(s, out),
				['б', s @ ..] => super::t12::n6610(s, out),
				['у', s @ ..] => super::t12::n6611(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t12::n1814(s, out),
		['ф', s @ ..] => super::t12::n1815(s, out),
		['х', s @ ..] => super::t12::n1816(s, out),
		['ц', s @ ..] => super::t12::n1817(s, out),
		_ => false,
	}
}

/// Words ending -ькать.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 324);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1720);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 5739);
					match s {
						[] => true,
						['л', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1721);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5740);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 13171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21799);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21800);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30168);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21801);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30169);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30170);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36035);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76444(s, out),
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
									super::push(out, 21802);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30171);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5741);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21803);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21804);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 13173);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1722);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5742);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 13174);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 21805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 13175);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 21806);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5743);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13176);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13177);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30172);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30173);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36036);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76445(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30174);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 36037);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t12::n76446(s, out),
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
									super::push(out, 21807);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t12::n50397(s, out),
										['а', s @ ..] => super::t12::n50398(s, out),
										['о', s @ ..] => super::t12::n50399(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t12::n17159(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t12::n6634(s, out),
				['о', s @ ..] => super::t12::n6635(s, out),
				['я', s @ ..] => super::t12::n6636(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t12::n1821(s, out),
		_ => false,
	}
}

/// Words ending -экать.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 325);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1724);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1725);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5749);
					match s {
						[] => true,
						['а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкать.
pub(crate) fn n330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1727);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13186);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5751);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21809);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30178);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30179);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 21810);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5752);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21811);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 21812);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1729);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5755);
					match s {
						[] => true,
						['ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 5756);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21813);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 21814);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30180);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36039);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76448(s, out),
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
									super::push(out, 21815);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1730);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13189);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13190);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13191);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5759);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 5760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13192);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13193);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13194);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1731);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 5761);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21816);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21817);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5762);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5763);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t12::n32809(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => super::t12::n17194(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t12::n6655(s, out),
				['т', s @ ..] => super::t12::n6656(s, out),
				['ю', s @ ..] => super::t12::n6657(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t12::n1831(s, out),
		_ => false,
	}
}

/// Words ending -якать.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5768);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13199);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13200);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21820);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13201);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21821);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21822);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30182);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1736);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5770);
					match s {
						[] => true,
						['и', 'п', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1737);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5771);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 13203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21824);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21825);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30183);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30184);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 21826);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21827);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30185);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30186);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5773);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1738);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5774);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5775);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5776);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 5777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5778);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13206);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13207);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5779);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5781);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 5783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1739);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1740);
			match s {
				[] => true,
				['б', s @ ..] => super::t12::n6677(s, out),
				['к', s @ ..] => {
					super::push(out, 5785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t12::n17224(s, out),
						['а', s @ ..] => super::t12::n17225(s, out),
						['о', s @ ..] => super::t12::n17226(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t12::n1840(s, out),
		['т', s @ ..] => super::t12::n1841(s, out),
		['у', s @ ..] => super::t12::n1842(s, out),
		_ => false,
	}
}

/// The word зажимать.
pub(crate) fn n32887(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -нажимать.
pub(crate) fn n17311(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21846);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дожимать.
pub(crate) fn n6740(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зимать.
pub(crate) fn n1871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1769);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иимать.
pub(crate) fn n1872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1770);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5824);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нимать.
pub(crate) fn n1873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1771);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5825);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21847);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30193);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21848);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21849);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21850);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30194);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 13248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21851);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5826);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21852);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30196);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5828);
			match s {
				[] => true,
				['о', 'п', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5829);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5830);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5831);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21853);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30197);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 21854);
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21855);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30198);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21856);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5832);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13250);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21858);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 21860);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13251);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5834);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21862);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30200);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5835);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5836);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21863);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30201);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word примать.
pub(crate) fn n1874(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ймать.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1772);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омать.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5837);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5838);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', '\u{1}', s @ ..] => {
							super::push(out, 13253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5839);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21864);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30202);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5840);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5841);
					match s {
						[] => true,
						['р', 'е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5842);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 13254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21865);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30203);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5843);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5846);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1774);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 5847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13256);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13257);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 13258);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21867);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13259);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21868);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13260);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21869);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 21870);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21871);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30204);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умать.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5848);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5849);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5850);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5851);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13261);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5852);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13262);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5853);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13263);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13264);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5854);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13265);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5855);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1776);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5856);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13267);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ымать.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1777);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['з', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5857);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пеленать.
pub(crate) fn n17390(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21873);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21874);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30205);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жнать.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 1793);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5879);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знать.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1794);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1795);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1796);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5880);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1797);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1798);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5883);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5884);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5885);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5886);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13275);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13276);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5887);
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1800);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13277);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13278);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21876);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инать.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1801);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5889);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5891);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5892);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5893);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5895);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5896);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13279);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13280);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21877);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13281);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13282);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5897);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1803);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1804);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21878);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30206);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36041);
													match s {
														[] => true,
														['п', s @ ..] => super::t12::n76449(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30207);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 21879);
									match s {
										[] => true,
										['в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => super::t12::n65316(s, out),
												['о', s @ ..] => super::t12::n65317(s, out),
												_ => false,
											}
										}
										['о', s @ ..] => super::t12::n50476(s, out),
										_ => false,
									}
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => super::t12::n17425(s, out),
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => super::t12::n6846(s, out),
				['у', '\u{1}', s @ ..] => super::t12::n6847(s, out),
				['ы', 'в', '\u{1}', s @ ..] => super::t12::n6848(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t12::n1910(s, out),
		['ч', s @ ..] => super::t12::n1911(s, out),
		_ => false,
	}
}

/// Words ending -йнать.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1807);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13288);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онать.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1808);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5906);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 13290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1809);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 5907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1810);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13291);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13292);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13293);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21882);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнать.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1811);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13294);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13295);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21883);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 13296);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнать.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 1812);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13298);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13299);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13300);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21884);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унать.
pub(crate) fn n360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word процарапать.
pub(crate) fn n50487(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -сцарапать.
pub(crate) fn n33000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21888);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыцарапать.
pub(crate) fn n33001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21889);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36044);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36045);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39645);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -драпать.
pub(crate) fn n6884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5916);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крапать.
pub(crate) fn n6885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5917);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13304);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13305);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13306);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фапать.
pub(crate) fn n1923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1818);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хапать.
pub(crate) fn n1924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1819);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5919);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цапать.
pub(crate) fn n1925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1820);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5920);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5921);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чапать.
pub(crate) fn n1926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1821);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5922);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5923);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5924);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епать.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5926);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21891);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21892);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 30210);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13309);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13310);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21893);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21894);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13311);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13312);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21895);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21896);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13314);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21897);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13315);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13316);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 21898);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 13317);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13318);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21899);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13319);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21901);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30211);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 13320);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13321);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1823);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5930);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t12::n6905(s, out),
				_ => false,
			}
		}
		['щ', s @ ..] => super::t12::n1929(s, out),
		_ => false,
	}
}

/// Words ending -ипать.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 359);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1825);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5938);
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5939);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5940);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 13336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5941);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13337);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 13338);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21906);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5942);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13339);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 13340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5943);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					super::push(out, 5944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1826);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5945);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5946);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5947);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13341);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 5951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13343);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13344);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21907);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1827);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5952);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5953);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5954);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5955);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13348);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5956);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13349);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5957);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13350);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13351);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13352);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 13353);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5959);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13355);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21908);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13356);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5960);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5962);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13358);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21909);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21910);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30213);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йпать.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1829);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5963);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опать.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5965);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5966);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5967);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5968);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5970);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13359);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5971);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5972);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5973);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 5975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13360);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13361);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21911);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21912);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 21913);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30215);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 21914);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13363);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21915);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1833);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13364);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13365);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t12::n1939(s, out),
		['ц', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпать.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13373);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13374);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5986);
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13375);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13376);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13377);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13378);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21917);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 21918);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13379);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21919);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30217);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 21920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13380);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13381);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21921);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спать.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1838);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1840);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5988);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1841);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5989);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1842);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5992);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13382);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5993);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упать.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 364);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5995);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5996);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 13383);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5997);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13385);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5998);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 13386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5999);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13387);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21923);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21924);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6002);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 13391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6003);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21926);
									s.is_empty()
								}
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
			super::push(out, 1845);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6004);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6005);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6006);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21928);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21929);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21930);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 21931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30218);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 21932);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30219);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1846);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 6007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1847);
			match s {
				[] => true,
				['с', s @ ..] => super::t12::n6993(s, out),
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 1848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6009);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6010);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => super::t12::n6998(s, out),
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => super::t12::n7000(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпать.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 1849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6013);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6014);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21940);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 21941);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30221);
											match s {
												[] => true,
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 13405);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21942);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 21943);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30222);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21944);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30223);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6015);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 13406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6017);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13408);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21945);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13409);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6018);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 13410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6019);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13411);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13412);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13413);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21948);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21949);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13414);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13415);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13416);
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 13417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6022);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 13418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6023);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6024);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13420);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21952);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21953);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30225);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юпать.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13421);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13422);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21954);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13423);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13424);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япать.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6026);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1853);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13425);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13426);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13427);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6028);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обобрать.
pub(crate) fn n17689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13431);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вобрать.
pub(crate) fn n7055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6038);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -добрать.
pub(crate) fn n7056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6039);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13432);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13433);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зобрать.
pub(crate) fn n7057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6040);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13434);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -побрать.
pub(crate) fn n7058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6041);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -робрать.
pub(crate) fn n7059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6042);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 13435);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -собрать.
pub(crate) fn n7060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6043);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13436);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30226);
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тобрать.
pub(crate) fn n7061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6044);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21956);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21957);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убрать.
pub(crate) fn n1970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1865);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыбрать.
pub(crate) fn n1971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1866);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -врать.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1867);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1868);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13438);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13439);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21958);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1869);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6046);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1870);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6047);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1871);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6048);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13440);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6050);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13441);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
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

/// Words ending -грать.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6052);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6053);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6054);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6055);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 21959);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30227);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6056);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13443);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1873);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6057);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6058);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6059);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13444);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21960);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 13445);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13446);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6060);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6061);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -драть.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1874);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1875);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1876);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6063);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1877);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6064);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13448);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1878);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6065);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6066);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13449);
							match s {
								[] => true,
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1880);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6067);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6068);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13450);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13451);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13452);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21961);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6070);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6072);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6074);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жрать.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1883);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1884);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1885);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6076);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13454);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1886);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1887);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6077);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирать.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21962);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30228);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 13456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6078);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13457);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30229);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36046);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76454(s, out),
														_ => false,
													}
												}
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
							super::push(out, 13458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21963);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30230);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21964);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6080);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13461);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21965);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13462);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 21967);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6081);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21968);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30231);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6083);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t12::n1994(s, out),
		['д', s @ ..] => super::t12::n1995(s, out),
		['ж', s @ ..] => super::t12::n1996(s, out),
		['з', s @ ..] => super::t12::n1997(s, out),
		['м', s @ ..] => super::t12::n1998(s, out),
		['п', s @ ..] => super::t12::n1999(s, out),
		['с', s @ ..] => super::t12::n2000(s, out),
		['т', s @ ..] => super::t12::n2001(s, out),
		_ => false,
	}
}

/// Words ending -йрать.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['х', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орать.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1898);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6111);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13484);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13485);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21985);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13486);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1901);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 6113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13487);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13488);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13489);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13490);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1902);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6114);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21987);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30235);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6115);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13491);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6116);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13492);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21988);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6118);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 13493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1903);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6119);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13494);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1904);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6120);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 1905);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6123);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6124);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13495);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прать.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1906);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['п', s @ ..] => {
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

/// Words ending -срать.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1908);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6129);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13496);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6131);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13497);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6133);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1914);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -исать.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1922);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1923);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6146);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13506);
							match s {
								[] => true,
								['т', 'и', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1924);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6147);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13508);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6149);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13509);
							match s {
								[] => true,
								['и', 'п', 'е', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1926);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6150);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 13510);
							match s {
								[] => true,
								['о', 'р', 'и', 'х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6151);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6152);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6154);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6155);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6156);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 13512);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21993);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 30236);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36049);
													s.is_empty()
												}
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
							super::push(out, 13513);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13515);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21995);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6159);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13516);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21996);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30237);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word максать.
pub(crate) fn n17916(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оксать.
pub(crate) fn n2034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6161);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мсать.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1930);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13517);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13519);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 13520);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13521);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21997);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осать.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1931);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13522);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13523);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21998);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30238);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21999);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13524);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13525);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13526);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22000);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6163);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6164);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 13527);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6165);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6167);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6168);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6169);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6170);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13530);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13532);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6174);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссать.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1933);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1934);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1935);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6177);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6179);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усать.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6180);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6181);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6182);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6183);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6184);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13537);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22001);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6186);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13539);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысать.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 1938);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясать.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1939);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6187);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13540);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13541);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22002);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30239);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22003);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13542);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13543);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22004);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30240);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 13545);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13546);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22005);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1940);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6188);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13547);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 13548);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13549);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22006);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13550);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13551);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1941);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6189);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13552);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22007);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ахватать.
pub(crate) fn n17996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13557);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 22009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бхватать.
pub(crate) fn n17997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13558);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22010);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дхватать.
pub(crate) fn n17998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13559);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22011);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ехватать.
pub(crate) fn n17999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13560);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 22012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22013);
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -охватать.
pub(crate) fn n18000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13561);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -схватать.
pub(crate) fn n18001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13562);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30241);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30242);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тхватать.
pub(crate) fn n18002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13563);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 22016);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катать.
pub(crate) fn n2048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1943);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6192);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6193);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13564);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6196);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -латать.
pub(crate) fn n2049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1944);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6197);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6198);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6199);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6200);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6201);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6202);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6203);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13567);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6204);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13569);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13570);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратать.
pub(crate) fn n2050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1945);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6206);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13571);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13572);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чатать.
pub(crate) fn n2051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1946);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6207);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22019);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22020);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22022);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30243);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шатать.
pub(crate) fn n2052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1947);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6208);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6209);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 13573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6211);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6212);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13576);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6214);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етать.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1948);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6215);
					match s {
						[] => true,
						['щ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22023);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22024);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30244);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22025);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30245);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30246);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22026);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30247);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1949);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6216);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 13578);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30248);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30249);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 36050);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76456(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30250);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 36051);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76457(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6217);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13579);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 6218);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13580);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 22027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1950);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6219);
					match s {
						[] => true,
						['р', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22028);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22029);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22030);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6220);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13582);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t12::n33369(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => super::t12::n7336(s, out),
				['в', s @ ..] => super::t12::n7337(s, out),
				['д', s @ ..] => super::t12::n7338(s, out),
				['е', s @ ..] => super::t12::n7339(s, out),
				['з', s @ ..] => super::t12::n7340(s, out),
				['и', s @ ..] => super::t12::n7341(s, out),
				['о', s @ ..] => super::t12::n7342(s, out),
				['п', s @ ..] => super::t12::n7343(s, out),
				['с', s @ ..] => super::t12::n7344(s, out),
				['т', s @ ..] => super::t12::n7345(s, out),
				['у', s @ ..] => super::t12::n7346(s, out),
				['ы', s @ ..] => super::t12::n7347(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t12::n2057(s, out),
		['н', s @ ..] => super::t12::n2058(s, out),
		['п', s @ ..] => super::t12::n2059(s, out),
		['р', s @ ..] => super::t12::n2060(s, out),
		['ч', s @ ..] => super::t12::n2061(s, out),
		_ => false,
	}
}

/// Words ending -итать.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1957);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6252);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6253);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13610);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6254);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6255);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6256);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6258);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6259);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6260);
					match s {
						[] => true,
						['о', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22047);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 30256);
											match s {
												[] => true,
												['е', 'н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
		['ч', s @ ..] => {
			super::push(out, 1960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6262);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6263);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6265);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13612);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22049);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6266);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6267);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13613);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30257);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30258);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36054);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76460(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30259);
											match s {
												[] => true,
												['с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30260);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 36055);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76461(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6268);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22050);
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13615);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22051);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22052);
									match s {
										[] => true,
										['е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13616);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22053);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t12::n7385(s, out),
				['т', s @ ..] => {
					super::push(out, 6270);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => super::t12::n18117(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t12::n7387(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктать.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1961);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6272);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22056);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30262);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1962);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6273);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13623);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6274);
					match s {
						[] => true,
						['е', 'щ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6275);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтать.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1963);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13625);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13626);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 13627);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22057);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 22058);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22059);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 22061);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 13629);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13630);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22062);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отать.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1964);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6276);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6277);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22063);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22064);
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['д', 'о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 22065);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22066);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 22067);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 22068);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22069);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 30264);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 30265);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30266);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22070);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22071);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22072);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30267);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6278);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13631);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22073);
									match s {
										[] => true,
										['ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6279);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13632);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22074);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30268);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30269);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1965);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13633);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22075);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6280);
					match s {
						[] => true,
						['о', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6281);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22076);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => super::t12::n7400(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t12::n2071(s, out),
		['л', s @ ..] => super::t12::n2072(s, out),
		['м', s @ ..] => super::t12::n2073(s, out),
		['п', s @ ..] => super::t12::n2074(s, out),
		['р', s @ ..] => super::t12::n2075(s, out),
		['ф', s @ ..] => super::t12::n2076(s, out),
		['х', s @ ..] => super::t12::n2077(s, out),
		['ш', s @ ..] => super::t12::n2078(s, out),
		_ => false,
	}
}

/// Words ending -птать.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 6304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13653);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13654);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13655);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 22113);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 13656);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22114);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1975);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13657);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13658);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13659);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13660);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13661);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13662);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13663);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13665);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22115);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртать.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6308);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стать.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1977);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6309);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13667);
							match s {
								[] => true,
								['р', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30293);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30294);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6310);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 13668);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22116);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22117);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 22118);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30295);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22119);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6312);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13669);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22120);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22121);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 22122);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22123);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30296);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 30297);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22124);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 22125);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30298);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t12::n7436(s, out),
				['ш', s @ ..] => {
					super::push(out, 6315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13678);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13679);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22132);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13680);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22133);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1979);
			match s {
				[] => true,
				['о', s @ ..] => {
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
		['в', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t12::n7439(s, out),
				['и', s @ ..] => super::t12::n7440(s, out),
				['о', 'п', '\u{1}', s @ ..] => super::t12::n7441(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t12::n2086(s, out),
		['е', s @ ..] => super::t12::n2087(s, out),
		['и', s @ ..] => super::t12::n2088(s, out),
		['о', s @ ..] => super::t12::n2089(s, out),
		['р', s @ ..] => super::t12::n2090(s, out),
		['с', s @ ..] => super::t12::n2091(s, out),
		['т', s @ ..] => super::t12::n2092(s, out),
		['у', s @ ..] => super::t12::n2093(s, out),
		['ы', s @ ..] => super::t12::n2094(s, out),
		['ю', s @ ..] => super::t12::n2095(s, out),
		['я', s @ ..] => super::t12::n2096(s, out),
		_ => false,
	}
}

/// Words ending -утать.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6338);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6339);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1993);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6340);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6341);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13702);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13703);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22170);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22171);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13704);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13705);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1994);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6343);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22172);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22173);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30323);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30324);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22174);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22175);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6344);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6345);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6347);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

