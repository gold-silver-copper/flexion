use flexion::Path;

/// Words ending -сывать.
pub(crate) fn n1480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4997);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
							super::push(out, 21354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29986);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 35972);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 21355);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 35973);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39625);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83154(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12408);
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4998);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['в', s @ ..] => {
							super::push(out, 21356);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29987);
									s.is_empty()
								}
								_ => false,
							}
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t19::n49665(s, out),
								['д', '\u{1}', s @ ..] => super::t19::n49666(s, out),
								['п', '\u{1}', s @ ..] => super::t19::n49667(s, out),
								['р', 'п', '\u{1}', s @ ..] => super::t19::n49668(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t19::n31564(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t19::n31565(s, out),
						['у', '\u{1}', s @ ..] => super::t19::n31566(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t19::n31567(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t19::n15830(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t19::n5674(s, out),
		['м', 'а', 'р', 'к', s @ ..] => super::t19::n5675(s, out),
		['с', s @ ..] => super::t19::n5676(s, out),
		['у', s @ ..] => super::t19::n5677(s, out),
		['я', s @ ..] => super::t19::n5678(s, out),
		_ => false,
	}
}

/// Words ending -тывать.
pub(crate) fn n1481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5003);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12416);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29997);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29998);
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 39629);
													match s {
														[] => true,
														['н', s @ ..] => super::t19::n83158(s, out),
														_ => false,
													}
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 29999);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => super::t19::n65030(s, out),
										_ => false,
									}
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t19::n49724(s, out),
								['з', 'а', 'р', '\u{1}', s @ ..] => super::t19::n49725(s, out),
								['и', 'р', 'п', '\u{1}', s @ ..] => super::t19::n49726(s, out),
								['о', s @ ..] => super::t19::n49727(s, out),
								['с', s @ ..] => super::t19::n49728(s, out),
								['т', 'о', '\u{1}', s @ ..] => super::t19::n49729(s, out),
								['ы', 'в', s @ ..] => super::t19::n49730(s, out),
								_ => false,
							}
						}
						['ы', 'в', '\u{1}', s @ ..] => super::t19::n31603(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t19::n15841(s, out),
				['г', 'о', 'г', 'о', 'п', '\u{1}', s @ ..] => super::t19::n15842(s, out),
				['к', s @ ..] => super::t19::n15843(s, out),
				['л', s @ ..] => super::t19::n15844(s, out),
				['м', s @ ..] => super::t19::n15845(s, out),
				['п', 'о', 'л', 'х', s @ ..] => super::t19::n15846(s, out),
				['р', 'б', 'о', '\u{1}', s @ ..] => super::t19::n15847(s, out),
				['х', s @ ..] => super::t19::n15848(s, out),
				['ч', s @ ..] => super::t19::n15849(s, out),
				['ш', s @ ..] => super::t19::n15850(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t19::n5680(s, out),
		['и', s @ ..] => super::t19::n5681(s, out),
		['к', s @ ..] => super::t19::n5682(s, out),
		['л', s @ ..] => super::t19::n5683(s, out),
		['о', s @ ..] => super::t19::n5684(s, out),
		['п', s @ ..] => super::t19::n5685(s, out),
		['р', s @ ..] => super::t19::n5686(s, out),
		['с', s @ ..] => super::t19::n5687(s, out),
		['у', s @ ..] => super::t19::n5688(s, out),
		['х', s @ ..] => super::t19::n5689(s, out),
		['ы', s @ ..] => super::t19::n5690(s, out),
		['я', s @ ..] => super::t19::n5691(s, out),
		_ => false,
	}
}

/// Words ending -цывать.
pub(crate) fn n1482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1399);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12441);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21399);
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
		_ => false,
	}
}

/// Words ending -ерегать.
pub(crate) fn n5741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5049);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12469);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12470);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21410);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30022);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 12471);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
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

/// Words ending -тегать.
pub(crate) fn n1490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1407);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5050);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12472);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12473);
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
				['д', s @ ..] => {
					super::push(out, 12474);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12475);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21411);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30023);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12476);
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12477);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12478);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21412);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остригать.
pub(crate) fn n15985(s: &[char], _out: &mut Path<'_>) -> bool {
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тигать.
pub(crate) fn n1500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1417);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', s @ ..] => {
					super::push(out, 12496);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word надстругать.
pub(crate) fn n76409(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -остругать.
pub(crate) fn n31887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21450);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30040);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 30041);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 30042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 30043);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35999);
					match s {
						[] => true,
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
			super::push(out, 30044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тстругать.
pub(crate) fn n31889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21452);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 30045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устругать.
pub(crate) fn n31890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21453);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыстругать.
pub(crate) fn n31891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21454);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 30046);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыругать.
pub(crate) fn n5802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5092);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шугать.
pub(crate) fn n1514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1431);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5093);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word попроведать.
pub(crate) fn n76413(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -редать.
pub(crate) fn n1544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1460);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5162);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12583);
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
				['д', s @ ..] => {
					super::push(out, 12584);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21486);
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
					super::push(out, 12585);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21487);
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
				['з', s @ ..] => {
					super::push(out, 12586);
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12587);
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12588);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21488);
							match s {
								[] => true,
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
					super::push(out, 12589);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21489);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5163);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12590);
					match s {
						[] => true,
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
			super::push(out, 5164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12591);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -седать.
pub(crate) fn n1545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1461);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5165);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21490);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21491);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30059);
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
							super::push(out, 21492);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 12593);
					match s {
						[] => true,
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
			super::push(out, 5166);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12594);
					match s {
						[] => true,
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
			super::push(out, 5167);
			match s {
				[] => true,
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5168);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'о', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъедать.
pub(crate) fn n1547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1463);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5170);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21493);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
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
		['з', s @ ..] => {
			super::push(out, 5171);
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -выедать.
pub(crate) fn n5905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5172);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word переграждать.
pub(crate) fn n76414(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -реждать.
pub(crate) fn n5912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5179);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 12606);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12607);
			match s {
				[] => true,
				['у', s @ ..] => {
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
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 12608);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иждать.
pub(crate) fn n1552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1468);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5180);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ождать.
pub(crate) fn n1553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1469);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5181);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21503);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 21504);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', '\u{1}', s @ ..] => {
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
						['о', s @ ..] => {
							super::push(out, 21505);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30066);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5182);
			match s {
				[] => true,
				['г', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
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
		['г', s @ ..] => {
			super::push(out, 5183);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 12610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12611);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12612);
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
		['м', s @ ..] => {
			super::push(out, 5185);
			match s {
				[] => true,
				['е', 'н', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12613);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21506);
							match s {
								[] => true,
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
					super::push(out, 12614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['и', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', 'з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12616);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5188);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рждать.
pub(crate) fn n1554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1470);
	match s {
		[] => true,
		['е', 'в', 'т', s @ ..] => {
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
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

/// Words ending -уждать.
pub(crate) fn n1555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1471);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5189);
			match s {
				[] => true,
				['з', 'о', 'в', s @ ..] => {
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
		['л', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21507);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21508);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21509);
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
		['н', s @ ..] => {
			super::push(out, 5191);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5192);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 5193);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12619);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30067);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36005);
											match s {
												[] => true,
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
									super::push(out, 30068);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36006);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12620);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12621);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12622);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30069);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36007);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 5194);
			match s {
				[] => true,
				['т', 'о', '\u{1}', s @ ..] => {
					super::push(out, 12623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыждать.
pub(crate) fn n1556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1472);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5195);
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

/// The word подзапоздать.
pub(crate) fn n76418(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -реподать.
pub(crate) fn n16281(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21527);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30073);
					match s {
						[] => true,
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
			super::push(out, 21528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30074);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -родать.
pub(crate) fn n1574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1490);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 5234);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
						_ => false,
					}
				}
				['с', 'а', 'р', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 12667);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21529);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -межать.
pub(crate) fn n1604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1514);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5297);
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нежать.
pub(crate) fn n1605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1515);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 5298);
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

/// Words ending -режать.
pub(crate) fn n1606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1516);
	match s {
		[] => true,
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5299);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21543);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['з', s @ ..] => {
			super::push(out, 5300);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word заверезжать.
pub(crate) fn n76422(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -юзжать.
pub(crate) fn n1613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1523);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5315);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5316);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21550);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21551);
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
							super::push(out, 21552);
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
						['ы', s @ ..] => {
							super::push(out, 21553);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30082);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word повыползать.
pub(crate) fn n76423(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word позамерзать.
pub(crate) fn n76424(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word раскумекать.
pub(crate) fn n76425(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -арекать.
pub(crate) fn n6327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5480);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 12900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12901);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21638);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36016);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36017);
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
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12902);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерекать.
pub(crate) fn n6330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5483);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 12904);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21639);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30102);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трекать.
pub(crate) fn n6334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5486);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21640);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30103);
							match s {
								[] => true,
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
					super::push(out, 21641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30104);
							match s {
								[] => true,
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
					super::push(out, 21642);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 30105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урекать.
pub(crate) fn n6335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5487);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 12905);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21644);
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

/// Words ending -секать.
pub(crate) fn n1706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1612);
	match s {
		[] => true,
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -текать.
pub(crate) fn n1707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1613);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5488);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12907);
					match s {
						[] => true,
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
			super::push(out, 5489);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12908);
					match s {
						[] => true,
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
			super::push(out, 5490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5491);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			super::push(out, 5492);
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			super::push(out, 5493);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5494);
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
			super::push(out, 5495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12909);
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21645);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21646);
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
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			super::push(out, 5496);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5497);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5498);
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

/// Words ending -хекать.
pub(crate) fn n1708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1614);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5499);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 12910);
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
		['к', s @ ..] => {
			super::push(out, 5500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word цекать.
pub(crate) fn n6361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5501);
	s.is_empty()
}

/// Words ending -чекать.
pub(crate) fn n1710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1616);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5502);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5503);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12911);
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

/// Words ending -шекать.
pub(crate) fn n1711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1617);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			super::push(out, 5504);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word прочиликать.
pub(crate) fn n76428(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -икликать.
pub(crate) fn n16763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12921);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 21659);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 30113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окликать.
pub(crate) fn n16764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12922);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 21660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скликать.
pub(crate) fn n16765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12923);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкликать.
pub(crate) fn n16766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12924);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21662);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уликать.
pub(crate) fn n6376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5513);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 12925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21663);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30114);
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

/// Words ending -шликать.
pub(crate) fn n6377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5514);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -никать.
pub(crate) fn n1721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1627);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 5515);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5516);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12926);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12927);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 21664);
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оикать.
pub(crate) fn n1722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1628);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 5517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пикать.
pub(crate) fn n1723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1629);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5518);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5519);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12928);
					match s {
						[] => true,
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
			super::push(out, 5520);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12929);
					match s {
						[] => true,
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
			super::push(out, 5521);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рикать.
pub(crate) fn n1724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1630);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5522);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12930);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 21665);
							match s {
								[] => true,
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
					super::push(out, 12931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21666);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21667);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сикать.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1631);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5523);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5524);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12932);
					match s {
						[] => true,
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
			super::push(out, 5525);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12933);
					match s {
						[] => true,
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
			super::push(out, 5526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тикать.
pub(crate) fn n1726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1632);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5527);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5528);
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
		['у', s @ ..] => {
			super::push(out, 5529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5530);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12935);
					match s {
						[] => true,
						['р', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 12936);
					match s {
						[] => true,
						['р', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хикать.
pub(crate) fn n1727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1633);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5531);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21668);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21669);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30115);
									match s {
										[] => true,
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
							super::push(out, 21670);
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
		['у', s @ ..] => {
			super::push(out, 5532);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чикать.
pub(crate) fn n1728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1634);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5533);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5534);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шикать.
pub(crate) fn n1729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1635);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5535);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5536);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5537);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12940);
					match s {
						[] => true,
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
			super::push(out, 5538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5539);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щикать.
pub(crate) fn n1730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1636);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -порастолкать.
pub(crate) fn n76429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39638);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оцокать.
pub(crate) fn n6461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5588);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 12980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12981);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отцокать.
pub(crate) fn n32536(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -чокать.
pub(crate) fn n1753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1659);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5590);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шокать.
pub(crate) fn n1754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1660);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еркать.
pub(crate) fn n1756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1662);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5597);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13003);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30120);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30121);
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
				['с', s @ ..] => {
					super::push(out, 13004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21708);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21709);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21710);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13006);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13007);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13008);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13009);
					match s {
						[] => true,
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
			super::push(out, 5599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13010);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13011);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['е', s @ ..] => {
					super::push(out, 13012);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21712);
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
					super::push(out, 13013);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 21714);
							match s {
								[] => true,
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
					super::push(out, 13014);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13015);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21716);
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

/// Words ending -чиркать.
pub(crate) fn n6477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5602);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13018);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ширкать.
pub(crate) fn n6478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5603);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13019);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13020);
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

/// Words ending -оркать.
pub(crate) fn n1758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1664);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5605);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13022);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21717);
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
		['м', s @ ..] => {
			super::push(out, 5606);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13023);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21718);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21719);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 30123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21720);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30124);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5607);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13024);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13025);
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
				['д', s @ ..] => {
					super::push(out, 13026);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13027);
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
		['х', s @ ..] => {
			super::push(out, 5608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13028);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13029);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13030);
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

/// Words ending -уркать.
pub(crate) fn n1759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1665);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5610);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5613);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13032);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13033);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5614);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыркать.
pub(crate) fn n1760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5615);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13034);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30125);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30126);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 36020);
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
									super::push(out, 30127);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36021);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13035);
					match s {
						[] => true,
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
			super::push(out, 5616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13036);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13037);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21722);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5618);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13038);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21723);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21724);
							match s {
								[] => true,
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
					super::push(out, 13039);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21725);
							match s {
								[] => true,
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
					super::push(out, 13040);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21726);
							match s {
								[] => true,
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
					super::push(out, 13041);
					match s {
						[] => true,
						['ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
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

/// The word юркать.
pub(crate) fn n6495(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поднатаскать.
pub(crate) fn n76432(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поистаскать.
pub(crate) fn n76433(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лескать.
pub(crate) fn n6499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5623);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21732);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21733);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30130);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 30131);
							match s {
								[] => true,
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
					super::push(out, 21734);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30132);
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
					super::push(out, 21735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30133);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 30134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 30135);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 36025);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 41590);
													match s {
														[] => true,
														['з', s @ ..] => super::t19::n86763(s, out),
														_ => false,
													}
												}
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
							super::push(out, 30136);
							match s {
								[] => true,
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
					super::push(out, 21736);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30137);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30138);
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
				['т', s @ ..] => {
					super::push(out, 21737);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30139);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 21738);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трескать.
pub(crate) fn n16967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13052);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21739);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21740);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30141);
					match s {
						[] => true,
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
			super::push(out, 21741);
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
		['ы', s @ ..] => {
			super::push(out, 21742);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -искать.
pub(crate) fn n1765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5625);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5626);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13053);
					match s {
						[] => true,
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
			super::push(out, 5627);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13054);
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
			super::push(out, 5628);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13055);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5629);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5630);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13057);
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
				['с', s @ ..] => {
					super::push(out, 13058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5631);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13060);
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
					super::push(out, 13061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5633);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оскать.
pub(crate) fn n1766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1672);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5634);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21747);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21748);
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
							super::push(out, 21749);
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
							super::push(out, 21750);
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
						['ы', s @ ..] => {
							super::push(out, 21751);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30143);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5635);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рскать.
pub(crate) fn n1767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1673);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5636);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21752);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21753);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30144);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5637);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ускать.
pub(crate) fn n1768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1674);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5638);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5639);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
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
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
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
					super::push(out, 13067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 21754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30145);
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
				['с', s @ ..] => {
					super::push(out, 13068);
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
								['е', s @ ..] => {
									super::push(out, 30146);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 36026);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 39639);
													match s {
														[] => true,
														['л', '\u{1}', s @ ..] => super::t19::n83168(s, out),
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
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13069);
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
								['о', s @ ..] => {
									super::push(out, 30147);
									match s {
										[] => true,
										['д', 'е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 36027);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыскать.
pub(crate) fn n1769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1675);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5640);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5642);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21755);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 36028);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 39640);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83169(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5643);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13073);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13074);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21757);
							match s {
								[] => true,
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
					super::push(out, 13075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21758);
							match s {
								[] => true,
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
					super::push(out, 13076);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 21759);
							match s {
								[] => true,
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
					super::push(out, 13077);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21761);
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
					super::push(out, 13078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21762);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21763);
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
						['о', s @ ..] => {
							super::push(out, 21764);
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
							super::push(out, 21765);
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
						['ы', s @ ..] => {
							super::push(out, 21766);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30148);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 13079);
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
		['с', s @ ..] => {
			super::push(out, 5644);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5645);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 5646);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13081);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 21767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юскать.
pub(crate) fn n1770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1676);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5647);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яскать.
pub(crate) fn n1771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1677);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5648);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13083);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5649);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 13085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word фукать.
pub(crate) fn n6559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5675);
	s.is_empty()
}

/// Words ending -афукать.
pub(crate) fn n6560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5676);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -офукать.
pub(crate) fn n6561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5677);
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

/// Words ending -уфукать.
pub(crate) fn n6562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5678);
	match s {
		[] => true,
		['ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цукать.
pub(crate) fn n1792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1694);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5679);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5680);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шукать.
pub(crate) fn n1793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1695);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5681);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5682);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13109);
					match s {
						[] => true,
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
			super::push(out, 5683);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21775);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21776);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21777);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30150);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30151);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36029);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яукать.
pub(crate) fn n1794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1696);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 5684);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13110);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13111);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13112);
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

/// The word покурлыкать.
pub(crate) fn n76439(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word прокурлыкать.
pub(crate) fn n76440(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word замурлыкать.
pub(crate) fn n76441(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word намурлыкать.
pub(crate) fn n76442(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -мыкать.
pub(crate) fn n1811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1713);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5705);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5706);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13133);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21787);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5708);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13134);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 21788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5709);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13135);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30162);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30163);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36034);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13136);
					match s {
						[] => true,
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
			super::push(out, 5710);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13137);
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
			super::push(out, 5711);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5713);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13141);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21791);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ныкать.
pub(crate) fn n1812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1714);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5716);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5717);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5718);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13143);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30164);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30165);
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
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5719);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13144);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13145);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 13146);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21794);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30167);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арыкать.
pub(crate) fn n6609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5720);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 13147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -брыкать.
pub(crate) fn n6610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5721);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13148);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13149);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13150);
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

/// Words ending -урыкать.
pub(crate) fn n6611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5722);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 13151);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тыкать.
pub(crate) fn n1814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1716);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5723);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5724);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21796);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21797);
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
		['б', s @ ..] => {
			super::push(out, 5725);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 13154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13155);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13156);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13157);
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
		['д', s @ ..] => {
			super::push(out, 5727);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 13158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5728);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 13159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5729);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13160);
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
			super::push(out, 5730);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13161);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 13162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5731);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13163);
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 13164);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13165);
					match s {
						[] => true,
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
			super::push(out, 5732);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 13166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5733);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5734);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13168);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21798);
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
		['ы', s @ ..] => {
			super::push(out, 5735);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фыкать.
pub(crate) fn n1815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1717);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 5736);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хыкать.
pub(crate) fn n1816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5737);
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

/// Words ending -цыкать.
pub(crate) fn n1817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5738);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word промелькать.
pub(crate) fn n76444(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word забренькать.
pub(crate) fn n76445(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word пробренькать.
pub(crate) fn n76446(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word тренькать.
pub(crate) fn n50397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30175);
	s.is_empty()
}

/// Words ending -атренькать.
pub(crate) fn n50398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30176);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 36038);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отренькать.
pub(crate) fn n50399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30177);
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

/// Words ending -тенькать.
pub(crate) fn n17159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13178);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инькать.
pub(crate) fn n6634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5744);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13179);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 21808);
					match s {
						[] => true,
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
			super::push(out, 13180);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'и', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онькать.
pub(crate) fn n6635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5745);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13181);
			match s {
				[] => true,
				['е', 'р', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янькать.
pub(crate) fn n6636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5746);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 13182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ськать.
pub(crate) fn n1821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1723);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5747);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13183);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13184);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13185);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word заулюлюкать.
pub(crate) fn n76448(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word затюкать.
pub(crate) fn n32809(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -матюкать.
pub(crate) fn n17194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13196);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21818);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 21819);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отюкать.
pub(crate) fn n6655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5764);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 13197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттюкать.
pub(crate) fn n6656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5765);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ютюкать.
pub(crate) fn n6657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5766);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 13198);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьюкать.
pub(crate) fn n1831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1733);
	match s {
		[] => true,
		['ф', s @ ..] => {
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

/// Words ending -брякать.
pub(crate) fn n6677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5784);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13210);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13211);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21830);
					match s {
						[] => true,
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
			super::push(out, 13212);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21831);
					match s {
						[] => true,
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
			super::push(out, 13213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13214);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13215);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word крякать.
pub(crate) fn n17224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13216);
	s.is_empty()
}

/// Words ending -акрякать.
pub(crate) fn n17225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13217);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окрякать.
pub(crate) fn n17226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13218);
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

/// Words ending -сякать.
pub(crate) fn n1840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1741);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 5786);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					super::push(out, 13219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тякать.
pub(crate) fn n1841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1742);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уякать.
pub(crate) fn n1842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1743);
	match s {
		[] => true,
		['х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13220);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13221);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 21834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13222);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21835);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -позапоминать.
pub(crate) fn n76449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39641);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -авспоминать.
pub(crate) fn n65316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36042);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 39642);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 39643);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овспоминать.
pub(crate) fn n65317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36043);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 39644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оспоминать.
pub(crate) fn n50476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30208);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word проминать.
pub(crate) fn n17425(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отминать.
pub(crate) fn n6846(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word уминать.
pub(crate) fn n6847(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выминать.
pub(crate) fn n6848(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -пинать.
pub(crate) fn n1910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1805);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5899);
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
			super::push(out, 5900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5901);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13284);
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
			super::push(out, 5902);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 13285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5903);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чинать.
pub(crate) fn n1911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1806);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5904);
			match s {
				[] => true,
				['н', s @ ..] => {
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5905);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13287);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21880);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трепать.
pub(crate) fn n6905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5931);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13324);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13325);
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
			super::push(out, 13326);
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
				['р', s @ ..] => {
					super::push(out, 21903);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30212);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 13327);
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 13328);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21904);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щепать.
pub(crate) fn n1929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1824);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5932);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5933);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13330);
					match s {
						[] => true,
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
			super::push(out, 5934);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13331);
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
			super::push(out, 5935);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13332);
					match s {
						[] => true,
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
			super::push(out, 5936);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13333);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21905);
							match s {
								[] => true,
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
					super::push(out, 13334);
					match s {
						[] => true,
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
			super::push(out, 5937);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -топать.
pub(crate) fn n1939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1834);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5978);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5979);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13368);
					match s {
						[] => true,
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
			super::push(out, 5980);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13369);
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
			super::push(out, 5981);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21916);
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
			super::push(out, 5982);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5984);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13372);
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
				['с', 'и', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ступать.
pub(crate) fn n6993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6008);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21934);
					match s {
						[] => true,
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
			super::push(out, 13394);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21935);
					match s {
						[] => true,
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
			super::push(out, 13395);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13396);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13397);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21936);
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
		['и', s @ ..] => {
			super::push(out, 13398);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13399);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 21937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21938);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			super::push(out, 13400);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13401);
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
		['ы', s @ ..] => {
			super::push(out, 13402);
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

/// Words ending -ощупать.
pub(crate) fn n6998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6011);
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

/// Words ending -ыщупать.
pub(crate) fn n7000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6012);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word поразбирать.
pub(crate) fn n76454(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -вирать.
pub(crate) fn n1994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1889);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6084);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 13463);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дирать.
pub(crate) fn n1995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1890);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6085);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6086);
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
		['з', s @ ..] => {
			super::push(out, 6087);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['о', s @ ..] => {
			super::push(out, 6088);
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
			super::push(out, 6089);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13465);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 21969);
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
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6091);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21970);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30232);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жирать.
pub(crate) fn n1996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1891);
	match s {
		[] => true,
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
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зирать.
pub(crate) fn n1997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1892);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6092);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13466);
					match s {
						[] => true,
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
		['в', s @ ..] => {
			super::push(out, 6093);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6094);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21971);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30233);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
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
			super::push(out, 6095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// Words ending -мирать.
pub(crate) fn n1998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1893);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6096);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 6097);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 13467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6098);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6099);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13471);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21972);
							match s {
								[] => true,
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
					super::push(out, 13472);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6101);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21974);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30234);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пирать.
pub(crate) fn n1999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1894);
	match s {
		[] => true,
		['а', s @ ..] => {
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
					super::push(out, 13473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21975);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21976);
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

/// Words ending -сирать.
pub(crate) fn n2000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1895);
	match s {
		[] => true,
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тирать.
pub(crate) fn n2001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1896);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6102);
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
			super::push(out, 6103);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6105);
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
			super::push(out, 6106);
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
			super::push(out, 6107);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13475);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21977);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21978);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21979);
							match s {
								[] => true,
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
					super::push(out, 13476);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13477);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13479);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 21981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21982);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36047);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 36048);
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
						['с', s @ ..] => {
							super::push(out, 21983);
							match s {
								[] => true,
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
					super::push(out, 13480);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13481);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13482);
					match s {
						[] => true,
						['л', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13483);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['у', s @ ..] => {
			super::push(out, 6108);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6109);
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

/// The word наклеветать.
pub(crate) fn n76456(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поклеветать.
pub(crate) fn n76457(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word налетать.
pub(crate) fn n33369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22031);
	s.is_empty()
}

/// Words ending -блетать.
pub(crate) fn n7336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6221);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13583);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -влетать.
pub(crate) fn n7337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6222);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -длетать.
pub(crate) fn n7338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6223);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13584);
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

/// Words ending -елетать.
pub(crate) fn n7339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6224);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			super::push(out, 13585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -злетать.
pub(crate) fn n7340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6225);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13587);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 22033);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -илетать.
pub(crate) fn n7341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6226);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 13588);
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

/// Words ending -олетать.
pub(crate) fn n7342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6227);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13590);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13591);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плетать.
pub(crate) fn n7343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6228);
	match s {
		[] => true,
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
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
				['д', '\u{1}', s @ ..] => {
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

/// Words ending -слетать.
pub(crate) fn n7344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6229);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тлетать.
pub(crate) fn n7345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6230);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13592);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22036);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улетать.
pub(crate) fn n7346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6231);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ылетать.
pub(crate) fn n7347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6232);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22037);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22038);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30251);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -метать.
pub(crate) fn n2057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1952);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6233);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6234);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 13594);
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 13595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6235);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13596);
					match s {
						[] => true,
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
			super::push(out, 6236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13598);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 22039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6238);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13599);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 22040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 13600);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6240);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6241);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13601);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13603);
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
			super::push(out, 6242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6243);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 13605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6245);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нетать.
pub(crate) fn n2058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1953);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6246);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
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

/// Words ending -петать.
pub(crate) fn n2059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1954);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6247);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13607);
					match s {
						[] => true,
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
			super::push(out, 6248);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22043);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22044);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22045);
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
				['р', s @ ..] => {
					super::push(out, 13609);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 22046);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30252);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30253);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30254);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30255);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36052);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 36053);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретать.
pub(crate) fn n2060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1955);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6249);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -четать.
pub(crate) fn n2061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1956);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6250);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word запричитать.
pub(crate) fn n76460(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отпричитать.
pub(crate) fn n76461(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -считать.
pub(crate) fn n7385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6269);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13617);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13618);
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
			super::push(out, 13619);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13620);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 30261);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 36056);
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
			super::push(out, 13621);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отчитать.
pub(crate) fn n18117(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ычитать.
pub(crate) fn n7387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6271);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыготать.
pub(crate) fn n7400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6282);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -котать.
pub(crate) fn n2071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1966);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6283);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13634);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 22077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30270);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36057);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13635);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30271);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30272);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36058);
											match s {
												[] => true,
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
									super::push(out, 30273);
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
									super::push(out, 30274);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 36059);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 13636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22078);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22079);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 22080);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30275);
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
							super::push(out, 22081);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22082);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30276);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6284);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22083);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22084);
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
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22085);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22086);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22087);
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
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22088);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22089);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22090);
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
							super::push(out, 22091);
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
				['ц', s @ ..] => {
					super::push(out, 13637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6285);
			match s {
				[] => true,
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лотать.
pub(crate) fn n2072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1967);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13638);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13639);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22092);
							match s {
								[] => true,
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
					super::push(out, 13640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22093);
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
					super::push(out, 13641);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22094);
							match s {
								[] => true,
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
					super::push(out, 13642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13643);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мотать.
pub(crate) fn n2073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1968);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6287);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6288);
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
			super::push(out, 6289);
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6290);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6291);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13644);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30277);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36060);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13645);
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
			super::push(out, 6292);
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
		['р', s @ ..] => {
			super::push(out, 6293);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13646);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30278);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30279);
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
									super::push(out, 30280);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 36061);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													super::push(out, 39646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30281);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36062);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6294);
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
			super::push(out, 6295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6296);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13647);
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

/// Words ending -потать.
pub(crate) fn n2074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1969);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6297);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22096);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22097);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30282);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 22098);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22099);
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
						['х', s @ ..] => {
							super::push(out, 22100);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30283);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30284);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36063);
											match s {
												[] => true,
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
									super::push(out, 30285);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36064);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 36065);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 36066);
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
									super::push(out, 30287);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 30288);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30289);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 36067);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13649);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22101);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22102);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30290);
									match s {
										[] => true,
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
							super::push(out, 22103);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротать.
pub(crate) fn n2075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1970);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6298);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6299);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22104);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22105);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30292);
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
							super::push(out, 22106);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фотать.
pub(crate) fn n2076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1971);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6300);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотать.
pub(crate) fn n2077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1972);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6302);
			match s {
				[] => true,
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22107);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22108);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22109);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22110);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22111);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22112);
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

/// Words ending -шотать.
pub(crate) fn n2078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1973);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 6303);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13652);
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
		_ => false,
	}
}

/// Words ending -растать.
pub(crate) fn n7436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6314);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13670);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 30299);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 36068);
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
							super::push(out, 30300);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 36069);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			super::push(out, 13671);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 13672);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13673);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22127);
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
		['е', s @ ..] => {
			super::push(out, 13674);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13675);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13676);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22129);
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
				['р', s @ ..] => {
					super::push(out, 22130);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 30301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22131);
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

/// The word встать.
pub(crate) fn n7439(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ивстать.
pub(crate) fn n7440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6317);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word повстать.
pub(crate) fn n7441(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -дстать.
pub(crate) fn n2086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6318);
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

/// Words ending -естать.
pub(crate) fn n2087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1982);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6319);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22134);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22135);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30302);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 30303);
									match s {
										[] => true,
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
							super::push(out, 22136);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30304);
									match s {
										[] => true,
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
							super::push(out, 22137);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30305);
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
							super::push(out, 22138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 30306);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 36070);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 30307);
									match s {
										[] => true,
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
							super::push(out, 22139);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30308);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 30309);
									match s {
										[] => true,
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
							super::push(out, 22140);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30310);
									match s {
										[] => true,
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
							super::push(out, 22141);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22142);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30311);
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
		['п', s @ ..] => {
			super::push(out, 6320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6321);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13682);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -истать.
pub(crate) fn n2088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1983);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6322);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22143);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22144);
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
							super::push(out, 22145);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30312);
									match s {
										[] => true,
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
						['и', s @ ..] => {
							super::push(out, 22146);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30313);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36071);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 22147);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22148);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30314);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13684);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13685);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 13686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22149);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22150);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13687);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22151);
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
					super::push(out, 13688);
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13689);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22152);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22153);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30315);
									match s {
										[] => true,
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
							super::push(out, 22154);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30316);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 13691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22155);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30317);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остать.
pub(crate) fn n2089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1984);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6325);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13692);
					match s {
						[] => true,
						['т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13693);
					match s {
						[] => true,
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
			super::push(out, 6326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13694);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13695);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22156);
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6327);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13697);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 22159);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30318);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рстать.
pub(crate) fn n2090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1985);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6328);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22160);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22161);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22162);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 30319);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36072);
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
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22164);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30320);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сстать.
pub(crate) fn n2091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1986);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6329);
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

/// Words ending -тстать.
pub(crate) fn n2092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1987);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6330);
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устать.
pub(crate) fn n2093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1988);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6331);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 6332);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 13698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстать.
pub(crate) fn n2094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1989);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6333);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13699);
					match s {
						[] => true,
						['л', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6334);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22165);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22166);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30321);
									match s {
										[] => true,
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
							super::push(out, 22167);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30322);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юстать.
pub(crate) fn n2095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1990);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6335);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 13700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22168);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22169);
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

/// Words ending -ястать.
pub(crate) fn n2096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1991);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6336);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6337);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word понабрехать.
pub(crate) fn n76477(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word утухать.
pub(crate) fn n18423(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -чухать.
pub(crate) fn n2158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2049);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6447);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6448);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13787);
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
			super::push(out, 6449);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13788);
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
		_ => false,
	}
}

/// The word провздыхать.
pub(crate) fn n76478(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -омыхать.
pub(crate) fn n7633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6463);
	match s {
		[] => true,
		['р', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22216);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22217);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30337);
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
					super::push(out, 22218);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пыхать.
pub(crate) fn n2165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2056);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6464);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6465);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22219);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30338);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6466);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13801);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13802);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыхать.
pub(crate) fn n2166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2057);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13803);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13804);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13805);
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

/// Words ending -сыхать.
pub(crate) fn n2167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2058);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13806);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22220);
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
		['д', s @ ..] => {
			super::push(out, 6468);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6469);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6470);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13808);
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
			super::push(out, 6471);
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
			super::push(out, 6472);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					super::push(out, 13809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13810);
					match s {
						[] => true,
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
			super::push(out, 6474);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6475);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22221);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30339);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тыхать.
pub(crate) fn n2168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2059);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6476);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13811);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13812);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30340);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30341);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36075);
											match s {
												[] => true,
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
									super::push(out, 30342);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36076);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word припиздюхать.
pub(crate) fn n76481(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word повытряхать.
pub(crate) fn n76482(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заподличать.
pub(crate) fn n76484(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word наподличать.
pub(crate) fn n76485(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ничать.
pub(crate) fn n2203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2093);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6528);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13849);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22247);
							match s {
								[] => true,
								['п', 'т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13850);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['х', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30356);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 13851);
					match s {
						[] => true,
						['л', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30357);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30358);
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
				['и', s @ ..] => {
					super::push(out, 13852);
					match s {
						[] => true,
						['л', 'у', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13853);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22248);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13854);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22249);
							match s {
								[] => true,
								['н', 'е', 'о', 'в', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', 'у', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 22250);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 30359);
									match s {
										[] => true,
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
				['е', s @ ..] => {
					super::push(out, 13855);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22251);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 30360);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 36082);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 36083);
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
						['ж', s @ ..] => {
							super::push(out, 22252);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30361);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36084);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 22253);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30362);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30363);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 36085);
											match s {
												[] => true,
												['х', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13856);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22254);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['т', s @ ..] => super::t19::n33832(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t19::n18554(s, out),
				['р', s @ ..] => super::t19::n18555(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t19::n7738(s, out),
		['е', s @ ..] => super::t19::n7739(s, out),
		['ж', s @ ..] => super::t19::n7740(s, out),
		['з', s @ ..] => super::t19::n7741(s, out),
		['й', s @ ..] => super::t19::n7742(s, out),
		['м', s @ ..] => super::t19::n7743(s, out),
		['н', s @ ..] => super::t20::n7744(s, out),
		['р', s @ ..] => super::t20::n7745(s, out),
		['с', s @ ..] => super::t20::n7746(s, out),
		['т', s @ ..] => super::t20::n7747(s, out),
		['у', s @ ..] => super::t20::n7748(s, out),
		['ч', s @ ..] => super::t20::n7749(s, out),
		['ш', s @ ..] => super::t20::n7750(s, out),
		['щ', s @ ..] => super::t20::n7751(s, out),
		['ь', s @ ..] => super::t20::n7752(s, out),
		_ => false,
	}
}

/// Words ending -ручать.
pub(crate) fn n2237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2125);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', s @ ..] => {
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
		['ы', 'в', s @ ..] => {
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

/// Words ending -исучать.
pub(crate) fn n7824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6585);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14000);
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

/// Words ending -тучать.
pub(crate) fn n2239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2127);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6587);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14001);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14002);
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
				['о', s @ ..] => {
					super::push(out, 14003);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14004);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пообветшать.
pub(crate) fn n76707(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поизветшать.
pub(crate) fn n76708(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -затрещать.
pub(crate) fn n34308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22484);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -натрещать.
pub(crate) fn n34309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22485);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отрещать.
pub(crate) fn n18918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14067);
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

/// Words ending -сещать.
pub(crate) fn n2292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2177);
	s.is_empty()
}

/// The word покоричневеть.
pub(crate) fn n76711(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заглянцеветь.
pub(crate) fn n76712(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ешеветь.
pub(crate) fn n8037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6710);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22503);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22504);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новеть.
pub(crate) fn n2357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2230);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6731);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6732);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14136);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30552);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30553);
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
		['о', s @ ..] => {
			super::push(out, 6733);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6734);
			match s {
				[] => true,
				['е', 'д', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роветь.
pub(crate) fn n2358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2231);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14137);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30554);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30555);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30556);
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
		['к', s @ ..] => {
			super::push(out, 6736);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6737);
			match s {
				[] => true,
				['д', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22520);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22521);
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
						['ы', s @ ..] => {
							super::push(out, 22522);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30557);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6738);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22523);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22524);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30558);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14140);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -советь.
pub(crate) fn n2359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2232);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6740);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14141);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14142);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моховеть.
pub(crate) fn n8081(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14144);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14145);
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

/// Words ending -цоветь.
pub(crate) fn n2361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2234);
	match s {
		[] => true,
		['н', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14146);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22525);
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
				['у', s @ ..] => {
					super::push(out, 14147);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30559);
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

/// The word возненавидеть.
pub(crate) fn n76713(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word подледенеть.
pub(crate) fn n34602(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оледенеть.
pub(crate) fn n34603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22598);
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

/// Words ending -зденеть.
pub(crate) fn n8356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6950);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14301);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22599);
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

/// Words ending -оденеть.
pub(crate) fn n8357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6951);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уденеть.
pub(crate) fn n8358(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22600);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ленеть.
pub(crate) fn n2520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2372);
	match s {
		[] => true,
		['е', 'з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14303);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14304);
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
		['к', 'е', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14305);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -менеть.
pub(crate) fn n2521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2373);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6952);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22601);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22602);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22603);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22604);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22605);
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
		['е', s @ ..] => {
			super::push(out, 6953);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14306);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30578);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30579);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36250);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенеть.
pub(crate) fn n2522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2374);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14307);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 22606);
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
				['ц', s @ ..] => {
					super::push(out, 14308);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22607);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22608);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренеть.
pub(crate) fn n2523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2375);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6954);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14309);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 14310);
					match s {
						[] => true,
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
			super::push(out, 6955);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6956);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6957);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенеть.
pub(crate) fn n2524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2376);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6958);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14313);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенеть.
pub(crate) fn n2525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2377);
	match s {
		[] => true,
		['с', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14314);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
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

/// Words ending -фенеть.
pub(crate) fn n2526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2378);
	match s {
		[] => true,
		['р', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14315);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченеть.
pub(crate) fn n2527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2379);
	match s {
		[] => true,
		['о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14317);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14318);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word запасмурнеть.
pub(crate) fn n76715(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тереть.
pub(crate) fn n2633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2472);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7077);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7078);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22650);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
				['н', s @ ..] => {
					super::push(out, 14437);
					match s {
						[] => true,
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
			super::push(out, 7079);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14438);
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
			super::push(out, 7080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7081);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14439);
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
			super::push(out, 7082);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14440);
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
			super::push(out, 7083);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14441);
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
		['с', s @ ..] => {
			super::push(out, 7085);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14442);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14443);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22651);
							match s {
								[] => true,
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
					super::push(out, 14444);
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
				['о', s @ ..] => {
					super::push(out, 14445);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22652);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7086);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14446);
					match s {
						[] => true,
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
			super::push(out, 7087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7088);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -охереть.
pub(crate) fn n8562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7089);
	s.is_empty()
}

/// Words ending -череть.
pub(crate) fn n2635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2474);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7090);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22653);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22654);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30592);
									match s {
										[] => true,
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
							super::push(out, 22655);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30593);
									match s {
										[] => true,
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
							super::push(out, 22656);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поприсмотреть.
pub(crate) fn n76716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39742);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -естреть.
pub(crate) fn n8617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7129);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 14467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22678);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30599);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остреть.
pub(crate) fn n8618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7130);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14468);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устреть.
pub(crate) fn n8619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7131);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 14469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 14470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстреть.
pub(crate) fn n8620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7132);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14471);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утреть.
pub(crate) fn n2664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2498);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 7133);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word поотвертеть.
pub(crate) fn n76718(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word зашелестеть.
pub(crate) fn n76719(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word пошелестеть.
pub(crate) fn n76720(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рстеть.
pub(crate) fn n2702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2535);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 7190);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 14514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устеть.
pub(crate) fn n2703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2536);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14515);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14516);
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
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14517);
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14518);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14519);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7193);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 14521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22708);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22709);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 22710);
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

/// Words ending -лоупотребить.
pub(crate) fn n76721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39743);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -робить.
pub(crate) fn n2782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2597);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7252);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7253);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14597);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 14598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14599);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14600);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14601);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14602);
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
				['о', s @ ..] => {
					super::push(out, 14603);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30620);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7256);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7257);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22739);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7260);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 14605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22741);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22742);
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
		['у', s @ ..] => {
			super::push(out, 7261);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -способить.
pub(crate) fn n35013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22744);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 30621);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 30622);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рубить.
pub(crate) fn n2796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2609);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7291);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7292);
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
			super::push(out, 7293);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14634);
					match s {
						[] => true,
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
			super::push(out, 7294);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14635);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14636);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 22765);
							match s {
								[] => true,
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
					super::push(out, 14637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22766);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 22768);
							match s {
								[] => true,
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
					super::push(out, 14638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14639);
					match s {
						[] => true,
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
			super::push(out, 7296);
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
			super::push(out, 7297);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7298);
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
		['и', s @ ..] => {
			super::push(out, 7299);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7300);
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
			super::push(out, 7301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14640);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14641);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 22770);
							match s {
								[] => true,
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
					super::push(out, 14642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22771);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22772);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22773);
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
					super::push(out, 14643);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 22774);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30625);
									match s {
										[] => true,
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
							super::push(out, 22775);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 14644);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22776);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7304);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14645);
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

/// The word выдавить.
pub(crate) fn n35088(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -жавить.
pub(crate) fn n2809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2622);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7332);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14660);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14661);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22786);
							match s {
								[] => true,
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
					super::push(out, 14662);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22787);
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

/// Words ending -завить.
pub(crate) fn n2810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2623);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7333);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7334);
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

/// Words ending -кавить.
pub(crate) fn n2811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2624);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7335);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14663);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30627);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30628);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7336);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22788);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22789);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30629);
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
						['с', s @ ..] => {
							super::push(out, 22790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лавить.
pub(crate) fn n2812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2625);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7337);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 7338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14664);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14665);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 14666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14667);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14668);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14669);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22791);
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
					super::push(out, 14670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 22792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 22793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22794);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30632);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 14671);
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
				['ы', s @ ..] => {
					super::push(out, 14672);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14673);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14675);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30633);
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -навить.
pub(crate) fn n2813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2626);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -равить.
pub(crate) fn n2814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2627);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 7341);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7342);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 7343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14678);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14679);
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
				['и', s @ ..] => {
					super::push(out, 14680);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14681);
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14682);
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
								['о', 'п', '\u{1}', s @ ..] => {
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
								['о', 'п', '\u{1}', s @ ..] => {
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
				['у', s @ ..] => {
					super::push(out, 14683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14684);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22796);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7344);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14685);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14686);
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
				['е', s @ ..] => {
					super::push(out, 14687);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 22797);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 30634);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 22798);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30635);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36261);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													super::push(out, 39744);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
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
					super::push(out, 14689);
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
				['ы', s @ ..] => {
					super::push(out, 14690);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22799);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30636);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 36262);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7345);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22800);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22801);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 22802);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30637);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t20::n20064(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -савить.
pub(crate) fn n2815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2628);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 7346);
			match s {
				[] => true,
				['н', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22808);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22809);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30642);
									match s {
										[] => true,
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
							super::push(out, 22810);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30643);
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

/// Words ending -тавить.
pub(crate) fn n2816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2629);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14693);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30644);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36263);
											match s {
												[] => true,
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
									super::push(out, 30645);
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7348);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14694);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14695);
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
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14696);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
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
					super::push(out, 14697);
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
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 22811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', 'и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 36264);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 22812);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 30646);
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 14698);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30647);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30648);
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

/// Words ending -чавить.
pub(crate) fn n2817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2630);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7349);
			match s {
				[] => true,
				['у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22814);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22815);
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

/// Words ending -шавить.
pub(crate) fn n2818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2631);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 7350);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14699);
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

/// The word посчастливить.
pub(crate) fn n76727(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word покреативить.
pub(crate) fn n76728(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ешивить.
pub(crate) fn n8983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7376);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14721);
			match s {
				[] => true,
				['п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ршивить.
pub(crate) fn n8984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7377);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14722);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30655);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30656);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьшивить.
pub(crate) fn n8985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7378);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14723);
			match s {
				[] => true,
				['а', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30657);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 30658);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 30659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ословить.
pub(crate) fn n20145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14738);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30663);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36267);
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
				['т', s @ ..] => {
					super::push(out, 30664);
					match s {
						[] => true,
						['и', 'л', 'о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22832);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30665);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36268);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30666);
					match s {
						[] => true,
						['н', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22833);
			match s {
				[] => true,
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 30667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22834);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 36269);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 39745);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'а', 'р', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 22835);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30668);
					match s {
						[] => true,
						['з', 'а', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30669);
					match s {
						[] => true,
						['е', 'в', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 39746);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 39747);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 41624);
											match s {
												[] => true,
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
									super::push(out, 39748);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 41625);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'т', 'с', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22836);
			match s {
				[] => true,
				['с', 'у', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 36270);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 39749);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обусловить.
pub(crate) fn n51685(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 36271);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39750);
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

/// Words ending -тловить.
pub(crate) fn n8998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7390);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word уловить.
pub(crate) fn n20148(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыловить.
pub(crate) fn n9000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7392);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14741);
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

/// Words ending -новить.
pub(crate) fn n2845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2657);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7393);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7394);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14742);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30670);
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
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7395);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7396);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14744);
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
			super::push(out, 7397);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7398);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14746);
					match s {
						[] => true,
						['д', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7399);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повить.
pub(crate) fn n2846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2658);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровить.
pub(crate) fn n2847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2659);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 7401);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14748);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14749);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22838);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7402);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14750);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 22839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22840);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 22841);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30671);
									match s {
										[] => true,
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
							super::push(out, 22842);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30672);
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
							super::push(out, 22843);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30673);
									match s {
										[] => true,
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
							super::push(out, 22844);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30674);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 30675);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 30676);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7403);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22845);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22846);
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
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совить.
pub(crate) fn n2848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2660);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7404);
			s.is_empty()
		}
		_ => false,
	}
}

