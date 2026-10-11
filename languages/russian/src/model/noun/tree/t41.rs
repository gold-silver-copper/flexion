use flexion::Path;

/// Words ending -езия.
pub(crate) fn n19622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19517);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44195);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72648);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 72649);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 44196);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72650);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 72651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97644);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 72652);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97645);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 115996);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 72653);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97646);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 115997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72654);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 72655);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97648);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115998);
									s.is_empty()
								}
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
			super::push(out, 44197);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72656);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97649);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115999);
									s.is_empty()
								}
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
			super::push(out, 44198);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72657);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 72658);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 97651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44199);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72659);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 44200);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72660);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97653);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116000);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127685);
											s.is_empty()
										}
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
					super::push(out, 72661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97654);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116001);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127686);
											s.is_empty()
										}
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
					super::push(out, 72662);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97656);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 97657);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116002);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127687);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134617);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72663);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 72664);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 97658);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116003);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127688);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97659);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116004);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127689);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 134618);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['б', s @ ..] => {
					super::push(out, 72665);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97660);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 72666);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44201);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72667);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97662);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116006);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127690);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134619);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72668);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 44202);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72669);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97664);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изия.
pub(crate) fn n19623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19518);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 44203);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 44204);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44205);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 44206);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72671);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 97665);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127691);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72672);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97666);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116009);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127692);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нзия.
pub(crate) fn n19624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19519);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44209);
			match s {
				[] => true,
				['т', 'о', s @ ..] => {
					super::push(out, 72673);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97667);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116010);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127693);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134620);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72674);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озия.
pub(crate) fn n19625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19520);
	match s {
		[] => true,
		['н', 'г', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72675);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97668);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116011);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 127694);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134621);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -узия.
pub(crate) fn n19627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19522);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44211);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 72677);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97669);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116012);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127695);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44212);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 44213);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44214);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 44215);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72679);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 97671);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127696);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эзия.
pub(crate) fn n19628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19523);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44217);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72680);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 97672);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', s @ ..] => {
							super::push(out, 97673);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 116015);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127697);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134622);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140003(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзия.
pub(crate) fn n19629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19524);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44218);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аркия.
pub(crate) fn n44667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44238);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 72702);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укия.
pub(crate) fn n19638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19532);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 44239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72703);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 97695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыкия.
pub(crate) fn n19639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19533);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 44240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -леномегалия.
pub(crate) fn n136028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134623);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 138569);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140788);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ефалия.
pub(crate) fn n73485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72722);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 97711);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 97712);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 116032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 127703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 116033);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 127704);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 134624);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 138570);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140789);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 116034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стфалия.
pub(crate) fn n98765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97713);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 116035);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 127705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -блия.
pub(crate) fn n19642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19536);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44256);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44257);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 72725);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глия.
pub(crate) fn n19643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19537);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 44258);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 44259);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елия.
pub(crate) fn n19644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19538);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44260);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 44261);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 44262);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72730);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 97715);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116037);
									s.is_empty()
								}
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
			super::push(out, 44263);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72731);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 97716);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116038);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127706);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134626);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44264);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 72732);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97717);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116039);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127707);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97718);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 97719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72733);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97720);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116040);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 127708);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97721);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116041);
									s.is_empty()
								}
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
			super::push(out, 44265);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72734);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97722);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116042);
									s.is_empty()
								}
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
			super::push(out, 44266);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72735);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 72736);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 72737);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44267);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72738);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97726);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116043);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 127709);
											s.is_empty()
										}
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
					super::push(out, 72739);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 97727);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 97728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72740);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97729);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116044);
									s.is_empty()
								}
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
			super::push(out, 44268);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 44269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72742);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 97730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илия.
pub(crate) fn n19645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19539);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44270);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 44271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72743);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 72744);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 44272);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72745);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 44273);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72746);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97733);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116045);
									s.is_empty()
								}
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
			super::push(out, 44274);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 44275);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72747);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 97734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72748);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 72749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 44276);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44277);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72751);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97738);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116046);
									s.is_empty()
								}
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
			super::push(out, 44278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72752);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97739);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72753);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97740);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 116048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 72754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44279);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72755);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97742);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72756);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97743);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 116050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 72757);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97744);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116051);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97745);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116052);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127710);
											s.is_empty()
										}
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
					super::push(out, 72758);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97746);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116053);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 116054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 97747);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 116055);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127711);
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
								['м', s @ ..] => {
									super::push(out, 116056);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127712);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 134627);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 116057);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127713);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134628);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 116058);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 127714);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134629);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t49::n44710(s, out),
		['х', s @ ..] => {
			super::push(out, 44281);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 44282);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72764);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97761);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 97762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йлия.
pub(crate) fn n19646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19540);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44283);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72765);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллия.
pub(crate) fn n19647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19541);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44284);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 44285);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72766);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 72767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97765);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 97766);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 116080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олия.
pub(crate) fn n19648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19542);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44286);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72768);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 72769);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 72770);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 97768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 44287);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72771);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 72772);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97770);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116081);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127735);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134644);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 44288);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44289);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72775);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97772);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116082);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127736);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134645);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44290);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72776);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97773);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116083);
									s.is_empty()
								}
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
			super::push(out, 44291);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72777);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97774);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116084);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127737);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134646);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44292);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72779);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97775);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72780);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97776);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116086);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127738);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44293);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 44294);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 72783);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97778);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116087);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127739);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'и', s @ ..] => {
					super::push(out, 72784);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 97779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плия.
pub(crate) fn n19649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19543);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44295);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 72785);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97780);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улия.
pub(crate) fn n19650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19544);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72786);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 72787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 44297);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 44298);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 72789);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 44299);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 44300);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word юлия.
pub(crate) fn n44731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44301);
	s.is_empty()
}

/// Words ending -рлипидемия.
pub(crate) fn n136052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134647);
	s.is_empty()
}

/// Words ending -перемия.
pub(crate) fn n98849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97795);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 116101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оремия.
pub(crate) fn n73574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72810);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 97796);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 116102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -премия.
pub(crate) fn n73575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72811);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 97797);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 116103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 97798);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 116104);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 127743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уремия.
pub(crate) fn n73577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72813);
	s.is_empty()
}

/// Words ending -яремия.
pub(crate) fn n73578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72814);
	s.is_empty()
}

/// Words ending -семия.
pub(crate) fn n44744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44314);
	s.is_empty()
}

/// Words ending -темия.
pub(crate) fn n44745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44315);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 72815);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 97799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 72816);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шемия.
pub(crate) fn n44746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44316);
	s.is_empty()
}

/// Words ending -змия.
pub(crate) fn n19654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19548);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44317);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72817);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44318);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 72818);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имия.
pub(crate) fn n19655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19549);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44319);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 72819);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116106);
									s.is_empty()
								}
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
			super::push(out, 44320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72820);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 72821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 44321);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97804);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 97805);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 97806);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116108);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127744);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 116109);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127745);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 116110);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 127746);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 97807);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116111);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127747);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 116112);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 127748);
											s.is_empty()
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 116113);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127749);
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
								['а', s @ ..] => {
									super::push(out, 116114);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127750);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 116115);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 116116);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127751);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 127752);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134648);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 116117);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 116118);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127753);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134649);
													s.is_empty()
												}
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
								['е', s @ ..] => {
									super::push(out, 116119);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127754);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 116120);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127755);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44322);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72822);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97808);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116121);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127756);
											s.is_empty()
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 116122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97809);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 97810);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72823);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 72824);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72825);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 44323);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72826);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97813);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ймия.
pub(crate) fn n19656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19550);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ммия.
pub(crate) fn n19657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19551);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44325);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72827);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омия.
pub(crate) fn n19658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19552);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 44326);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72828);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44327);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72829);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97816);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116126);
									s.is_empty()
								}
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
			super::push(out, 44328);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72831);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116127);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 127758);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134650);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 116128);
									s.is_empty()
								}
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
			super::push(out, 44329);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72832);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 72833);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97819);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116129);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127759);
											s.is_empty()
										}
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
					super::push(out, 72834);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97820);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 97821);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116130);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127760);
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
								['н', s @ ..] => {
									super::push(out, 116131);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127761);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 134651);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 116132);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127762);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134652);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 116133);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127763);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 134653);
													s.is_empty()
												}
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
							super::push(out, 97822);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116134);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127764);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44330);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97823);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116135);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 127765);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 134654);
													s.is_empty()
												}
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
							super::push(out, 97824);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72835);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97825);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116137);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127766);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44331);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72836);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97826);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116138);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127767);
											match s {
												[] => true,
												['г', s @ ..] => super::t49::n136060(s, out),
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
							super::push(out, 97827);
							s.is_empty()
						}
						['ф', s @ ..] => super::t49::n98888(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t49::n73603(s, out),
				['к', s @ ..] => super::t49::n73604(s, out),
				['о', s @ ..] => super::t49::n73605(s, out),
				['с', s @ ..] => super::t49::n73606(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмия.
pub(crate) fn n19659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19553);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44332);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72841);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 72842);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97841);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72843);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97842);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 97843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 44333);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 72844);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97844);
							match s {
								[] => true,
								['с', 'к', 'а', s @ ..] => {
									super::push(out, 116147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 72845);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 72846);
					match s {
						[] => true,
						['о', 'е', s @ ..] => {
							super::push(out, 97845);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116148);
									s.is_empty()
								}
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
			super::push(out, 44334);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 72847);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97846);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116149);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127775);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134661);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -смия.
pub(crate) fn n19660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19554);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44335);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тмия.
pub(crate) fn n19661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19555);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44336);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72848);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умия.
pub(crate) fn n19662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19556);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 44337);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьмия.
pub(crate) fn n19663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19557);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44338);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72849);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эмия.
pub(crate) fn n19664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19558);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44339);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юмия.
pub(crate) fn n19665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19559);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 44340);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72850);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 97848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ереживания.
pub(crate) fn n136067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134662);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 138582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -споминания.
pub(crate) fn n136068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134663);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138583);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140797);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еофания.
pub(crate) fn n73632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72865);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 97866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 116161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гния.
pub(crate) fn n19667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19561);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72866);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 72867);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97867);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116162);
									s.is_empty()
								}
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
			super::push(out, 44351);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72868);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ения.
pub(crate) fn n19668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19562);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 44352);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72869);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97869);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116163);
									s.is_empty()
								}
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
			super::push(out, 44353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72870);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 72871);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97870);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72872);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97871);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116165);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 127781);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 134664);
													match s {
														[] => true,
														['л', s @ ..] => super::t49::n140019(s, out),
														_ => false,
													}
												}
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
								['о', s @ ..] => {
									super::push(out, 116166);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127782);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 116167);
									s.is_empty()
								}
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
			super::push(out, 44354);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72873);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97872);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72874);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97873);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116169);
									s.is_empty()
								}
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
			super::push(out, 44355);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72875);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 97874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44356);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 44357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44358);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72877);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44359);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72878);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 97876);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116170);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127783);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134665);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72879);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97877);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 116171);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 127784);
											s.is_empty()
										}
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
					super::push(out, 72880);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 72881);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97879);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116172);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127785);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 134666);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140020(s, out),
														_ => false,
													}
												}
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
							super::push(out, 97880);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116173);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 127786);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 44361);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72883);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44362);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 72885);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 44363);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72886);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 72887);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 72888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44364);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72889);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 72890);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72891);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97886);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116175);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127787);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134667);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72892);
					match s {
						[] => true,
						['а', s @ ..] => super::t49::n98948(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t49::n44795(s, out),
		['ш', s @ ..] => super::t49::n44796(s, out),
		_ => false,
	}
}

/// Words ending -иния.
pub(crate) fn n19669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19563);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44367);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 44368);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 44369);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72894);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97889);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116177);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127789);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134668);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['з', s @ ..] => {
							super::push(out, 97890);
							match s {
								[] => true,
								['и', 'м', '\u{1}', s @ ..] => {
									super::push(out, 116178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 97891);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 44370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72895);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97893);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97894);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 97895);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44371);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 44372);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 44373);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 72898);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97897);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116183);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127790);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134669);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72899);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 97898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72900);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 97899);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116184);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127791);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134670);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 44374);
			match s {
				[] => true,
				['к', 'а', s @ ..] => {
					super::push(out, 72902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72903);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97902);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 116185);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127792);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44375);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 72904);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 97903);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 72905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 44376);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лния.
pub(crate) fn n19670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19564);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44377);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 72907);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 97905);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116187);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127793);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 134671);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140021(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мния.
pub(crate) fn n19671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19565);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44378);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 72908);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 72909);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97907);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116189);
									s.is_empty()
								}
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
			super::push(out, 44379);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72910);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97908);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116190);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127794);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134672);
													s.is_empty()
												}
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
							super::push(out, 97909);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116191);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127795);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44380);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72911);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97910);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 72912);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нния.
pub(crate) fn n19672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19566);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44381);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 72913);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 97912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72914);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 97913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 72915);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ония.
pub(crate) fn n19673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19567);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 44382);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 44383);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72917);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 72918);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 72919);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 97914);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116193);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127796);
											s.is_empty()
										}
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
					super::push(out, 72920);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 97915);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116194);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 127797);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134673);
													s.is_empty()
												}
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
							super::push(out, 97916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 72921);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97917);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127798);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72922);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 72923);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97919);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 97920);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116196);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127799);
											s.is_empty()
										}
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
					super::push(out, 72924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 44385);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 44386);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44387);
			match s {
				[] => true,
				['р', 'б', 'м', s @ ..] => {
					super::push(out, 72927);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 97923);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116197);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127800);
											s.is_empty()
										}
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
					super::push(out, 72928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 44388);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44389);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72930);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 72931);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 97925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 72932);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97926);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116198);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127801);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134674);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72933);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97927);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116199);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127802);
											s.is_empty()
										}
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
					super::push(out, 72934);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44390);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 72935);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97929);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116200);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127803);
											s.is_empty()
										}
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
					super::push(out, 72936);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97930);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116201);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 97931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116202);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 127805);
											s.is_empty()
										}
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
					super::push(out, 72937);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 97933);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116204);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127806);
											s.is_empty()
										}
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
					super::push(out, 72938);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97934);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116205);
									s.is_empty()
								}
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
			super::push(out, 44391);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 44392);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 72940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t49::n44823(s, out),
		['с', s @ ..] => {
			super::push(out, 44394);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72947);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 72948);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t49::n44825(s, out),
		['ф', s @ ..] => super::t49::n44826(s, out),
		_ => false,
	}
}

/// Words ending -пния.
pub(crate) fn n19674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19568);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72959);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 72960);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97964);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116232);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127818);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 134678);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44397);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 72961);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97965);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рния.
pub(crate) fn n19675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19569);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44398);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 72962);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 97966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 44399);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72963);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 97967);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116234);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127819);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134679);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97968);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фния.
pub(crate) fn n19677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44400);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72965);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 72966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 97969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хния.
pub(crate) fn n19678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19572);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юния.
pub(crate) fn n19679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19573);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тахромотипия.
pub(crate) fn n136085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134680);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138587);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 140800);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иопия.
pub(crate) fn n44844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44413);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 72975);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 97979);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116241);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127823);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 134681);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 138588);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72976);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 72977);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 72978);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 97980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -копия.
pub(crate) fn n44845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44414);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 72979);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 72980);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97981);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116243);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 127824);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 134682);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138589);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n142246(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72981);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 97982);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 97983);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 97984);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 97985);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116245);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 127825);
									s.is_empty()
								}
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
			super::push(out, 72982);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 97986);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лопия.
pub(crate) fn n44846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44415);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72983);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97987);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116246);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 97988);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 116247);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127827);
									s.is_empty()
								}
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
			super::push(out, 72984);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 97989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'и', 'ц', 'н', s @ ..] => {
			super::push(out, 72985);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 97990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 72986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анопия.
pub(crate) fn n44847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44416);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 72987);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97991);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116250);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 127828);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134683);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ропия.
pub(crate) fn n44848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44417);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 72988);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97992);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 116251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 97993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116252);
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
				['е', s @ ..] => {
					super::push(out, 97994);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 116253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 97995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116254);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 116255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 97996);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 116256);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 116257);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127830);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134684);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 138590);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -топия.
pub(crate) fn n44849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72989);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 72990);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 97997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 72991);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 97998);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 72992);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 97999);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 98000);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116260);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 127831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 98001);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116261);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 127832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 98002);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 98003);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116263);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127833);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134685);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72993);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98004);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 116264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 72994);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 98005);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 116265);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127834);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 134686);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138591);
													match s {
														[] => true,
														['р', s @ ..] => super::t49::n142247(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 98006);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116266);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 127835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпия.
pub(crate) fn n19685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19579);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44419);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 72995);
					s.is_empty()
				}
				['к', 'о', s @ ..] => {
					super::push(out, 72996);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98007);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116267);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127836);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134687);
													s.is_empty()
												}
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
							super::push(out, 98008);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116268);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127837);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44420);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 72997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98009);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 98010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упия.
pub(crate) fn n19686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19580);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 44421);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эпия.
pub(crate) fn n19687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19581);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44422);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 72998);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98011);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -япия.
pub(crate) fn n19688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19582);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44423);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72999);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поллинария.
pub(crate) fn n136093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134688);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138592);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word кулинария.
pub(crate) fn n136094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134689);
	s.is_empty()
}

/// Words ending -антеннария.
pub(crate) fn n136095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134690);
	s.is_empty()
}

/// Words ending -нтипатария.
pub(crate) fn n136096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134691);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уманитария.
pub(crate) fn n136097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134692);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 138594);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -брия.
pub(crate) fn n19690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19584);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44435);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 44436);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73016);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 98022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 73017);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98023);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -грия.
pub(crate) fn n19691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19585);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 44437);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73018);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 98024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 73019);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 98025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 73020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дрия.
pub(crate) fn n19692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19586);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 44438);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73021);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 98026);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116283);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127846);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 98027);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 98028);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 98029);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116284);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127847);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134693);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140030(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73022);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 98030);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116285);
									s.is_empty()
								}
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
			super::push(out, 44439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерия.
pub(crate) fn n19693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19587);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44440);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 44441);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73023);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 73024);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73025);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98031);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116286);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127848);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 116287);
									s.is_empty()
								}
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
			super::push(out, 44442);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 73026);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98032);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 116288);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127849);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 134694);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140031(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44443);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 73027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 44444);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 44445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73029);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 98033);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 98034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 73030);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98035);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116289);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 127850);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 134695);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73031);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73032);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116290);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 116291);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127851);
											s.is_empty()
										}
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
					super::push(out, 73033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44447);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98037);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 98038);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116292);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 127852);
											s.is_empty()
										}
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
					super::push(out, 73034);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98039);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116293);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127853);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 134696);
													match s {
														[] => true,
														['л', s @ ..] => super::t49::n140032(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73035);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98040);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116294);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 98041);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116295);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127855);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 116296);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127856);
											s.is_empty()
										}
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
					super::push(out, 73036);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 73037);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 98043);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 98044);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116298);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127857);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 134697);
													s.is_empty()
												}
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
								['п', s @ ..] => super::t49::n117548(s, out),
								['у', s @ ..] => {
									super::push(out, 116300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t49::n73811(s, out),
				['ю', s @ ..] => super::t49::n73812(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t49::n44879(s, out),
		['п', s @ ..] => super::t49::n44880(s, out),
		['р', s @ ..] => super::t49::n44881(s, out),
		['с', s @ ..] => super::t49::n44882(s, out),
		['т', s @ ..] => super::t49::n44883(s, out),
		['ф', s @ ..] => super::t49::n44884(s, out),
		['х', s @ ..] => super::t49::n44885(s, out),
		['ц', s @ ..] => super::t49::n44886(s, out),
		_ => false,
	}
}

/// Words ending -ирия.
pub(crate) fn n19694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19588);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 44456);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98067);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116321);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127876);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44457);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 73058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98068);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 73059);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98069);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116323);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 127877);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44458);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73060);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 44459);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 73061);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 98071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 44460);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 73062);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 98072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 44461);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 73063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 44462);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73064);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98073);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116324);
									s.is_empty()
								}
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
			super::push(out, 44463);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73065);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 98074);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116325);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 127878);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йрия.
pub(crate) fn n19695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19589);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44464);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73066);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 44465);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 73067);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98076);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ория.
pub(crate) fn n19696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19590);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44466);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 44467);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 73068);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73069);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98077);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116327);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127879);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 134709);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44468);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 44469);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 73071);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 98078);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116328);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127880);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134710);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44470);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73072);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 98079);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116329);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127881);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134711);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44471);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73073);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 73074);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73075);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 98081);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116330);
									s.is_empty()
								}
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
			super::push(out, 44472);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73076);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 73077);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 98082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 44473);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 44474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73078);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98083);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116331);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127882);
											s.is_empty()
										}
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
					super::push(out, 73079);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44475);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73080);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98085);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 127883);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134712);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140041(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['б', s @ ..] => {
											super::push(out, 127884);
											s.is_empty()
										}
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
					super::push(out, 73081);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 98086);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 98087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 73082);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98088);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 98089);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 116333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 98090);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116334);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127885);
											s.is_empty()
										}
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
					super::push(out, 73083);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98091);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 98092);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 73084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 44476);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73085);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98094);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 116336);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 127886);
											s.is_empty()
										}
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
					super::push(out, 73086);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 98095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 73087);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 98096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 73088);
					match s {
						[] => true,
						['п', s @ ..] => super::t49::n99162(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t49::n73862(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t49::n44908(s, out),
		['ь', s @ ..] => super::t49::n44909(s, out),
		_ => false,
	}
}

/// Words ending -трия.
pub(crate) fn n19698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19592);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44479);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73092);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 98107);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 98108);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116352);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 127896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 98109);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 73093);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98110);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 73094);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98111);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 73095);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 73096);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98113);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 98114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 73097);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t49::n44911(s, out),
		['к', s @ ..] => {
			super::push(out, 44481);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 44482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73100);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 98121);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116382);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 127940);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44484);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 73101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 73102);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98123);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116384);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 127941);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 134759);
													match s {
														[] => true,
														['б', s @ ..] => super::t49::n140077(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73103);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98124);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116385);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127942);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 134760);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73104);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 98125);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116386);
									s.is_empty()
								}
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
			super::push(out, 44485);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 73105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урия.
pub(crate) fn n19699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19593);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44486);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 44487);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73107);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 73108);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98128);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116387);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 116388);
									s.is_empty()
								}
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
			super::push(out, 44488);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 73109);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 98129);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116389);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127943);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 134761);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44489);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73110);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44490);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 44491);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44492);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 73112);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98131);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116390);
									s.is_empty()
								}
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
			super::push(out, 44493);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73113);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44494);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73115);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 98132);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 73116);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 73117);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98133);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 116392);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127944);
											s.is_empty()
										}
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
					super::push(out, 73118);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98134);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116393);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 127945);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134762);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116394);
									s.is_empty()
								}
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
			super::push(out, 44495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73120);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрия.
pub(crate) fn n19701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19595);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 44496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрия.
pub(crate) fn n19702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19596);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44497);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 44498);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярия.
pub(crate) fn n19703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19597);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73121);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98136);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 98137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 98138);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116397);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 127946);
											s.is_empty()
										}
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
					super::push(out, 73122);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 98139);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 73123);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 98140);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116399);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 127947);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134763);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n140078(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73124);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98141);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116400);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 127948);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 134764);
													match s {
														[] => true,
														['т', s @ ..] => super::t49::n140079(s, out),
														_ => false,
													}
												}
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
							super::push(out, 98142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 73125);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98143);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 116401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 73126);
					match s {
						[] => true,
						['к', 'и', 'д', s @ ..] => {
							super::push(out, 98144);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116402);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 127949);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134765);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140080(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'о', s @ ..] => {
							super::push(out, 98145);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127950);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аксия.
pub(crate) fn n44942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44511);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 73139);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98156);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 116410);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127954);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134766);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 73140);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 98157);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 116411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 73141);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98158);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116412);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 98159);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116413);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127956);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134767);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 116414);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127957);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134768);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 116415);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127958);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 134769);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 127959);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 134770);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 138647);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 73142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98160);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 98161);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116416);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 127960);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134771);
											s.is_empty()
										}
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
					super::push(out, 98162);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 116417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ексия.
pub(crate) fn n44943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44512);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 73143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 98164);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116419);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 127961);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134772);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138648);
													s.is_empty()
												}
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
							super::push(out, 116420);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 127962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 98165);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116421);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 127963);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134773);
											s.is_empty()
										}
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
					super::push(out, 98166);
					match s {
						[] => true,
						['е', 'р', 'а', '\u{1}', s @ ..] => {
							super::push(out, 116422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 73144);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 98167);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 73145);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 98168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 73146);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98169);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 116425);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 127964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127965);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 127966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 116426);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127967);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 134774);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 73147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98170);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 116427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сфиксия.
pub(crate) fn n99239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98171);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 116428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 127968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анксия.
pub(crate) fn n73922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73149);
	s.is_empty()
}

/// Words ending -оксия.
pub(crate) fn n44946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44515);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 73150);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 98172);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 73151);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 73152);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98174);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 116430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 73153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 98175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уксия.
pub(crate) fn n44947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44516);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 73154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юксия.
pub(crate) fn n44948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44517);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 73155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нсия.
pub(crate) fn n19708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44518);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 44519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осия.
pub(crate) fn n19709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19603);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44520);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 44521);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73156);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98176);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
									super::push(out, 116431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псия.
pub(crate) fn n19710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19604);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44522);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98177);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116432);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 127969);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 98178);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116433);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 127970);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 98179);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 73158);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98180);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 98181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 73159);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98182);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116435);
									s.is_empty()
								}
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
			super::push(out, 44523);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73160);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98183);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116436);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127971);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134775);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140084(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73161);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98184);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116437);
									s.is_empty()
								}
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
			super::push(out, 44524);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73162);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 44525);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73163);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 73164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсия.
pub(crate) fn n19711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19605);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44526);
			match s {
				[] => true,
				['в', 'а', s @ ..] => {
					super::push(out, 73165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссия.
pub(crate) fn n19712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44527);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 44528);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 73167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98188);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 98189);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 116438);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127972);
											s.is_empty()
										}
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
					super::push(out, 73168);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 98190);
							s.is_empty()
						}
						['п', 'с', s @ ..] => {
							super::push(out, 98191);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 73169);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 73170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44529);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 73171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 44530);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73172);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 98192);
							match s {
								[] => true,
								['и', 'л', s @ ..] => {
									super::push(out, 116440);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127973);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134776);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 116441);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127974);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 127975);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134777);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140085(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 127976);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134778);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98193);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 98194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 44531);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тсия.
pub(crate) fn n19713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19607);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44532);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73174);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усия.
pub(crate) fn n19714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19608);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьсия.
pub(crate) fn n19715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19609);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44534);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 73175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рахроматия.
pub(crate) fn n136184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134779);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 138651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рихроматия.
pub(crate) fn n136185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134780);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 138652);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олисоматия.
pub(crate) fn n136186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134781);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 138653);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word автоматия.
pub(crate) fn n136187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134782);
	s.is_empty()
}

/// Words ending -рестоматия.
pub(crate) fn n136188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134783);
	s.is_empty()
}

/// Words ending -патия.
pub(crate) fn n44968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44537);
	match s {
		[] => true,
		['а', 'м', s @ ..] => {
			super::push(out, 73178);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 98201);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116448);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 127983);
									s.is_empty()
								}
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
			super::push(out, 73179);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 98202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 98203);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116449);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 127984);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134784);
											s.is_empty()
										}
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
					super::push(out, 98204);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 116450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 73180);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98205);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116451);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 98206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 73181);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 98207);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 98208);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', s @ ..] => {
					super::push(out, 98209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 116453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 98210);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 116454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 98211);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 116455);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127987);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 134785);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 116456);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127988);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 134786);
											s.is_empty()
										}
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
					super::push(out, 98212);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'в', s @ ..] => {
					super::push(out, 98213);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116458);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 127989);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 134787);
											s.is_empty()
										}
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
					super::push(out, 98214);
					match s {
						[] => true,
						['е', 'д', 'а', 'ф', s @ ..] => {
							super::push(out, 116459);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127990);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134788);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 138654);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127991);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 127992);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134789);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138655);
													s.is_empty()
												}
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
							super::push(out, 116460);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127993);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 134790);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138656);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 98215);
					match s {
						[] => true,
						['в', 'е', 'н', 'и', s @ ..] => {
							super::push(out, 116461);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127994);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 134791);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 138657);
													s.is_empty()
												}
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
							super::push(out, 116462);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127995);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 116463);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127996);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134792);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							super::push(out, 116464);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127997);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 134793);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 116465);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127998);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134794);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t49::n99284(s, out),
				['х', s @ ..] => super::t49::n99285(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етия.
pub(crate) fn n19717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19611);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44539);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73184);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73185);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98219);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116470);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128002);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 134798);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140095(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44540);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73186);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98220);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116471);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128003);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44541);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73187);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 44542);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 73188);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ития.
pub(crate) fn n19718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19612);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 44543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 44544);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73190);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 98222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 44545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73191);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73192);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44546);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73193);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 44547);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 73194);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 44548);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктия.
pub(crate) fn n19719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19613);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44549);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73196);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98226);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 73197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтия.
pub(crate) fn n19720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19614);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44550);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 73198);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98227);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 116473);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128004);
											s.is_empty()
										}
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
					super::push(out, 73199);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98228);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128005);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 128006);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134799);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 73201);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 98230);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116474);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128007);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 134800);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44551);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73202);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116475);
									s.is_empty()
								}
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
			super::push(out, 44552);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отия.
pub(crate) fn n19721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19615);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44553);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73204);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116476);
									s.is_empty()
								}
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
			super::push(out, 44554);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 73205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 44555);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 73206);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98233);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртия.
pub(crate) fn n19722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19616);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44556);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73207);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98234);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116478);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128008);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 134801);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140096(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 44557);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 73209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стия.
pub(crate) fn n19723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19617);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44558);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 73210);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98236);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116479);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128009);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134802);
													s.is_empty()
												}
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
							super::push(out, 98237);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116480);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128010);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 134803);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73211);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 73212);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98239);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116481);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128011);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 116482);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128012);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134805);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 128013);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134806);
													match s {
														[] => true,
														['ф', s @ ..] => super::t49::n140097(s, out),
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
									super::push(out, 116483);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128014);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134807);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44559);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 73213);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 98240);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116484);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128015);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44560);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 73214);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98241);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 116485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 98242);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 73215);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98243);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116487);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128016);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134808);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98244);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 98245);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 116488);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128017);
											s.is_empty()
										}
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
					super::push(out, 73217);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98246);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116489);
									s.is_empty()
								}
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
			super::push(out, 44561);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 73218);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 73219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтия.
pub(crate) fn n19725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19619);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44562);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -графия.
pub(crate) fn n73996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73222);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 98249);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 116490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 128018);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 134809);
									s.is_empty()
								}
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
			super::push(out, 98250);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 116491);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 128019);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134810);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138663);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 98251);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 116492);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 128020);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134811);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 128021);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 134812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 116493);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 128022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 116494);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 128023);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138665);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 98252);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 116495);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 116496);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 128024);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 134814);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138666);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140833);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n143530(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 116497);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 128025);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 116498);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 128026);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 134816);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 138667);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140834);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n143531(s, out),
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
									super::push(out, 134817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 128027);
							match s {
								[] => true,
								['и', 'м', s @ ..] => {
									super::push(out, 134818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 116499);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 128028);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134819);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138668);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140835);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n143532(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 116500);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 128029);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 134820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 116501);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 116502);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 128030);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 116503);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 128031);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134822);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 138669);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140836);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 98253);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 116504);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 128032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринеорафия.
pub(crate) fn n136228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134823);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138670);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140837);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерниорафия.
pub(crate) fn n136229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134824);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 138671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ольпорафия.
pub(crate) fn n136230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134825);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 138672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -псефия.
pub(crate) fn n73998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73224);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 98257);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 116508);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 128036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ифия.
pub(crate) fn n19728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19622);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44566);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 44567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мфия.
pub(crate) fn n19729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19623);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44568);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98258);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -офия.
pub(crate) fn n19730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19624);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 44569);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73227);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 98259);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116510);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128037);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 134826);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44570);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73228);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 98260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 44571);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73229);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73230);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98261);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 98262);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 98263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пфия.
pub(crate) fn n19731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44572);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рфия.
pub(crate) fn n19732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19626);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44573);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 44574);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 73232);
					match s {
						[] => true,
						['о', 'е', s @ ..] => {
							super::push(out, 98264);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116512);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128038);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 134827);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уфия.
pub(crate) fn n19733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19627);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 44575);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 44576);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73233);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьфия.
pub(crate) fn n19734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19628);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44577);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 73235);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 73236);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 98266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ендация.
pub(crate) fn n99347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98278);
	match s {
		[] => true,
		['м', 'о', s @ ..] => {
			super::push(out, 116522);
			match s {
				[] => true,
				['к', 'е', 'р', '-', s @ ..] => {
					super::push(out, 128042);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134828);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138673);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 140838);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142082);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n144246(s, out),
														_ => false,
													}
												}
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
							super::push(out, 134829);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 138674);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 140839);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142083);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n144247(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зация.
pub(crate) fn n45024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44592);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 73254);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 98279);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116523);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 116524);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128043);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 134830);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138675);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n142285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 98280);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 116525);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 128044);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 134831);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 116526);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128045);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134832);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138676);
													match s {
														[] => true,
														['ф', s @ ..] => super::t49::n142286(s, out),
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
									super::push(out, 128046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 98281);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116527);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 128047);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 134833);
											s.is_empty()
										}
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
					super::push(out, 98282);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116528);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128048);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 134834);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138677);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 98283);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 98284);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116529);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128049);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 134835);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138678);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n142287(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ревиация.
pub(crate) fn n99354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98285);
	s.is_empty()
}

/// Words ending -хиация.
pub(crate) fn n74030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 98286);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 116530);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 128050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -циация.
pub(crate) fn n74031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73257);
	s.is_empty()
}

/// Words ending -кация.
pub(crate) fn n45026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44594);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 98287);
					match s {
						[] => true,
						['у', 'м', 'м', 'о', 'к', 'с', s @ ..] => {
							super::push(out, 116531);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128051);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 134836);
											s.is_empty()
										}
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
					super::push(out, 98288);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 116532);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128052);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 134837);
											s.is_empty()
										}
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
					super::push(out, 98289);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128053);
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									super::push(out, 128054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мация.
pub(crate) fn n45027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44595);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 73258);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 98290);
					match s {
						[] => true,
						['к', 'е', 'р', s @ ..] => {
							super::push(out, 116533);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 128055);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 134838);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138679);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n142288(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 73259);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 73260);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98291);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 116534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128056);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 128057);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134839);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 138680);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 98292);
					match s {
						[] => true,
						['ф', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 128058);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134840);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 138681);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 128059);
									s.is_empty()
								}
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
			super::push(out, 73261);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 98293);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 116535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нация.
pub(crate) fn n45028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44596);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 98294);
					match s {
						[] => true,
						['м', 'о', 'к', '-', s @ ..] => {
							super::push(out, 116536);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 128060);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 134841);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138682);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n142289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 98295);
					match s {
						[] => true,
						['р', 'о', 'о', s @ ..] => {
							super::push(out, 116537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рация.
pub(crate) fn n45029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44597);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 73262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 98296);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 98297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 73263);
			match s {
				[] => true,
				['д', 'е', s @ ..] => {
					super::push(out, 98298);
					match s {
						[] => true,
						['ф', 'н', s @ ..] => {
							super::push(out, 116538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 98299);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 116539);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128061);
									s.is_empty()
								}
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
			super::push(out, 73264);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 73265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сация.
pub(crate) fn n45030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44598);
	s.is_empty()
}

/// Words ending -тация.
pub(crate) fn n45031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44599);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 73266);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 98300);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'е', 'р', s @ ..] => {
					super::push(out, 98301);
					match s {
						[] => true,
						['к', 'к', 'а', 'с', s @ ..] => {
							super::push(out, 116541);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128062);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134842);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 73267);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98302);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 116542);
							match s {
								[] => true,
								['е', 'р', 'п', '-', s @ ..] => {
									super::push(out, 128063);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134843);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138683);
													match s {
														[] => true,
														['в', s @ ..] => super::t49::n142290(s, out),
														_ => false,
													}
												}
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
							super::push(out, 116543);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128064);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134844);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 73268);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 98303);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							super::push(out, 116544);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 128065);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 134845);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 138684);
													match s {
														[] => true,
														['д', s @ ..] => super::t49::n142291(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еция.
pub(crate) fn n19743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19637);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 44600);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 73269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 44601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 44602);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73271);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98304);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116545);
									s.is_empty()
								}
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
			super::push(out, 44603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73272);
					match s {
						[] => true,
						['р', 'т', '-', s @ ..] => {
							super::push(out, 98305);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116546);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 128066);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134846);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140120(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44604);
			match s {
				[] => true,
				['к', 'у', s @ ..] => {
					super::push(out, 73273);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иция.
pub(crate) fn n19744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19638);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 44605);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 44606);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73274);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98307);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116548);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128067);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44607);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73275);
					match s {
						[] => true,
						['а', 'п', 'е', s @ ..] => {
							super::push(out, 98308);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128068);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 98309);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116550);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128069);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 134847);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кция.
pub(crate) fn n19745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19639);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44608);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 73277);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 98310);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116551);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128070);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134848);
													match s {
														[] => true,
														['г', s @ ..] => super::t49::n140121(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нция.
pub(crate) fn n19746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19640);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44609);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98311);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116552);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128071);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134849);
													s.is_empty()
												}
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
							super::push(out, 98312);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116553);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128072);
											match s {
												[] => true,
												['к', 'е', 'л', 'э', '-', s @ ..] => {
													super::push(out, 134850);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n140122(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44610);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 73278);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98313);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116554);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128073);
											s.is_empty()
										}
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
					super::push(out, 73279);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 73280);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 98314);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 73281);
					match s {
						[] => true,
						['е', 'ф', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116556);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 116557);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128074);
											s.is_empty()
										}
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
					super::push(out, 73282);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 73283);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 73284);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98315);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116558);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128075);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134851);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140123(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73285);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 98316);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116559);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128076);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 134852);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 116560);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128077);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134853);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140124(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пция.
pub(crate) fn n19747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19641);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 44611);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 44612);
			match s {
				[] => true,
				['р', 'р', s @ ..] => {
					super::push(out, 73286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рция.
pub(crate) fn n19748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19642);
	match s {
		[] => true,
		['о', 'п', s @ ..] => {
			super::push(out, 44613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юция.
pub(crate) fn n19749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19643);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73287);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73288);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яция.
pub(crate) fn n19750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19644);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73289);
					match s {
						[] => true,
						['е', 'ф', s @ ..] => {
							super::push(out, 98317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'к', 'р', s @ ..] => {
					super::push(out, 73290);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 73291);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98318);
							match s {
								[] => true,
								['и', 'р', s @ ..] => {
									super::push(out, 116561);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отиворечия.
pub(crate) fn n136259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134854);
	s.is_empty()
}

/// Words ending -полномочия.
pub(crate) fn n136260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134855);
	s.is_empty()
}

/// Words ending -редседателя.
pub(crate) fn n140125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138690);
	s.is_empty()
}

/// Words ending --ля-тополя.
pub(crate) fn n136264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134859);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 138691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -воображуля.
pub(crate) fn n136265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134860);
	s.is_empty()
}

/// Words ending -руля.
pub(crate) fn n19838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19732);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 44721);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 44722);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суля.
pub(crate) fn n19839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19733);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44723);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73379);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44724);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73380);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 73381);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 98369);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116591);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128090);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44725);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 73383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туля.
pub(crate) fn n19840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19734);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 44726);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 73384);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98371);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116592);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128091);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44727);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73385);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очуля.
pub(crate) fn n45162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44728);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 73386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -червоводня.
pub(crate) fn n136268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134863);
	s.is_empty()
}

/// Words ending -тронь-меня.
pub(crate) fn n136269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134864);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 138693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ист-разиня.
pub(crate) fn n136270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134865);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138694);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ть-героиня.
pub(crate) fn n136271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134866);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138695);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 140851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -подлокотня.
pub(crate) fn n136272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134867);
	s.is_empty()
}

/// Words ending -подворотня.
pub(crate) fn n136273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134868);
	s.is_empty()
}

/// Words ending -либерастня.
pub(crate) fn n136274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134869);
	s.is_empty()
}

/// Words ending -кистня.
pub(crate) fn n74303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73526);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 98457);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 116639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утня.
pub(crate) fn n19953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19847);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44883);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мытня.
pub(crate) fn n45318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44884);
	s.is_empty()
}

/// Words ending -ютня.
pub(crate) fn n19955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19849);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 44885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятня.
pub(crate) fn n19956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19850);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 44886);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 73528);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 73529);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98459);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116640);
									s.is_empty()
								}
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
			super::push(out, 44887);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рика-кухня.
pub(crate) fn n136275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134870);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 138696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кормокухня.
pub(crate) fn n136276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134871);
	s.is_empty()
}

/// Words ending -нюшня.
pub(crate) fn n45364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44930);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 73571);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 98485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -благостыня.
pub(crate) fn n136277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134872);
	s.is_empty()
}

/// The word сновальня.
pub(crate) fn n136278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134873);
	s.is_empty()
}

/// The word штопальня.
pub(crate) fn n136279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134874);
	s.is_empty()
}

/// Words ending -а-читальня.
pub(crate) fn n136280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134875);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 138697);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 140852);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 138698);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140853);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142093);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142801);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143181);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 143367);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трудящаяся.
pub(crate) fn n136289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134884);
	s.is_empty()
}

/// Words ending -кайпиринья.
pub(crate) fn n136290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134885);
	s.is_empty()
}

/// Words ending -унья.
pub(crate) fn n20087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19981);
	match s {
		[] => true,
		['г', 'н', s @ ..] => {
			super::push(out, 45054);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 73677);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98560);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116701);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 128147);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134886);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'е', s @ ..] => {
			super::push(out, 45055);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 73678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 45056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73679);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98562);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 116702);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 128148);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134887);
													match s {
														[] => true,
														['ц', s @ ..] => super::t49::n140136(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 73680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 45057);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 73681);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98563);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116703);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 128149);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 134888);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140137(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 45058);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 45059);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 45060);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 73682);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 73683);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 73684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-да-марья.
pub(crate) fn n136294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134889);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138703);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елка-летяга.
pub(crate) fn n136311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134906);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 138712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стреча-беседа.
pub(crate) fn n142304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140859);
	s.is_empty()
}

/// Words ending -лекция-беседа.
pub(crate) fn n142305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140860);
	s.is_empty()
}

/// Words ending -полуборозда.
pub(crate) fn n136318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134913);
	s.is_empty()
}

/// Words ending -атигастрида.
pub(crate) fn n136320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134915);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -панк-легенда.
pub(crate) fn n136322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134917);
	s.is_empty()
}

/// Words ending -пол-огорода.
pub(crate) fn n136324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134919);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 138717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -калибр-пробка.
pub(crate) fn n142309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140864);
	s.is_empty()
}

/// Words ending -ефон-трубка.
pub(crate) fn n136335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134930);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138722);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140865);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вль-красавка.
pub(crate) fn n140159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138724);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140866);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142100);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 142804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word бестужевка.
pub(crate) fn n136340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134935);
	s.is_empty()
}

/// Words ending -тимирязевка.
pub(crate) fn n136341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134936);
	s.is_empty()
}

/// Words ending -ковиевка.
pub(crate) fn n100053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98980);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 116911);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 128257);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134937);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 138725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риевка.
pub(crate) fn n46556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46115);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 74408);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98981);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 116912);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 128258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сиевка.
pub(crate) fn n46557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46116);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 74409);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 98982);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 116913);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 128259);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134938);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -левка.
pub(crate) fn n21224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46117);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 46118);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 74410);
					match s {
						[] => true,
						['р', 'х', s @ ..] => {
							super::push(out, 98983);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116914);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128260);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134939);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46119);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74411);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98984);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 98985);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 98986);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116916);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128261);
											s.is_empty()
										}
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
					super::push(out, 74412);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98987);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116917);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 128262);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134940);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74413);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98988);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116918);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 98989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 46120);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46121);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 46122);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74415);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -невка.
pub(crate) fn n21225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21112);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 46123);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 46124);
			match s {
				[] => true,
				['х', 'е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 98992);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 98993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46125);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 74417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 46126);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74418);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98995);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116919);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128264);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46127);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 74419);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98996);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оевка.
pub(crate) fn n21226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21113);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 46128);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 46129);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46130);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 74421);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 98997);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116921);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128266);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 134941);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140161(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ревка.
pub(crate) fn n21228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21115);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46131);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46132);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 74422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98998);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 74423);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98999);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 116923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -севка.
pub(crate) fn n21229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21116);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 46133);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 46134);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утевка.
pub(crate) fn n46576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46135);
	s.is_empty()
}

/// Words ending -уевка.
pub(crate) fn n21231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21118);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 46136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 46137);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74426);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 99001);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116924);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128267);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цевка.
pub(crate) fn n21232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21119);
	s.is_empty()
}

/// Words ending -чевка.
pub(crate) fn n21233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21120);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46138);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шевка.
pub(crate) fn n21234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21121);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46140);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 74427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 46141);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74428);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99002);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116925);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128268);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щевка.
pub(crate) fn n21235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21122);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьевка.
pub(crate) fn n21236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21123);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 46143);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 46144);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 74429);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99003);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116926);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128269);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юевка.
pub(crate) fn n21237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21124);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 46145);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74430);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99004);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116927);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128270);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134942);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -проторговка.
pub(crate) fn n136348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134943);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 138727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рошмандовка.
pub(crate) fn n136349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134944);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 138728);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гродовка.
pub(crate) fn n100097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99024);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 116936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 128274);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 134945);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 138729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ходовка.
pub(crate) fn n75236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74458);
	match s {
		[] => true,
		['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
			super::push(out, 99025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удовка.
pub(crate) fn n46615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46174);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 74459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядовка.
pub(crate) fn n46616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46175);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 74460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жовка.
pub(crate) fn n21251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21138);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46176);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 74461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 46177);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74462);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116937);
									s.is_empty()
								}
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
			super::push(out, 46178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зовка.
pub(crate) fn n21252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21139);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46179);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 74463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46180);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74464);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99029);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116938);
									s.is_empty()
								}
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
			super::push(out, 46181);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 74465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 46182);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74466);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99030);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116939);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128275);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46183);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74467);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 74468);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 46184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковка.
pub(crate) fn n21253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21140);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46185);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 46186);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74470);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 74471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 74472);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 74473);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 74474);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99035);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116940);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128276);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134946);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74475);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99036);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116941);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128277);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134947);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74476);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 99037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116942);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128278);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46187);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 46188);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 74477);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99038);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116943);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128279);
											s.is_empty()
										}
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
					super::push(out, 74478);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 74479);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99039);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116944);
									s.is_empty()
								}
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
			super::push(out, 46189);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46190);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74480);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 74481);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99040);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116945);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128280);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 134948);
													match s {
														[] => true,
														['д', s @ ..] => super::t49::n140165(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46191);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74482);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 74483);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 99042);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116946);
									s.is_empty()
								}
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
			super::push(out, 46192);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74484);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116947);
									s.is_empty()
								}
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
			super::push(out, 46193);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74485);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 74486);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 74487);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 74488);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 99044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 46194);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 74490);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99046);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 99047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 46195);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 74491);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99048);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116948);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 128281);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134949);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n140166(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46196);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74492);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 74493);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99050);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 74494);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловка.
pub(crate) fn n21254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46197);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99051);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 99052);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116950);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128282);
											s.is_empty()
										}
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
					super::push(out, 74496);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 74497);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 74498);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 74499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 46198);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74500);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 99054);
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
				['е', s @ ..] => {
					super::push(out, 74501);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 74502);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46199);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74503);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99057);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 116951);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 128283);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 134950);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74504);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99058);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 116952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 74505);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 99059);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116953);
									s.is_empty()
								}
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
			super::push(out, 46200);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 74506);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 46201);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74507);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 46202);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74508);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 74509);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 74510);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 74511);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 74512);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99064);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 116955);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128284);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134951);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140167(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 74513);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 74514);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99065);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 116956);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 99066);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 99067);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 99068);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 99069);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 116959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 74515);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 46203);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74516);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99071);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116960);
									s.is_empty()
								}
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
			super::push(out, 46204);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74517);
					match s {
						[] => true,
						['ы', s @ ..] => super::t49::n100145(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t49::n75296(s, out),
				['г', s @ ..] => super::t49::n75297(s, out),
				['з', s @ ..] => super::t49::n75298(s, out),
				['к', s @ ..] => super::t49::n75299(s, out),
				['м', s @ ..] => super::t49::n75300(s, out),
				['р', s @ ..] => super::t49::n75301(s, out),
				['с', s @ ..] => super::t49::n75302(s, out),
				['т', s @ ..] => super::t49::n75303(s, out),
				['х', s @ ..] => super::t49::n75304(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t49::n46647(s, out),
		['с', s @ ..] => super::t49::n46648(s, out),
		['т', s @ ..] => super::t49::n46649(s, out),
		['у', s @ ..] => super::t49::n46650(s, out),
		['ы', s @ ..] => super::t49::n46651(s, out),
		['я', s @ ..] => super::t49::n46652(s, out),
		_ => false,
	}
}

/// Words ending -мовка.
pub(crate) fn n21255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46210);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 74539);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99103);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128297);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46211);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 74540);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 74541);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 99104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 74542);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99105);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116983);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128298);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134958);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46212);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 74543);
					match s {
						[] => true,
						['д', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 99106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'а', 'р', 'г', 'т', 'о', 'с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
			super::push(out, 46213);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 46214);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 74544);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 99107);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116984);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128299);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134959);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46215);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74545);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 99108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 74546);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новка.
pub(crate) fn n21256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21143);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46216);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 74547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99109);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116985);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128300);
											s.is_empty()
										}
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
					super::push(out, 74548);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116986);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128301);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 134960);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n140171(s, out),
														_ => false,
													}
												}
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
							super::push(out, 99111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 74549);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99112);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 116988);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128302);
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 116989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 99113);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116990);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128303);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134961);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74550);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99114);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116991);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128304);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 99115);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116992);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128305);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 134962);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74551);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 99116);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 116993);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128306);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 134963);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 74552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99117);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116994);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128307);
											s.is_empty()
										}
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
					super::push(out, 74553);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99118);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116995);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128308);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 99119);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116996);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 74554);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99120);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 116997);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128309);
											s.is_empty()
										}
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
					super::push(out, 74555);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 74556);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99122);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 116998);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128310);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46217);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 46218);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 74557);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46219);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 74558);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 99124);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 74559);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 74560);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 74561);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 99127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 74562);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 99128);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t49::n46663(s, out),
		['л', s @ ..] => {
			super::push(out, 46221);
			s.is_empty()
		}
		['о', s @ ..] => super::t49::n46665(s, out),
		['р', s @ ..] => {
			super::push(out, 46223);
			match s {
				[] => true,
				['е', s @ ..] => super::t49::n75356(s, out),
				['и', s @ ..] => super::t49::n75357(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t49::n46667(s, out),
		['у', s @ ..] => super::t49::n46668(s, out),
		['ы', s @ ..] => super::t49::n46669(s, out),
		['я', s @ ..] => super::t49::n46670(s, out),
		_ => false,
	}
}

