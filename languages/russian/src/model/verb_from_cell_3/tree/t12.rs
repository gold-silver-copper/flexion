use flexion::Path;

/// Words ending -нтируешься.
pub(crate) fn n27588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26037);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30186);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32449);
					s.is_empty()
				}
				['и', 'р', 'о', 'с', s @ ..] => {
					super::push(out, 32450);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33266);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 33267);
							s.is_empty()
						}
						['е', 'ц', s @ ..] => {
							super::push(out, 33268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33518);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 33519);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 33520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33269);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 33270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 32451);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30187);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32452);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33272);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 33522);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 33597);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 33598);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 33599);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 33273);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 32453);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32454);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обаллотируешься.
pub(crate) fn n35268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33276);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 33524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птируешься.
pub(crate) fn n27590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26039);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 30189);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32455);
					match s {
						[] => true,
						['а', 'с', s @ ..] => {
							super::push(out, 33277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30190);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 30191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртируешься.
pub(crate) fn n27591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26040);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32457);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 30192);
			match s {
				[] => true,
				['в', 'н', 'о', s @ ..] => {
					super::push(out, 32459);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 33525);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 33526);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 33600);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30193);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32460);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 32461);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 32462);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -стируешься.
pub(crate) fn n27592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26041);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', 'н', 'и', 'о', s @ ..] => {
					super::push(out, 32464);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 33280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', s @ ..] => {
					super::push(out, 32465);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 30194);
			s.is_empty()
		}
		['о', 'п', 'м', 'о', 'к', 'е', s @ ..] => {
			super::push(out, 30195);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'г', 'е', 'д', 'о', s @ ..] => {
			super::push(out, 30196);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 30197);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 32468);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утируешься.
pub(crate) fn n27593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26042);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 30198);
			match s {
				[] => true,
				['м', 'о', 'к', 'е', s @ ..] => {
					super::push(out, 32469);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтируешься.
pub(crate) fn n27594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26043);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33284);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 33285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['п', 'а', 'т', 'а', 'к', 'с', s @ ..] => {
							super::push(out, 33286);
							s.is_empty()
						}
						['с', 'н', 'о', 'к', 'о', s @ ..] => {
							super::push(out, 33287);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 33527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютируешься.
pub(crate) fn n27595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26044);
	s.is_empty()
}

