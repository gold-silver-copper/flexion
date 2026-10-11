use flexion::Path;

/// Words ending -цаемый.
pub(crate) fn n3002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2896);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7042);
			match s {
				[] => true,
				['н', 'о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20616);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29790);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 29791);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 31762);
													s.is_empty()
												}
												['з', s @ ..] => {
													super::push(out, 31763);
													s.is_empty()
												}
												['к', s @ ..] => {
													super::push(out, 31764);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 31765);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 31766);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 31767);
													s.is_empty()
												}
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
							super::push(out, 20617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чаемый.
pub(crate) fn n3003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2897);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 7044);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7045);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7046);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13577);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 20618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7047);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13579);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13580);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 7048);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шаемый.
pub(crate) fn n3004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2898);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7049);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 7050);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20620);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26242);
									s.is_empty()
								}
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
			super::push(out, 7051);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 13584);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13585);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20621);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щаемый.
pub(crate) fn n3005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2899);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7053);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13587);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20623);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'в', 'з', 'о', s @ ..] => {
					super::push(out, 13588);
					match s {
						[] => true,
						['в', 'е', s @ ..] => {
							super::push(out, 20624);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26244);
									s.is_empty()
								}
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
			super::push(out, 7054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гитимизируемый.
pub(crate) fn n34426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32802);
	s.is_empty()
}

/// Words ending -улируемый.
pub(crate) fn n21692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20651);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26263);
			match s {
				[] => true,
				['т', 'а', 'п', 'о', s @ ..] => {
					super::push(out, 29800);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 31770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мируемый.
pub(crate) fn n14284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13614);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20652);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 20653);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 20654);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31771);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31773);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31774);
									match s {
										[] => true,
										['д', 'е', s @ ..] => {
											super::push(out, 32803);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 33307);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 32804);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31775);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нируемый.
pub(crate) fn n14285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13615);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 20655);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 20656);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 26264);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29802);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 26265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20657);
			s.is_empty()
		}
		['о', 'р', s @ ..] => {
			super::push(out, 20658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пируемый.
pub(crate) fn n14286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13616);
	match s {
		[] => true,
		['и', 'к', s @ ..] => {
			super::push(out, 20659);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 26266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20660);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 26267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сируемый.
pub(crate) fn n14288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13618);
	s.is_empty()
}

/// Words ending -тируемый.
pub(crate) fn n14289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13619);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20662);
			match s {
				[] => true,
				['у', 'л', 'п', 'с', 'к', 'э', 'е', s @ ..] => {
					super::push(out, 26268);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20663);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 20664);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 26269);
					match s {
						[] => true,
						['р', 'о', 'к', 'е', s @ ..] => {
							super::push(out, 29805);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31777);
									s.is_empty()
								}
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
			super::push(out, 20665);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 26270);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26271);
					match s {
						[] => true,
						['м', 'е', 'р', 'е', s @ ..] => {
							super::push(out, 29806);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31778);
									s.is_empty()
								}
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
			super::push(out, 20666);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 20667);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29807);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20668);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 20669);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 26272);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цируемый.
pub(crate) fn n14290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13620);
	s.is_empty()
}

/// Words ending -оруемый.
pub(crate) fn n7400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7075);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -труемый.
pub(crate) fn n7401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7076);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 13622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уруемый.
pub(crate) fn n7402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7077);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 13623);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 13624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фруемый.
pub(crate) fn n7403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7078);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 13625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оуправляемый.
pub(crate) fn n33339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31779);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 32805);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 32806);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етиноподобный.
pub(crate) fn n34431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32807);
	s.is_empty()
}

/// Words ending -листоподобный.
pub(crate) fn n34432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32808);
	s.is_empty()
}

/// The word трехпалубный.
pub(crate) fn n33350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31790);
	s.is_empty()
}

/// Words ending -орегрессивный.
pub(crate) fn n33354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31794);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 32810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идепрессивный.
pub(crate) fn n33355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31795);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 32811);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одепрессивный.
pub(crate) fn n33356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31796);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 32812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -о-репрессивный.
pub(crate) fn n33357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31797);
	s.is_empty()
}

/// Words ending -тивный.
pub(crate) fn n3222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3103);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7367);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13837);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 13838);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13839);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20775);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 26334);
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
						['д', s @ ..] => {
							super::push(out, 20776);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26335);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20777);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 20778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13840);
					match s {
						[] => true,
						['и', 'л', s @ ..] => {
							super::push(out, 20779);
							match s {
								[] => true,
								['п', 'п', s @ ..] => {
									super::push(out, 26336);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29843);
											s.is_empty()
										}
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
					super::push(out, 13841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20780);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 20781);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 20782);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'р', 'о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20783);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 20784);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26339);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26340);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29844);
											s.is_empty()
										}
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
					super::push(out, 13842);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20785);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13843);
					match s {
						[] => true,
						['а', 'п', 'е', s @ ..] => {
							super::push(out, 20787);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 26341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26342);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 26343);
									match s {
										[] => true,
										['е', 'г', 'е', 'д', s @ ..] => {
											super::push(out, 29845);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31798);
													match s {
														[] => true,
														['р', s @ ..] => super::t9::n34437(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', 'о', '-', s @ ..] => {
									super::push(out, 26344);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29846);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 31799);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 26345);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 29847);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 31800);
													s.is_empty()
												}
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
							super::push(out, 20788);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 26346);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 29848);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20789);
							match s {
								[] => true,
								['и', 'л', s @ ..] => {
									super::push(out, 26347);
									match s {
										[] => true,
										['е', 'м', 'о', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 31801);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 31802);
													match s {
														[] => true,
														['г', s @ ..] => super::t9::n34438(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 31803);
													match s {
														[] => true,
														['е', s @ ..] => super::t9::n34439(s, out),
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
									super::push(out, 26348);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 26349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => super::t9::n27651(s, out),
								['н', s @ ..] => {
									super::push(out, 26351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t9::n21841(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t9::n14536(s, out),
				['т', s @ ..] => super::t9::n14537(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t9::n7713(s, out),
		['к', s @ ..] => super::t9::n7714(s, out),
		['н', s @ ..] => super::t9::n7715(s, out),
		['о', s @ ..] => super::t9::n7716(s, out),
		['п', s @ ..] => super::t9::n7717(s, out),
		['р', 'о', s @ ..] => super::t9::n7718(s, out),
		['у', s @ ..] => super::t9::n7719(s, out),
		['я', s @ ..] => super::t9::n7720(s, out),
		_ => false,
	}
}

/// Words ending -упнозвездный.
pub(crate) fn n33372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31812);
	s.is_empty()
}

/// The word краснозвездный.
pub(crate) fn n33373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31813);
	s.is_empty()
}

/// The word четырехзвездный.
pub(crate) fn n33374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31814);
	s.is_empty()
}

/// Words ending -никовидный.
pub(crate) fn n27706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26401);
	s.is_empty()
}

/// Words ending -тиковидный.
pub(crate) fn n27707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26402);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 29875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лковидный.
pub(crate) fn n21918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20860);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 26403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оковидный.
pub(crate) fn n21919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20861);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26404);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тковидный.
pub(crate) fn n21920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20862);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26406);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 29877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чковидный.
pub(crate) fn n21921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20863);
	match s {
		[] => true,
		['о', 'б', s @ ..] => {
			super::push(out, 26408);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 26409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шковидный.
pub(crate) fn n21922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20864);
	match s {
		[] => true,
		['е', 'п', 'е', 'л', '\u{1}', s @ ..] => {
			super::push(out, 26410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ловидный.
pub(crate) fn n14636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13939);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26411);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 26412);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word черноземовидный.
pub(crate) fn n14637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13940);
	s.is_empty()
}

/// Words ending -новидный.
pub(crate) fn n14638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13941);
	s.is_empty()
}

/// Words ending -ровидный.
pub(crate) fn n14639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13942);
	s.is_empty()
}

/// Words ending -совидный.
pub(crate) fn n14640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13943);
	match s {
		[] => true,
		['е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
			super::push(out, 20866);
			s.is_empty()
		}
		['с', 'е', 'л', '\u{1}', s @ ..] => {
			super::push(out, 20867);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 20868);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товидный.
pub(crate) fn n14641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13944);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гидный.
pub(crate) fn n3267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3146);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7449);
			match s {
				[] => true,
				['д', 'ь', 'л', 'а', '\u{1}', s @ ..] => {
					super::push(out, 13945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дидный.
pub(crate) fn n3268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3147);
	s.is_empty()
}

/// Words ending -еидный.
pub(crate) fn n3269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3148);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 7451);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зидный.
pub(crate) fn n3270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3149);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7452);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лидный.
pub(crate) fn n3271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3150);
	s.is_empty()
}

/// Words ending -мидный.
pub(crate) fn n3272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3151);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13948);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13949);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13950);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13951);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7453);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7454);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13952);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нидный.
pub(crate) fn n3273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3152);
	s.is_empty()
}

/// Words ending -оидный.
pub(crate) fn n3274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3153);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', 'м', 'а', '\u{1}', s @ ..] => {
					super::push(out, 13953);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13954);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'и', 'ш', 'м', s @ ..] => {
			super::push(out, 7455);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'г', s @ ..] => {
			super::push(out, 7456);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 7457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13957);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20872);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7458);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20874);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 20875);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20876);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 26415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13958);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20878);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20879);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13959);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13960);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13961);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20880);
							s.is_empty()
						}
						['а', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26417);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26418);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29879);
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
								['д', s @ ..] => {
									super::push(out, 26419);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 26420);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26421);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 29880);
											s.is_empty()
										}
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
					super::push(out, 13963);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7459);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13964);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13965);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7460);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20882);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20883);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20885);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 20886);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20887);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 20888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13967);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 13968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'и', 'л', '\u{1}', s @ ..] => {
			super::push(out, 7461);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7462);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13969);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13970);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20889);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 26422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 20890);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13971);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7463);
			s.is_empty()
		}
		['т', 'а', 'м', 'в', 'е', 'р', 'и', s @ ..] => {
			super::push(out, 7464);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 13972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', 'и', s @ ..] => {
			super::push(out, 7465);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пидный.
pub(crate) fn n3275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3154);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7466);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7467);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7468);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 7469);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ридный.
pub(crate) fn n3276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3155);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7470);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 7471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сидный.
pub(crate) fn n3277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3156);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7472);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7473);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					super::push(out, 13975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тидный.
pub(crate) fn n3278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3157);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7474);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7475);
			match s {
				[] => true,
				['е', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20894);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 20895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фидный.
pub(crate) fn n3279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3158);
	s.is_empty()
}

/// Words ending -хидный.
pub(crate) fn n3280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3159);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7476);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7477);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7478);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7479);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цидный.
pub(crate) fn n3281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3160);
	match s {
		[] => true,
		['а', 'т', s @ ..] => {
			super::push(out, 7480);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7481);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7482);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13979);
					match s {
						[] => true,
						['е', 'м', 'а', '\u{1}', s @ ..] => {
							super::push(out, 20896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтепроводный.
pub(crate) fn n33376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31816);
	s.is_empty()
}

/// Words ending -одопроводный.
pub(crate) fn n33377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31817);
	s.is_empty()
}

/// Words ending -риодный.
pub(crate) fn n7863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7508);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14015);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йодный.
pub(crate) fn n3290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3169);
	s.is_empty()
}

/// Words ending -лодный.
pub(crate) fn n3291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3170);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7509);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14017);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14018);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20920);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20921);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26433);
									s.is_empty()
								}
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
			super::push(out, 7510);
			match s {
				[] => true,
				['о', 'н', s @ ..] => {
					super::push(out, 14020);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20922);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 20923);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -модный.
pub(crate) fn n3292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3171);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7511);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7512);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нодный.
pub(crate) fn n3293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3172);
	match s {
		[] => true,
		['а', 'о', s @ ..] => {
			super::push(out, 7513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -подный.
pub(crate) fn n3294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3173);
	s.is_empty()
}

/// Words ending -родный.
pub(crate) fn n3295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3174);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7514);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7515);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 26436);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 29890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 26437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20924);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 26438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20925);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7516);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14021);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14022);
					match s {
						[] => true,
						['а', 'м', 'у', 'с', '\u{1}', s @ ..] => {
							super::push(out, 20927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7517);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 14023);
					s.is_empty()
				}
				['л', 'г', 'у', 'о', s @ ..] => {
					super::push(out, 14024);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20928);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 20929);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26440);
									s.is_empty()
								}
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
			super::push(out, 7518);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7519);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7520);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14025);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 14026);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20930);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 14027);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20932);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26442);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 26443);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29891);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 20933);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26444);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29892);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20934);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20935);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20936);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20937);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14028);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20938);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26448);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29893);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 29894);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 29895);
											match s {
												[] => true,
												['л', 'г', 'у', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n34447(s, out),
														['-', s @ ..] => super::t9::n34448(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 29896);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29897);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 31818);
													s.is_empty()
												}
												['т', 'с', 'и', s @ ..] => {
													match s {
														[] => true,
														['д', s @ ..] => super::t9::n34449(s, out),
														['р', s @ ..] => super::t9::n34450(s, out),
														['т', s @ ..] => super::t9::n34451(s, out),
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
									super::push(out, 26449);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29898);
											s.is_empty()
										}
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
					super::push(out, 14029);
					match s {
						[] => true,
						['с', 'и', 'к', s @ ..] => super::t9::n22002(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14030);
					match s {
						[] => true,
						['а', 'с', 'е', s @ ..] => {
							super::push(out, 20939);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t9::n14737(s, out),
				['п', s @ ..] => super::t9::n14738(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t9::n7876(s, out),
		['т', s @ ..] => super::t9::n7877(s, out),
		['ю', s @ ..] => super::t9::n7878(s, out),
		_ => false,
	}
}

/// Words ending -тодный.
pub(crate) fn n3296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3175);
	s.is_empty()
}

/// Words ending -ходный.
pub(crate) fn n3297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3176);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7524);
			match s {
				[] => true,
				['р', 'е', 'п', 'е', s @ ..] => {
					super::push(out, 14036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7525);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14037);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 20947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14038);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20948);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26458);
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
							super::push(out, 20949);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14039);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14040);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7527);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7528);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14042);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -равносложный.
pub(crate) fn n33383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31821);
	s.is_empty()
}

/// Words ending -ессудорожный.
pub(crate) fn n33384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31822);
	s.is_empty()
}

/// Words ending -ообразный.
pub(crate) fn n22108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21037);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', 'м', 'а', '\u{1}', s @ ..] => {
					super::push(out, 29924);
					s.is_empty()
				}
				['о', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
					super::push(out, 29925);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 29926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'а', s @ ..] => {
			super::push(out, 26504);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29927);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 26505);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29929);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31827);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29930);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31828);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29931);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31829);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 32829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29932);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31830);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29933);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 29934);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31831);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 31832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 29935);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 29936);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31833);
							match s {
								[] => true,
								['п', 'е', 'л', '\u{1}', s @ ..] => {
									super::push(out, 32832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 26506);
			match s {
				[] => true,
				['а', 'с', s @ ..] => {
					super::push(out, 29937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29938);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 31836);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32833);
									s.is_empty()
								}
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
			super::push(out, 26507);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 26508);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 26509);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29939);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31837);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 32834);
									s.is_empty()
								}
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
			super::push(out, 26510);
			match s {
				[] => true,
				['е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 29940);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29941);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31838);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32835);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29942);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31839);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29943);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31840);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 31841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29944);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31842);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 32838);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 33312);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26511);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29945);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тозный.
pub(crate) fn n3472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3346);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7778);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26523);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29946);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 26524);
									match s {
										[] => true,
										['у', 'н', 'а', 'р', 'г', '\u{1}', s @ ..] => {
											super::push(out, 29947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21067);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26525);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 29948);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29949);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 29950);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 31844);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7779);
			match s {
				[] => true,
				['а', 'л', 'з', s @ ..] => {
					super::push(out, 14249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'е', 'м', 'а', 'к', 'и', 'д', 'е', 'м', 'з', s @ ..] => {
			super::push(out, 7780);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 7781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стийный.
pub(crate) fn n8250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7870);
	match s {
		[] => true,
		['о', 'в', s @ ..] => {
			super::push(out, 14339);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытийный.
pub(crate) fn n8251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7871);
	s.is_empty()
}

/// Words ending -ятийный.
pub(crate) fn n8252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7872);
	match s {
		[] => true,
		['н', 'о', 'п', '-', s @ ..] => {
			super::push(out, 14340);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21108);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хийный.
pub(crate) fn n3530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3400);
	s.is_empty()
}

/// Words ending -томный.
pub(crate) fn n3591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3457);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7952);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7953);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14412);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14413);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14414);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 14415);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21167);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 26556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14416);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 21168);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21169);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14417);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21171);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14418);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14419);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 26558);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 26559);
									s.is_empty()
								}
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
			super::push(out, 7954);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14420);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14421);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21173);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21174);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 21175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7956);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7957);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7958);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14426);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7959);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14428);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21177);
							match s {
								[] => true,
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 26561);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14429);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ванный.
pub(crate) fn n3615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3481);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7990);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7991);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14454);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14455);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7992);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 14456);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7993);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14457);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14458);
					match s {
						[] => true,
						['з', 'и', s @ ..] => {
							super::push(out, 21196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14459);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14460);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21197);
							match s {
								[] => true,
								['м', 'з', s @ ..] => {
									super::push(out, 26564);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29958);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 21198);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 21199);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26565);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 21200);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26566);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29960);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 21201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14461);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14462);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14463);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14464);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21203);
							match s {
								[] => true,
								['о', 'к', 'ы', s @ ..] => {
									super::push(out, 26567);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29961);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 21204);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26568);
									s.is_empty()
								}
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
			super::push(out, 7994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14465);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14466);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21205);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 21206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26569);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14467);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 14468);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14469);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21207);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26571);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 21208);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26572);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29963);
											s.is_empty()
										}
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
					super::push(out, 14470);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7995);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14471);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21210);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 21211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14473);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14474);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21212);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t9::n8387(s, out),
		['р', s @ ..] => {
			super::push(out, 7997);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14491);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21282);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26656);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'а', s @ ..] => {
							super::push(out, 21284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7998);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14493);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 7999);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -данный.
pub(crate) fn n3617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3483);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8009);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8010);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14502);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21297);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21299);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21300);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14505);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14506);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8011);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8012);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14508);
					match s {
						[] => true,
						['р', 'п', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26666);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26667);
									s.is_empty()
								}
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
			super::push(out, 8013);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14509);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26668);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26669);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30121);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8014);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14510);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21303);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26670);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21304);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26671);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 26672);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30122);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30123);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32009);
													s.is_empty()
												}
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
							super::push(out, 21305);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21306);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14511);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21307);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26674);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 21308);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14512);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21309);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26676);
									s.is_empty()
								}
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
			super::push(out, 8015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14513);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14514);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21310);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14515);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21312);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26677);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21313);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14516);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8016);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14517);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14518);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14519);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21316);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21317);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26679);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21318);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14520);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26682);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26683);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30125);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21320);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21321);
							s.is_empty()
						}
						['с', s @ ..] => super::t9::n22401(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t9::n15257(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t9::n8408(s, out),
		['й', s @ ..] => super::t9::n8409(s, out),
		['о', s @ ..] => super::t9::n8410(s, out),
		['р', s @ ..] => super::t9::n8411(s, out),
		['с', s @ ..] => super::t9::n8412(s, out),
		['т', s @ ..] => super::t9::n8413(s, out),
		['ы', s @ ..] => super::t9::n8414(s, out),
		_ => false,
	}
}

/// Words ending -здержанный.
pub(crate) fn n22416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21335);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26697);
			s.is_empty()
		}
		['о', 'в', 'е', s @ ..] => {
			super::push(out, 26698);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдержанный.
pub(crate) fn n22418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21337);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26699);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26700);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдержанный.
pub(crate) fn n22420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21339);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26701);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -занный.
pub(crate) fn n3620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3486);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8028);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14539);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21340);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26702);
									s.is_empty()
								}
								['н', 'з', s @ ..] => {
									super::push(out, 26703);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21341);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21342);
							match s {
								[] => true,
								['д', 'е', s @ ..] => {
									super::push(out, 26704);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30132);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 26705);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30133);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 26706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21343);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26707);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26708);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30134);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 30135);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32012);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 30136);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 32013);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26709);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26710);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26711);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30137);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 21344);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21345);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26712);
									match s {
										[] => true,
										['ш', 'ы', 'в', '\u{1}', s @ ..] => {
											super::push(out, 30138);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21346);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14540);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21347);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21348);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21349);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21350);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26714);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 30139);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32014);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30140);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21351);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26715);
									s.is_empty()
								}
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
			super::push(out, 8029);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21352);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21353);
							match s {
								[] => true,
								['о', 'е', s @ ..] => {
									super::push(out, 26716);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30141);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32015);
													s.is_empty()
												}
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
							super::push(out, 21354);
							s.is_empty()
						}
						['з', 'и', s @ ..] => {
							super::push(out, 21355);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21356);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21357);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26717);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30142);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 21358);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26718);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21359);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14541);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21360);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26720);
									s.is_empty()
								}
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
			super::push(out, 8030);
			match s {
				[] => true,
				['л', 'ы', s @ ..] => {
					super::push(out, 14542);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8031);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14544);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t9::n8423(s, out),
		['я', s @ ..] => super::t9::n8424(s, out),
		_ => false,
	}
}

/// Words ending -ианный.
pub(crate) fn n3621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3487);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8034);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 8035);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 8036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -канный.
pub(crate) fn n3622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8037);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14548);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26724);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26725);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30145);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32016);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 26726);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30146);
											s.is_empty()
										}
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
					super::push(out, 14549);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8038);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 14550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8039);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 8040);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14551);
					s.is_empty()
				}
				['о', 'т', 'ы', s @ ..] => {
					super::push(out, 14552);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8041);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 8042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14553);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14554);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21370);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 21371);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 26727);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30147);
											s.is_empty()
										}
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
					super::push(out, 14555);
					match s {
						[] => true,
						['м', 'с', 'ы', s @ ..] => {
							super::push(out, 21372);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26728);
									s.is_empty()
								}
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
			super::push(out, 8044);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21373);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21374);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26729);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26730);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26731);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30148);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30149);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 26732);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30150);
											s.is_empty()
										}
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
					super::push(out, 14556);
					match s {
						[] => true,
						['л', 'п', 'ы', s @ ..] => {
							super::push(out, 21375);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14557);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21376);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21377);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14558);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21378);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26735);
									s.is_empty()
								}
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
			super::push(out, 8045);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14559);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 21379);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8046);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 8047);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21381);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21382);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26737);
									s.is_empty()
								}
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
			super::push(out, 8048);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 8049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ланный.
pub(crate) fn n3623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8050);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14561);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8051);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8052);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14562);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21384);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21385);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 26738);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21386);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26739);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 26740);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30152);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 32017);
													s.is_empty()
												}
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
							super::push(out, 21387);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21388);
							match s {
								[] => true,
								['о', 'е', s @ ..] => {
									super::push(out, 26741);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21389);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26742);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30154);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30155);
											s.is_empty()
										}
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
					super::push(out, 14563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8053);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14564);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8054);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14565);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8055);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14566);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21392);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21393);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 26743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14567);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 14568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8056);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					super::push(out, 14569);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -манный.
pub(crate) fn n3624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3490);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8058);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8059);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8060);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 14572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8061);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14573);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14574);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8062);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8063);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14576);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21396);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21397);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21398);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21399);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14577);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21400);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 26745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14578);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 21401);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26746);
									s.is_empty()
								}
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
			super::push(out, 8064);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8065);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14580);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14581);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8066);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14582);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21404);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 21405);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30156);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30157);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30158);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21406);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21407);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26748);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21408);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26749);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 26750);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26751);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 30159);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21409);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8067);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нанный.
pub(crate) fn n3625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3491);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 8068);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14586);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14587);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8069);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14589);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26753);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26754);
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
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26755);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30160);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'е', s @ ..] => {
									super::push(out, 26756);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30161);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', 'е', s @ ..] => {
									super::push(out, 26757);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30162);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'о', 'е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 21410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8070);
			match s {
				[] => true,
				['я', 'п', 'а', 'з', 'е', s @ ..] => {
					super::push(out, 14591);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -панный.
pub(crate) fn n3626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3492);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8071);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14592);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14593);
					match s {
						[] => true,
						['а', 'ц', 'ы', s @ ..] => {
							super::push(out, 21413);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26758);
									s.is_empty()
								}
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
			super::push(out, 8072);
			match s {
				[] => true,
				['р', 'т', 'с', 'а', s @ ..] => {
					super::push(out, 14594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8073);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14595);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 14596);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21415);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 14597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8074);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14598);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8075);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21417);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21418);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26760);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 26761);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21419);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26762);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30164);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 32018);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8076);
			match s {
				[] => true,
				['а', 'ш', 'о', s @ ..] => {
					super::push(out, 14600);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 21420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8077);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8078);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 8079);
			match s {
				[] => true,
				['с', 'ы', s @ ..] => {
					super::push(out, 14602);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 8080);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14603);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 21422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ранный.
pub(crate) fn n3627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3493);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8082);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14604);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14606);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14607);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 14608);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21423);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26763);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26764);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30165);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 30166);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21424);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26765);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30167);
											s.is_empty()
										}
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
					super::push(out, 14609);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21425);
							match s {
								[] => true,
								['п', 'е', s @ ..] => {
									super::push(out, 26766);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30168);
											s.is_empty()
										}
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
					super::push(out, 14610);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21426);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 26767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14611);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21427);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14612);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21429);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8084);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 8085);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14614);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 21430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14615);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21431);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26769);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 26770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21432);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 21433);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 21434);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26772);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30169);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 26773);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21435);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21436);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 26774);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 26775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21437);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14616);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14617);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21439);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14618);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21440);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 30171);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32019);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 30172);
											match s {
												[] => true,
												['т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 32020);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14619);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8086);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14620);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 8087);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8088);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14623);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26778);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26779);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26780);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30173);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8089);
			match s {
				[] => true,
				['э', s @ ..] => super::t9::n15368(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14625);
					s.is_empty()
				}
				['т', s @ ..] => super::t9::n15370(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t9::n8482(s, out),
		['с', s @ ..] => super::t9::n8483(s, out),
		['т', s @ ..] => super::t9::n8484(s, out),
		['у', s @ ..] => super::t9::n8485(s, out),
		['ф', s @ ..] => super::t9::n8486(s, out),
		['х', s @ ..] => super::t9::n8487(s, out),
		_ => false,
	}
}

/// Words ending -санный.
pub(crate) fn n3628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3494);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8097);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8098);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21453);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21454);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14634);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21455);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30176);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30177);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8099);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14635);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21456);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 26789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14636);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21457);
							match s {
								[] => true,
								['н', 'е', s @ ..] => {
									super::push(out, 26790);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 30178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 21458);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21459);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26791);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26792);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30179);
											s.is_empty()
										}
										['ш', s @ ..] => {
											super::push(out, 30180);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 26793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21460);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21461);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26794);
									s.is_empty()
								}
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
			super::push(out, 8100);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14637);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8101);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14638);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14639);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21463);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26795);
									s.is_empty()
								}
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
			super::push(out, 8102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14640);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8103);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 8104);
			match s {
				[] => true,
				['л', 'п', 'т', s @ ..] => {
					super::push(out, 14641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танный.
pub(crate) fn n3629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3495);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8105);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8106);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14642);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21466);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21467);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21468);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26797);
									s.is_empty()
								}
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
			super::push(out, 8107);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14643);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21469);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21470);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21471);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21472);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30181);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32021);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 32022);
													match s {
														[] => true,
														['е', s @ ..] => super::t9::n34633(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30182);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 32023);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14644);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21473);
							match s {
								[] => true,
								['д', 'е', s @ ..] => {
									super::push(out, 26798);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30183);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', 'е', s @ ..] => {
									super::push(out, 26799);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30184);
											s.is_empty()
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
								['а', s @ ..] => {
									super::push(out, 26800);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26801);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26802);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26803);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21474);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26804);
									s.is_empty()
								}
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
			super::push(out, 8108);
			match s {
				[] => true,
				['о', 'б', 'з', s @ ..] => {
					super::push(out, 14645);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21475);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 26805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 21476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8109);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14646);
					match s {
						[] => true,
						['ф', 'о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26806);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26807);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 30186);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8110);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14647);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26808);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 26809);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26810);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 26811);
									match s {
										[] => true,
										['о', 'е', s @ ..] => {
											super::push(out, 30187);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32024);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 26812);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30188);
											s.is_empty()
										}
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
					super::push(out, 14648);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21477);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26813);
									s.is_empty()
								}
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
			super::push(out, 8111);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14649);
					s.is_empty()
				}
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21478);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 26814);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 30189);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21479);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26815);
									s.is_empty()
								}
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
			super::push(out, 8112);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14650);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14651);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t9::n8504(s, out),
		['у', s @ ..] => super::t9::n8505(s, out),
		['ш', s @ ..] => {
			super::push(out, 8115);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8116);
			s.is_empty()
		}
		['ь', s @ ..] => super::t9::n8508(s, out),
		['я', s @ ..] => {
			super::push(out, 8118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уанный.
pub(crate) fn n3630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3496);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 8119);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14663);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фанный.
pub(crate) fn n3631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3497);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8120);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8121);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8122);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ханный.
pub(crate) fn n3632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3498);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8123);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14667);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8124);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8125);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14669);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21491);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							super::push(out, 21492);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26822);
									s.is_empty()
								}
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
			super::push(out, 8126);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14670);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 14671);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14672);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14673);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21495);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 26823);
									s.is_empty()
								}
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
			super::push(out, 8127);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14674);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21496);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14675);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21497);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26824);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 26825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8128);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чанный.
pub(crate) fn n3634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8129);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21499);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21500);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26826);
									s.is_empty()
								}
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
			super::push(out, 8130);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14678);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8131);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21502);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21503);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26827);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30191);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8132);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шанный.
pub(crate) fn n3635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3501);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8134);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14681);
					s.is_empty()
				}
				['м', 'с', s @ ..] => {
					super::push(out, 14682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8135);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14683);
					s.is_empty()
				}
				['л', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21505);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26828);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 26829);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21506);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21507);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26830);
									s.is_empty()
								}
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
			super::push(out, 8136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щанный.
pub(crate) fn n3636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3502);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -венный.
pub(crate) fn n3639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3505);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8145);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8146);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 14696);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8147);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14698);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14699);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14700);
					match s {
						[] => true,
						['е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 21516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 14701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8148);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14702);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14703);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21519);
							match s {
								[] => true,
								['у', 'н', 'в', '\u{1}', s @ ..] => {
									super::push(out, 26832);
									s.is_empty()
								}
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
			super::push(out, 8149);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14704);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26833);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30193);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 26834);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 30194);
											s.is_empty()
										}
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
					super::push(out, 14705);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 14706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8150);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14707);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 14708);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21520);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14709);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14710);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21521);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 21522);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21523);
							match s {
								[] => true,
								['о', 'к', 'и', 'р', 'п', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30195);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 21524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14711);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21525);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26836);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 26837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14712);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21527);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 26838);
									s.is_empty()
								}
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
			super::push(out, 8151);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t9::n8543(s, out),
		['ч', s @ ..] => {
			super::push(out, 8153);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14720);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26873);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26874);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30246);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8154);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14721);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -могенный.
pub(crate) fn n15481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14731);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21554);
			match s {
				[] => true,
				['г', 'е', s @ ..] => {
					super::push(out, 26879);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21555);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 26880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ногенный.
pub(crate) fn n15482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14732);
	match s {
		[] => true,
		['а', 'с', s @ ..] => {
			super::push(out, 21556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', s @ ..] => {
			super::push(out, 21557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26882);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -погенный.
pub(crate) fn n15483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14733);
	s.is_empty()
}

/// Words ending -рогенный.
pub(crate) fn n15484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14734);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21558);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 26883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 21559);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26884);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 26885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21560);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21561);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 21562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -согенный.
pub(crate) fn n15485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14735);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21563);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тогенный.
pub(crate) fn n15486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14736);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21564);
			match s {
				[] => true,
				['м', 'е', s @ ..] => {
					super::push(out, 26887);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 30248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 26888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21565);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21566);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26889);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 30249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 26890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 26891);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 26892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21568);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фогенный.
pub(crate) fn n15487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14737);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ргенный.
pub(crate) fn n8550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8159);
	match s {
		[] => true,
		['е', 'л', 'л', 'а', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21571);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26894);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26895);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -денный.
pub(crate) fn n3641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3507);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8160);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8161);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14738);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14739);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26896);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26897);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30252);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8162);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14740);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14741);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 14742);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21573);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26898);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30253);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 26899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14743);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21574);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21575);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26900);
									match s {
										[] => true,
										['р', 'е', s @ ..] => {
											super::push(out, 30254);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32074);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 26901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21576);
							match s {
								[] => true,
								['р', 'п', 'е', s @ ..] => {
									super::push(out, 26902);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 30255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21577);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21578);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14744);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 14745);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14746);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 8163);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14747);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14748);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21581);
							match s {
								[] => true,
								['о', 'п', 'е', s @ ..] => {
									super::push(out, 26904);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21582);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26905);
									match s {
										[] => true,
										['о', 'п', 'е', s @ ..] => {
											super::push(out, 30257);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32075);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 26906);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 26907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14749);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14750);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21584);
							match s {
								[] => true,
								['о', 'в', 'с', s @ ..] => super::t9::n28243(s, out),
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 21585);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => super::t9::n22678(s, out),
						['р', s @ ..] => super::t9::n22679(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t9::n15502(s, out),
				['у', s @ ..] => super::t9::n15503(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t9::n8555(s, out),
		['й', s @ ..] => super::t9::n8556(s, out),
		['л', s @ ..] => super::t9::n8557(s, out),
		['о', s @ ..] => super::t9::n8558(s, out),
		['у', s @ ..] => super::t9::n8559(s, out),
		['ы', s @ ..] => super::t9::n8560(s, out),
		['ю', s @ ..] => super::t9::n8561(s, out),
		['я', s @ ..] => super::t9::n8562(s, out),
		_ => false,
	}
}

/// Words ending -еенный.
pub(crate) fn n3642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3508);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8172);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 14764);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женный.
pub(crate) fn n3643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3509);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8173);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14765);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 14766);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14767);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21604);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14768);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21606);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21607);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 26927);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 21608);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 26928);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30274);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21609);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21610);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21611);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21612);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21613);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14769);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14770);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14771);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21615);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 21616);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30275);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 30276);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32084);
													match s {
														[] => true,
														['б', s @ ..] => super::t9::n34709(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 32085);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'н', s @ ..] => {
											super::push(out, 30277);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 32086);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30278);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 21617);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21618);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26932);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 30279);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 26933);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 26934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21619);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21620);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 21621);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21622);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26937);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30280);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 32087);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14772);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21623);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21624);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 21625);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 26938);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30281);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21626);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 14773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8175);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8176);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14775);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14776);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t9::n15528(s, out),
				['м', s @ ..] => {
					super::push(out, 14778);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14779);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21628);
							s.is_empty()
						}
						['с', s @ ..] => super::t9::n22723(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t9::n15531(s, out),
				['ц', s @ ..] => {
					super::push(out, 14781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t9::n8568(s, out),
		['з', s @ ..] => super::t9::n8569(s, out),
		['и', s @ ..] => super::t9::n8570(s, out),
		['л', s @ ..] => super::t9::n8571(s, out),
		['н', s @ ..] => super::t9::n8572(s, out),
		['о', s @ ..] => super::t9::n8573(s, out),
		['р', s @ ..] => super::t9::n8574(s, out),
		['у', s @ ..] => super::t9::n8575(s, out),
		['ы', s @ ..] => super::t9::n8576(s, out),
		['ю', s @ ..] => super::t9::n8577(s, out),
		['я', s @ ..] => super::t9::n8578(s, out),
		_ => false,
	}
}

/// Words ending -зенный.
pub(crate) fn n3644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3510);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8188);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8189);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14814);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21686);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27001);
									s.is_empty()
								}
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
			super::push(out, 8190);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8191);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иенный.
pub(crate) fn n3645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3511);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8192);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14817);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кенный.
pub(crate) fn n3646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3512);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8193);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ленный.
pub(crate) fn n3647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3513);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8195);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8196);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14818);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21688);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21689);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21690);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 21691);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 27002);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30317);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21692);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14819);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21693);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 27004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 21694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14820);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 14821);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21695);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14822);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21696);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27006);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 27007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21697);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27008);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 27009);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30318);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14823);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 21699);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 21700);
							match s {
								[] => true,
								['а', 'р', 'к', 'а', 'н', 'е', s @ ..] => {
									super::push(out, 27011);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 30319);
											s.is_empty()
										}
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
					super::push(out, 14824);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14825);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21702);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21703);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21704);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14826);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14827);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14828);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21706);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14829);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21707);
							match s {
								[] => true,
								['м', 'з', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30320);
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											super::push(out, 30321);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8197);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14830);
					match s {
						[] => true,
						['л', s @ ..] => super::t9::n22805(s, out),
						['р', s @ ..] => super::t9::n22806(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t9::n15584(s, out),
				['и', s @ ..] => super::t9::n15585(s, out),
				['л', s @ ..] => super::t9::n15586(s, out),
				['м', s @ ..] => super::t9::n15587(s, out),
				['о', s @ ..] => super::t9::n15588(s, out),
				['р', s @ ..] => super::t9::n15589(s, out),
				['у', s @ ..] => super::t9::n15590(s, out),
				['ы', s @ ..] => super::t9::n15591(s, out),
				['ю', s @ ..] => super::t9::n15592(s, out),
				['я', s @ ..] => super::t9::n15593(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t9::n8589(s, out),
		['г', s @ ..] => super::t9::n8590(s, out),
		['д', s @ ..] => super::t9::n8591(s, out),
		['е', s @ ..] => super::t9::n8592(s, out),
		['з', s @ ..] => super::t9::n8593(s, out),
		['и', s @ ..] => super::t9::n8594(s, out),
		['к', s @ ..] => super::t9::n8595(s, out),
		['м', s @ ..] => super::t9::n8596(s, out),
		['о', s @ ..] => super::t9::n8597(s, out),
		['п', s @ ..] => super::t9::n8598(s, out),
		['р', s @ ..] => super::t9::n8599(s, out),
		['с', s @ ..] => super::t10::n8600(s, out),
		['т', s @ ..] => super::t10::n8601(s, out),
		['у', s @ ..] => super::t10::n8602(s, out),
		['ф', s @ ..] => super::t10::n8603(s, out),
		['х', s @ ..] => super::t10::n8604(s, out),
		['ч', s @ ..] => super::t10::n8605(s, out),
		['ш', s @ ..] => super::t10::n8606(s, out),
		['ы', s @ ..] => super::t10::n8607(s, out),
		['я', s @ ..] => super::t10::n8608(s, out),
		_ => false,
	}
}

/// Words ending -менный.
pub(crate) fn n3648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3514);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8218);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14919);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 21865);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21866);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21867);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27180);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 27181);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 27182);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30448);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 27183);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 27184);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30449);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 32158);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30450);
											s.is_empty()
										}
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
					super::push(out, 14920);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14921);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21869);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27185);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27186);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30451);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8219);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8220);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8221);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8222);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14925);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21871);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21872);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27187);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 30452);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 30453);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32159);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30454);
											match s {
												[] => true,
												['в', 'с', 'е', s @ ..] => {
													super::push(out, 32160);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27188);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 30455);
											match s {
												[] => true,
												['а', 'л', 'б', '\u{1}', s @ ..] => {
													super::push(out, 32161);
													s.is_empty()
												}
												['л', s @ ..] => {
													super::push(out, 32162);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 30456);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 32163);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30457);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 32164);
													match s {
														[] => true,
														['о', s @ ..] => super::t10::n34738(s, out),
														_ => false,
													}
												}
												['з', s @ ..] => {
													super::push(out, 32165);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n34739(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 32166);
													match s {
														[] => true,
														['д', s @ ..] => super::t10::n34740(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 30458);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30459);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21873);
							match s {
								[] => true,
								['б', s @ ..] => super::t10::n28538(s, out),
								['п', s @ ..] => super::t10::n28539(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t10::n22978(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t10::n8614(s, out),
		['и', s @ ..] => super::t10::n8615(s, out),
		['й', s @ ..] => super::t10::n8616(s, out),
		['о', s @ ..] => super::t10::n8617(s, out),
		['р', s @ ..] => super::t10::n8618(s, out),
		['с', s @ ..] => super::t10::n8619(s, out),
		['т', s @ ..] => super::t10::n8620(s, out),
		['у', s @ ..] => super::t10::n8621(s, out),
		['ч', s @ ..] => super::t10::n8622(s, out),
		['ы', s @ ..] => super::t10::n8623(s, out),
		['ь', s @ ..] => super::t10::n8624(s, out),
		_ => false,
	}
}

/// Words ending -ненный.
pub(crate) fn n3649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3515);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8234);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14949);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 14950);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 21893);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 27217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14951);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14952);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 14953);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21895);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21896);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21897);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21898);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21899);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21900);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21901);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 21902);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 21903);
							match s {
								[] => true,
								['о', 'е', s @ ..] => {
									super::push(out, 27220);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30473);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21904);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21905);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21906);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27223);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 21907);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27224);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30474);
											match s {
												[] => true,
												['р', 'п', 'с', 'а', 'р', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n34743(s, out),
														['е', s @ ..] => super::t10::n34744(s, out),
														['о', s @ ..] => super::t10::n34745(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30475);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 30476);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 21908);
							match s {
								[] => true,
								['о', 'с', 'о', s @ ..] => {
									super::push(out, 27225);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 30477);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8235);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14956);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27226);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30478);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27227);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30479);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30480);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32173);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 27228);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27229);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30481);
											s.is_empty()
										}
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
					super::push(out, 14957);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27230);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 27231);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27232);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 27233);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30482);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8238);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14959);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 21909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					super::push(out, 14960);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14961);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t10::n8630(s, out),
		['ж', s @ ..] => {
			super::push(out, 8240);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21922);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 21923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t10::n8632(s, out),
		['и', s @ ..] => super::t10::n8633(s, out),
		['л', s @ ..] => super::t10::n8634(s, out),
		['м', s @ ..] => super::t10::n8635(s, out),
		['о', s @ ..] => super::t10::n8636(s, out),
		['п', s @ ..] => {
			super::push(out, 8246);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t10::n8638(s, out),
		['с', s @ ..] => super::t10::n8639(s, out),
		['т', s @ ..] => super::t10::n8640(s, out),
		['ч', s @ ..] => {
			super::push(out, 8250);
			match s {
				[] => true,
				['о', s @ ..] => super::t10::n15752(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t10::n8642(s, out),
		['я', s @ ..] => super::t10::n8643(s, out),
		_ => false,
	}
}

/// Words ending -оенный.
pub(crate) fn n3650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3516);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14998);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14999);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21975);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21976);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21977);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 21978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15000);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 21979);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 21980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15001);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15002);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21982);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15003);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8254);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 15005);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8255);
			match s {
				[] => true,
				['о', 'п', 'с', 'е', 'б', 'о', s @ ..] => {
					super::push(out, 15006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8256);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8257);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8258);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15009);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 21985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8259);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15010);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21986);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21987);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15011);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21988);
							match s {
								[] => true,
								['а', 'н', 'е', s @ ..] => {
									super::push(out, 27289);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30507);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 27290);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 27291);
									s.is_empty()
								}
								['о', 'д', 'е', s @ ..] => {
									super::push(out, 27292);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 30508);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 27293);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30509);
											s.is_empty()
										}
										['е', 'н', s @ ..] => {
											super::push(out, 30510);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 32185);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 32186);
													match s {
														[] => true,
														['ж', s @ ..] => super::t10::n34750(s, out),
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
									super::push(out, 27294);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30511);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 21989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27295);
									s.is_empty()
								}
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
			super::push(out, 8260);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пенный.
pub(crate) fn n3651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3517);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8261);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8262);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15012);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8263);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15013);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 21991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренный.
pub(crate) fn n3652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3518);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8264);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15014);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 15015);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21992);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21993);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21994);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 21995);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27296);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30512);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21996);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27297);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30513);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 27298);
									match s {
										[] => true,
										['е', 'п', 'е', s @ ..] => {
											super::push(out, 30514);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32187);
													s.is_empty()
												}
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
							super::push(out, 21997);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21998);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21999);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27299);
									match s {
										[] => true,
										['и', 'п', '\u{1}', s @ ..] => {
											super::push(out, 30515);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 27300);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30516);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 32188);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 32189);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 27301);
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									super::push(out, 27302);
									s.is_empty()
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30517);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 30518);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22000);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22001);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22002);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22003);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 15016);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22004);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22005);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 27304);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30519);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 27305);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30520);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27306);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 27307);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30521);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 15018);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15019);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22007);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15020);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15021);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 22009);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15023);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22011);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27309);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 22012);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22013);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15024);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22014);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 27312);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 30522);
											s.is_empty()
										}
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
					super::push(out, 15025);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15026);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22016);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27313);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t10::n8656(s, out),
		['в', s @ ..] => {
			super::push(out, 8266);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t10::n8658(s, out),
		['д', s @ ..] => super::t10::n8659(s, out),
		['е', s @ ..] => super::t10::n8660(s, out),
		['з', s @ ..] => super::t10::n8661(s, out),
		['и', s @ ..] => super::t10::n8662(s, out),
		['о', s @ ..] => super::t10::n8663(s, out),
		['п', s @ ..] => super::t10::n8664(s, out),
		['т', s @ ..] => super::t10::n8665(s, out),
		['у', s @ ..] => super::t10::n8666(s, out),
		['щ', s @ ..] => super::t10::n8667(s, out),
		['ы', s @ ..] => super::t10::n8668(s, out),
		['ю', s @ ..] => {
			super::push(out, 8278);
			s.is_empty()
		}
		['я', s @ ..] => super::t10::n8670(s, out),
		_ => false,
	}
}

/// Words ending -сенный.
pub(crate) fn n3653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3519);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8280);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15084);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22096);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27377);
									s.is_empty()
								}
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
			super::push(out, 8281);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15085);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22097);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15086);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22098);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22099);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22100);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15087);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 22101);
							match s {
								[] => true,
								['е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 27380);
									s.is_empty()
								}
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
			super::push(out, 8282);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 15088);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенный.
pub(crate) fn n3654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3520);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8284);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15089);
					match s {
						[] => true,
						['п', 'е', s @ ..] => {
							super::push(out, 22103);
							match s {
								[] => true,
								['р', 'е', 'п', 'о', s @ ..] => {
									super::push(out, 27381);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 30560);
											s.is_empty()
										}
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
					super::push(out, 15090);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 22104);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15091);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15092);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27383);
									match s {
										[] => true,
										['и', 'о', s @ ..] => {
											super::push(out, 30561);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32199);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 27384);
									match s {
										[] => true,
										['р', 'п', 'о', s @ ..] => {
											super::push(out, 30562);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32200);
													match s {
														[] => true,
														['о', s @ ..] => super::t10::n34753(s, out),
														_ => false,
													}
												}
												['г', s @ ..] => {
													super::push(out, 32201);
													match s {
														[] => true,
														['а', s @ ..] => super::t10::n34754(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8285);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8286);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15094);
					match s {
						[] => true,
						['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 22105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15095);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8287);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15096);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8288);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15097);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15098);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22108);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 22109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15099);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22110);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15100);
					s.is_empty()
				}
				['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 15101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8289);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15102);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 22111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 15103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 8290);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15104);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15105);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22112);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27386);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 27387);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22113);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15106);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 15107);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8291);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15108);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уенный.
pub(crate) fn n3655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3521);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8292);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 8293);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 8294);
			match s {
				[] => true,
				['е', 'ч', 'о', '\u{1}', s @ ..] => {
					super::push(out, 15109);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фенный.
pub(crate) fn n3656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3522);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 8295);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ценный.
pub(crate) fn n3657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3523);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8296);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8297);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15111);
					match s {
						[] => true,
						['а', 'р', 'д', 'е', s @ ..] => {
							super::push(out, 22117);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ченный.
pub(crate) fn n3658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8299);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15113);
					match s {
						[] => true,
						['х', 'ы', s @ ..] => {
							super::push(out, 22118);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27390);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15114);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15115);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22119);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 27391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15116);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22120);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15117);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 22121);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15118);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22122);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 22123);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22124);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27395);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27396);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27397);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30563);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32202);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 27398);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30564);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15119);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15120);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27399);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27400);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30565);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30566);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 32203);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15121);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15122);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 22126);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22127);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 22128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 8300);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15123);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 15124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8301);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15125);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22129);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 27402);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30567);
											s.is_empty()
										}
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
					super::push(out, 15126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22130);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22131);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 22132);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 22133);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 22134);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22135);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22136);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15127);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22137);
							match s {
								[] => true,
								['з', 'е', s @ ..] => super::t10::n28762(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t10::n15886(s, out),
				['р', s @ ..] => super::t10::n15887(s, out),
				['с', s @ ..] => super::t10::n15888(s, out),
				['ш', s @ ..] => super::t10::n15889(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n8693(s, out),
		['л', s @ ..] => super::t10::n8694(s, out),
		['н', s @ ..] => super::t10::n8695(s, out),
		['о', s @ ..] => super::t10::n8696(s, out),
		['п', s @ ..] => super::t10::n8697(s, out),
		['р', s @ ..] => super::t10::n8698(s, out),
		['у', s @ ..] => super::t10::n8699(s, out),
		['ы', s @ ..] => super::t10::n8700(s, out),
		['ь', s @ ..] => super::t10::n8701(s, out),
		['ю', s @ ..] => super::t10::n8702(s, out),
		['я', s @ ..] => super::t10::n8703(s, out),
		_ => false,
	}
}

/// Words ending -шенный.
pub(crate) fn n3659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3525);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8313);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22220);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22221);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 22222);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22223);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 22224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15175);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 15176);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15177);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22227);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27484);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30612);
											s.is_empty()
										}
										['о', 'в', 'о', 'р', 'п', 'о', s @ ..] => {
											super::push(out, 30613);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 32224);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 27485);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15178);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22228);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27487);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 27488);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30614);
											s.is_empty()
										}
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
					super::push(out, 15179);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22229);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27489);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 27490);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27491);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30615);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32225);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30616);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 32226);
													s.is_empty()
												}
												['л', s @ ..] => {
													super::push(out, 32227);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 32228);
													match s {
														[] => true,
														['м', s @ ..] => super::t10::n34760(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 30617);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27492);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 27493);
									match s {
										[] => true,
										['з', 'а', s @ ..] => {
											super::push(out, 30618);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 27494);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30619);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22230);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27495);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 30620);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8314);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8315);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22231);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22232);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15182);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22233);
							match s {
								[] => true,
								['н', 'а', 'з', 'е', s @ ..] => {
									super::push(out, 27497);
									match s {
										[] => true,
										['н', s @ ..] => super::t10::n32123(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t10::n23355(s, out),
						['з', s @ ..] => super::t10::n23356(s, out),
						['о', s @ ..] => super::t10::n23357(s, out),
						['ы', s @ ..] => super::t10::n23358(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t10::n15941(s, out),
				['п', s @ ..] => super::t10::n15942(s, out),
				['р', s @ ..] => super::t10::n15943(s, out),
				['т', s @ ..] => super::t10::n15944(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n8707(s, out),
		['м', s @ ..] => super::t10::n8708(s, out),
		['о', s @ ..] => super::t10::n8709(s, out),
		['п', s @ ..] => super::t10::n8710(s, out),
		['р', s @ ..] => super::t10::n8711(s, out),
		['у', s @ ..] => super::t10::n8712(s, out),
		['ч', s @ ..] => super::t10::n8713(s, out),
		['ы', s @ ..] => super::t10::n8714(s, out),
		['ь', s @ ..] => super::t10::n8715(s, out),
		['ю', s @ ..] => super::t10::n8716(s, out),
		_ => false,
	}
}

/// Words ending -щенный.
pub(crate) fn n3660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3526);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8326);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8327);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15209);
					match s {
						[] => true,
						['о', 'б', 'о', 'о', s @ ..] => {
							super::push(out, 22293);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15210);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15211);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27530);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27531);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 30638);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 32236);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15212);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15213);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22295);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27532);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 27533);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 30639);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 32237);
													s.is_empty()
												}
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
							super::push(out, 22296);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30640);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 30641);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30642);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 22297);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 27534);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30643);
											s.is_empty()
										}
										['о', 'в', 'е', s @ ..] => {
											super::push(out, 30644);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32238);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 27535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 22298);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 22299);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 22300);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22301);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15214);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22302);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22303);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 27537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8328);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15216);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22304);
							match s {
								[] => true,
								['о', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 27538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 22305);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22306);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8329);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15217);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22308);
							s.is_empty()
						}
						['с', 'о', 'р', s @ ..] => {
							super::push(out, 22309);
							match s {
								[] => true,
								['п', 'е', s @ ..] => {
									super::push(out, 27539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15218);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22310);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30645);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 32239);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', 'л', s @ ..] => {
											super::push(out, 30646);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 32240);
													s.is_empty()
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 30647);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 32241);
													match s {
														[] => true,
														['р', s @ ..] => super::t10::n34761(s, out),
														_ => false,
													}
												}
												['у', s @ ..] => {
													super::push(out, 32242);
													s.is_empty()
												}
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
							super::push(out, 22311);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15219);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 15220);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22313);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t10::n8721(s, out),
		['л', s @ ..] => {
			super::push(out, 8331);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 8332);
			s.is_empty()
		}
		['о', s @ ..] => super::t10::n8724(s, out),
		['р', s @ ..] => {
			super::push(out, 8334);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t10::n8726(s, out),
		['ы', s @ ..] => super::t10::n8727(s, out),
		['ь', s @ ..] => super::t10::n8728(s, out),
		['ю', s @ ..] => super::t10::n8729(s, out),
		['я', s @ ..] => super::t10::n8730(s, out),
		_ => false,
	}
}

/// Words ending -ионный.
pub(crate) fn n3687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3552);
	match s {
		[] => true,
		['д', 'а', s @ ..] => {
			super::push(out, 8430);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8431);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15329);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15330);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27579);
									match s {
										[] => true,
										['л', 'е', 'т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32252);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 32253);
													match s {
														[] => true,
														['о', s @ ..] => super::t10::n34766(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 32254);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', '\u{1}', s @ ..] => {
											super::push(out, 30663);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 27580);
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 32255);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 32256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 27581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', 'н', s @ ..] => {
							super::push(out, 22381);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'е', 'ц', 'и', 'л', '-', s @ ..] => {
					super::push(out, 15331);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'р', 'о', 'к', '-', s @ ..] => {
					super::push(out, 15332);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15333);
					match s {
						[] => true,
						['ф', 'ф', 'и', 'д', '\u{1}', s @ ..] => {
							super::push(out, 22384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', 'л', 'л', 'а', s @ ..] => {
					super::push(out, 15334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8432);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15335);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22385);
							match s {
								[] => true,
								['и', 'л', '\u{1}', s @ ..] => {
									super::push(out, 27583);
									s.is_empty()
								}
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
			super::push(out, 8433);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15336);
					match s {
						[] => true,
						['и', 'м', 'х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 27584);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 27585);
									s.is_empty()
								}
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
			super::push(out, 8434);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15337);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22386);
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
					super::push(out, 15339);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 22387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15340);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8435);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15341);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['и', 'д', 'и', s @ ..] => {
											super::push(out, 30664);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 32257);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', 'о', s @ ..] => {
											super::push(out, 30665);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 32258);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 27586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 22389);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27587);
									s.is_empty()
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
							super::push(out, 22390);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['г', 'е', 'р', 'о', s @ ..] => {
											super::push(out, 30666);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 32259);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 30667);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'б', s @ ..] => {
									super::push(out, 27588);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30668);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 27589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t10::n23522(s, out),
						['у', 'к', 'с', 'и', 'д', 'з', s @ ..] => {
							super::push(out, 22392);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t10::n8838(s, out),
		['ц', s @ ..] => super::t10::n8839(s, out),
		_ => false,
	}
}

/// Words ending -лонный.
pub(crate) fn n3689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3554);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8444);
			match s {
				[] => true,
				['с', 'о', s @ ..] => {
					super::push(out, 15362);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22428);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 22429);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 22430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15363);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 15364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8446);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 15365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8447);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 8448);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8449);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8450);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монный.
pub(crate) fn n3690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3555);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8451);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нонный.
pub(crate) fn n3691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3556);
	s.is_empty()
}

/// Words ending -понный.
pub(crate) fn n3692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3557);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ронный.
pub(crate) fn n3693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3558);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8454);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', '\u{1}', s @ ..] => {
			super::push(out, 8455);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 15370);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15371);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8456);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8457);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15374);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 15375);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22433);
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
				['\u{1}', s @ ..] => {
					super::push(out, 15376);
					s.is_empty()
				}
				['а', 'н', s @ ..] => {
					super::push(out, 15377);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 22434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15378);
					s.is_empty()
				}
				['й', 'е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22435);
							s.is_empty()
						}
						['н', 'и', s @ ..] => {
							super::push(out, 22436);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'е', 'л', 'э', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 22437);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['и', 'д', s @ ..] => {
									super::push(out, 27663);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30754);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 27664);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30755);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'с', s @ ..] => {
									super::push(out, 27665);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 30756);
											s.is_empty()
										}
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
						['л', s @ ..] => {
							super::push(out, 22438);
							s.is_empty()
						}
						['н', 'и', s @ ..] => {
							super::push(out, 22439);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22440);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 22441);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27667);
									s.is_empty()
								}
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
			super::push(out, 8458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сонный.
pub(crate) fn n3694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3559);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8459);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8460);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 8461);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8462);
			match s {
				[] => true,
				['е', 'б', '\u{1}', s @ ..] => {
					super::push(out, 15379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тонный.
pub(crate) fn n3695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3560);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8464);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8465);
			match s {
				[] => true,
				['б', 'о', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27668);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30757);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 27669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22442);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 22443);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 22444);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27670);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30758);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 30759);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 32331);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 27671);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 27672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 15380);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15381);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 15382);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 15383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15384);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15385);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8466);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15387);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8467);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22447);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22449);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 22450);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 27673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15389);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22451);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 27674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22452);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27675);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 27676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 15390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22453);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22454);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 27677);
									s.is_empty()
								}
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
			super::push(out, 8468);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8469);
			match s {
				[] => true,
				['а', 'к', 'о', s @ ..] => {
					super::push(out, 15392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15393);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15394);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8470);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 8471);
			match s {
				[] => true,
				['е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 15396);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22456);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27678);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 8472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фонный.
pub(crate) fn n3696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3561);
	match s {
		[] => true,
		['а', 'г', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15397);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15399);
					match s {
						[] => true,
						['е', 'т', 'с', s @ ..] => {
							super::push(out, 22458);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8473);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15401);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'м', 'а', 'р', 'г', '\u{1}', s @ ..] => {
					super::push(out, 15402);
					s.is_empty()
				}
				['с', 'к', 'а', 'т', s @ ..] => {
					super::push(out, 15403);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8474);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 8475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хонный.
pub(crate) fn n3697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3562);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8476);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьонный.
pub(crate) fn n3698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3563);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8477);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15406);
					match s {
						[] => true,
						['т', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27680);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 27681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 8478);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 15408);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 22461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word глюонный.
pub(crate) fn n3699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3564);
	s.is_empty()
}

/// The word трехструнный.
pub(crate) fn n33941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32332);
	s.is_empty()
}

/// Words ending -тырехструнный.
pub(crate) fn n33942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32333);
	match s {
		[] => true,
		['е', 'ч', '\u{1}', s @ ..] => {
			super::push(out, 33150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оянный.
pub(crate) fn n3719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3582);
	match s {
		[] => true,
		['т', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15445);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15446);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22484);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22485);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 27690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15447);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15448);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рянный.
pub(crate) fn n3720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3583);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8519);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15449);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15450);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уянный.
pub(crate) fn n3721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3584);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8520);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 8521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьянный.
pub(crate) fn n3722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3585);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8522);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8523);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8524);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 8525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -модерный.
pub(crate) fn n16297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15515);
	s.is_empty()
}

/// Words ending -ядерный.
pub(crate) fn n9008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8588);
	match s {
		[] => true,
		['о', 'м', s @ ..] => {
			super::push(out, 15516);
			s.is_empty()
		}
		['ъ', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22530);
					s.is_empty()
				}
				['х', 'е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 27710);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 27711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еерный.
pub(crate) fn n3762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3623);
	s.is_empty()
}

/// Words ending -жерный.
pub(crate) fn n3763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8589);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8590);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8591);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8592);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8593);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 15520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зерный.
pub(crate) fn n3764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8594);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15521);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 15522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8595);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8596);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8597);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15525);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 22531);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8598);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15527);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йерный.
pub(crate) fn n3765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3626);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -керный.
pub(crate) fn n3766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3627);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8600);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8601);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8602);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15530);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15532);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 15533);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лерный.
pub(crate) fn n3767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3628);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8604);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8605);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 8606);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 8607);
			match s {
				[] => true,
				['е', 'п', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 15535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8608);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15536);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 15537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8609);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8610);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15539);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мерный.
pub(crate) fn n3768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3629);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8611);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15540);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15541);
					match s {
						[] => true,
						['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 22535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15542);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8612);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 15544);
					match s {
						[] => true,
						['и', 'л', 'е', s @ ..] => {
							super::push(out, 22537);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 27713);
									s.is_empty()
								}
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
			super::push(out, 8613);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22538);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 27714);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30765);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8614);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15545);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15546);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 22540);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22541);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27715);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22542);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27716);
									s.is_empty()
								}
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
			super::push(out, 8615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8616);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15548);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22543);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8617);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15550);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 22544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8618);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 8619);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27718);
									s.is_empty()
								}
								['и', 'з', 'а', 'в', 'к', '\u{1}', s @ ..] => {
									super::push(out, 27719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 22545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерный.
pub(crate) fn n3769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3630);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8620);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15551);
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 22546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15552);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 15553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8621);
			match s {
				[] => true,
				['ж', 'н', 'и', 'о', s @ ..] => {
					super::push(out, 15554);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8622);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 8623);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15555);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8624);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -перный.
pub(crate) fn n3770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3631);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8626);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15557);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 15558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8627);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 8628);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15559);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15560);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8629);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8630);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8631);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15562);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 22552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -серный.
pub(crate) fn n3771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3632);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8632);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15563);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8633);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8634);
			match s {
				[] => true,
				['а', 'п', 'с', 'и', 'д', 'б', s @ ..] => {
					super::push(out, 15566);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 22553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -терный.
pub(crate) fn n3772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3633);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8636);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8637);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8638);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22554);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27721);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 30766);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22555);
							match s {
								[] => true,
								['а', 'х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30767);
											s.is_empty()
										}
										['е', 'н', s @ ..] => {
											super::push(out, 30768);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30769);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30770);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 27722);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30771);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8639);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 15567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8640);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 8641);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					super::push(out, 15568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8642);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15569);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 27723);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 27724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8643);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 8644);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8645);
			match s {
				[] => true,
				['ь', 'п', 'м', 'о', 'к', 'о', s @ ..] => {
					super::push(out, 15572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уерный.
pub(crate) fn n3773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3634);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ферный.
pub(crate) fn n3774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3635);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8647);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8648);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15574);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 15575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8649);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 8650);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8651);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8652);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15579);
					match s {
						[] => true,
						['т', 'и', 'л', s @ ..] => {
							super::push(out, 22556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27725);
									s.is_empty()
								}
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
			super::push(out, 8653);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херный.
pub(crate) fn n3775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3636);
	s.is_empty()
}

/// Words ending -церный.
pub(crate) fn n3776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3637);
	s.is_empty()
}

/// Words ending -черный.
pub(crate) fn n3777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3638);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8654);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 8655);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15581);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22557);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 27726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 27727);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 27728);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 27729);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30772);
											match s {
												[] => true,
												['о', 'г', 'у', '\u{1}', s @ ..] => {
													super::push(out, 32334);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 27730);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 30773);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8656);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8657);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8658);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шерный.
pub(crate) fn n3778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3639);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 8659);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8660);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щерный.
pub(crate) fn n3779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3640);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8662);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 8663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьерный.
pub(crate) fn n3780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3641);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8664);
			s.is_empty()
		}
		['м', 'е', 'р', 'п', 'д', s @ ..] => {
			super::push(out, 8665);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -о-разговорный.
pub(crate) fn n33944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32335);
	s.is_empty()
}

/// Words ending -ухоморный.
pub(crate) fn n23732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22578);
	s.is_empty()
}

/// Words ending -норный.
pub(crate) fn n3805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3666);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8717);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8718);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порный.
pub(crate) fn n3806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3667);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8719);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8720);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15617);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					super::push(out, 15618);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 15619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'а', s @ ..] => {
			super::push(out, 8721);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15620);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 22580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8722);
			match s {
				[] => true,
				['е', 'ч', 'о', 'л', 'е', 'щ', '\u{1}', s @ ..] => {
					super::push(out, 15622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рорный.
pub(crate) fn n3807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3668);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 8723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сорный.
pub(crate) fn n3808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3669);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8724);
			match s {
				[] => true,
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22581);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8725);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 15623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 8726);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22583);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27738);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 27739);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30778);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 27740);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 27741);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 22584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8727);
			s.is_empty()
		}
		_ => false,
	}
}

