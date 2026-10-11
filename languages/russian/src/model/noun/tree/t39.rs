use flexion::Path;

/// Words ending -сорт.
pub(crate) fn n17048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16949);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40392);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 40393);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68462);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 93995);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113155);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 125507);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40394);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68463);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93996);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113156);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 113157);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125508);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132969);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138820(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торт.
pub(crate) fn n17049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16950);
	s.is_empty()
}

/// Words ending -форт.
pub(crate) fn n17050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16951);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40395);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 40396);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68464);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93997);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113158);
									s.is_empty()
								}
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
			super::push(out, 40397);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 40398);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68465);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 93998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хорт.
pub(crate) fn n17051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16952);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40399);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чорт.
pub(crate) fn n17052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16953);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шорт.
pub(crate) fn n17053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16954);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льт-цербст.
pub(crate) fn n134342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132972);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137403);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word благовест.
pub(crate) fn n134343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132973);
	s.is_empty()
}

/// The word перекрест.
pub(crate) fn n134344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132974);
	s.is_empty()
}

/// Words ending -ефест.
pub(crate) fn n40864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40455);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 68515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94029);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ифест.
pub(crate) fn n40865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40456);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 68516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цест.
pub(crate) fn n17087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16988);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 40457);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68517);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шест.
pub(crate) fn n17088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16989);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40458);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 40459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --примитивист.
pub(crate) fn n134345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132975);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137404);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 140060);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141632);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 142542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ик-архивист.
pub(crate) fn n134346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132976);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137405);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140061);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141633);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 142543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word виндсерфингист.
pub(crate) fn n114391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113184);
	s.is_empty()
}

/// The word скайсерфингист.
pub(crate) fn n114392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113185);
	s.is_empty()
}

/// The word сапсерфингист.
pub(crate) fn n114393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113186);
	s.is_empty()
}

/// The word кайтсерфингист.
pub(crate) fn n114394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113187);
	s.is_empty()
}

/// Words ending -ргист.
pub(crate) fn n40880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40471);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 68529);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 94041);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113188);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125519);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132977);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дист.
pub(crate) fn n17093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16994);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40472);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68530);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 94042);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113189);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125520);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 132978);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138825(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40473);
			match s {
				[] => true,
				['а', 'г', s @ ..] => {
					super::push(out, 68531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40474);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 68532);
					match s {
						[] => true,
						['е', 'м', '-', s @ ..] => {
							super::push(out, 94043);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113190);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125521);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132979);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138826(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еист.
pub(crate) fn n17094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16995);
	s.is_empty()
}

/// Words ending -жист.
pub(crate) fn n17095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16996);
	s.is_empty()
}

/// Words ending -зист.
pub(crate) fn n17096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16997);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40476);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68533);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94044);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113191);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125522);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 132980);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138827(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кист.
pub(crate) fn n17097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16998);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 40477);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 68534);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лист.
pub(crate) fn n17098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16999);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40478);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 40479);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68535);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 68536);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 68537);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68538);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 40480);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68540);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94050);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113193);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125523);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132981);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68541);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 68542);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94051);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 113194);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125524);
											match s {
												[] => true,
												['р', 'к', '-', s @ ..] => {
													super::push(out, 132982);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138828(s, out),
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
									super::push(out, 113195);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 125525);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132983);
													match s {
														[] => true,
														['д', s @ ..] => super::t48::n138829(s, out),
														_ => false,
													}
												}
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
							super::push(out, 94052);
							match s {
								[] => true,
								['и', 'ц', 'а', 'н', s @ ..] => {
									super::push(out, 113196);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125526);
											match s {
												[] => true,
												['е', 'т', 'н', 'и', '-', s @ ..] => {
													super::push(out, 132984);
													match s {
														[] => true,
														['н', s @ ..] => super::t48::n138830(s, out),
														_ => false,
													}
												}
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
							super::push(out, 94053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68543);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 68544);
					match s {
						[] => true,
						['н', 'е', 'м', 'а', s @ ..] => {
							super::push(out, 94054);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 113197);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125527);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132985);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138831(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 68545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 40481);
			match s {
				[] => true,
				['а', 'т', 'н', 'р', 'е', 'т', '\u{1}', s @ ..] => {
					super::push(out, 68546);
					s.is_empty()
				}
				['й', 'е', 'т', 'н', 'р', 'е', 'т', '\u{1}', s @ ..] => {
					super::push(out, 68547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 40482);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40483);
			match s {
				[] => true,
				['н', 'и', 'ж', 'д', 'р', 'е', 'в', '\u{1}', s @ ..] => {
					super::push(out, 68549);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 68550);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94055);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113198);
									match s {
										[] => true,
										['л', 'о', 'и', 'в', '\u{1}', s @ ..] => {
											super::push(out, 125528);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t48::n40893(s, out),
		['к', s @ ..] => super::t48::n40894(s, out),
		['л', s @ ..] => super::t48::n40895(s, out),
		['о', s @ ..] => super::t48::n40896(s, out),
		['у', s @ ..] => {
			super::push(out, 40488);
			match s {
				[] => true,
				['л', s @ ..] => super::t48::n69284(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мист.
pub(crate) fn n17099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17000);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 40489);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 40490);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68564);
					match s {
						[] => true,
						['р', 'г', 'о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 113210);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125538);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132991);
													match s {
														[] => true,
														['н', s @ ..] => super::t48::n138836(s, out),
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
									super::push(out, 113211);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125539);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 132992);
													match s {
														[] => true,
														['ь', s @ ..] => super::t48::n138837(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40491);
			match s {
				[] => true,
				['н', 'о', 'к', s @ ..] => {
					super::push(out, 68565);
					match s {
						[] => true,
						['э', '-', s @ ..] => {
							super::push(out, 94068);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113212);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125540);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132993);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138838(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нист.
pub(crate) fn n17100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40493);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 40494);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68566);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94069);
							match s {
								[] => true,
								['а', 'д', '\u{1}', s @ ..] => {
									super::push(out, 113213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 68567);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 68568);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94070);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 113214);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 125541);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 132994);
													match s {
														[] => true,
														['л', s @ ..] => super::t48::n138839(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 68569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94071);
							match s {
								[] => true,
								['м', 'о', s @ ..] => {
									super::push(out, 113215);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125542);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132995);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138840(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40495);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40496);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					super::push(out, 68571);
					match s {
						[] => true,
						['о', 'м', 'т', s @ ..] => {
							super::push(out, 94072);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113216);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125543);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 132996);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138841(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40497);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68572);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94073);
							match s {
								[] => true,
								['п', '-', s @ ..] => {
									super::push(out, 113217);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125544);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132997);
													match s {
														[] => true,
														['н', s @ ..] => super::t48::n138842(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оист.
pub(crate) fn n17101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17002);
	s.is_empty()
}

/// Words ending -пист.
pub(crate) fn n17102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17003);
	s.is_empty()
}

/// Words ending -рист.
pub(crate) fn n17103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17004);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68573);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 68574);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94074);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40499);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 40500);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 40501);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 40502);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 40503);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68577);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94076);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 68578);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94077);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 113220);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125545);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сист.
pub(crate) fn n17104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17005);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 40504);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40505);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тист.
pub(crate) fn n17105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17006);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40506);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 68581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94079);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 94080);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 113221);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125546);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132998);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'м', s @ ..] => {
			super::push(out, 40507);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94081);
							s.is_empty()
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
					super::push(out, 68584);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94082);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113222);
									match s {
										[] => true,
										['п', 'с', 'э', '\u{1}', s @ ..] => {
											super::push(out, 125547);
											s.is_empty()
										}
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
					super::push(out, 68585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 40508);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 68586);
					match s {
						[] => true,
						['а', 'р', 'а', 'п', '-', s @ ..] => {
							super::push(out, 94083);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 113223);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 125548);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132999);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138843(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фист.
pub(crate) fn n17106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17007);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40509);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68587);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94084);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68588);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 40510);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 68589);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хист.
pub(crate) fn n17107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17008);
	s.is_empty()
}

/// Words ending -цист.
pub(crate) fn n17108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17009);
	s.is_empty()
}

/// Words ending -чист.
pub(crate) fn n17109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17010);
	s.is_empty()
}

/// Words ending -шист.
pub(crate) fn n17110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17011);
	s.is_empty()
}

/// Words ending -виброхвост.
pub(crate) fn n134370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133000);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137425);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word лисохвост.
pub(crate) fn n134371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133001);
	s.is_empty()
}

/// Words ending -пост.
pub(crate) fn n17125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17026);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40536);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 40537);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68610);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94101);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113234);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125552);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40538);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 40539);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 68612);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 94102);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113235);
									s.is_empty()
								}
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
			super::push(out, 40540);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40541);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 40542);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68616);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68617);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94105);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113236);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125553);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40543);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68618);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94106);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113237);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 125554);
											s.is_empty()
										}
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
						['в', s @ ..] => {
							super::push(out, 94107);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'и', 'л', 'д', '\u{1}', s @ ..] => {
							super::push(out, 94108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94109);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113239);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 125555);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 94110);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113240);
									s.is_empty()
								}
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
			super::push(out, 40544);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68619);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94111);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 113241);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125556);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133002);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68620);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 94112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 40545);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68621);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 94113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 40546);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 68622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ворост.
pub(crate) fn n69350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68627);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 94115);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113242);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -норост.
pub(crate) fn n69351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68628);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 94116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ророст.
pub(crate) fn n69352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68629);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 94117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ырост.
pub(crate) fn n40964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40553);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 68630);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94118);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сост.
pub(crate) fn n17127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17028);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 40554);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тост.
pub(crate) fn n17128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17029);
	s.is_empty()
}

/// Words ending -хост.
pub(crate) fn n17129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17030);
	s.is_empty()
}

/// Words ending -рандахлыст.
pub(crate) fn n134373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133003);
	match s {
		[] => true,
		['б', '\u{1}', s @ ..] => {
			super::push(out, 137426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оннектикут.
pub(crate) fn n134375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133005);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ман-диспут.
pub(crate) fn n134376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133006);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137428);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word проститут.
pub(crate) fn n134377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133007);
	s.is_empty()
}

/// Words ending -граф.
pub(crate) fn n17520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17421);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40943);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 40944);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68905);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 68906);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94283);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 113327);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125597);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133018);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 68907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94284);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113328);
									s.is_empty()
								}
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
			super::push(out, 40945);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 40946);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68909);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 94286);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113329);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125598);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40947);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 68910);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94287);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 68911);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 94288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 68912);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 94289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40948);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 68913);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94290);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68914);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94291);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113332);
									s.is_empty()
								}
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
			super::push(out, 40949);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 68915);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 68916);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94292);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 113333);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125599);
											s.is_empty()
										}
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
					super::push(out, 68917);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94293);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113334);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125600);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 94294);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 68918);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 94295);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113336);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 125601);
											s.is_empty()
										}
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
					super::push(out, 68919);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 94296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68920);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 68921);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94298);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113337);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125602);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133019);
													s.is_empty()
												}
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
							super::push(out, 94299);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113338);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125603);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 40950);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94300);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 113339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t48::n41362(s, out),
		['р', s @ ..] => {
			super::push(out, 40952);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 40953);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 40954);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 68941);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94349);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113392);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 125646);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133040);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранц-иосиф.
pub(crate) fn n134411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133041);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бульмастиф.
pub(crate) fn n134412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133042);
	s.is_empty()
}

/// Words ending -т-продавец.
pub(crate) fn n134427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133057);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -репродавец.
pub(crate) fn n134428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133058);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137453);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 140084);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гопродавец.
pub(crate) fn n134429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133059);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137454);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word вроцлавец.
pub(crate) fn n134430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133060);
	s.is_empty()
}

/// Words ending -евец.
pub(crate) fn n17871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17770);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41312);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 69226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41313);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 41314);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 41315);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69227);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94518);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113476);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125693);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 133061);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69228);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 69229);
					match s {
						[] => true,
						['е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 94520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 69230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94521);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113477);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125694);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133062);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41316);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69231);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94522);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 94523);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 113478);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125695);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 133063);
													s.is_empty()
												}
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
							super::push(out, 94524);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 113479);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125696);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133064);
													s.is_empty()
												}
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
							super::push(out, 94525);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113480);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125697);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133065);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69232);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94526);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113481);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 125698);
											s.is_empty()
										}
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
					super::push(out, 69233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 69234);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94528);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113482);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125699);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41317);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69235);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 94529);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 94530);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 94531);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 113483);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 125700);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133066);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69236);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 94532);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113484);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125701);
											s.is_empty()
										}
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
					super::push(out, 69237);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 69238);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94534);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 113485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 69239);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94535);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113486);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125702);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41318);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 69240);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 41319);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 69241);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94537);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 94538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69242);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94539);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 94540);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 94541);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t48::n69968(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t48::n41735(s, out),
		['с', s @ ..] => super::t48::n41736(s, out),
		['т', s @ ..] => super::t48::n41737(s, out),
		['у', s @ ..] => super::t48::n41738(s, out),
		['ц', s @ ..] => super::t48::n41739(s, out),
		['ч', s @ ..] => super::t48::n41740(s, out),
		['ш', s @ ..] => super::t48::n41741(s, out),
		['щ', s @ ..] => super::t48::n41742(s, out),
		['ь', s @ ..] => super::t48::n41743(s, out),
		['я', s @ ..] => super::t48::n41744(s, out),
		_ => false,
	}
}

/// Words ending -отливец.
pub(crate) fn n95575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94555);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 113501);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 125711);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 133069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енивец.
pub(crate) fn n69982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69257);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 94557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тивец.
pub(crate) fn n41750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41335);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 69258);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94558);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 113502);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 94559);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113503);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125713);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 133070);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шивец.
pub(crate) fn n41751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41336);
	s.is_empty()
}

/// Words ending -овец.
pub(crate) fn n17873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17772);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41337);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69259);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 69260);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94560);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113504);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125714);
											s.is_empty()
										}
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
					super::push(out, 69261);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 41338);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69262);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94562);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 113505);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125715);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133071);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69263);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94563);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113506);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125716);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 133072);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69264);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94564);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 69265);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94565);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113508);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 125717);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133073);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n138875(s, out),
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
									super::push(out, 113509);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125718);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 133074);
													s.is_empty()
												}
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
							super::push(out, 94566);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113510);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125719);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133075);
													match s {
														[] => true,
														['ь', s @ ..] => super::t48::n138876(s, out),
														_ => false,
													}
												}
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
							super::push(out, 94567);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113511);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 125720);
											s.is_empty()
										}
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
					super::push(out, 69266);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94568);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 69267);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94569);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 69268);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 94570);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113514);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125721);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41339);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69269);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 69270);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 69271);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 69272);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 69273);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94571);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113515);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125722);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133076);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69274);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94572);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113516);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 125723);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 133077);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138877(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 69275);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 69276);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 69277);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94573);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 113517);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t48::n127024(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t48::n70003(s, out),
				['я', s @ ..] => super::t48::n70004(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t48::n41755(s, out),
		['й', s @ ..] => super::t48::n41756(s, out),
		['к', s @ ..] => super::t48::n41757(s, out),
		['л', s @ ..] => super::t48::n41758(s, out),
		['м', s @ ..] => super::t48::n41759(s, out),
		['н', s @ ..] => super::t48::n41760(s, out),
		['п', s @ ..] => super::t48::n41761(s, out),
		['р', s @ ..] => super::t48::n41762(s, out),
		['с', s @ ..] => super::t48::n41763(s, out),
		['т', s @ ..] => super::t48::n41764(s, out),
		['ф', s @ ..] => super::t48::n41765(s, out),
		['х', s @ ..] => super::t48::n41766(s, out),
		['ш', s @ ..] => super::t48::n41767(s, out),
		_ => false,
	}
}

/// Words ending -рвец.
pub(crate) fn n17874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17773);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41353);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 69359);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 94684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твец.
pub(crate) fn n17875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17774);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 41354);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69360);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 94685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -швец.
pub(crate) fn n17876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17775);
	s.is_empty()
}

/// Words ending -явец.
pub(crate) fn n17877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17776);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41355);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69361);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 94686);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113616);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 125797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инчепингец.
pub(crate) fn n134496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133126);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аугсбургец.
pub(crate) fn n134497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133127);
	s.is_empty()
}

/// Words ending -икоустюгец.
pub(crate) fn n134498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133128);
	s.is_empty()
}

/// Words ending -мармеладец.
pub(crate) fn n134499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133129);
	s.is_empty()
}

/// Words ending -армагеддец.
pub(crate) fn n134500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133130);
	s.is_empty()
}

/// Words ending -дортмундец.
pub(crate) fn n134501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133131);
	s.is_empty()
}

/// Words ending -одец.
pub(crate) fn n17887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17786);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 41376);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69379);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 94704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 69380);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94705);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113628);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125808);
											s.is_empty()
										}
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
					super::push(out, 69381);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 94706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113629);
									s.is_empty()
								}
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
			super::push(out, 41377);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69382);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 94707);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113630);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125809);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133132);
													s.is_empty()
												}
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
							super::push(out, 94708);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 113631);
									s.is_empty()
								}
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
			super::push(out, 41378);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69383);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 94709);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 113632);
									s.is_empty()
								}
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
			super::push(out, 41379);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69384);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94710);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 94711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 94712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41380);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69385);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113634);
									s.is_empty()
								}
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
			super::push(out, 41381);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69386);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 94714);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 94715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 69387);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94716);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113635);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125810);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 133133);
													match s {
														[] => true,
														['б', s @ ..] => super::t48::n138895(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 69388);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113636);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 113637);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 125811);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133134);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 113638);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125812);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 125813);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133135);
													s.is_empty()
												}
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
							super::push(out, 94717);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 69389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 41382);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 69390);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94718);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113640);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					super::push(out, 69391);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94719);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113641);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125815);
											s.is_empty()
										}
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
					super::push(out, 69392);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t48::n95742(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t48::n70119(s, out),
				['ы', s @ ..] => {
					super::push(out, 69394);
					match s {
						[] => true,
						['в', s @ ..] => super::t48::n95744(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -брадфордец.
pub(crate) fn n134507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133137);
	s.is_empty()
}

/// Words ending -удец.
pub(crate) fn n17889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17788);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 41385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41386);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 41387);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 69399);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 94726);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 113646);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125821);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41388);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69400);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 94727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдец.
pub(crate) fn n17890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17789);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41389);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69401);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 94728);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113647);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 125822);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133138);
													match s {
														[] => true,
														['б', s @ ..] => super::t48::n138897(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядец.
pub(crate) fn n17891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17790);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 41390);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69402);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 69403);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 69404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94731);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113649);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125823);
											s.is_empty()
										}
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
					super::push(out, 69405);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхнеуфалеец.
pub(crate) fn n134509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133139);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137479);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -западноевропеец.
pub(crate) fn n134510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133140);
	s.is_empty()
}

/// Words ending -кембриджец.
pub(crate) fn n134511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133141);
	s.is_empty()
}

/// Words ending -всеволожец.
pub(crate) fn n134512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133142);
	s.is_empty()
}

/// Words ending -ог-громовержец.
pub(crate) fn n134513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133143);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 137480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 140095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word гамбуржец.
pub(crate) fn n134514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133144);
	s.is_empty()
}

/// Words ending -смен-олимпиец.
pub(crate) fn n134515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133145);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137481);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140096);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковентриец.
pub(crate) fn n134516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133146);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137482);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ейкьявикец.
pub(crate) fn n134517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133147);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137483);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word пиздострадалец.
pub(crate) fn n134518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133148);
	s.is_empty()
}

/// Words ending -материалец.
pub(crate) fn n134519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133149);
	s.is_empty()
}

/// Words ending -палец.
pub(crate) fn n41860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41444);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69472);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 69473);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 94784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69474);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 94785);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125844);
									s.is_empty()
								}
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
			super::push(out, 69475);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94786);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113680);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псалец.
pub(crate) fn n70205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69477);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 94787);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -талец.
pub(crate) fn n41863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41447);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69478);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 69479);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94788);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 94789);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113683);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125846);
									s.is_empty()
								}
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
			super::push(out, 69480);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94790);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -блец.
pub(crate) fn n17933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17832);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69481);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глец.
pub(crate) fn n17934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17833);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41449);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41450);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 69483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 41451);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 69484);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -длец.
pub(crate) fn n17935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17834);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 41452);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 69485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елец.
pub(crate) fn n17936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17835);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 41453);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 41454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 41455);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69487);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113684);
									s.is_empty()
								}
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
			super::push(out, 41456);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94793);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 113685);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125847);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133150);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69489);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 69490);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 69491);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94795);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 113686);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125848);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 133151);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138903(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'н', s @ ..] => {
					super::push(out, 69492);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94796);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113687);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125849);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69493);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94797);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69494);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41459);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69495);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 94799);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113689);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 125850);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 133152);
													match s {
														[] => true,
														['л', s @ ..] => super::t48::n138904(s, out),
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
									super::push(out, 113690);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125851);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133153);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69496);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94800);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 69497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41460);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69498);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94801);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 69499);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 94802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 41461);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 41462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69500);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 69501);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113693);
									s.is_empty()
								}
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
			super::push(out, 41463);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жлец.
pub(crate) fn n17937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17836);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 41464);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 69502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -злец.
pub(crate) fn n17938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17837);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 41465);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 41466);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 69503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илец.
pub(crate) fn n17939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41467);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 69504);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 94806);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113694);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125852);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41468);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 41469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69505);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 69506);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94808);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 94809);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113696);
									s.is_empty()
								}
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
			super::push(out, 41470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69507);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41471);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69508);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 69509);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94811);
							match s {
								[] => true,
								['к', '-', s @ ..] => {
									super::push(out, 113697);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 125853);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133154);
													match s {
														[] => true,
														['л', s @ ..] => super::t48::n138905(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41472);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41473);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -клец.
pub(crate) fn n17940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17839);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41474);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 69511);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 94813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олец.
pub(crate) fn n17941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17840);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41475);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69512);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94814);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 94815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113700);
									s.is_empty()
								}
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
			super::push(out, 41476);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69513);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 69514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 41477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69515);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 69516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 41478);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69518);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94817);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113701);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125854);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41479);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 41480);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69520);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94818);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113702);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125855);
											s.is_empty()
										}
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
					super::push(out, 69521);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94819);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113703);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 125856);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 133155);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n138906(s, out),
														_ => false,
													}
												}
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
							super::push(out, 94820);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113704);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125857);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 94821);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113705);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 94822);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 113706);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125859);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 133156);
													match s {
														[] => true,
														['с', s @ ..] => super::t48::n138907(s, out),
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
									super::push(out, 113707);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 125860);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133157);
													s.is_empty()
												}
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
							super::push(out, 94823);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113708);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125861);
											s.is_empty()
										}
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
					super::push(out, 69522);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94824);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 94825);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113710);
									s.is_empty()
								}
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
			super::push(out, 41481);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 41482);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 69524);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94826);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113711);
									s.is_empty()
								}
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
			super::push(out, 41483);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69525);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69526);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 69527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94829);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 94830);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлец.
pub(crate) fn n17942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17841);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 41484);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улец.
pub(crate) fn n17943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41485);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 41486);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69530);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94831);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113714);
									s.is_empty()
								}
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
			super::push(out, 41487);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 41488);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 69531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word флец.
pub(crate) fn n41905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41489);
	s.is_empty()
}

/// Words ending -шлец.
pub(crate) fn n17945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17844);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41490);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 69532);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 94833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41491);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 69533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылец.
pub(crate) fn n17946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17845);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41492);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69534);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 94835);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113715);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125862);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 133158);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41493);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 69535);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41494);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 69536);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялец.
pub(crate) fn n17947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17846);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 41495);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 69537);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 94838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 41496);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 69538);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94839);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113717);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125863);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133159);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхошижемец.
pub(crate) fn n134530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133160);
	s.is_empty()
}

/// Words ending -бомец.
pub(crate) fn n41926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41510);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 69556);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94853);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -комец.
pub(crate) fn n41927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41511);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69557);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 94854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромец.
pub(crate) fn n41928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41512);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69558);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 69559);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 94855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -томец.
pub(crate) fn n41929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41513);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 69560);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 94856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 69561);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хомец.
pub(crate) fn n41930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41514);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 69562);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умец.
pub(crate) fn n17952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17851);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 41515);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 41516);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 41517);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 41518);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69564);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94860);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьмец.
pub(crate) fn n17954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17853);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41519);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69565);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 94861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 69566);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 94862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69567);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94863);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113735);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 125871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 94864);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 113736);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125872);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133161);
													match s {
														[] => true,
														['б', s @ ..] => super::t48::n138908(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмец.
pub(crate) fn n17955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17854);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 41520);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямец.
pub(crate) fn n17956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17855);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 41521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ванец.
pub(crate) fn n41939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41523);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69569);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94866);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 94867);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113737);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125873);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133162);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69570);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 69571);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 69572);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 69573);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69574);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94870);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113738);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125874);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 133163);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137490);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69575);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94871);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113739);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 113740);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 125875);
									match s {
										[] => true,
										['и', 'п', '\u{1}', s @ ..] => {
											super::push(out, 133164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 113741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125876);
									s.is_empty()
								}
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
			super::push(out, 69576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ганец.
pub(crate) fn n41940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69577);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 69578);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69579);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94874);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 94875);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 94876);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 69580);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94877);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 113747);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 125877);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 133165);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137491);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n141541(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аданец.
pub(crate) fn n70310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69581);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 94878);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 94879);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 94880);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113748);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 113749);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word киданец.
pub(crate) fn n114962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113750);
	s.is_empty()
}

/// Words ending -уданец.
pub(crate) fn n70312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69583);
	s.is_empty()
}

/// Words ending -еанец.
pub(crate) fn n41942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41526);
	s.is_empty()
}

/// Words ending -жанец.
pub(crate) fn n41943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41527);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 69584);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 69585);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94882);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113751);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133166);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94883);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 94884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -занец.
pub(crate) fn n41944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41528);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69587);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94885);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113753);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 113754);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ианец.
pub(crate) fn n41945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41529);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 69588);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94886);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125880);
									s.is_empty()
								}
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
			super::push(out, 69589);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -канец.
pub(crate) fn n41946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41530);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69590);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 94887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69591);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94888);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113757);
							match s {
								[] => true,
								['у', 'п', 'с', 'е', 'р', '-', s @ ..] => {
									super::push(out, 125881);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133167);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137492);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n141542(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 94889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69592);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94891);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 94892);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113759);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125882);
									s.is_empty()
								}
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
			super::push(out, 69594);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ланец.
pub(crate) fn n41947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41531);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 94895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 69595);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94896);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113761);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125883);
									s.is_empty()
								}
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
			super::push(out, 69596);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 94897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94898);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 94899);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 94900);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 94901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 69598);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 94902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 69599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -манец.
pub(crate) fn n41948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41532);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69600);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 69601);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94904);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 94905);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 94906);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113764);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125884);
									s.is_empty()
								}
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
			super::push(out, 69602);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 94908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нанец.
pub(crate) fn n41949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41533);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 69603);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94909);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -панец.
pub(crate) fn n41950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41534);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69604);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94910);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 113768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69605);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94911);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 113769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125886);
									s.is_empty()
								}
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
			super::push(out, 69606);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 94912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранец.
pub(crate) fn n41951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41535);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69607);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 69608);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 69609);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 69610);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94914);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125887);
									s.is_empty()
								}
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
			super::push(out, 69611);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69612);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94916);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113771);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125888);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133168);
											s.is_empty()
										}
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
					super::push(out, 94917);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 94918);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113772);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125889);
									s.is_empty()
								}
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
			super::push(out, 69613);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 94919);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125890);
									s.is_empty()
								}
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
			super::push(out, 69614);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 69615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69616);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94921);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 94922);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санец.
pub(crate) fn n41952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41536);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69617);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 94923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танец.
pub(crate) fn n41953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41537);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69618);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 69619);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 94924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113774);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 125891);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 133169);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69620);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69621);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 94926);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 94927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 69622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94928);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113776);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125892);
									s.is_empty()
								}
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
			super::push(out, 69623);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уанец.
pub(crate) fn n41954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41538);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 69624);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 69625);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 94930);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113777);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ханец.
pub(crate) fn n41955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41539);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69626);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 69627);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69628);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69629);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94933);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чанец.
pub(crate) fn n41956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41540);
	s.is_empty()
}

/// Words ending -шанец.
pub(crate) fn n41957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41541);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69630);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 69631);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 94934);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113779);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125894);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 133170);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гнец.
pub(crate) fn n17958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17857);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енец.
pub(crate) fn n17959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17858);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41543);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 69633);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94935);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113780);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125895);
											s.is_empty()
										}
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
					super::push(out, 69634);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 94936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 41544);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69635);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94937);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 113781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69636);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 94938);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 69637);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94939);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113783);
									s.is_empty()
								}
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
			super::push(out, 41545);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69638);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 94940);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113784);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125896);
											s.is_empty()
										}
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
					super::push(out, 69639);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 94941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 69640);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 94942);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113785);
									s.is_empty()
								}
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
			super::push(out, 41546);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94943);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 94944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 94945);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113787);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 125897);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 133171);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138912(s, out),
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
									super::push(out, 113788);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125898);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 94946);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113789);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 125899);
											s.is_empty()
										}
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
					super::push(out, 69642);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 94947);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 94948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 69643);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94949);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 69644);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94950);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 113791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 94951);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 113792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 69645);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94952);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113793);
									s.is_empty()
								}
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
			super::push(out, 41547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69646);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94953);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 94954);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 69647);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 69648);
					match s {
						[] => true,
						['у', s @ ..] => super::t48::n95977(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t48::n70380(s, out),
				['и', s @ ..] => super::t48::n70381(s, out),
				['о', s @ ..] => {
					super::push(out, 69650);
					s.is_empty()
				}
				['р', s @ ..] => super::t48::n70383(s, out),
				['у', s @ ..] => {
					super::push(out, 69652);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t48::n70385(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t48::n41964(s, out),
		['й', s @ ..] => super::t48::n41965(s, out),
		['к', s @ ..] => super::t48::n41966(s, out),
		['л', s @ ..] => super::t48::n41967(s, out),
		['м', s @ ..] => super::t48::n41968(s, out),
		['н', s @ ..] => super::t48::n41969(s, out),
		['п', s @ ..] => super::t48::n41970(s, out),
		['р', s @ ..] => super::t48::n41971(s, out),
		['с', s @ ..] => super::t48::n41972(s, out),
		['т', s @ ..] => super::t48::n41973(s, out),
		['ф', s @ ..] => super::t48::n41974(s, out),
		['х', s @ ..] => super::t48::n41975(s, out),
		['ч', s @ ..] => super::t48::n41976(s, out),
		['ш', s @ ..] => super::t48::n41977(s, out),
		['щ', s @ ..] => super::t48::n41978(s, out),
		['ь', s @ ..] => super::t48::n41979(s, out),
		_ => false,
	}
}

/// Words ending -жнец.
pub(crate) fn n17960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17859);
	s.is_empty()
}

/// Words ending -знец.
pub(crate) fn n17961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17860);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41564);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 69697);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 41565);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 69698);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инец.
pub(crate) fn n17962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41566);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 69699);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95019);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 113836);
									s.is_empty()
								}
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
			super::push(out, 41567);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 69700);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95020);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113837);
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
						['л', s @ ..] => {
							super::push(out, 95021);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113838);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 95022);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 113839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 69701);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 69702);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69703);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95026);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 95027);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 113840);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125924);
											s.is_empty()
										}
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
					super::push(out, 69704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 69705);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95029);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 113841);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125925);
											s.is_empty()
										}
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
					super::push(out, 69706);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95030);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113842);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 125926);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69707);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95031);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113843);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125927);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133184);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69708);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95032);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 95033);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 69709);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95034);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113846);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 95035);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113847);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125929);
											s.is_empty()
										}
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
					super::push(out, 69710);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95036);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113848);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125930);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133185);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69711);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 41569);
			match s {
				[] => true,
				['а', 'щ', s @ ..] => {
					super::push(out, 69712);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95038);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113849);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125931);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133186);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['к', s @ ..] => {
							super::push(out, 95039);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 95040);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113850);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 125932);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133187);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69713);
					match s {
						[] => true,
						['а', s @ ..] => super::t48::n96062(s, out),
						['и', s @ ..] => {
							super::push(out, 95042);
							match s {
								[] => true,
								['в', s @ ..] => super::t48::n115064(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t48::n70452(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t48::n41986(s, out),
		['е', s @ ..] => super::t48::n41987(s, out),
		['ж', s @ ..] => super::t48::n41988(s, out),
		['з', s @ ..] => super::t48::n41989(s, out),
		['к', s @ ..] => super::t48::n41990(s, out),
		['л', s @ ..] => super::t49::n41991(s, out),
		['м', s @ ..] => super::t49::n41992(s, out),
		['н', s @ ..] => super::t49::n41993(s, out),
		['п', s @ ..] => super::t49::n41994(s, out),
		['р', s @ ..] => super::t49::n41995(s, out),
		['с', s @ ..] => super::t49::n41996(s, out),
		['т', s @ ..] => super::t49::n41997(s, out),
		['у', s @ ..] => super::t49::n41998(s, out),
		['х', s @ ..] => super::t49::n41999(s, out),
		['ц', s @ ..] => super::t49::n42000(s, out),
		['ч', s @ ..] => super::t49::n42001(s, out),
		['ш', s @ ..] => super::t49::n42002(s, out),
		['щ', s @ ..] => super::t49::n42003(s, out),
		_ => false,
	}
}

/// Words ending -йнец.
pub(crate) fn n17963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17862);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мнец.
pub(crate) fn n17964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17863);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41588);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 69817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ннец.
pub(crate) fn n17965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17864);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41589);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 69818);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95180);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113951);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125973);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41590);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онец.
pub(crate) fn n17966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17865);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41591);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 41592);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 69819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95181);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 95182);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113952);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125974);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133202);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n138928(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 69820);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 69821);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95183);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 113953);
									s.is_empty()
								}
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
			super::push(out, 41593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69822);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 69823);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 69824);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 95184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 41594);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69825);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 69826);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 69827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95186);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113954);
									s.is_empty()
								}
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
			super::push(out, 41595);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 41596);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 41597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69829);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 69830);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 95188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 69831);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95189);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 113956);
									s.is_empty()
								}
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
			super::push(out, 41598);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69832);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95190);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113957);
									s.is_empty()
								}
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
			super::push(out, 41599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69833);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95191);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113958);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 125975);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41600);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69834);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 69835);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95192);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113959);
									s.is_empty()
								}
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
			super::push(out, 41601);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 41602);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69836);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 69837);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 69838);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 69839);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 69840);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95195);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113961);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125976);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133203);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69841);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95196);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113962);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125977);
											s.is_empty()
										}
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
					super::push(out, 69842);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95197);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113963);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125978);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 95198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 41603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 41604);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 41605);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнец.
pub(crate) fn n17967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17866);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41606);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 69846);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41607);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 69847);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95201);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 113964);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 125979);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 133204);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69848);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 69849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41608);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 41609);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 69850);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95204);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113965);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125980);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унец.
pub(crate) fn n17968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17867);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41610);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 41611);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69852);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95205);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 113966);
									s.is_empty()
								}
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
			super::push(out, 41612);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 69853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95206);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113967);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 69854);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95207);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 69855);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 41613);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 69856);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95209);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113969);
									s.is_empty()
								}
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
			super::push(out, 41614);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69857);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 41615);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 69858);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95211);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113970);
									s.is_empty()
								}
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
			super::push(out, 41616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69860);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 69861);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 69862);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95213);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113971);
									s.is_empty()
								}
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
			super::push(out, 41618);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 69863);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95214);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 69864);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 95215);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 113973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69865);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69866);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 69867);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 95217);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 113974);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 125981);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынец.
pub(crate) fn n17969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17868);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 41620);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 41621);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69868);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95218);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113975);
									s.is_empty()
								}
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
			super::push(out, 41622);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 41623);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69869);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95219);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113976);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 125982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьнец.
pub(crate) fn n17970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17869);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41624);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95220);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 95221);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113977);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 125983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнец.
pub(crate) fn n17971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17870);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 41625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янец.
pub(crate) fn n17972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17871);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 41626);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 41627);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 69870);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 41628);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 69871);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 95223);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 113978);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125984);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41629);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 69872);
					match s {
						[] => true,
						['с', 'о', 'р', '\u{1}', s @ ..] => {
							super::push(out, 95224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 41630);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41631);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 69874);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 41632);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 69875);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95227);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 113979);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 125985);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133205);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41633);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 41634);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69876);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95228);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 113980);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 41635);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 69877);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95229);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113981);
									s.is_empty()
								}
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
			super::push(out, 41636);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 69878);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оперекопец.
pub(crate) fn n134576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133206);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -святокупец.
pub(crate) fn n134577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133207);
	s.is_empty()
}

/// The word самоварец.
pub(crate) fn n134578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133208);
	s.is_empty()
}

/// Words ending -амритсарец.
pub(crate) fn n134579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133209);
	s.is_empty()
}

/// Words ending -грец.
pub(crate) fn n17989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17888);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41681);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 69927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 41682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69928);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дрец.
pub(crate) fn n17990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17889);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 41683);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 69929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерец.
pub(crate) fn n17991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17890);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 41684);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69930);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69931);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114001);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 114002);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125996);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 133210);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n138930(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 69932);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95258);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114003);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125997);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 133211);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41685);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69933);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95260);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 95261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114004);
									s.is_empty()
								}
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
			super::push(out, 41686);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 41687);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 41688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69936);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 41689);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 69937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 41690);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69938);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95263);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114005);
									s.is_empty()
								}
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
			super::push(out, 41691);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 69939);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 69940);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95264);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114006);
									s.is_empty()
								}
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
			super::push(out, 41692);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 69941);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 69942);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95266);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114007);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 125998);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133212);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69943);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 69944);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95267);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114008);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'и', '\u{1}', s @ ..] => {
					super::push(out, 69945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41695);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69947);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 69948);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95269);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114009);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126000);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 133213);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69949);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95270);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 95271);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 95272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 69950);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95273);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114010);
									s.is_empty()
								}
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
			super::push(out, 41696);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69951);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирец.
pub(crate) fn n17993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17892);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41697);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 41698);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 41699);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 41700);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 41701);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 69952);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95275);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114011);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 114012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69953);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95276);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114013);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 126001);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -крец.
pub(crate) fn n17994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17893);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 41702);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41703);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 69955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орец.
pub(crate) fn n17995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17894);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41704);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69956);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 69957);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 69958);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95278);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 95279);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 95280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 69959);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95281);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 114016);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 126002);
											s.is_empty()
										}
										['с', 'е', 'д', '\u{1}', s @ ..] => {
											super::push(out, 126003);
											s.is_empty()
										}
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
					super::push(out, 69960);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 69961);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95282);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114017);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 126004);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133214);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n138931(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41705);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 69962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95283);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 95284);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 95285);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114018);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 114019);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126005);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133215);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n138932(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 69963);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95286);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 69964);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 95287);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 114021);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126006);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 133216);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 69965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95288);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 95289);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 95290);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114022);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126007);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41706);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69966);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 69967);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95291);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 95292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 41707);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69968);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95293);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 95294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114024);
									s.is_empty()
								}
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
			super::push(out, 41708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69969);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 69970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41709);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69972);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 69973);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 69974);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95296);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114025);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126008);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трец.
pub(crate) fn n17997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17896);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41711);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 69975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 41712);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69976);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 41713);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69977);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урец.
pub(crate) fn n17998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17897);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 41714);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 69978);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114026);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 114027);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126009);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 133217);
													s.is_empty()
												}
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
							super::push(out, 95299);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 114028);
									s.is_empty()
								}
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
			super::push(out, 41715);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 41716);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 41717);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 41718);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 41719);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 41720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ырец.
pub(crate) fn n17999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17898);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 41721);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69982);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 95301);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 95302);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114030);
									s.is_empty()
								}
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
			super::push(out, 41722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 41723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69984);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 69985);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 95303);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114031);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126010);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133218);
													s.is_empty()
												}
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
							super::push(out, 95304);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114032);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126011);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133219);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n138933(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95305);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114033);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 126012);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133220);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n138934(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41724);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69986);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95306);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрец.
pub(crate) fn n18000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17899);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 41725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярец.
pub(crate) fn n18001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17900);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 41726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ильяндисец.
pub(crate) fn n134591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133221);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 137516);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word самописец.
pub(crate) fn n134592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133222);
	s.is_empty()
}

/// Words ending -ысотописец.
pub(crate) fn n134593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133223);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 137517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ентспилсец.
pub(crate) fn n134594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133224);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 137518);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осец.
pub(crate) fn n18007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17906);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41740);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95318);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 114044);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126019);
											s.is_empty()
										}
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
					super::push(out, 70001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 41741);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70002);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114045);
									s.is_empty()
								}
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
			super::push(out, 41742);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 41743);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70003);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 70004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 41744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70005);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95320);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114046);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126020);
											s.is_empty()
										}
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
					super::push(out, 70006);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95321);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114047);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126021);
											match s {
												[] => true,
												['б', '-', s @ ..] => {
													super::push(out, 133225);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n138938(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95322);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 70007);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 95323);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114049);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126022);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 133226);
													match s {
														[] => true,
														['д', s @ ..] => super::t49::n138939(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 126023);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114050);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 114051);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126024);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133227);
													match s {
														[] => true,
														['т', s @ ..] => super::t49::n138940(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95324);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 95325);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 95326);
							match s {
								[] => true,
								['е', s @ ..] => super::t49::n115265(s, out),
								['и', s @ ..] => super::t49::n115266(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t49::n96351(s, out),
						['т', s @ ..] => super::t49::n96352(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t49::n70755(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t49::n42162(s, out),
		['с', s @ ..] => super::t49::n42163(s, out),
		_ => false,
	}
}

/// Words ending -алипсец.
pub(crate) fn n96357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95333);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 114061);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126032);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 133234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссец.
pub(crate) fn n18009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17908);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41748);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усец.
pub(crate) fn n18010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17909);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41749);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70013);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 70014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41750);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 70015);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 95335);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114062);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126033);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41751);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 41752);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70016);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95336);
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
				['к', s @ ..] => {
					super::push(out, 70017);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 70018);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 70019);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114063);
									s.is_empty()
								}
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
			super::push(out, 41753);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70020);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 70021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95338);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 114064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юсец.
pub(crate) fn n18011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17910);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 41754);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 70022);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95339);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114065);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перехватец.
pub(crate) fn n134605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133235);
	s.is_empty()
}

/// Words ending -инетец.
pub(crate) fn n70789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70042);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 95355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ретец.
pub(crate) fn n42190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41769);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 70043);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 70044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сетец.
pub(crate) fn n42191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41770);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 70045);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95356);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 114075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фетец.
pub(crate) fn n42192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41771);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 70046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -четец.
pub(crate) fn n42193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41772);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 70047);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -итец.
pub(crate) fn n18015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17914);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 41773);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 41774);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 41775);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41776);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70051);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 70052);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95359);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114076);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126041);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 41778);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 70054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтец.
pub(crate) fn n18016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17915);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ктец.
pub(crate) fn n18017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17916);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41780);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70055);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95360);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтец.
pub(crate) fn n18018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17917);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41781);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нтец.
pub(crate) fn n18019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17918);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41782);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 41783);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70056);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 70057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70058);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 95362);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114078);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126042);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 133236);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41784);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 70059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отец.
pub(crate) fn n18020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17919);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 41785);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 41786);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70061);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114079);
									s.is_empty()
								}
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
			super::push(out, 41787);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70062);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 41788);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70063);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95366);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114080);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126043);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41789);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 70064);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 41790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70065);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41791);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70066);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95369);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114081);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126044);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 95370);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114082);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126045);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41792);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70067);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртец.
pub(crate) fn n18021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17920);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41793);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 41794);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 70068);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95372);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114083);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126046);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 133237);
													match s {
														[] => true,
														['ф', s @ ..] => super::t49::n138943(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стец.
pub(crate) fn n18022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17921);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41795);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70069);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 70070);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41796);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70071);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70072);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95376);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 114084);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 126047);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 133238);
													s.is_empty()
												}
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
							super::push(out, 95377);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 95378);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114085);
									s.is_empty()
								}
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
			super::push(out, 41797);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70073);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41798);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 70074);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 95380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 70075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95381);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 114086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70076);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41799);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70077);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95383);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114087);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 126048);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41800);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттец.
pub(crate) fn n18023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17922);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41801);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70079);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95384);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114088);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 126049);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 133239);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n138944(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41802);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70080);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утец.
pub(crate) fn n18024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17923);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 41803);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70081);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 70082);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95385);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114089);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 126050);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 133240);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n138945(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41804);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41805);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 70084);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штец.
pub(crate) fn n18026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17925);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтец.
pub(crate) fn n18027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17926);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70085);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 95387);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114090);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гдановичец.
pub(crate) fn n134611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133241);
	s.is_empty()
}

/// Words ending -ей-древоточец.
pub(crate) fn n134612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133242);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 137527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140114);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -деход-тягач.
pub(crate) fn n138949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137530);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 140116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обиль-тягач.
pub(crate) fn n138950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137531);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сир-толкач.
pub(crate) fn n134618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133248);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137532);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 140118);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нщина-врач.
pub(crate) fn n134619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133249);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137533);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 140119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерев-косач.
pub(crate) fn n134620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133250);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137534);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тель-лихач.
pub(crate) fn n134621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133251);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137535);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word густавович.
pub(crate) fn n138955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137536);
	s.is_empty()
}

/// Words ending -севолодович.
pub(crate) fn n138956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137537);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 140120);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -талинович.
pub(crate) fn n127383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126083);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 133258);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 137538);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 140121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онович.
pub(crate) fn n70945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70198);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 95467);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 114146);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 126084);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133259);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пович.
pub(crate) fn n42378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41957);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 70199);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 95468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рович.
pub(crate) fn n42379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41958);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 70200);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 70201);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 95469);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 114148);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'м', 'и', 'д', s @ ..] => {
			super::push(out, 70202);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95470);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114149);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 126085);
									s.is_empty()
								}
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
			super::push(out, 70203);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 95472);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 114151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 95473);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 114152);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 126086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 114153);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 126087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 95474);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 114154);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 126088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 95475);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 114155);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126089);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133260);
											s.is_empty()
										}
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
						['и', s @ ..] => {
							super::push(out, 114156);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 126090);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 133261);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137539);
													s.is_empty()
												}
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
							super::push(out, 114157);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 126091);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133262);
											s.is_empty()
										}
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
					super::push(out, 95476);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 114158);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 126092);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 133263);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 70204);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95477);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 114159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тович.
pub(crate) fn n42381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41960);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 70205);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 70206);
			match s {
				[] => true,
				['е', 'б', 'о', s @ ..] => {
					super::push(out, 95478);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126094);
									s.is_empty()
								}
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
			super::push(out, 70207);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 95479);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 114161);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 126095);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133264);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фович.
pub(crate) fn n42382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41961);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 70208);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 95480);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 114162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126096);
									s.is_empty()
								}
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
			super::push(out, 70209);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 95481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 114163);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126097);
									s.is_empty()
								}
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
			super::push(out, 70210);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95482);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хович.
pub(crate) fn n42383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41962);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 70211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95483);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 114164);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126098);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 133265);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 70212);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95484);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 114165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ол-украины.
pub(crate) fn n134647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133277);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 137543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иготовлины.
pub(crate) fn n134648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133278);
	s.is_empty()
}

/// Words ending -тивитамины.
pub(crate) fn n134649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133279);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 137544);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еты-выборы.
pub(crate) fn n134652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133282);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 137546);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 140123);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ухры-мухры.
pub(crate) fn n134653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133283);
	s.is_empty()
}

/// Words ending -ирнармассы.
pub(crate) fn n134656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133286);
	s.is_empty()
}

/// Words ending -ронавирусы.
pub(crate) fn n134657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133287);
	s.is_empty()
}

/// The word полминуты.
pub(crate) fn n134659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133289);
	s.is_empty()
}

/// Words ending -прапраотцы.
pub(crate) fn n134662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133292);
	s.is_empty()
}

/// Words ending -пятипальцы.
pub(crate) fn n134663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133293);
	s.is_empty()
}

/// Words ending -кто-нибудь.
pub(crate) fn n134667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133297);
	s.is_empty()
}

/// Words ending -что-нибудь.
pub(crate) fn n134668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133298);
	s.is_empty()
}

/// Words ending -потреблядь.
pub(crate) fn n134669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133299);
	s.is_empty()
}

/// Words ending -непроглядь.
pub(crate) fn n134670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133300);
	s.is_empty()
}

/// Words ending -знес-связь.
pub(crate) fn n134672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133302);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137551);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 140126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фельдсвязь.
pub(crate) fn n134673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133303);
	s.is_empty()
}

/// Words ending -радиосвязь.
pub(crate) fn n134674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133304);
	s.is_empty()
}

/// Words ending -есдаль.
pub(crate) fn n71555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70807);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 95840);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 114344);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126184);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133305);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 137552);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удаль.
pub(crate) fn n43156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42734);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 70808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еаль.
pub(crate) fn n18894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18789);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 42735);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 70809);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 70810);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жаль.
pub(crate) fn n18895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18790);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 42736);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70811);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95843);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114345);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заль.
pub(crate) fn n18896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42737);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 70812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42738);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70813);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иаль.
pub(crate) fn n18897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18792);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 42739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70814);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 70815);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 42740);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70816);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95845);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114346);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126185);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70817);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 70818);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 95847);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 114347);
									s.is_empty()
								}
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
			super::push(out, 42741);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70819);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 42742);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 42743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70820);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 42744);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70821);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95850);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каль.
pub(crate) fn n18898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18793);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42745);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42746);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 70823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42747);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 70824);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95852);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114349);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126186);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70825);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114350);
									s.is_empty()
								}
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
			super::push(out, 42749);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70826);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 70827);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95855);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114351);
									s.is_empty()
								}
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
			super::push(out, 42750);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70828);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маль.
pub(crate) fn n18899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18794);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42751);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42752);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 70830);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 70831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70832);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95859);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114354);
									s.is_empty()
								}
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
			super::push(out, 42753);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70833);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 42754);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 42755);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наль.
pub(crate) fn n18900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18795);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42756);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 42757);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 70835);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95861);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 114355);
									s.is_empty()
								}
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
			super::push(out, 42758);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 70836);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95862);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114356);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 126187);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42759);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70837);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70838);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95864);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114357);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 126188);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133306);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42760);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 70839);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 42761);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 70840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паль.
pub(crate) fn n18901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18796);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 42762);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70841);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 70842);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95867);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 95868);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114359);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126189);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раль.
pub(crate) fn n18902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18797);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42763);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70843);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 70844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 42764);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70845);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 70846);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 95870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42765);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70847);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 70848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95871);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114360);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126190);
											s.is_empty()
										}
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
					super::push(out, 70849);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95872);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114361);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126191);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 133307);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42766);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 70850);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 42767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42768);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 70853);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 95875);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70854);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 70855);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42769);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70856);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95877);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114363);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 126192);
											s.is_empty()
										}
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
					super::push(out, 70857);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 42770);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 70858);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 70859);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114364);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126193);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133308);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 114365);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126194);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 95880);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114366);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126195);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42771);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70861);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 70862);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95882);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114367);
									s.is_empty()
								}
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
			super::push(out, 42772);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 42773);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 70863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -саль.
pub(crate) fn n18903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18798);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 42774);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 42775);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70865);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95884);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114368);
									s.is_empty()
								}
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
			super::push(out, 42776);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70866);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95885);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114369);
									s.is_empty()
								}
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
			super::push(out, 42777);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70867);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 42778);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70868);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95887);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114370);
									s.is_empty()
								}
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
			super::push(out, 42779);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70869);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42780);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70870);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таль.
pub(crate) fn n18904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18799);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42781);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 42782);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 70871);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 95889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 42783);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70872);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 70873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42784);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 70874);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 70875);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95892);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 70876);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 70877);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114372);
									s.is_empty()
								}
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
			super::push(out, 42785);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 70878);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95894);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 70879);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95895);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114374);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 126196);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133309);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n138972(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 133310);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n138973(s, out),
														['р', s @ ..] => super::t49::n138974(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95896);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114375);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 126197);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70880);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95897);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 114376);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126198);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42787);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70881);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 70882);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 95900);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114378);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 126199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 95901);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114379);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126200);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 133311);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n138975(s, out),
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
									super::push(out, 114380);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 126201);
											s.is_empty()
										}
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
					super::push(out, 70883);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95902);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114381);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126202);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 133312);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 42788);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 70884);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 70885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 42789);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t49::n71635(s, out),
				['ц', s @ ..] => {
					super::push(out, 70888);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t49::n43213(s, out),
		_ => false,
	}
}

/// Words ending -уаль.
pub(crate) fn n18905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18800);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 42791);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 42792);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 70890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фаль.
pub(crate) fn n18906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18801);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42793);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -халь.
pub(crate) fn n18907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42794);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 70892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42795);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70893);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95911);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 70894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цаль.
pub(crate) fn n18908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18803);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 42796);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70895);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чаль.
pub(crate) fn n18909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18804);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42797);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 70896);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шаль.
pub(crate) fn n18910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18805);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42798);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 70897);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95914);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щаль.
pub(crate) fn n18911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18806);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 42799);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 70898);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лабрадудль.
pub(crate) fn n134686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133316);
	s.is_empty()
}

/// Words ending -альцгебель.
pub(crate) fn n134687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133317);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 137558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фешенебель.
pub(crate) fn n134688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133318);
	s.is_empty()
}

/// The word коктебель.
pub(crate) fn n134689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133319);
	s.is_empty()
}

/// The word стебель.
pub(crate) fn n115621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114409);
	s.is_empty()
}

/// Words ending -фебель.
pub(crate) fn n71682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70934);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 95942);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 114410);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126220);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133320);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 137559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хебель.
pub(crate) fn n71683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70935);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 95943);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114411);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 126221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ибель.
pub(crate) fn n43249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42826);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 70936);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95944);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 114412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбель.
pub(crate) fn n43250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42827);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 70937);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 95945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обель.
pub(crate) fn n43251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42828);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 70938);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 70939);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 70940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95947);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 95948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 70941);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 95949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 70942);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 95950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тбель.
pub(crate) fn n43252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42829);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 70943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95951);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убель.
pub(crate) fn n43253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42830);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 70944);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 95952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 70945);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 95953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 70946);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 70947);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 95955);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 114415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбель.
pub(crate) fn n43254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42831);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 70948);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95956);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 114416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбель.
pub(crate) fn n43255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42832);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 70949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вель.
pub(crate) fn n18928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18823);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42833);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 42834);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70950);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 70951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70952);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 95958);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 114417);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126222);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 114418);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126223);
											s.is_empty()
										}
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
					super::push(out, 70953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42835);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 70954);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 95960);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114419);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 126224);
											s.is_empty()
										}
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
					super::push(out, 70955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42836);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42837);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70957);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 42838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гель.
pub(crate) fn n18929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18824);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 42839);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70958);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95962);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114420);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126225);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42840);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70959);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95963);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114421);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126226);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 133321);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 70960);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 70961);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 95964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42841);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70962);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 70963);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42842);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70964);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 95966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 70966);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 95968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70967);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 70968);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 70969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42843);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70970);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 70971);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42844);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70972);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42845);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70974);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42846);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 70975);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 95974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 42847);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 70976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 42848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70977);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 70978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дель.
pub(crate) fn n18930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18825);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42849);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70979);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 70980);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95975);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 70981);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95976);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 114424);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 126227);
											s.is_empty()
										}
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
					super::push(out, 70982);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95977);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114425);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126228);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133322);
													s.is_empty()
												}
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
							super::push(out, 95978);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 114426);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126229);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133323);
													match s {
														[] => true,
														['ф', s @ ..] => super::t49::n138979(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 70983);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 95979);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 114427);
									s.is_empty()
								}
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
			super::push(out, 42850);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 42851);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 70985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42852);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 70986);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70987);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 70988);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 70989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42853);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70990);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 70991);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95983);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114428);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126230);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 133324);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n138980(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95984);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114429);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126231);
											s.is_empty()
										}
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
					super::push(out, 70992);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95985);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 70993);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95986);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114431);
									s.is_empty()
								}
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
			super::push(out, 42854);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95988);
							s.is_empty()
						}
						['-', s @ ..] => super::t49::n97016(s, out),
						['а', s @ ..] => {
							super::push(out, 95990);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114437);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126237);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 95991);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 95992);
							s.is_empty()
						}
						['о', s @ ..] => super::t49::n97020(s, out),
						['р', s @ ..] => super::t49::n97021(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t49::n71744(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t49::n43278(s, out),
		['у', s @ ..] => super::t49::n43279(s, out),
		['я', s @ ..] => super::t49::n43280(s, out),
		_ => false,
	}
}

/// Words ending -жель.
pub(crate) fn n18931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18826);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 42858);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 42859);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71002);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зель.
pub(crate) fn n18932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18827);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42860);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 71004);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 71005);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96001);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 71006);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96002);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114444);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 126242);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42861);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71007);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 71008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42862);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71009);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 71010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 42863);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71011);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114445);
									s.is_empty()
								}
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
			super::push(out, 42864);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71012);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96006);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 96007);
							s.is_empty()
						}
						['р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 96008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42865);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 42866);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 42867);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71015);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иель.
pub(crate) fn n18933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18828);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 42868);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71016);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96010);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114446);
									s.is_empty()
								}
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
			super::push(out, 42869);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71017);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 71018);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96011);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126243);
											s.is_empty()
										}
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
					super::push(out, 71019);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 96012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кель.
pub(crate) fn n18934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18829);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42870);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 71020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42871);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71021);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 71022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42872);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71023);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лель.
pub(crate) fn n18935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18830);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42873);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 42874);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71024);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 42875);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71025);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96015);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114449);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 126244);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 133333);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n138983(s, out),
														_ => false,
													}
												}
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
							super::push(out, 96016);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

