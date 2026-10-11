use flexion::Path;

/// Words ending -кационный.
pub(crate) fn n23532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22398);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30680);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30681);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30682);
					match s {
						[] => true,
						['у', 'м', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33130);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 30683);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33132);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33133);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33489);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33611);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 33134);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33490);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33612);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27600);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32277);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 32278);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32279);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лационный.
pub(crate) fn n23533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мационный.
pub(crate) fn n23534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27602);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30684);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30685);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32281);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33491);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 33492);
											s.is_empty()
										}
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
					super::push(out, 30686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27603);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30687);
					s.is_empty()
				}
				['н', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32282);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27604);
			match s {
				[] => true,
				['о', 'ф', 'н', s @ ..] => {
					match s {
						[] => true,
						['и', 'о', s @ ..] => {
							super::push(out, 32284);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33135);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32285);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 33136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -национный.
pub(crate) fn n23535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22401);
	match s {
		[] => true,
		['г', 'е', s @ ..] => {
			super::push(out, 27605);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30689);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30690);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30692);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30693);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32287);
							s.is_empty()
						}
						['н', 'ы', s @ ..] => {
							super::push(out, 32288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пационный.
pub(crate) fn n23536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22402);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27606);
			s.is_empty()
		}
		['у', 'к', 'к', 'о', '\u{1}', s @ ..] => {
			super::push(out, 27607);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рационный.
pub(crate) fn n23537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22403);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27608);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'и', s @ ..] => {
			match s {
				[] => true,
				['в', 'и', s @ ..] => {
					super::push(out, 30695);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', 'а', 'т', 'с', 'е', 'р', '-', s @ ..] => {
			super::push(out, 27609);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30698);
					match s {
						[] => true,
						['т', 'н', 'и', '\u{1}', s @ ..] => {
							super::push(out, 32290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'м', 'э', s @ ..] => {
					super::push(out, 30699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30700);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'о', 'л', 'г', 'а', '-', s @ ..] => {
					super::push(out, 30701);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30702);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33137);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33138);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 33139);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33140);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 33141);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33493);
											match s {
												[] => true,
												['р', s @ ..] => {
													match s {
														[] => true,
														['т', '\u{1}', s @ ..] => super::t11::n35324(s, out),
														['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => super::t11::n35325(s, out),
														_ => false,
													}
												}
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
							super::push(out, 32294);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 33142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27610);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27611);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30704);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 30705);
					match s {
						[] => true,
						['р', 'е', 'п', '-', 'о', 'н', 'т', 'е', 'ч', 'с', '\u{1}', s @ ..] => {
							super::push(out, 32296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'е', 'б', 'а', 'з', s @ ..] => {
			super::push(out, 27612);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', 'н', s @ ..] => {
							super::push(out, 32297);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 33143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32298);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 32299);
							s.is_empty()
						}
						['ю', 'л', 'л', s @ ..] => {
							super::push(out, 32300);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 33144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 30706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27613);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 27614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сационный.
pub(crate) fn n23538(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 27615);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 27616);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27617);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тационный.
pub(crate) fn n23539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22404);
	match s {
		[] => true,
		['а', 'у', s @ ..] => {
			super::push(out, 27618);
			match s {
				[] => true,
				['л', 'п', 'с', 'к', 'э', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32301);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32302);
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
				['в', s @ ..] => {
					super::push(out, 30710);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30711);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'а', 'л', s @ ..] => {
			super::push(out, 27619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32304);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'и', 'л', s @ ..] => {
							super::push(out, 32305);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'р', 'ф', 'н', 'о', 'к', 'т', s @ ..] => {
					super::push(out, 30713);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'о', s @ ..] => {
			super::push(out, 27620);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'у', 'г', s @ ..] => {
			super::push(out, 27621);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27622);
			s.is_empty()
		}
		['ь', 'л', 'у', 'с', 'н', 'о', 'к', '-', s @ ..] => {
			super::push(out, 27623);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туационный.
pub(crate) fn n23540(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27624);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 27625);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 27626);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сорбционный.
pub(crate) fn n16110(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'а', '-', s @ ..] => {
			super::push(out, 22405);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22406);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иционный.
pub(crate) fn n16111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15345);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22407);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27629);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27631);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27632);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 32307);
									match s {
										[] => true,
										['о', 'к', '-', s @ ..] => {
											super::push(out, 33147);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33494);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 32308);
									s.is_empty()
								}
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
			super::push(out, 22408);
			s.is_empty()
		}
		['н', 'у', s @ ..] => {
			super::push(out, 22409);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22410);
			match s {
				[] => true,
				['е', 'п', 'е', 'р', '\u{1}', s @ ..] => {
					super::push(out, 27634);
					s.is_empty()
				}
				['с', 'е', 'в', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30719);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 30720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кционный.
pub(crate) fn n16112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15346);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22411);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27635);
					s.is_empty()
				}
				['р', 'ф', s @ ..] => {
					super::push(out, 27636);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30721);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32309);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'н', s @ ..] => {
							super::push(out, 30722);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 30723);
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
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30724);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 30725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27637);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27638);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30727);
							s.is_empty()
						}
						['р', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32311);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'и', s @ ..] => {
					super::push(out, 27639);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30729);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'г', s @ ..] => {
							super::push(out, 30730);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 27640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'т', 'с', 'е', s @ ..] => {
			super::push(out, 22412);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27642);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30731);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 30732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27643);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30733);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нционный.
pub(crate) fn n16113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15347);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пционный.
pub(crate) fn n16114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15348);
	match s {
		[] => true,
		['у', 'р', 'р', 'о', 'к', '-', s @ ..] => {
			super::push(out, 22414);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рционный.
pub(crate) fn n16115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15349);
	s.is_empty()
}

/// Words ending -уционный.
pub(crate) fn n16116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15350);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22415);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'и', 'т', 'с', 'н', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27646);
					s.is_empty()
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30734);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 30735);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 30736);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 30737);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 30738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юционный.
pub(crate) fn n16117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15351);
	match s {
		[] => true,
		['л', 'о', 'в', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27647);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32317);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 32318);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32319);
									s.is_empty()
								}
								['р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 32320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27648);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 27649);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яционный.
pub(crate) fn n16118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15352);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', 'г', s @ ..] => {
					super::push(out, 27650);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'р', 'о', 'к', 'о', s @ ..] => {
					super::push(out, 27651);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 30740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27652);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 30741);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 30742);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 30743);
							s.is_empty()
						}
						['т', 'н', 'е', 'в', '\u{1}', s @ ..] => {
							super::push(out, 30744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27653);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30745);
							match s {
								[] => true,
								['п', 'а', 'з', s @ ..] => {
									super::push(out, 32321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 30746);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32322);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33148);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 32323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'з', 'и', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 30747);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'л', s @ ..] => {
							super::push(out, 30748);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32325);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27654);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27655);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'о', s @ ..] => {
									super::push(out, 32326);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 33149);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 32327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'ь', s @ ..] => {
							super::push(out, 30750);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 30751);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 30752);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 32329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 27656);
					match s {
						[] => true,
						['н', 'и', 'р', s @ ..] => {
							super::push(out, 30753);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонагревательный.
pub(crate) fn n35159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33496);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 33613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ронагревательный.
pub(crate) fn n35160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33497);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 33614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овательный.
pub(crate) fn n29492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28092);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30929);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 30930);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 30931);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 30932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30933);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32394);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32395);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азывательный.
pub(crate) fn n34007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32397);
	s.is_empty()
}

/// Words ending -мывательный.
pub(crate) fn n32465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30936);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гательный.
pub(crate) fn n24444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23254);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28094);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32399);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32400);
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
				['б', s @ ..] => {
					super::push(out, 30937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28095);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30939);
					match s {
						[] => true,
						['д', 'о', 'р', s @ ..] => {
							super::push(out, 32402);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 30940);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30941);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32403);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33165);
									s.is_empty()
								}
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
			super::push(out, 28096);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30942);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30943);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28097);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 28098);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 28099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дательный.
pub(crate) fn n24445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23255);
	s.is_empty()
}

/// Words ending -жательный.
pub(crate) fn n24446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23256);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 28100);
			match s {
				[] => true,
				['е', 'д', 'о', 'с', 'о', s @ ..] => {
					super::push(out, 30945);
					match s {
						[] => true,
						['л', 'а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 32405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зательный.
pub(crate) fn n24447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23257);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28101);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32406);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 32407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 28102);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 28103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кательный.
pub(crate) fn n24448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23258);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28104);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30947);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 28105);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 28106);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30948);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 30949);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30950);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'п', 'с', 'и', 'е', s @ ..] => {
					super::push(out, 30951);
					match s {
						[] => true,
						['ч', 'о', s @ ..] => {
							super::push(out, 32410);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 33166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 30952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 28107);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30953);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лательный.
pub(crate) fn n24449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23259);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30954);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32412);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 33167);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33498);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 33168);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 33169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 30955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мательный.
pub(crate) fn n24450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23260);
	s.is_empty()
}

/// Words ending -нательный.
pub(crate) fn n24451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23261);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28108);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 28109);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 28110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пательный.
pub(crate) fn n24452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23262);
	match s {
		[] => true,
		['и', 'щ', 'е', 'ш', s @ ..] => {
			super::push(out, 28111);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28112);
			match s {
				[] => true,
				['к', 'о', s @ ..] => {
					super::push(out, 30956);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30957);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30958);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33170);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33171);
									match s {
										[] => true,
										['п', '-', s @ ..] => {
											super::push(out, 33499);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33615);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рательный.
pub(crate) fn n24453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28113);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30959);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30960);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сательный.
pub(crate) fn n24454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23264);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28114);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 28115);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32416);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тательный.
pub(crate) fn n24455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23265);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30961);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 30962);
					match s {
						[] => true,
						['о', 'с', '\u{1}', s @ ..] => {
							super::push(out, 32418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28116);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 28117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хательный.
pub(crate) fn n24456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23266);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 28118);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 28119);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30963);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цательный.
pub(crate) fn n24457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23267);
	match s {
		[] => true,
		['и', 'р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30964);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30965);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 32419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чательный.
pub(crate) fn n24458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23268);
	s.is_empty()
}

/// Words ending -щательный.
pub(crate) fn n24459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23269);
	match s {
		[] => true,
		['е', 'в', 'о', s @ ..] => {
			super::push(out, 28120);
			match s {
				[] => true,
				['и', 'д', 'а', 'р', 'е', s @ ..] => {
					super::push(out, 30967);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етельный.
pub(crate) fn n17330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16485);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 23270);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 23271);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 23272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28121);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ительный.
pub(crate) fn n17331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16486);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23273);
			match s {
				[] => true,
				['е', 'р', 'т', 'о', 'п', 'у', 'е', 'щ', s @ ..] => {
					super::push(out, 28122);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28123);
					s.is_empty()
				}
				['у', 'л', s @ ..] => {
					super::push(out, 28124);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 30969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23274);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', 'д', s @ ..] => {
							super::push(out, 30970);
							s.is_empty()
						}
						['т', 'с', 'д', 'е', 'р', 'п', '-', s @ ..] => {
							super::push(out, 30971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32421);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33172);
											s.is_empty()
										}
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
					super::push(out, 28125);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28126);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', 'т', 'с', 'с', s @ ..] => {
									super::push(out, 32422);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 32423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30972);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 30973);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['а', 'з', 'о', 'с', s @ ..] => {
											super::push(out, 33173);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33500);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 33174);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33501);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28127);
					s.is_empty()
				}
				['я', 'ъ', s @ ..] => {
					super::push(out, 28128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23275);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28129);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30974);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 30975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'в', 'о', s @ ..] => {
					super::push(out, 28130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 23276);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28131);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30976);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 30977);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 32424);
									s.is_empty()
								}
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
			super::push(out, 23277);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28132);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30978);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32425);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33175);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 30979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 28133);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 28134);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'п', 'с', s @ ..] => {
									super::push(out, 32426);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 32427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28135);
					s.is_empty()
				}
				['с', 'и', 'ч', 'ы', s @ ..] => {
					super::push(out, 28136);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28137);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28138);
					match s {
						[] => true,
						['ы', 'р', '\u{1}', s @ ..] => {
							super::push(out, 30980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23279);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 23280);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28139);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30982);
							match s {
								[] => true,
								['х', 'о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33176);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33177);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33502);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'м', 'и', s @ ..] => {
					super::push(out, 28140);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 28141);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30983);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 30984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28142);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28143);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 28144);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30985);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32429);
									s.is_empty()
								}
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
			super::push(out, 23281);
			match s {
				[] => true,
				['р', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30986);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30987);
							match s {
								[] => true,
								['д', 'а', s @ ..] => {
									super::push(out, 32430);
									s.is_empty()
								}
								['з', 'о', 'в', 'о', 'р', s @ ..] => {
									super::push(out, 32431);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33178);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33503);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 32432);
									s.is_empty()
								}
								['т', 'е', 'л', 'о', s @ ..] => {
									match s {
										[] => true,
										['м', 'а', 'с', '\u{1}', s @ ..] => {
											super::push(out, 33179);
											s.is_empty()
										}
										['т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
											super::push(out, 33180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t11::n24472(s, out),
		['р', s @ ..] => super::t11::n24473(s, out),
		['с', s @ ..] => super::t11::n24474(s, out),
		['т', s @ ..] => super::t11::n24475(s, out),
		['ч', s @ ..] => super::t11::n24476(s, out),
		['ш', s @ ..] => super::t11::n24477(s, out),
		['щ', s @ ..] => {
			super::push(out, 23288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отельный.
pub(crate) fn n17332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16487);
	s.is_empty()
}

/// Words ending -ртельный.
pub(crate) fn n17333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16488);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23289);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28167);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стельный.
pub(crate) fn n17334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16489);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 23290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятельный.
pub(crate) fn n17335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16490);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23291);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23292);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23293);
			match s {
				[] => true,
				['л', 'в', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31002);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 31003);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32441);
									s.is_empty()
								}
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
			super::push(out, 23294);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28170);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивавшийся.
pub(crate) fn n24880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23673);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28363);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 28364);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 28365);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 28366);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 28367);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 28368);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31056);
					s.is_empty()
				}
				['е', 'р', s @ ..] => {
					super::push(out, 31057);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32455);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32456);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'р', s @ ..] => {
					super::push(out, 31059);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31060);
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					super::push(out, 31061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 28369);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 28370);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31062);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32458);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', s @ ..] => {
					super::push(out, 31063);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 31064);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32463);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31065);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31066);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32466);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 32467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31067);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31068);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 31069);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 31070);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28371);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 28372);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 28373);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 28374);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 28375);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 28376);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овавшийся.
pub(crate) fn n24881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23674);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31071);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 31072);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 28377);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 28378);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 28379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31075);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32471);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 32472);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31076);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 28380);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31078);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31079);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 31080);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 28381);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31081);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 32475);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 32476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31082);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 32477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 28382);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28383);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 28384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31084);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 32478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 31085);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32479);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 32480);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 32481);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32482);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 32483);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33195);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 33504);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33196);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33505);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32484);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 32485);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33197);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 33198);
									s.is_empty()
								}
								['р', 'о', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33506);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33616);
													s.is_empty()
												}
												['р', 'е', s @ ..] => {
													super::push(out, 33617);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33618);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33619);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 33620);
													s.is_empty()
												}
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
							super::push(out, 32486);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33199);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33200);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33507);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 33201);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 32487);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32488);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32489);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32490);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33204);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33621);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 33622);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 33205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 32491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31086);
					match s {
						[] => true,
						['д', s @ ..] => super::t11::n34107(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t11::n32623(s, out),
				['ф', s @ ..] => super::t11::n32624(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t11::n29800(s, out),
		['т', s @ ..] => super::t11::n29801(s, out),
		['х', s @ ..] => super::t11::n29802(s, out),
		_ => false,
	}
}

/// Words ending -вырвавшийся.
pub(crate) fn n32634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31098);
	s.is_empty()
}

/// Words ending -увавшийся.
pub(crate) fn n24883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23676);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 28390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывавшийся.
pub(crate) fn n24884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23677);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 28391);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31099);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 31100);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31101);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 31102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 28392);
			match s {
				[] => true,
				['о', 'з', s @ ..] => {
					super::push(out, 31103);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							super::push(out, 32499);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33208);
									s.is_empty()
								}
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
			super::push(out, 28393);
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31104);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32500);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 32501);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 32502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', s @ ..] => {
					super::push(out, 31105);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31106);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 31108);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 31109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31110);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31111);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31112);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32509);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 32510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 28394);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 28395);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					super::push(out, 31113);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 31114);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 32512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31115);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31116);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31117);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28396);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 28397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гавшийся.
pub(crate) fn n17890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23678);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23679);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28398);
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 31119);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31120);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 31121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23680);
			match s {
				[] => true,
				['в', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31122);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31123);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33209);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 32515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', s @ ..] => {
							super::push(out, 31124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', s @ ..] => {
			super::push(out, 23681);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', s @ ..] => {
			super::push(out, 23682);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28401);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 28402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23683);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28403);
					s.is_empty()
				}
				['р', 'ы', s @ ..] => {
					super::push(out, 28404);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -давшийся.
pub(crate) fn n17891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17002);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23684);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28405);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31126);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 23685);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23686);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жавшийся.
pub(crate) fn n17892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17003);
	s.is_empty()
}

/// Words ending -завшийся.
pub(crate) fn n17893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17004);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23687);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31129);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31130);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 32516);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33210);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23688);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 31131);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 31132);
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 31133);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31134);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32517);
									s.is_empty()
								}
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
			super::push(out, 23689);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23690);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 23691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кавшийся.
pub(crate) fn n17894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17005);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23692);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31135);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 23693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28408);
					match s {
						[] => true,
						['р', 'а', 'к', 'ы', s @ ..] => {
							super::push(out, 31137);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32518);
									s.is_empty()
								}
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
			super::push(out, 23694);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23695);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23696);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23697);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23698);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28410);
					match s {
						[] => true,
						['м', 'с', 'ы', s @ ..] => {
							super::push(out, 31138);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32519);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28411);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31139);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32520);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 33211);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33508);
													s.is_empty()
												}
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
							super::push(out, 31140);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'т', s @ ..] => {
					super::push(out, 28412);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28413);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23699);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 23700);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28414);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 31143);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32522);
									s.is_empty()
								}
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
			super::push(out, 23701);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23702);
			match s {
				[] => true,
				['м', 'с', s @ ..] => {
					super::push(out, 28416);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 28417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лавшийся.
pub(crate) fn n17895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17006);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23703);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23704);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 23705);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23706);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мавшийся.
pub(crate) fn n17896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17007);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23708);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23709);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23710);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -навшийся.
pub(crate) fn n17897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17008);
	s.is_empty()
}

/// Words ending -павшийся.
pub(crate) fn n17898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17009);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23712);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28421);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 31144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23713);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23714);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28423);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23715);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28426);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 28427);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23716);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23717);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32523);
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									super::push(out, 32524);
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 32525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 31147);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31148);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32526);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33212);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23718);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -равшийся.
pub(crate) fn n17899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17010);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23719);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28429);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 28430);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -савшийся.
pub(crate) fn n17900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17011);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23721);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23722);
			s.is_empty()
		}
		['и', 'п', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28431);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 28432);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23723);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 23724);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28433);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28434);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тавшийся.
pub(crate) fn n17901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17012);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23725);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28435);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31152);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 31153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28436);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 28437);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23726);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23727);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23728);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23729);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28439);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31155);
							match s {
								[] => true,
								['р', 'ы', s @ ..] => {
									super::push(out, 32527);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33213);
											s.is_empty()
										}
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
					super::push(out, 28440);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28441);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 23730);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 23731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28443);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31156);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 32528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 31157);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 31158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23732);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28445);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31159);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32529);
									s.is_empty()
								}
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
			super::push(out, 23733);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 23734);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23735);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 23736);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28447);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хавшийся.
pub(crate) fn n17902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17013);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23737);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28448);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23738);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28450);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28451);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 28452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23739);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23740);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23741);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28455);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31162);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32530);
									s.is_empty()
								}
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
			super::push(out, 23742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чавшийся.
pub(crate) fn n17903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17014);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23743);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23744);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23745);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28456);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28457);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23746);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 23747);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23748);
			match s {
				[] => true,
				['р', 'о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31165);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23749);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28458);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28459);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28460);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 23750);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шавшийся.
pub(crate) fn n17904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17015);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23751);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23752);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23753);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23754);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28464);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28465);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28466);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28467);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23755);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28469);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28470);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щавшийся.
pub(crate) fn n17905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17016);
	s.is_empty()
}

/// Words ending -ыявившийся.
pub(crate) fn n29928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28502);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дившийся.
pub(crate) fn n17914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17025);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23780);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28503);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 28504);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28505);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31191);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31192);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 31193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28506);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28507);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31194);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32539);
									s.is_empty()
								}
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
			super::push(out, 23781);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 23782);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28508);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 23783);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28509);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31196);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 31197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23784);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28511);
					match s {
						[] => true,
						['о', 'в', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32541);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32542);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33215);
											s.is_empty()
										}
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
					super::push(out, 28512);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28513);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31198);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 28514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23785);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28515);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28516);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31199);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28517);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31200);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 32545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 31201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 28518);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23786);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 23787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еившийся.
pub(crate) fn n17915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17026);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жившийся.
pub(crate) fn n17916(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28519);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 31203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23789);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28521);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 28522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 23790);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23791);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23792);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23793);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28524);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28525);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31206);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28526);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 31207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28527);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28528);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 31208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28529);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32547);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32548);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33216);
											s.is_empty()
										}
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
					super::push(out, 28530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31209);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32549);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 31210);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 31211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28531);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 23794);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28533);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зившийся.
pub(crate) fn n17917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17027);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23795);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28534);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31213);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31214);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28535);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 31215);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32551);
									s.is_empty()
								}
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
			super::push(out, 23796);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23797);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28536);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23798);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23799);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28538);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28539);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23800);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28540);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31217);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32552);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32553);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33217);
											s.is_empty()
										}
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
					super::push(out, 28541);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 28542);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лившийся.
pub(crate) fn n17918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17028);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28543);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31219);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31220);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 31221);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32554);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 33218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 31222);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31223);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 31224);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 28544);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31227);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28545);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23801);
			s.is_empty()
		}
		['г', 'у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28546);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28548);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 31231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23802);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28550);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 28551);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31232);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31233);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31234);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32558);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 31235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28553);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28554);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 28555);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31236);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 32559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 23803);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23804);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28557);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28558);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31238);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 31239);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31240);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31241);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28559);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23805);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23806);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23807);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28561);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 23808);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23809);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23810);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31244);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31245);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32561);
									s.is_empty()
								}
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
			super::push(out, 23811);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 23812);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28567);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 23813);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мившийся.
pub(crate) fn n17919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17029);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23814);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23815);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28570);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31246);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28571);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28572);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28574);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 28575);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23816);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 23817);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31249);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31250);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нившийся.
pub(crate) fn n17920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17030);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28576);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28577);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23818);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 28579);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28580);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28581);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28582);
					s.is_empty()
				}
				['ц', 'с', s @ ..] => {
					super::push(out, 28583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28584);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 23819);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 23820);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28586);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 28587);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28588);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31256);
							s.is_empty()
						}
						['к', 'в', '\u{1}', s @ ..] => {
							super::push(out, 31257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28589);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23821);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28591);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 31259);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32564);
									s.is_empty()
								}
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
			super::push(out, 23822);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31260);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31261);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 32565);
									s.is_empty()
								}
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
			super::push(out, 23823);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28592);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28593);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31262);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32566);
									s.is_empty()
								}
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
			super::push(out, 23824);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 28594);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31263);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оившийся.
pub(crate) fn n17921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17031);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23825);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28595);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23826);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23827);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28597);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 31265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28598);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28599);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31266);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28600);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 31267);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пившийся.
pub(crate) fn n17922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17032);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23829);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23830);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23831);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28601);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28602);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31268);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31269);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28603);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31270);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28604);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31271);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 31272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рившийся.
pub(crate) fn n17923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17033);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23832);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28605);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31273);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31274);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 28606);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 28607);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 28608);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31275);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31276);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31277);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32574);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 32575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 31278);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28609);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 28610);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 23833);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 23834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 23835);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28612);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28613);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23836);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28615);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28616);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28617);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28618);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 23837);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23838);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28619);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28620);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'с', 'и', '\u{1}', s @ ..] => {
			super::push(out, 23839);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23840);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28621);
					match s {
						[] => true,
						['о', 'г', 'ы', s @ ..] => {
							super::push(out, 31285);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 31286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31287);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31288);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28622);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31289);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31290);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 32579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28623);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31291);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 31292);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32581);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 32582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31293);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 32583);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33220);
											s.is_empty()
										}
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
					super::push(out, 28624);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28625);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 23841);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28626);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31295);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32584);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33221);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 32585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 28627);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23842);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28630);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 28631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 23843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сившийся.
pub(crate) fn n17924(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23844);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28632);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28633);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23845);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28634);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 28635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23846);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28636);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28637);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28638);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31300);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32586);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 32587);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33222);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 31301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23847);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23848);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тившийся.
pub(crate) fn n17925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17034);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23849);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28640);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28641);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31302);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28642);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 31303);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28643);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31304);
							s.is_empty()
						}
						['к', 'о', 'р', s @ ..] => {
							super::push(out, 31305);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 32590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 31306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32591);
									s.is_empty()
								}
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
			super::push(out, 23850);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28644);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28645);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31308);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 31309);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28646);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31310);
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
				['н', s @ ..] => {
					super::push(out, 28647);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 28648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23851);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28649);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 28650);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28651);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28652);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28653);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31314);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31315);
							match s {
								[] => true,
								['в', 'ы', s @ ..] => {
									super::push(out, 32594);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33223);
											s.is_empty()
										}
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
					super::push(out, 28654);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23852);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28655);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28656);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23853);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28657);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 28658);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 31318);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28659);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 31319);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32596);
									s.is_empty()
								}
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
			super::push(out, 23854);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28660);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 31320);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32597);
									s.is_empty()
								}
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
			super::push(out, 23855);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 23856);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28662);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31321);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 31322);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31323);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фившийся.
pub(crate) fn n17926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17035);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чившийся.
pub(crate) fn n17927(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23858);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28663);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23859);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 23860);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 23861);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31325);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 32599);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 31326);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31327);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28666);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23862);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28667);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31329);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 31330);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 31331);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28668);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23863);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23864);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28670);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28671);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31333);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 31334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23865);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28672);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31335);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 31336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23866);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28673);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28674);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28675);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28676);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28677);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28678);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28679);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31338);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 28680);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 23867);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28681);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 31340);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28682);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31341);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32607);
									s.is_empty()
								}
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
			super::push(out, 23868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шившийся.
pub(crate) fn n17928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17036);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23869);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 23870);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23871);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28684);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28685);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23872);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23873);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 23874);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23875);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28688);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28689);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28690);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28691);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31345);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32608);
									s.is_empty()
								}
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
			super::push(out, 23876);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'н', 'е', 'м', 'у', '\u{1}', s @ ..] => {
			super::push(out, 23877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щившийся.
pub(crate) fn n17929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17037);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23878);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28693);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 23879);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23880);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23881);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28697);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 31347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 23882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ливающийся.
pub(crate) fn n30189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28748);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32621);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 32622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32623);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 32624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32625);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32626);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'а', s @ ..] => {
			super::push(out, 31383);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33230);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33509);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 32627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 31384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32628);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31385);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31386);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 31387);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 31388);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 31389);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 31390);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 31391);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32631);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поривающийся.
pub(crate) fn n34248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32632);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 33232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хивающийся.
pub(crate) fn n30193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28752);
	s.is_empty()
}

/// Words ending -чивающийся.
pub(crate) fn n30194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28753);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31394);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31395);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32633);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 33233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шивающийся.
pub(crate) fn n30195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28754);
	s.is_empty()
}

/// Words ending -щивающийся.
pub(crate) fn n30196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28755);
	s.is_empty()
}

/// Words ending -овающийся.
pub(crate) fn n25170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23948);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 28756);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увающийся.
pub(crate) fn n25171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23949);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 28757);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывающийся.
pub(crate) fn n25172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23950);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 28758);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31397);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31398);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31399);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 28759);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 28760);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 28761);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32635);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 32636);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 32637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', s @ ..] => {
					super::push(out, 31401);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31402);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 31403);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 31404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31405);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 31406);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31407);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31408);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31409);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32643);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 32644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 28762);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 28763);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31410);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32645);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 31411);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 32646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28764);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 28765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гающийся.
pub(crate) fn n17984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17087);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23951);
			s.is_empty()
		}
		['и', 'в', 'д', '\u{1}', s @ ..] => {
			super::push(out, 23952);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23953);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					super::push(out, 28766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'е', s @ ..] => {
			super::push(out, 23954);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28767);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 28768);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дающийся.
pub(crate) fn n17985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17088);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23955);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28769);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23956);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 23957);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28772);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31414);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 31415);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 28773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 23958);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 23959);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 28774);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28775);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28776);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23960);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 23961);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 23962);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28777);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23963);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23964);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 23965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жающийся.
pub(crate) fn n17986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17089);
	s.is_empty()
}

/// Words ending -зающийся.
pub(crate) fn n17987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17090);
	s.is_empty()
}

/// Words ending -кающийся.
pub(crate) fn n17988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17091);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23966);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23967);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23968);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 23969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23970);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28781);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 28782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28783);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31418);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 31419);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23971);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28785);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28787);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 28788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лающийся.
pub(crate) fn n17989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17092);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23972);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23973);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мающийся.
pub(crate) fn n17990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17093);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23975);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23976);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28790);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нающийся.
pub(crate) fn n17991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17094);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 23977);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31420);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 31421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23978);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пающийся.
pub(crate) fn n17992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17095);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23980);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28791);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23981);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28792);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 31423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23982);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23983);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28793);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23984);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 23985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рающийся.
pub(crate) fn n17993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17096);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сающийся.
pub(crate) fn n17994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17097);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23987);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23988);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тающийся.
pub(crate) fn n17995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17098);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23989);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28796);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31424);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 31425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28797);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 28798);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23990);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23991);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23992);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 23993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28800);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31427);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 32650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 31428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28801);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23994);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 23995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хающийся.
pub(crate) fn n17996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17099);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23997);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23998);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 23999);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 24000);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 24001);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чающийся.
pub(crate) fn n17997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17100);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 24002);
			match s {
				[] => true,
				['р', 'о', 'в', '\u{1}', s @ ..] => {
					super::push(out, 28808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 24003);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28809);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28810);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 24004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шающийся.
pub(crate) fn n17998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17101);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			super::push(out, 24005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'л', s @ ..] => {
			super::push(out, 24006);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28813);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щающийся.
pub(crate) fn n17999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17102);
	s.is_empty()
}

/// Words ending -рошествовавший.
pub(crate) fn n34901(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33511);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 33512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сшествовавший.
pub(crate) fn n34306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32688);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ушествовавший.
pub(crate) fn n34307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32689);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 33252);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ществовавший.
pub(crate) fn n33055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31506);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 32690);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33253);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33254);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йствовавший.
pub(crate) fn n30526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29076);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31507);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32691);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нствовавший.
pub(crate) fn n30528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29078);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31509);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31510);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33256);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 33257);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 32693);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 32694);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33258);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 33259);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32695);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 33260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33261);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 32696);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33263);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 33264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31511);
			s.is_empty()
		}
		['о', 'к', s @ ..] => {
			super::push(out, 31512);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 32697);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 33265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 31513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пствовавший.
pub(crate) fn n30529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29079);
	s.is_empty()
}

/// Words ending -рствовавший.
pub(crate) fn n30530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29080);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32698);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 32699);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 32700);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 33266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33267);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33268);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33518);
									s.is_empty()
								}
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
			super::push(out, 31514);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31515);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32701);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 33269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 32702);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31516);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32703);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 32704);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32705);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 32706);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 33273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32707);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 32708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тствовавший.
pub(crate) fn n30531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29081);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 31517);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32709);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 33274);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фствовавший.
pub(crate) fn n30532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29082);
	s.is_empty()
}

/// Words ending -ьствовавший.
pub(crate) fn n30533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29083);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32710);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32711);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32712);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 33275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32713);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33276);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 33277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'х', s @ ..] => {
					super::push(out, 32714);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ировавший.
pub(crate) fn n18998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18062);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29100);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32716);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 32717);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33279);
											s.is_empty()
										}
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
					super::push(out, 29101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 24651);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29102);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31523);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 32718);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33280);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 31524);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32719);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 32720);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 33281);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33520);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 33282);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33521);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29103);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29104);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32721);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 32722);
									s.is_empty()
								}
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
			super::push(out, 24652);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 24653);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29105);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 29106);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29107);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32723);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 32724);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 32725);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33283);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 31525);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31526);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 32727);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 33284);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 32728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29108);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29109);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 24654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29110);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 29111);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							super::push(out, 31528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 24655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29112);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31529);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 31530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29113);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29114);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31531);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31532);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32730);
									s.is_empty()
								}
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
			super::push(out, 24656);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 24657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29115);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29116);
					s.is_empty()
				}
				['и', 'п', 'с', 'а', s @ ..] => {
					super::push(out, 29117);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29119);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31535);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 31536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29120);
					match s {
						[] => true,
						['а', 'м', 'е', s @ ..] => {
							super::push(out, 31537);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 32732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29121);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32733);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 32734);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33285);
											s.is_empty()
										}
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
					super::push(out, 29122);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t11::n25910(s, out),
		['м', s @ ..] => super::t11::n25911(s, out),
		['н', s @ ..] => super::t11::n25912(s, out),
		['п', s @ ..] => super::t11::n25913(s, out),
		['р', s @ ..] => {
			super::push(out, 24662);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29147);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 29148);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31569);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31570);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t11::n25915(s, out),
		['т', s @ ..] => super::t11::n25916(s, out),
		['у', s @ ..] => super::t11::n25917(s, out),
		['ф', s @ ..] => super::t11::n25918(s, out),
		['ц', s @ ..] => super::t11::n25919(s, out),
		['ш', s @ ..] => super::t11::n25920(s, out),
		_ => false,
	}
}

/// Words ending -енерализованный.
pub(crate) fn n35197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33531);
	s.is_empty()
}

/// Words ending -еминерализованный.
pub(crate) fn n35198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33532);
	s.is_empty()
}

/// Words ending -еавторизованный.
pub(crate) fn n33434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31869);
	s.is_empty()
}

/// Words ending -кторизованный.
pub(crate) fn n33435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31870);
	s.is_empty()
}

/// Words ending -тризованный.
pub(crate) fn n31465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29988);
	s.is_empty()
}

/// Words ending -ляризованный.
pub(crate) fn n31466(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'п', 'е', s @ ..] => {
			super::push(out, 31871);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32852);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 31872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьзованный.
pub(crate) fn n22305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21228);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26588);
			match s {
				[] => true,
				['о', 'п', 'с', 'и', 'е', s @ ..] => {
					super::push(out, 29989);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ированный.
pub(crate) fn n22334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21257);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 26616);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30005);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30006);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32855);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 32856);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 33325);
											s.is_empty()
										}
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
					super::push(out, 30007);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31885);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31886);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32857);
									s.is_empty()
								}
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
			super::push(out, 26617);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30008);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31887);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33326);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 33327);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33533);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 33328);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33534);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30009);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32858);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32859);
									match s {
										[] => true,
										['м', 'е', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33535);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 33536);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 32860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 31888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30010);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30011);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 32861);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 32862);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33329);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 33330);
											match s {
												[] => true,
												['о', 'к', 'а', s @ ..] => {
													super::push(out, 33537);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 26618);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30012);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 30013);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32863);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33331);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 32864);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33332);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 32865);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 33333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 31890);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32866);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 32867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30015);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 32868);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33334);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 32869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 30016);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30017);
					match s {
						[] => true,
						['а', s @ ..] => super::t11::n33461(s, out),
						['е', s @ ..] => super::t11::n33462(s, out),
						['и', s @ ..] => super::t11::n33463(s, out),
						['о', s @ ..] => super::t11::n33464(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n31498(s, out),
				['р', s @ ..] => super::t11::n31499(s, out),
				['у', s @ ..] => super::t11::n31500(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t11::n27943(s, out),
		['з', s @ ..] => super::t11::n27944(s, out),
		['и', s @ ..] => super::t11::n27945(s, out),
		['к', s @ ..] => super::t11::n27946(s, out),
		['л', s @ ..] => super::t11::n27947(s, out),
		['м', s @ ..] => super::t11::n27948(s, out),
		['н', s @ ..] => super::t11::n27949(s, out),
		['о', s @ ..] => super::t11::n27950(s, out),
		['п', s @ ..] => super::t11::n27951(s, out),
		['р', s @ ..] => super::t11::n27952(s, out),
		['с', s @ ..] => super::t11::n27953(s, out),
		['т', s @ ..] => super::t11::n27954(s, out),
		['у', s @ ..] => super::t11::n27955(s, out),
		['ф', s @ ..] => super::t11::n27956(s, out),
		['ц', s @ ..] => super::t11::n27957(s, out),
		['ч', s @ ..] => super::t11::n27958(s, out),
		['ш', s @ ..] => super::t11::n27959(s, out),
		_ => false,
	}
}

/// Words ending -ивно-следственный.
pub(crate) fn n35256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33588);
	s.is_empty()
}

/// Words ending -дественный.
pub(crate) fn n28181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26848);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 30205);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33000);
							s.is_empty()
						}
						['т', '-', s @ ..] => {
							super::push(out, 33001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жественный.
pub(crate) fn n28182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26849);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30206);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30207);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32037);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 32038);
					match s {
						[] => true,
						['у', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33439);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 33440);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33589);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33441);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 32039);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 33002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32040);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30209);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32042);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мественный.
pub(crate) fn n28183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26850);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30210);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тественный.
pub(crate) fn n28184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26851);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 30211);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33004);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 33005);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чественный.
pub(crate) fn n28185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26852);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30212);
			match s {
				[] => true,
				['к', 'о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33006);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 33007);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 33444);
									s.is_empty()
								}
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
			super::push(out, 30213);
			match s {
				[] => true,
				['т', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33008);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 33009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30214);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33010);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 33445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шественный.
pub(crate) fn n28186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26853);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30215);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32044);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щественный.
pub(crate) fn n28187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26854);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33012);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 33013);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 33014);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33446);
									s.is_empty()
								}
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
			super::push(out, 30217);
			match s {
				[] => true,
				['в', 'е', s @ ..] => {
					super::push(out, 32046);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30218);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32047);
					match s {
						[] => true,
						['о', 'м', '\u{1}', s @ ..] => {
							super::push(out, 33016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32048);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32049);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иственный.
pub(crate) fn n22626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21537);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26855);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30219);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 30220);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 30221);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30222);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32050);
							match s {
								[] => true,
								['р', 'е', 'в', 'т', '\u{1}', s @ ..] => {
									super::push(out, 33019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 33020);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33021);
									match s {
										[] => true,
										['р', 'и', 'ш', '\u{1}', s @ ..] => {
											super::push(out, 33447);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 33022);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 33448);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32051);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 33023);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 33024);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33449);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йственный.
pub(crate) fn n22627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21538);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26856);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30223);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30224);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32053);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 33025);
									s.is_empty()
								}
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
			super::push(out, 26857);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30225);
					match s {
						[] => true,
						['у', 'о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 33026);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30226);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32054);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32055);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33029);
									s.is_empty()
								}
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
			super::push(out, 26858);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					super::push(out, 30227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26859);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 26860);
			match s {
				[] => true,
				['з', 'о', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32056);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 32057);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32058);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32059);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33031);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33450);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 33032);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 33033);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33034);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 33451);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32060);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32061);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мственный.
pub(crate) fn n22628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21539);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26861);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26862);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26863);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30231);
					match s {
						[] => true,
						['е', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33038);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 33039);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33452);
											s.is_empty()
										}
										['о', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33590);
													s.is_empty()
												}
												['е', 'н', '\u{1}', s @ ..] => {
													super::push(out, 33591);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 33040);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33453);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 33041);
									s.is_empty()
								}
								['и', 'р', 'т', 'у', 'н', 'в', '\u{1}', s @ ..] => {
									super::push(out, 33042);
									s.is_empty()
								}
								['о', 'н', 'з', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 33043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30232);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нственный.
pub(crate) fn n22629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21540);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26865);
			match s {
				[] => true,
				['р', 'т', 'с', 'о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['-', 'о', 'н', 'м', 'е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 32063);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 32064);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33044);
									s.is_empty()
								}
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
			super::push(out, 26866);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30233);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30234);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 30235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26867);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 26868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рственный.
pub(crate) fn n22630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21541);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30236);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30237);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32066);
							s.is_empty()
						}
						['у', 'с', 'о', 'г', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33045);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33454);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33592);
													s.is_empty()
												}
												_ => false,
											}
										}
										['щ', s @ ..] => {
											super::push(out, 33455);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 33593);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 33046);
									match s {
										[] => true,
										['е', 'м', '\u{1}', s @ ..] => {
											super::push(out, 33456);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 33047);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33457);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33594);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 33458);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33048);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 33459);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 33460);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 33595);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 33461);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33596);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30238);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 30239);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 32067);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сственный.
pub(crate) fn n22631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21542);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 26869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тственный.
pub(crate) fn n22632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21543);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26870);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 32068);
							s.is_empty()
						}
						['т', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33050);
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 33051);
									s.is_empty()
								}
								['з', 'е', 'б', '\u{1}', s @ ..] => {
									super::push(out, 33052);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33053);
									s.is_empty()
								}
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
			super::push(out, 26871);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 30240);
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33054);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33055);
									s.is_empty()
								}
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
			super::push(out, 26872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьственный.
pub(crate) fn n22633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21544);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30241);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 32069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30242);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32070);
							match s {
								[] => true,
								['и', 'в', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33597);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 33598);
													match s {
														[] => true,
														['о', s @ ..] => super::t11::n35317(s, out),
														_ => false,
													}
												}
												['е', 'н', 'в', s @ ..] => {
													super::push(out, 33599);
													s.is_empty()
												}
												['ж', s @ ..] => {
													super::push(out, 33600);
													match s {
														[] => true,
														['е', 'м', '\u{1}', s @ ..] => super::t11::n35318(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 33601);
													s.is_empty()
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['в', s @ ..] => super::t11::n35319(s, out),
														['р', s @ ..] => super::t11::n35320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 33462);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33602);
													match s {
														[] => true,
														['к', 'о', 'п', '\u{1}', s @ ..] => super::t11::n35321(s, out),
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
									super::push(out, 33056);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33463);
											s.is_empty()
										}
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
					super::push(out, 30243);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32071);
							match s {
								[] => true,
								['а', 'н', 'е', s @ ..] => {
									super::push(out, 33057);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33464);
											s.is_empty()
										}
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
					super::push(out, 30244);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32072);
							match s {
								[] => true,
								['о', 'д', 'о', 'р', 'п', 'е', s @ ..] => {
									super::push(out, 33058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30245);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 32073);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едмобилизационный.
pub(crate) fn n35322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33650);
	s.is_empty()
}

/// Words ending -демобилизационный.
pub(crate) fn n35323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33651);
	s.is_empty()
}

/// The word трехоперационный.
pub(crate) fn n35324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33652);
	s.is_empty()
}

/// The word четырехоперационный.
pub(crate) fn n35325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33653);
	s.is_empty()
}

/// Words ending -пительный.
pub(crate) fn n24472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23282);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28145);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 28146);
			s.is_empty()
		}
		['о', 'т', s @ ..] => {
			super::push(out, 28147);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 28148);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рительный.
pub(crate) fn n24473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23283);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28149);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 28150);
			s.is_empty()
		}
		['е', 'в', s @ ..] => {
			super::push(out, 28151);
			s.is_empty()
		}
		['з', 'о', s @ ..] => {
			super::push(out, 28152);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 28153);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28154);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30989);
					match s {
						[] => true,
						['т', 'е', s @ ..] => {
							super::push(out, 32433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30990);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32434);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 33181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 28155);
			s.is_empty()
		}
		['у', 'к', s @ ..] => {
			super::push(out, 28156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сительный.
pub(crate) fn n24474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23284);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28157);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30992);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тительный.
pub(crate) fn n24475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23285);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28158);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30994);
					s.is_empty()
				}
				['р', 'в', s @ ..] => {
					super::push(out, 30995);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32435);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28159);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30996);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33183);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30997);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 30998);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33185);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33186);
									s.is_empty()
								}
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
			super::push(out, 28160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чительный.
pub(crate) fn n24476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23286);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28161);
			match s {
				[] => true,
				['н', 'з', 'о', s @ ..] => {
					super::push(out, 30999);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32438);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 32439);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33187);
									s.is_empty()
								}
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
			super::push(out, 28162);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					super::push(out, 31000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шительный.
pub(crate) fn n24477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23287);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28163);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 28164);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28165);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 28166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -доровавшийся.
pub(crate) fn n34107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32492);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 33206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тровавшийся.
pub(crate) fn n32623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31087);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 32493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фровавшийся.
pub(crate) fn n32624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31088);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 32494);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -совавшийся.
pub(crate) fn n29800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31089);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31090);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 31091);
			match s {
				[] => true,
				['р', 'ы', s @ ..] => {
					super::push(out, 32495);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31092);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товавшийся.
pub(crate) fn n29801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28386);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31094);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31095);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 31096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховавшийся.
pub(crate) fn n29802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28387);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лировавший.
pub(crate) fn n25910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24658);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29123);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31539);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 31540);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 29124);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31542);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31543);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29125);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31544);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 31545);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29126);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 29127);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29128);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31546);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 31547);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31548);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33286);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 33287);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33288);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 33522);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32739);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32740);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32741);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мировавший.
pub(crate) fn n25911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24659);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29130);
			s.is_empty()
		}
		['е', 'р', 'п', s @ ..] => {
			super::push(out, 29131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29132);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31551);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31552);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31553);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29133);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29134);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29135);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32744);
							s.is_empty()
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33291);
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33523);
											s.is_empty()
										}
										['р', 'е', s @ ..] => {
											super::push(out, 33524);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33623);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 33292);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 33525);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33293);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33294);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33526);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 33527);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33624);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'т', s @ ..] => {
			super::push(out, 29136);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нировавший.
pub(crate) fn n25912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24660);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					super::push(out, 31556);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 31557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29137);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31558);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32746);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 32747);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 31559);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29138);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29139);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пировавший.
pub(crate) fn n25913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24661);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29140);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 29141);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31562);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 31563);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 31564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29142);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31565);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 29143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31567);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29144);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 29145);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 29146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сировавший.
pub(crate) fn n25915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24663);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29149);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31571);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29150);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31573);
					match s {
						[] => true,
						['б', 'ы', s @ ..] => {
							super::push(out, 32755);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33296);
									s.is_empty()
								}
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
			super::push(out, 29151);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29152);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 29153);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31574);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 31575);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32756);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 33297);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 33298);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 33528);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33625);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 33299);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33529);
											s.is_empty()
										}
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
					super::push(out, 31576);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32757);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 33300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тировавший.
pub(crate) fn n25916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24664);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29154);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31578);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 31579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32758);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29155);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 29156);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 29157);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 29158);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31580);
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					super::push(out, 31581);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 32760);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33301);
									s.is_empty()
								}
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
			super::push(out, 29159);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29160);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31582);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31583);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32762);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'ю', s @ ..] => {
			super::push(out, 29161);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 31584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уировавший.
pub(crate) fn n25917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24665);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 29163);
			match s {
				[] => true,
				['а', 'т', 'ы', s @ ..] => {
					super::push(out, 31585);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фировавший.
pub(crate) fn n25918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24666);
	s.is_empty()
}

/// Words ending -цировавший.
pub(crate) fn n25919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24667);
	s.is_empty()
}

/// Words ending -шировавший.
pub(crate) fn n25920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24668);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29164);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29165);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -андированный.
pub(crate) fn n33461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31891);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 32870);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 32871);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 32872);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33337);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ендированный.
pub(crate) fn n33462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31892);
	match s {
		[] => true,
		['р', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33338);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 33339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -индированный.
pub(crate) fn n33463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31893);
	s.is_empty()
}

/// Words ending -ондированный.
pub(crate) fn n33464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31894);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 32873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одированный.
pub(crate) fn n31498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30018);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 31895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдированный.
pub(crate) fn n31499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30019);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31897);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 31898);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удированный.
pub(crate) fn n31500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30020);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32876);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32878);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 32879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жированный.
pub(crate) fn n27943(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30021);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 30022);
			match s {
				[] => true,
				['р', 'д', s @ ..] => {
					super::push(out, 31899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зированный.
pub(crate) fn n27944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26620);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30024);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31900);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 31901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32881);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32882);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31902);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31903);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 31904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30025);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 30026);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32884);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', 'о', 'л', 'о', s @ ..] => {
					super::push(out, 31905);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32886);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31906);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32887);
							match s {
								[] => true,
								['и', 'ц', 'е', s @ ..] => {
									super::push(out, 33342);
									match s {
										[] => true,
										['п', 'с', 'е', s @ ..] => {
											super::push(out, 33538);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33626);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 33343);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33539);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33627);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 33344);
									match s {
										[] => true,
										['о', 'и', 'ц', 'у', s @ ..] => {
											super::push(out, 33540);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 33345);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 33541);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 33542);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33628);
													s.is_empty()
												}
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
							super::push(out, 32888);
							match s {
								[] => true,
								['б', 'а', 'т', 'с', 'е', s @ ..] => {
									super::push(out, 33346);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33543);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32890);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 33347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 32891);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32892);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33348);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33544);
											s.is_empty()
										}
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
							super::push(out, 32893);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33349);
									s.is_empty()
								}
								['к', 'и', s @ ..] => {
									super::push(out, 33350);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'м', 'о', s @ ..] => {
									super::push(out, 33351);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 33546);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32894);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33352);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											super::push(out, 33547);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'л', s @ ..] => {
							super::push(out, 32895);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32897);
							s.is_empty()
						}
						['е', 'т', s @ ..] => {
							super::push(out, 32898);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 33354);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 33548);
											s.is_empty()
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
								['р', s @ ..] => {
									super::push(out, 33355);
									s.is_empty()
								}
								['т', 'о', s @ ..] => {
									super::push(out, 33356);
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
						['а', 'в', s @ ..] => super::t11::n34530(s, out),
						['е', s @ ..] => {
							super::push(out, 32900);
							s.is_empty()
						}
						['и', s @ ..] => super::t11::n34532(s, out),
						['о', s @ ..] => super::t11::n34533(s, out),
						['р', 'а', 'д', 'н', 'а', 'т', 'с', 'е', s @ ..] => super::t11::n34534(s, out),
						_ => false,
					}
				}
				['х', s @ ..] => super::t11::n33484(s, out),
				_ => false,
			}
		}
		['н', 'е', 'ц', s @ ..] => super::t11::n31507(s, out),
		['о', s @ ..] => super::t11::n31508(s, out),
		_ => false,
	}
}

/// Words ending -иированный.
pub(crate) fn n27945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26621);
	match s {
		[] => true,
		['р', 'т', 'а', 'п', 'е', 'р', 'е', s @ ..] => {
			super::push(out, 30028);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', 'о', 'с', 'с', 'а', 'о', s @ ..] => {
			super::push(out, 30029);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кированный.
pub(crate) fn n27946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26622);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30030);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31912);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30031);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30032);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30033);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32907);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32908);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32909);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33362);
									s.is_empty()
								}
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
			super::push(out, 30034);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32910);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32911);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33363);
									s.is_empty()
								}
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
			super::push(out, 30035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лированный.
pub(crate) fn n27947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26623);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30036);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31914);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31915);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 30037);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31916);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30038);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31918);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 31919);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31920);
					match s {
						[] => true,
						['р', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 32916);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33365);
									s.is_empty()
								}
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
			super::push(out, 30039);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 31921);
					match s {
						[] => true,
						['о', 'р', 'п', 'е', 'н', s @ ..] => {
							super::push(out, 32917);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33366);
									s.is_empty()
								}
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
			super::push(out, 30040);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31922);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 30041);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31923);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31924);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32919);
							match s {
								[] => true,
								['с', 'и', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33552);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 33553);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30042);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31925);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31926);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 31927);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32921);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 33367);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33368);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 33554);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32922);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32923);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31929);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31930);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30043);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 30044);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31931);
					s.is_empty()
				}
				['м', 'и', 'с', s @ ..] => {
					super::push(out, 31932);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32926);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31933);
					match s {
						[] => true,
						['а', 'р', 'г', 'е', s @ ..] => {
							super::push(out, 32927);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мированный.
pub(crate) fn n27948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30045);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 30046);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 30047);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					super::push(out, 31934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30048);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31935);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31936);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33373);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31937);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30049);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30050);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31938);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30051);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31940);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31941);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33375);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 32930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33377);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33378);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33555);
											s.is_empty()
										}
										['р', 'е', s @ ..] => {
											super::push(out, 33556);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33631);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 33379);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 33557);
											match s {
												[] => true,
												['о', s @ ..] => {
													match s {
														[] => true,
														['л', s @ ..] => super::t11::n35326(s, out),
														['р', 'п', 'е', s @ ..] => super::t11::n35327(s, out),
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
									super::push(out, 33380);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33558);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 33559);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33632);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30052);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 30053);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31942);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31943);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нированный.
pub(crate) fn n27949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30054);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32932);
							match s {
								[] => true,
								['з', 'е', s @ ..] => {
									super::push(out, 33381);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33560);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30055);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31944);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32935);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33383);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33384);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33561);
											s.is_empty()
										}
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
					super::push(out, 31945);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30056);
			match s {
				[] => true,
				['д', 'р', 'о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32937);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 33385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31946);
					match s {
						[] => true,
						['п', 'и', 'ц', 'с', 'и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33386);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31947);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32939);
							s.is_empty()
						}
						['о', 'н', 'е', s @ ..] => {
							super::push(out, 32940);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 32941);
							match s {
								[] => true,
								['л', 'а', s @ ..] => {
									super::push(out, 33389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31948);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', 'к', 'а', 'в', 'е', s @ ..] => {
					super::push(out, 31949);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30057);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31950);
					match s {
						[] => true,
						['ц', 'к', 'н', s @ ..] => {
							super::push(out, 32944);
							match s {
								[] => true,
								['а', 'с', 'е', s @ ..] => {
									super::push(out, 33390);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31951);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32945);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 33391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 32946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31952);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31953);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33392);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33393);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33394);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33562);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 33395);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 33563);
											s.is_empty()
										}
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
					super::push(out, 31954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оированный.
pub(crate) fn n27950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26626);
	match s {
		[] => true,
		['р', 'т', '\u{1}', s @ ..] => {
			super::push(out, 30059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пированный.
pub(crate) fn n27951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26627);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 30060);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30061);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31956);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 30062);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31957);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30063);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 30064);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рированный.
pub(crate) fn n27952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26628);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 30066);
			match s {
				[] => true,
				['к', 'е', 'д', 'а', 'з', 'е', s @ ..] => {
					super::push(out, 31959);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 30067);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 30068);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31960);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32950);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 33396);
									s.is_empty()
								}
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
			super::push(out, 30069);
			match s {
				[] => true,
				['п', 'с', 'а', 'е', s @ ..] => {
					super::push(out, 31961);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30070);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30071);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31962);
					match s {
						[] => true,
						['е', 'ц', 'н', 'о', 'к', 'с', s @ ..] => {
							super::push(out, 32952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'и', s @ ..] => {
					super::push(out, 31963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 30072);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31964);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сированный.
pub(crate) fn n27953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26629);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 30073);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31965);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31966);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33397);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33398);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33564);
											s.is_empty()
										}
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
					super::push(out, 31967);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31968);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32955);
							match s {
								[] => true,
								['а', 'б', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33565);
											s.is_empty()
										}
										['с', 'е', s @ ..] => {
											super::push(out, 33566);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33633);
													s.is_empty()
												}
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
							super::push(out, 32956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31969);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32957);
							s.is_empty()
						}
						['п', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33399);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											super::push(out, 33567);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 33400);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33568);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'н', 'а', 'е', s @ ..] => {
					super::push(out, 31970);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30075);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31971);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32959);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 32960);
							s.is_empty()
						}
						['м', '\u{1}', s @ ..] => {
							super::push(out, 32961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31972);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33401);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33569);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 33570);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 33634);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 33402);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33571);
											s.is_empty()
										}
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
					super::push(out, 31973);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 32962);
							match s {
								[] => true,
								['е', 'р', 'с', 'е', s @ ..] => {
									super::push(out, 33403);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 33572);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тированный.
pub(crate) fn n27954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26630);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30077);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30078);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 31975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32964);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 33404);
									s.is_empty()
								}
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
			super::push(out, 30079);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 30080);
			match s {
				[] => true,
				['е', 'ф', s @ ..] => {
					super::push(out, 31976);
					match s {
						[] => true,
						['ф', 'а', '\u{1}', s @ ..] => {
							super::push(out, 32965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30081);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31977);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32967);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 32968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31978);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32969);
							match s {
								[] => true,
								['р', 'о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33635);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 33636);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 33573);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 33574);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 33637);
													match s {
														[] => true,
														['г', s @ ..] => super::t11::n35328(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 33638);
													match s {
														[] => true,
														['р', s @ ..] => super::t11::n35329(s, out),
														_ => false,
													}
												}
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
							super::push(out, 32970);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 33406);
									s.is_empty()
								}
								['г', 'а', s @ ..] => {
									super::push(out, 33407);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 33408);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 33409);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 33575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'т', 'о', 'е', s @ ..] => {
							super::push(out, 32971);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32972);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 33411);
									s.is_empty()
								}
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
			super::push(out, 30082);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 31979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 30083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31980);
					match s {
						[] => true,
						['д', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33412);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33413);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 33414);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33415);
									s.is_empty()
								}
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
			super::push(out, 30084);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31981);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31982);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['н', 'о', s @ ..] => {
									super::push(out, 33416);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 33417);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 33576);
											s.is_empty()
										}
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
					super::push(out, 31983);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32974);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 30085);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31984);
					match s {
						[] => true,
						['в', 'н', 'и', 'о', 'д', s @ ..] => {
							super::push(out, 32976);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32977);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 33419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31985);
					match s {
						[] => true,
						['н', 'г', 'а', 'и', 'д', 'е', s @ ..] => {
							super::push(out, 32979);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 33420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31986);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 31987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30086);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 30087);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t11::n34620(s, out),
						['о', s @ ..] => {
							super::push(out, 32982);
							match s {
								[] => true,
								['к', s @ ..] => super::t11::n35081(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => super::t11::n34622(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уированный.
pub(crate) fn n27955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26631);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 30088);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 30089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фированный.
pub(crate) fn n27956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30090);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30091);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32984);
							s.is_empty()
						}
						['р', 'е', 'п', 'и', 'г', '\u{1}', s @ ..] => {
							super::push(out, 32985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цированный.
pub(crate) fn n27957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26633);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30092);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31988);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 31989);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32986);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33424);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 33425);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 33579);
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
								['б', s @ ..] => {
									super::push(out, 33426);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 33580);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 33427);
									match s {
										[] => true,
										['а', 'г', 'е', s @ ..] => {
											super::push(out, 33581);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33639);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t11::n35330(s, out),
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
									super::push(out, 33428);
									match s {
										[] => true,
										['а', 'в', 'к', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													match s {
														[] => true,
														['н', s @ ..] => super::t11::n35331(s, out),
														['р', s @ ..] => super::t11::n35332(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 33640);
													match s {
														[] => true,
														['к', s @ ..] => super::t11::n35333(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 33641);
													match s {
														[] => true,
														['и', s @ ..] => super::t11::n35334(s, out),
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
									super::push(out, 33429);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', 'в', 'е', s @ ..] => {
											super::push(out, 33582);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33642);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 33583);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['р', 'е', 'в', 'и', 'д', 'о', s @ ..] => {
											super::push(out, 33584);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 33643);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 33585);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 33430);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33586);
											s.is_empty()
										}
										['р', 'е', 'с', 'е', s @ ..] => {
											super::push(out, 33587);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 33644);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'и', '-', s @ ..] => {
							super::push(out, 32987);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 33431);
									s.is_empty()
								}
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
					super::push(out, 31990);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 31991);
					match s {
						[] => true,
						['р', 'е', 'ф', 'ф', 'и', 'д', 'е', s @ ..] => {
							super::push(out, 32988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30093);
			s.is_empty()
		}
		['у', 'д', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31992);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чированный.
pub(crate) fn n27958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26634);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 30094);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шированный.
pub(crate) fn n27959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26635);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30095);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 30096);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 30097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 30098);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -о-правительственный.
pub(crate) fn n35317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33645);
	s.is_empty()
}

/// The word межправительственный.
pub(crate) fn n35318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33646);
	s.is_empty()
}

/// Words ending -воправительственный.
pub(crate) fn n35319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33647);
	s.is_empty()
}

/// Words ending -роправительственный.
pub(crate) fn n35320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33648);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 33662);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word покровительственный.
pub(crate) fn n35321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33649);
	s.is_empty()
}

/// Words ending -ватизированный.
pub(crate) fn n34530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32899);
	match s {
		[] => true,
		['и', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33549);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33550);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 33630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итизированный.
pub(crate) fn n34532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32901);
	match s {
		[] => true,
		['л', 'о', 'п', 'е', s @ ..] => {
			super::push(out, 33357);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отизированный.
pub(crate) fn n34533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32902);
	s.is_empty()
}

/// Words ending -естандартизированный.
pub(crate) fn n34534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32903);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 33358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хизированный.
pub(crate) fn n33484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31907);
	match s {
		[] => true,
		['р', 'а', 'р', 'е', 'и', 'е', s @ ..] => {
			super::push(out, 32904);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цензированный.
pub(crate) fn n31507(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 31908);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 31909);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					super::push(out, 32905);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озированный.
pub(crate) fn n31508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30027);
	s.is_empty()
}

/// Words ending -лоинформированный.
pub(crate) fn n35326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33654);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -епроинформированный.
pub(crate) fn n35327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33655);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 33664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -глоориентированный.
pub(crate) fn n35328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33656);
	s.is_empty()
}

/// Words ending -ртоориентированный.
pub(crate) fn n35329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33657);
	s.is_empty()
}

/// Words ending -альтированный.
pub(crate) fn n34620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32981);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 33421);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 33422);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кольтированный.
pub(crate) fn n35081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33423);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 33578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ультированный.
pub(crate) fn n34622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32983);
	s.is_empty()
}

/// The word негазифицированный.
pub(crate) fn n35330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33658);
	s.is_empty()
}

/// Words ending -неквалифицированный.
pub(crate) fn n35331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33659);
	s.is_empty()
}

/// Words ending -реквалифицированный.
pub(crate) fn n35332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33660);
	s.is_empty()
}

/// Words ending -коквалифицированный.
pub(crate) fn n35333(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 33665);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -исквалифицированный.
pub(crate) fn n35334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33661);
	s.is_empty()
}

