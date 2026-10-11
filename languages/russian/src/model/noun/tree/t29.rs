use flexion::Path;

/// Words ending -ыня.
pub(crate) fn n4914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4903);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19874);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44931);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19875);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44932);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73573);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98486);
									s.is_empty()
								}
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
			super::push(out, 19876);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19877);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44933);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73574);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 73575);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 98487);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116654);
											s.is_empty()
										}
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
					super::push(out, 44934);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 44935);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73577);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98488);
									s.is_empty()
								}
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
			super::push(out, 19878);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 44936);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19879);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 44937);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73579);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98489);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116655);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128120);
													match s {
														[] => true,
														['б', s @ ..] => super::t41::n136277(s, out),
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
									super::push(out, 98490);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116656);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128121);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 98491);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116657);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128122);
													s.is_empty()
												}
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
							super::push(out, 73580);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 44938);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73581);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьня.
pub(crate) fn n4915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4904);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19880);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44939);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73582);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98494);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 116658);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128123);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t41::n136278(s, out),
														_ => false,
													}
												}
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
							super::push(out, 73583);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 73584);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98495);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116659);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 128124);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t41::n136279(s, out),
														_ => false,
													}
												}
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
							super::push(out, 73585);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98496);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 116660);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 128125);
													match s {
														[] => true,
														['а', s @ ..] => super::t41::n136280(s, out),
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
									super::push(out, 98497);
									match s {
										[] => true,
										['м', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
											super::push(out, 116661);
											s.is_empty()
										}
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
					super::push(out, 44940);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 44941);
					match s {
						[] => true,
						['д', 'я', 'р', 'п', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 73586);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 73587);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 98498);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116662);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128126);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44942);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 44943);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73588);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юня.
pub(crate) fn n4916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4905);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19881);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 44944);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73589);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98500);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116663);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19883);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 44946);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яня.
pub(crate) fn n4917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4906);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 19884);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44947);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73591);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98501);
									s.is_empty()
								}
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
			super::push(out, 19885);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44948);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ья-палуоя.
pub(crate) fn n129477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128127);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 134876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-января.
pub(crate) fn n129478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128128);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -л-октября.
pub(crate) fn n129479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128129);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 134878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-лагеря.
pub(crate) fn n129480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128130);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -охоря.
pub(crate) fn n45404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44970);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 73607);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 98512);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пря.
pub(crate) fn n4934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4923);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 19912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44971);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уря.
pub(crate) fn n4935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4924);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фря.
pub(crate) fn n4936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4925);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыря.
pub(crate) fn n4937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4926);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 19915);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44972);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73609);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98513);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116670);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128131);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юря.
pub(crate) fn n4938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4927);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19916);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 44973);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73610);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98514);
									s.is_empty()
								}
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
			super::push(out, 19917);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учившееся.
pub(crate) fn n129482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128132);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 134880);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 138699);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сбывшееся.
pub(crate) fn n129483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128133);
	s.is_empty()
}

/// Words ending -ыкающееся.
pub(crate) fn n129484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128134);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 134881);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word брачущиеся.
pub(crate) fn n129485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128135);
	s.is_empty()
}

/// Words ending -ачующиеся.
pub(crate) fn n129486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128136);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 134882);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 138700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кровосися.
pub(crate) fn n129487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128137);
	s.is_empty()
}

/// Words ending -чащийся.
pub(crate) fn n99593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98521);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 116677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ющийся.
pub(crate) fn n74395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73618);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 98522);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 116678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 128138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 98523);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 116679);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 128139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящийся.
pub(crate) fn n74396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73619);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 98524);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 116680);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 128140);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ося.
pub(crate) fn n4943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4932);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19926);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44984);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уся.
pub(crate) fn n4944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4933);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19927);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44985);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19928);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44986);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19929);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44987);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73623);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98526);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 116681);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128141);
													s.is_empty()
												}
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
							super::push(out, 73624);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98527);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 116682);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19930);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44988);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44989);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 73626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44990);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19933);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44991);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юся.
pub(crate) fn n4945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4934);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19934);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яся.
pub(crate) fn n4946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4935);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19935);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 44992);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73629);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 98528);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 116683);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 73630);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 98529);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 116684);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128142);
													match s {
														[] => true,
														['т', s @ ..] => super::t41::n136289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ассакалья.
pub(crate) fn n129493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128143);
	s.is_empty()
}

/// Words ending -гулья.
pub(crate) fn n45462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45028);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 73650);
			match s {
				[] => true,
				['а', 'г', '\u{1}', s @ ..] => {
					super::push(out, 98539);
					s.is_empty()
				}
				['о', 'г', '\u{1}', s @ ..] => {
					super::push(out, 98540);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тулья.
pub(crate) fn n45463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45029);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 73651);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ылья.
pub(crate) fn n20079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19973);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 45030);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73652);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 98541);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116690);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128144);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мья.
pub(crate) fn n4967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19974);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45031);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19975);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45032);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73654);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 45033);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73655);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98543);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116691);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19976);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 45034);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73656);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98544);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116692);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нья.
pub(crate) fn n4968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4957);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19977);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 45035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73657);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45036);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73658);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 45037);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 45038);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73659);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 45039);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19978);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45040);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73661);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45041);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 73662);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98549);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116693);
											s.is_empty()
										}
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
					super::push(out, 45042);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73663);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98550);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116694);
											s.is_empty()
										}
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
					super::push(out, 45043);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73664);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98551);
									s.is_empty()
								}
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
			super::push(out, 19979);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45044);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73665);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 73666);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98552);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116695);
											s.is_empty()
										}
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
					super::push(out, 45045);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73667);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98553);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116696);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128145);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 45046);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73668);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98554);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116697);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 73669);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98555);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 116698);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128146);
													match s {
														[] => true,
														['к', s @ ..] => super::t41::n136290(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 45047);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73670);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 73671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 45048);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73672);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 73673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 45049);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73674);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 45050);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73675);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98557);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116699);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19980);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45051);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 45052);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73676);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98558);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 98559);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116700);
											s.is_empty()
										}
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
					super::push(out, 45053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t41::n20087(s, out),
		['ы', s @ ..] => {
			super::push(out, 19982);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45061);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73685);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 45062);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73686);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98567);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116705);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пья.
pub(crate) fn n4969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4958);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19983);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45063);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 73687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рья.
pub(crate) fn n4970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19984);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73688);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 73689);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98568);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 116706);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 73690);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98569);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 116707);
											s.is_empty()
										}
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
					super::push(out, 45065);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 73691);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98570);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 116708);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 128150);
													match s {
														[] => true,
														['н', s @ ..] => super::t41::n136294(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 45066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19985);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45067);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 73693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19986);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45068);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 73694);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сья.
pub(crate) fn n4971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4960);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19987);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19988);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19989);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 19990);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45070);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тья.
pub(crate) fn n4972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4961);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19991);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45071);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45072);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73698);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 45073);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73699);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 45074);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73700);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 98575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 45075);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19992);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45076);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 73702);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 45077);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 19993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45078);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 73704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19994);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45079);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73705);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45080);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 73706);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98578);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116709);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 73707);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98579);
									s.is_empty()
								}
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
			super::push(out, 19995);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45081);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19996);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 45082);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19997);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фья.
pub(crate) fn n4973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4962);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19998);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 45084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 45085);
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 73712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 45086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73713);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 73714);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98580);
									s.is_empty()
								}
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
			super::push(out, 19999);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 45087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 45088);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73716);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 45089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20000);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45090);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хья.
pub(crate) fn n4974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4963);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 20001);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 45091);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73718);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98582);
									s.is_empty()
								}
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
			super::push(out, 20002);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 45092);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73719);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98583);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 116710);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45093);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чья.
pub(crate) fn n4975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4964);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20004);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45094);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73720);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 98584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 45095);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73721);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98585);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116711);
											s.is_empty()
										}
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
					super::push(out, 45096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20005);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45097);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73722);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98586);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шья.
pub(crate) fn n4976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4965);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 20006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45098);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ддис-абеба.
pub(crate) fn n129501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128151);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 134890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -секс-бомба.
pub(crate) fn n129502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128152);
	s.is_empty()
}

/// Words ending -биль-бомба.
pub(crate) fn n129503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128153);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 134891);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138704);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вать-тумба.
pub(crate) fn n129504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128154);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 134892);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -либр-скоба.
pub(crate) fn n129505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128155);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 134893);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 138706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -болт-скоба.
pub(crate) fn n129506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128156);
	s.is_empty()
}

/// Words ending -жихад-арба.
pub(crate) fn n129507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128157);
	s.is_empty()
}

/// Words ending -го-де-куба.
pub(crate) fn n129508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128158);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 134894);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 138707);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыба-труба.
pub(crate) fn n129509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128159);
	s.is_empty()
}

/// Words ending -игла-труба.
pub(crate) fn n129510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128160);
	s.is_empty()
}

/// Words ending -атье-труба.
pub(crate) fn n129511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128161);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 134895);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 138708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтан-труба.
pub(crate) fn n129512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128162);
	s.is_empty()
}

/// Words ending -сабля-рыба.
pub(crate) fn n129513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128163);
	s.is_empty()
}

/// Words ending -ей-усадьба.
pub(crate) fn n129514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128164);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 134896);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 138709);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 140857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытьба.
pub(crate) fn n20246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20140);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 45212);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озд-деряба.
pub(crate) fn n129515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128165);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 134897);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 138710);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азрыв-трава.
pub(crate) fn n136304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134899);
	s.is_empty()
}

/// Words ending -доброгнева.
pub(crate) fn n129521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128171);
	s.is_empty()
}

/// Words ending -пол-архива.
pub(crate) fn n129522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128172);
	s.is_empty()
}

/// Words ending -орвиголова.
pub(crate) fn n129523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128173);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 134900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -полуголова.
pub(crate) fn n129524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128174);
	s.is_empty()
}

/// Words ending -зык-основа.
pub(crate) fn n129525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128175);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 134901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 138711);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгрова.
pub(crate) fn n45740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45305);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 73847);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 98657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орова.
pub(crate) fn n20351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20245);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 45306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трова.
pub(crate) fn n20352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20246);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 45307);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73848);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 98658);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116748);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128176);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134902);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сова.
pub(crate) fn n5132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5121);
	s.is_empty()
}

/// Words ending -хова.
pub(crate) fn n5133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5122);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 20247);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 45308);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бодхисатва.
pub(crate) fn n129527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128177);
	s.is_empty()
}

/// The word полцарства.
pub(crate) fn n129528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128178);
	s.is_empty()
}

/// Words ending -огга-уогга.
pub(crate) fn n129531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128181);
	s.is_empty()
}

/// Words ending -ихтиостега.
pub(crate) fn n129532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128182);
	s.is_empty()
}

/// The word расстрига.
pub(crate) fn n129533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128183);
	s.is_empty()
}

/// Words ending -аман-байга.
pub(crate) fn n129534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128184);
	s.is_empty()
}

/// Words ending -доголонога.
pub(crate) fn n129535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128185);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 134903);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word кондопога.
pub(crate) fn n129536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128186);
	s.is_empty()
}

/// The word недотрога.
pub(crate) fn n129537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128187);
	s.is_empty()
}

/// Words ending -петербурга.
pub(crate) fn n129538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128188);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 134904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -псевдодуга.
pub(crate) fn n129539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128189);
	s.is_empty()
}

/// Words ending -пол-округа.
pub(crate) fn n129540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128190);
	s.is_empty()
}

/// Words ending -струга.
pub(crate) fn n45908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45473);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 73947);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 98703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 73948);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 98704);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туга.
pub(crate) fn n5254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5243);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 20458);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 45474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фуга.
pub(crate) fn n5255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5244);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20459);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45475);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуга.
pub(crate) fn n5256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5245);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20460);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20461);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20462);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 45479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20463);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 45480);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 73950);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98705);
									s.is_empty()
								}
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
			super::push(out, 20464);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73951);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98706);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116771);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20465);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45482);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73952);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 73953);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуга.
pub(crate) fn n5257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5246);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щуга.
pub(crate) fn n5258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5247);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20467);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45483);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73954);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 98709);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116772);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20468);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45484);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аркобарыга.
pub(crate) fn n129541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128191);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 134905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рюга.
pub(crate) fn n5277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5265);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20518);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45532);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 45533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 20519);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45534);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20520);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 45535);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 73991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 45536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 45537);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73993);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98728);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116781);
											s.is_empty()
										}
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
					super::push(out, 45538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20521);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20522);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45540);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73994);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98729);
									s.is_empty()
								}
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
			super::push(out, 20523);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45541);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 45542);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 73996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рняга.
pub(crate) fn n20652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20545);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 45565);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 45566);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 74012);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98737);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 116785);
									s.is_empty()
								}
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
			super::push(out, 45567);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74013);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 98738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тняга.
pub(crate) fn n20653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20546);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 45568);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 74014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 45569);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74015);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 45570);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74016);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 98741);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -муняга.
pub(crate) fn n46007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45571);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 74017);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 98742);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 116786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шняга.
pub(crate) fn n20655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20548);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 45572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьняга.
pub(crate) fn n20656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20549);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 45573);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74018);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 98743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ряга.
pub(crate) fn n5288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5276);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20550);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 20551);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45574);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 74019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98744);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116787);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20552);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 20553);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 45576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20554);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20555);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 20556);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45578);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 45579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 20557);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 45580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сяга.
pub(crate) fn n5289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5277);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20558);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45581);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тяга.
pub(crate) fn n5290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5278);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20559);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 20560);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 45582);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74022);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98745);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116788);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20561);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45583);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 74023);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98746);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116789);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128192);
													match s {
														[] => true,
														['е', s @ ..] => super::t41::n136311(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20562);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 45584);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74024);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98747);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 45585);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74025);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 45586);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74026);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 45587);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 74027);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 45588);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74028);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98751);
									s.is_empty()
								}
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
				['е', s @ ..] => {
					super::push(out, 45589);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 74029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 45590);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74030);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98752);
									s.is_empty()
								}
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
			super::push(out, 20563);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 45591);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 45592);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74032);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98753);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 116790);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 74033);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98754);
									s.is_empty()
								}
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
			super::push(out, 20564);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45593);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 45594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 45595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -амидомонада.
pub(crate) fn n136312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134907);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138713);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 140858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трихомонада.
pub(crate) fn n136313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134908);
	s.is_empty()
}

/// Words ending -беседа.
pub(crate) fn n46084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45648);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 74067);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98767);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 116798);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 128196);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134909);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138714);
													match s {
														[] => true,
														['с', s @ ..] => super::t41::n142304(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 98768);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116799);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 128197);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134910);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138715);
													match s {
														[] => true,
														['л', s @ ..] => super::t41::n142305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оседа.
pub(crate) fn n20730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20622);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 45649);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74068);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анк-звезда.
pub(crate) fn n129548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128198);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134911);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рок-звезда.
pub(crate) fn n129549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128199);
	s.is_empty()
}

/// Words ending -лэм-звезда.
pub(crate) fn n129550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128200);
	s.is_empty()
}

/// Words ending -поп-звезда.
pub(crate) fn n129551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128201);
	s.is_empty()
}

/// Words ending -мегазвезда.
pub(crate) fn n129552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128202);
	s.is_empty()
}

/// Words ending -телезвезда.
pub(crate) fn n129553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128203);
	s.is_empty()
}

/// Words ending -антизвезда.
pub(crate) fn n129554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128204);
	s.is_empty()
}

/// Words ending -адиозвезда.
pub(crate) fn n129555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128205);
	s.is_empty()
}

/// Words ending -кинозвезда.
pub(crate) fn n129556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128206);
	s.is_empty()
}

/// Words ending -ротозвезда.
pub(crate) fn n129557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128207);
	s.is_empty()
}

/// Words ending -рзвезда.
pub(crate) fn n74853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74076);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 98780);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 116810);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 128208);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хзвезда.
pub(crate) fn n74854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74077);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 98781);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 116811);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 128209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пизда.
pub(crate) fn n20742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20634);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 45659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мзда.
pub(crate) fn n5332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5320);
	s.is_empty()
}

/// Words ending -озда.
pub(crate) fn n5333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5321);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 20635);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45660);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74078);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98782);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 98783);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116812);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128210);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136318(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узда.
pub(crate) fn n5334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5322);
	s.is_empty()
}

/// Words ending -юзда.
pub(crate) fn n5335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5323);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20636);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 45661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ламида.
pub(crate) fn n46116(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 74090);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 98789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 74091);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирамида.
pub(crate) fn n74869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74092);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 98790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хамида.
pub(crate) fn n46118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45681);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 74093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шамида.
pub(crate) fn n46119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45682);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 74094);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 98791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емида.
pub(crate) fn n20773(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 45683);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74095);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98792);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116814);
									s.is_empty()
								}
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
			super::push(out, 45684);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74096);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -змида.
pub(crate) fn n20774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20664);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 45685);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74097);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 98794);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116815);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128211);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134914);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 45686);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 74098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имида.
pub(crate) fn n20775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20665);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 45687);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74099);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омида.
pub(crate) fn n20776(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 45688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74100);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98796);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 116816);
									s.is_empty()
								}
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
			super::push(out, 45689);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 74101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98797);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 116817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмида.
pub(crate) fn n20777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20666);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 45690);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нида.
pub(crate) fn n5347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5335);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 20667);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 45691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'в', s @ ..] => {
			super::push(out, 20668);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'л', 'и', 'ф', 'м', s @ ..] => {
			super::push(out, 20669);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оида.
pub(crate) fn n5348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5336);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20671);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20672);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45695);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 45696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20673);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 45697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 20674);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45698);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74106);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98799);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116818);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20675);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45699);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пида.
pub(crate) fn n5349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5337);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20676);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20677);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45701);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 45702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рида.
pub(crate) fn n5350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5338);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20678);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 45703);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74108);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 45704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 20679);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74109);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 74110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20680);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 45705);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 45706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45707);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74112);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 98801);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 116819);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 74113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 45708);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 45709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 20681);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 45710);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74114);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116820);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 20682);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20683);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45712);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 74115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98803);
									s.is_empty()
								}
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
			super::push(out, 20684);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 45713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20685);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45714);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 20686);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 45715);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74117);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98804);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116821);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 128212);
													match s {
														[] => true,
														['а', s @ ..] => super::t41::n136320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20687);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 45716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74118);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98805);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116822);
											s.is_empty()
										}
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
					super::push(out, 45717);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 20688);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сида.
pub(crate) fn n5351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20689);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 20690);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20691);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45720);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74120);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98806);
									s.is_empty()
								}
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
			super::push(out, 20692);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 45721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20694);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20695);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 45724);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98807);
									s.is_empty()
								}
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
			super::push(out, 20696);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45725);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тида.
pub(crate) fn n5352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20697);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20698);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45727);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74123);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98808);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116823);
											s.is_empty()
										}
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
					super::push(out, 45728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74124);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98809);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116824);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128213);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20699);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20700);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45730);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 45731);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74126);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98810);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 116825);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128214);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 45732);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20701);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 45733);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уида.
pub(crate) fn n5353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5341);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 20702);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45734);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фида.
pub(crate) fn n5354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5342);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 20703);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 45735);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хида.
pub(crate) fn n5355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5343);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20704);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45736);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20705);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 20706);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 45737);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74131);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цида.
pub(crate) fn n5356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5344);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 20707);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45738);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word караганда.
pub(crate) fn n129565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128215);
	s.is_empty()
}

/// The word одноманда.
pub(crate) fn n129566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128216);
	s.is_empty()
}

/// Words ending -ньяруанда.
pub(crate) fn n118088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116829);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 128217);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134916);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енда.
pub(crate) fn n5367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5355);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20737);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45767);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98826);
									s.is_empty()
								}
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
			super::push(out, 20738);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45768);
					match s {
						[] => true,
						['л', '-', s @ ..] => {
							super::push(out, 74158);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98827);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 116830);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128218);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136322(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45769);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 45770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инда.
pub(crate) fn n5368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5356);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 20741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45772);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 45773);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 74160);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 45774);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74161);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 74162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 20742);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 45775);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 45776);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 20743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онда.
pub(crate) fn n5369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5357);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 20744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45777);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74164);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98829);
									s.is_empty()
								}
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
			super::push(out, 20745);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 20746);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45778);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 45779);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 45780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унда.
pub(crate) fn n5370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5358);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 20747);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45781);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 20748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 20749);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 20750);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45784);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74167);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98831);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116831);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128219);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20751);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ында.
pub(crate) fn n5371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5359);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 20752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45786);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 45787);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -авиапогода.
pub(crate) fn n129570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128220);
	s.is_empty()
}

/// Words ending -ень-колода.
pub(crate) fn n129571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128221);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -апода.
pub(crate) fn n20883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20771);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 45804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74185);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 98841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ипода.
pub(crate) fn n20884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20772);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 45805);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 74186);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опода.
pub(crate) fn n20885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20773);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 45806);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 74187);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спода.
pub(crate) fn n20886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20774);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 45807);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 74188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рода.
pub(crate) fn n5383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5371);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 20775);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 45808);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20776);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 45809);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 45810);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74190);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98845);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116836);
											s.is_empty()
										}
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
					super::push(out, 45811);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74191);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 74192);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 74193);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 74194);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 98846);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116837);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128222);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136324(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 45812);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 45813);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 45814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20777);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сода.
pub(crate) fn n5384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5372);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 20778);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45815);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74197);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98848);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116838);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тода.
pub(crate) fn n5385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5373);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20779);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45816);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 20780);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45818);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74199);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98849);
									s.is_empty()
								}
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
			super::push(out, 20782);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45819);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 74200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 45820);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -держиморда.
pub(crate) fn n129573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128223);
	s.is_empty()
}

/// Words ending -урда-мурда.
pub(crate) fn n129574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128224);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 134920);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -страхолюда.
pub(crate) fn n129575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128225);
	s.is_empty()
}

/// Words ending -пол-отряда.
pub(crate) fn n129576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128226);
	s.is_empty()
}

/// Words ending -ибозим-полимераза.
pub(crate) fn n143548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142097);
	s.is_empty()
}

/// Words ending -ланинаминотрансфераза.
pub(crate) fn n142308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140863);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маунт-айза.
pub(crate) fn n129583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128233);
	s.is_empty()
}

/// Words ending -онтроверза.
pub(crate) fn n129584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128234);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 134926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пол-арбуза.
pub(crate) fn n129585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128235);
	s.is_empty()
}

/// Words ending -многосвяза.
pub(crate) fn n129586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128236);
	s.is_empty()
}

/// Words ending -умагомарака.
pub(crate) fn n136333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134928);
	s.is_empty()
}

/// The word выжеребка.
pub(crate) fn n129592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128242);
	s.is_empty()
}

/// Words ending -хлопкоробка.
pub(crate) fn n129593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128243);
	s.is_empty()
}

/// Words ending -пробка.
pub(crate) fn n46493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46053);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 74350);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 98936);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 116883);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128244);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 134929);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138721);
													match s {
														[] => true,
														['к', s @ ..] => super::t41::n142309(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тробка.
pub(crate) fn n46494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46054);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 74351);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 98937);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 116884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -собка.
pub(crate) fn n21189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21076);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 46055);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -офобка.
pub(crate) fn n46497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46057);
	s.is_empty()
}

/// Words ending -рбка.
pub(crate) fn n5568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5556);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21078);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 46058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убка.
pub(crate) fn n5569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5557);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 21079);
			match s {
				[] => true,
				['е', 'ш', 'у', 'д', '\u{1}', s @ ..] => {
					super::push(out, 46059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21080);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 21081);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 46060);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74353);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46061);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74354);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98939);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116885);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128245);
													s.is_empty()
												}
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
							super::push(out, 74355);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 74356);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 98940);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116886);
											s.is_empty()
										}
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
					super::push(out, 46062);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74357);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 98941);
									s.is_empty()
								}
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
			super::push(out, 21082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46063);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74358);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46064);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46065);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46066);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 74361);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98944);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 98945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 46067);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 74362);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98946);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116887);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 128246);
													match s {
														[] => true,
														['е', s @ ..] => super::t41::n136335(s, out),
														_ => false,
													}
												}
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
							super::push(out, 74363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 46068);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98947);
									s.is_empty()
								}
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
			super::push(out, 21084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыбка.
pub(crate) fn n5570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5558);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 21085);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21086);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 46069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21087);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46070);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74365);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбка.
pub(crate) fn n5571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5559);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21088);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 21089);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21090);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21091);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46072);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 46073);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46074);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74366);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 74367);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98949);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116888);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 128247);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21092);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -водоплавка.
pub(crate) fn n129598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128248);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 134931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -правка.
pub(crate) fn n46528(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 74378);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 74379);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 98959);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116895);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128249);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134932);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 138723);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 74380);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 98960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вытравка.
pub(crate) fn n75158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74381);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 98961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -савка.
pub(crate) fn n21214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21101);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46089);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 46090);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74382);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116897);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 116898);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 128250);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 134933);
													match s {
														[] => true,
														['в', s @ ..] => super::t41::n140159(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тавка.
pub(crate) fn n21215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21102);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46091);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 74383);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 98962);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116899);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128251);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46092);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74384);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 74385);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 74386);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 74387);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 74388);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98964);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 116900);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128252);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134934);
													s.is_empty()
												}
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
							super::push(out, 98965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 74389);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 98966);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хавка.
pub(crate) fn n21216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21103);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46093);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шавка.
pub(crate) fn n21217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21104);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -евка.
pub(crate) fn n5573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5561);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21105);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46095);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74391);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98968);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116902);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 128253);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46096);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 46097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 46098);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74393);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98969);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116903);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128254);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21106);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46099);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 46100);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 74394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98970);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116904);
											s.is_empty()
										}
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
					super::push(out, 46101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74395);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 46102);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'н', 'е', 'л', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 46103);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 46104);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74397);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 98972);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116905);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21107);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46105);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74398);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 46106);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74399);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 46107);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 74400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 46108);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74401);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98975);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116906);
											s.is_empty()
										}
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
					super::push(out, 46109);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74402);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98976);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 116907);
											s.is_empty()
										}
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
					super::push(out, 46110);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74403);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98977);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116908);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21108);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46111);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 46112);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 74405);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98978);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116909);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128255);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t41::n136340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21109);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 46113);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74406);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98979);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116910);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128256);
													match s {
														[] => true,
														['т', s @ ..] => super::t41::n136341(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21110);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74407);
							match s {
								[] => true,
								['к', s @ ..] => super::t41::n100053(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t41::n46556(s, out),
				['с', s @ ..] => super::t41::n46557(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t41::n21224(s, out),
		['н', s @ ..] => super::t41::n21225(s, out),
		['о', s @ ..] => super::t41::n21226(s, out),
		['п', s @ ..] => {
			super::push(out, 21114);
			s.is_empty()
		}
		['р', s @ ..] => super::t41::n21228(s, out),
		['с', s @ ..] => super::t41::n21229(s, out),
		['т', s @ ..] => {
			super::push(out, 21117);
			match s {
				[] => true,
				['у', s @ ..] => super::t41::n46576(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t41::n21231(s, out),
		['ц', s @ ..] => super::t41::n21232(s, out),
		['ч', s @ ..] => super::t41::n21233(s, out),
		['ш', s @ ..] => super::t41::n21234(s, out),
		['щ', s @ ..] => super::t41::n21235(s, out),
		['ь', s @ ..] => super::t41::n21236(s, out),
		['ю', s @ ..] => super::t41::n21237(s, out),
		_ => false,
	}
}

/// Words ending -ивка.
pub(crate) fn n5574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5562);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21125);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 46146);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74431);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99005);
									s.is_empty()
								}
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
			super::push(out, 21126);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 21127);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21128);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46147);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74432);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99006);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116928);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21129);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 46148);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99007);
									s.is_empty()
								}
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
			super::push(out, 21130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21131);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 21132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46150);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 46151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 46152);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46153);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 46154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74435);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116929);
											s.is_empty()
										}
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
					super::push(out, 46155);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74436);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99009);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 116930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвка.
pub(crate) fn n5575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5563);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овка.
pub(crate) fn n5576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5564);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21134);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 46156);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 46157);
					s.is_empty()
				}
				['у', 'р', s @ ..] => {
					super::push(out, 46158);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 46159);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99011);
									s.is_empty()
								}
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
			super::push(out, 21135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46161);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 74439);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46162);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74440);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 74441);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 74442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 74443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46163);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74444);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99014);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116931);
											s.is_empty()
										}
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
					super::push(out, 46164);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74445);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99015);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128271);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 128272);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136348(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46165);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74446);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 74447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 74448);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99018);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116932);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21137);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'п', s @ ..] => {
					super::push(out, 46167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 46168);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46169);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 74452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 74453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 46170);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74454);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99021);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116933);
											s.is_empty()
										}
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
					super::push(out, 46171);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74455);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99022);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116934);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 99023);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 116935);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128273);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136349(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46172);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74456);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 74457);
							match s {
								[] => true,
								['г', s @ ..] => super::t41::n100097(s, out),
								_ => false,
							}
						}
						['х', s @ ..] => super::t41::n75236(s, out),
						_ => false,
					}
				}
				['р', 'о', 'м', '\u{1}', s @ ..] => {
					super::push(out, 46173);
					s.is_empty()
				}
				['у', s @ ..] => super::t41::n46615(s, out),
				['я', s @ ..] => super::t41::n46616(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t41::n21251(s, out),
		['з', s @ ..] => super::t41::n21252(s, out),
		['к', s @ ..] => super::t41::n21253(s, out),
		['л', s @ ..] => super::t41::n21254(s, out),
		['м', s @ ..] => super::t41::n21255(s, out),
		['н', s @ ..] => super::t41::n21256(s, out),
		['п', s @ ..] => super::t42::n21257(s, out),
		['р', s @ ..] => super::t42::n21258(s, out),
		['с', s @ ..] => super::t42::n21259(s, out),
		['т', s @ ..] => super::t42::n21260(s, out),
		['ф', s @ ..] => super::t42::n21261(s, out),
		['х', s @ ..] => super::t42::n21262(s, out),
		['ц', s @ ..] => super::t42::n21263(s, out),
		['ч', s @ ..] => super::t42::n21264(s, out),
		['ш', s @ ..] => super::t42::n21265(s, out),
		_ => false,
	}
}

/// Words ending -увка.
pub(crate) fn n5577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5565);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21153);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21154);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46270);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 46271);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74666);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывка.
pub(crate) fn n5578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5566);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 21155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ювка.
pub(crate) fn n5579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5567);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21156);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 46272);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74667);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явка.
pub(crate) fn n5580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5568);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21157);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 46273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21158);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46274);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74668);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99226);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117070);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 128357);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21159);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 46275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 21160);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46276);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46277);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21161);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21162);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46279);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 74672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46280);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21163);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46281);
					match s {
						[] => true,
						['е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 74674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 46282);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 74675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 46283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21164);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 21166);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 21167);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ет-посадка.
pub(crate) fn n129708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128358);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 134992);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 138750);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 140876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ощадка.
pub(crate) fn n46740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46296);
	match s {
		[] => true,
		['л', 'п', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 99238);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 117076);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128359);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134993);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138751);
													match s {
														[] => true,
														['т', s @ ..] => super::t42::n142322(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 99239);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 117077);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 128360);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134994);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 138752);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едка.
pub(crate) fn n5583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21179);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 46297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21180);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74689);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 74690);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99240);
									s.is_empty()
								}
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
			super::push(out, 21181);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46298);
					match s {
						[] => true,
						['е', 'м', '\u{1}', s @ ..] => {
							super::push(out, 74691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46299);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 74692);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99241);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117078);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128361);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46300);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74693);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99242);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46301);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74694);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99243);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117079);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128362);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21182);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21183);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 46303);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21184);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46304);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46305);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46306);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74698);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99244);
									s.is_empty()
								}
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
			super::push(out, 21185);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 46307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21186);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46308);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21187);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46309);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 46310);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46311);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74701);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 74702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оездка.
pub(crate) fn n46757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46312);
	s.is_empty()
}

/// Words ending -ыездка.
pub(crate) fn n46758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46313);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 74703);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идка.
pub(crate) fn n5585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5573);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21189);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46314);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74704);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99247);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117080);
											s.is_empty()
										}
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
					super::push(out, 46315);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 74705);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99248);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117081);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21190);
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46316);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 74706);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99249);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117082);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128363);
													match s {
														[] => true,
														['с', s @ ..] => super::t42::n136400(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21191);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 46318);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 46319);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99250);
									s.is_empty()
								}
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
			super::push(out, 21192);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46320);
					match s {
						[] => true,
						['в', 'н', 'и', '\u{1}', s @ ..] => {
							super::push(out, 74708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46321);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74709);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 99251);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 117083);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21193);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 21194);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46323);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99252);
									s.is_empty()
								}
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
			super::push(out, 21195);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46324);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74711);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99253);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117084);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21196);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 46325);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99254);
									s.is_empty()
								}
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
			super::push(out, 21197);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46326);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 74713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46327);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 46328);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74714);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99256);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117085);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндка.
pub(crate) fn n5586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21198);
			match s {
				[] => true,
				['л', 'р', s @ ..] => {
					super::push(out, 46329);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одка.
pub(crate) fn n5587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5575);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21199);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46330);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74716);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 99257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117086);
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
								['г', s @ ..] => {
									super::push(out, 99258);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 117087);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 99259);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117088);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128364);
													s.is_empty()
												}
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
							super::push(out, 74717);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99260);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117089);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128365);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46331);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46332);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74719);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99261);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117090);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 99262);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 74720);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 99263);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117091);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 74721);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99264);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117092);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 74722);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99265);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117093);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 74723);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99266);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 99267);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 74724);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99268);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117094);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 74725);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99269);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 117095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 74726);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 74727);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99271);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117096);
											s.is_empty()
										}
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
					super::push(out, 46333);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 46334);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99272);
									s.is_empty()
								}
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
			super::push(out, 21200);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46335);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 74730);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99273);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117097);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128366);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46336);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74731);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99274);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 74732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 46337);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21201);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					super::push(out, 46338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t42::n21316(s, out),
		['х', s @ ..] => super::t42::n21317(s, out),
		_ => false,
	}
}

/// Words ending -рдка.
pub(crate) fn n5588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5576);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21204);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 46349);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74750);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99290);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 117111);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21205);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 46350);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74751);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117112);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21206);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 46351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 21207);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46352);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 74752);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удка.
pub(crate) fn n5589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5577);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21208);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 21209);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 21210);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46353);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74753);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99293);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117113);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21211);
			match s {
				[] => true,
				['г', 'о', s @ ..] => {
					super::push(out, 46354);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74754);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99294);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117114);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128375);
													match s {
														[] => true,
														['п', s @ ..] => super::t42::n136402(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'т', s @ ..] => {
							super::push(out, 74755);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99295);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117115);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128376);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдка.
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5578);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21212);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46355);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -людка.
pub(crate) fn n5591(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21213);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядка.
pub(crate) fn n5592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5579);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21215);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 46356);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 74757);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99296);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117116);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21216);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74758);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99297);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117117);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 128377);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136403(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'х', s @ ..] => {
							super::push(out, 74759);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99298);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117118);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128378);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46357);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74760);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99299);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117119);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21217);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46358);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 46359);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74761);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99300);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117120);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128379);
													match s {
														[] => true,
														['т', s @ ..] => super::t42::n136404(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46360);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 74762);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99301);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117121);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 46361);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осква-река.
pub(crate) fn n129730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128380);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 135000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отека.
pub(crate) fn n21348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21234);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46380);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 74773);
					match s {
						[] => true,
						['б', 'и', 'б', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 117125);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 128381);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135001);
													match s {
														[] => true,
														['з', s @ ..] => super::t42::n140190(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 128382);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135002);
													match s {
														[] => true,
														['г', s @ ..] => super::t42::n140191(s, out),
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
									super::push(out, 117126);
									s.is_empty()
								}
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
			super::push(out, 46381);
			match s {
				[] => true,
				['с', 'и', s @ ..] => {
					super::push(out, 74774);
					match s {
						[] => true,
						['д', '-', 'б', s @ ..] => {
							super::push(out, 99310);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117127);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128383);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135003);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n140192(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аптека.
pub(crate) fn n46830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46383);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 74775);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 99311);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 117128);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128384);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135004);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138758);
													match s {
														[] => true,
														['а', s @ ..] => super::t42::n142325(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трикотажка.
pub(crate) fn n129735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128385);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бабка-ежка.
pub(crate) fn n129736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128386);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 135006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онькобежка.
pub(crate) fn n129737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128387);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 135007);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --тележка.
pub(crate) fn n100396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99320);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 117132);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 128388);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 135008);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 138759);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 140881);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вылежка.
pub(crate) fn n75568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74784);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 99321);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -межка.
pub(crate) fn n21361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21247);
	s.is_empty()
}

/// Words ending -нежка.
pub(crate) fn n21362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21248);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46395);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74785);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 74786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46396);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 74787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46397);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74788);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99323);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117133);
									s.is_empty()
								}
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
			super::push(out, 46398);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оежка.
pub(crate) fn n21363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21249);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 46399);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 74790);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99324);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117134);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128389);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 46400);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74791);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 99325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46401);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 74792);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 46402);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 74793);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99327);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117135);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128390);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -режка.
pub(crate) fn n21364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21250);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46403);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 74794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 46404);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 74795);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46405);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74796);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 74797);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тежка.
pub(crate) fn n21365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21251);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 46406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ижка.
pub(crate) fn n5608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5595);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21252);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46407);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							super::push(out, 74798);
							match s {
								[] => true,
								['е', 'п', '-', s @ ..] => {
									super::push(out, 99328);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117136);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128391);
													match s {
														[] => true,
														['е', s @ ..] => super::t42::n136414(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21254);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21255);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 46409);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 74799);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99329);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117137);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожка.
pub(crate) fn n5609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5596);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21256);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 46410);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74800);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 99330);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117138);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21257);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 46411);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 46412);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46413);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74801);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99331);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117139);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128392);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21258);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46414);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 74802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21259);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 46415);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 46416);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74804);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46417);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74805);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99333);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117140);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 74806);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 74807);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99335);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117141);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 74808);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 99336);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117142);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 74809);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99337);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117143);
											s.is_empty()
										}
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
					super::push(out, 46418);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46419);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74811);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 99338);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117144);
											s.is_empty()
										}
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
					super::push(out, 46420);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74812);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 99339);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 117145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 74813);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99340);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117146);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128393);
													s.is_empty()
												}
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
							super::push(out, 74814);
							match s {
								[] => true,
								['л', s @ ..] => super::t42::n100417(s, out),
								_ => false,
							}
						}
						['д', s @ ..] => super::t42::n75599(s, out),
						['к', s @ ..] => super::t42::n75600(s, out),
						['л', s @ ..] => super::t42::n75601(s, out),
						['н', s @ ..] => super::t42::n75602(s, out),
						['р', s @ ..] => {
							super::push(out, 74818);
							match s {
								[] => true,
								['п', s @ ..] => super::t42::n100425(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t42::n75604(s, out),
						['т', s @ ..] => super::t42::n75605(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t42::n46868(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t42::n21374(s, out),
		_ => false,
	}
}

/// Words ending -ржка.
pub(crate) fn n5610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5597);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21261);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46423);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74827);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 74828);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 74829);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99358);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117158);
											s.is_empty()
										}
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
					super::push(out, 46424);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74830);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99359);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117159);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128401);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21262);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21263);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 46425);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74831);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99360);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117160);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128402);
													s.is_empty()
												}
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
							super::push(out, 74832);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99361);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117161);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128403);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужка.
pub(crate) fn n5611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5598);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21264);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46426);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99362);
									s.is_empty()
								}
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
			super::push(out, 21265);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46427);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 46428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21266);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46429);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21267);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46430);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 46431);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74836);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 46432);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 74837);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99364);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117162);
											s.is_empty()
										}
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
					super::push(out, 46433);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 21268);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46434);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46435);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 46436);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 74841);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 99365);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117163);
											s.is_empty()
										}
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
					super::push(out, 46437);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжка.
pub(crate) fn n5612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5599);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 21269);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46438);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99366);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117164);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21270);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 46439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южка.
pub(crate) fn n5613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5600);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 21271);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46440);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74845);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 46441);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 74846);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99368);
									s.is_empty()
								}
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
			super::push(out, 21272);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46442);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74847);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжка.
pub(crate) fn n5614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5601);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21274);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46444);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74848);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99370);
									s.is_empty()
								}
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
			super::push(out, 21275);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46445);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74849);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99371);
									s.is_empty()
								}
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
			super::push(out, 21276);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46446);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 74850);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99372);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117165);
											s.is_empty()
										}
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
					super::push(out, 46447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21277);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46448);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74851);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 46449);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74852);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99374);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117166);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128404);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46450);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 46451);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74854);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99375);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 117167);
											s.is_empty()
										}
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
					super::push(out, 46452);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74855);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99376);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117168);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 74856);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 74857);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99378);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117169);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46453);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74858);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 46454);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 74859);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117170);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21279);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46455);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74860);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 99381);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117171);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128405);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136416(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46456);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 46457);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74861);
							match s {
								[] => true,
								['р', '-', s @ ..] => {
									super::push(out, 99382);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117172);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 128406);
													match s {
														[] => true,
														['и', s @ ..] => super::t42::n136417(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46458);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льм-сказка.
pub(crate) fn n129757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128407);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 135013);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 138763);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лубоглазка.
pub(crate) fn n129758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128408);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135014);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 138764);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word водолазка.
pub(crate) fn n129759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128409);
	s.is_empty()
}

/// Words ending -назка.
pub(crate) fn n21397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21283);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46468);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 74875);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 99398);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хазка.
pub(crate) fn n21399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21285);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 46470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езка.
pub(crate) fn n5616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5603);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 21286);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46471);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74877);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117182);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21287);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46472);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74878);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99400);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117183);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128410);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46473);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 74879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46474);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 74880);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99401);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117184);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 99402);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117185);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21288);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46475);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 74881);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99403);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117186);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128411);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136420(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46476);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 74882);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99404);
									match s {
										[] => true,
										['л', 'х', '\u{1}', s @ ..] => {
											super::push(out, 117187);
											s.is_empty()
										}
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
					super::push(out, 46477);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99405);
									s.is_empty()
								}
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
			super::push(out, 21289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изка.
pub(crate) fn n5617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5604);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 21290);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46479);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74884);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99406);
									s.is_empty()
								}
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
			super::push(out, 21291);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46480);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74885);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 99407);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117188);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', 'о', 'ж', '\u{1}', s @ ..] => {
							super::push(out, 74886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21292);
			s.is_empty()
		}
		['р', 'ф', '\u{1}', s @ ..] => {
			super::push(out, 21293);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 21294);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46481);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 74887);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99408);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117189);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озка.
pub(crate) fn n5618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5605);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 21295);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46482);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 74888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46483);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46484);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74889);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99410);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117190);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128412);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46485);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99411);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 99412);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117191);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128413);
													match s {
														[] => true,
														['л', s @ ..] => super::t42::n136421(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21296);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46486);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74891);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99413);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117192);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21297);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46487);
					match s {
						[] => true,
						['м', 'ы', s @ ..] => {
							super::push(out, 74892);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99414);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21298);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 46488);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74893);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99415);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117194);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узка.
pub(crate) fn n5619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21299);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 46489);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74894);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99416);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117195);
											s.is_empty()
										}
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
					super::push(out, 46490);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21300);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46491);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74896);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 99417);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117196);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 128414);
													s.is_empty()
												}
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
							super::push(out, 74897);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99418);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117197);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21301);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46492);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74898);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99419);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117198);
											s.is_empty()
										}
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
					super::push(out, 46493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21302);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 46494);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74899);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 74900);
							match s {
								[] => true,
								['а', 'р', '-', s @ ..] => {
									super::push(out, 99420);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117199);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128415);
													match s {
														[] => true,
														['з', s @ ..] => super::t42::n136422(s, out),
														_ => false,
													}
												}
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
							super::push(out, 74901);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 74902);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99421);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117200);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word кыргызка.
pub(crate) fn n5620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5607);
	s.is_empty()
}

/// Words ending -эзка.
pub(crate) fn n5621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5608);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 21303);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 46495);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74903);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язка.
pub(crate) fn n5622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5609);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46496);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 46497);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 46498);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 46499);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 46500);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46501);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 74905);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99423);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117201);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128416);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46502);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 74906);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99424);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117202);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 128417);
													match s {
														[] => true,
														['а', s @ ..] => super::t42::n136423(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46503);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 46504);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -талика.
pub(crate) fn n46969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46521);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 74922);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 99433);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уалика.
pub(crate) fn n46970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46522);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 74923);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 99434);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 117204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -блика.
pub(crate) fn n21436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21321);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46523);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74924);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99435);
							match s {
								[] => true,
								['е', 'р', '-', s @ ..] => {
									super::push(out, 117205);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128418);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135019);
													match s {
														[] => true,
														['р', s @ ..] => super::t42::n140202(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -влика.
pub(crate) fn n21437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21322);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46524);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74925);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 99436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елика.
pub(crate) fn n21438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21323);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 46525);
			match s {
				[] => true,
				['н', 'а', '\u{1}', s @ ..] => {
					super::push(out, 74926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 46526);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74927);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 99437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 46527);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 74928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илика.
pub(crate) fn n21439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21324);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 46528);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 74929);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 46529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 74930);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 99440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 46530);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 74931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клика.
pub(crate) fn n21440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21325);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 46531);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 74932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олика.
pub(crate) fn n21441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21326);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 46532);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74933);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 99441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 46533);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 46534);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плика.
pub(crate) fn n21442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21327);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46535);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 74935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлика.
pub(crate) fn n21443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21328);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 46536);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 74936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46537);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улика.
pub(crate) fn n21444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21329);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 46538);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 74937);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мика.
pub(crate) fn n5632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5619);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21330);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21331);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21332);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21333);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46539);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 74938);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99443);
									s.is_empty()
								}
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
			super::push(out, 21334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ника.
pub(crate) fn n5633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5620);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21335);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 21336);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 46540);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 46541);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 74939);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 46542);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 46543);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 46544);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 46545);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 74942);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 99445);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117206);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21337);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 46546);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74943);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99446);
									s.is_empty()
								}
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
			super::push(out, 21338);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 46547);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74944);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117207);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21339);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 46548);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74945);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99448);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117208);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128419);
													match s {
														[] => true,
														['н', s @ ..] => super::t42::n136425(s, out),
														_ => false,
													}
												}
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
							super::push(out, 74946);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99449);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117209);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128420);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46549);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 74947);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99450);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117210);
											s.is_empty()
										}
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
					super::push(out, 46550);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74948);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 74949);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 46551);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 74950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21340);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46552);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 46553);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74952);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117211);
											s.is_empty()
										}
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
					super::push(out, 46554);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74953);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99454);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117212);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128421);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21341);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 46555);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 74954);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99455);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117213);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21342);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46556);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 46557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 46558);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74956);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99456);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117214);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128422);
													s.is_empty()
												}
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
							super::push(out, 74957);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 74958);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99458);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t42::n118477(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t42::n75744(s, out),
						['х', s @ ..] => super::t42::n75745(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t42::n47007(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t42::n21458(s, out),
		['с', s @ ..] => super::t42::n21459(s, out),
		['у', s @ ..] => super::t42::n21460(s, out),
		['х', s @ ..] => super::t42::n21461(s, out),
		['ь', s @ ..] => super::t42::n21462(s, out),
		['ю', s @ ..] => super::t42::n21463(s, out),
		['я', s @ ..] => super::t42::n21464(s, out),
		_ => false,
	}
}

/// Words ending -пика.
pub(crate) fn n5634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5621);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21350);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рика.
pub(crate) fn n5635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5622);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21351);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 21352);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 21353);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46576);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 74976);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99471);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117224);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 117225);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128428);
													match s {
														[] => true,
														['п', s @ ..] => super::t42::n136429(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 117226);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128429);
													match s {
														[] => true,
														['о', s @ ..] => super::t42::n136430(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46577);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 74977);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99472);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117227);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21354);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46578);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 74978);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 99473);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117228);
											s.is_empty()
										}
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
					super::push(out, 46579);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74979);
							match s {
								[] => true,
								['з', 'э', s @ ..] => {
									super::push(out, 99474);
									s.is_empty()
								}
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
			super::push(out, 21355);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21356);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 46581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21357);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46582);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 46583);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74980);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 99475);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117229);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21358);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сика.
pub(crate) fn n5636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5623);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21359);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 46585);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74982);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99476);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117230);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128430);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21360);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21361);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46586);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46587);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 74984);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99477);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117231);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тика.
pub(crate) fn n5637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21362);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99478);
									match s {
										[] => true,
										['б', 'о', s @ ..] => {
											super::push(out, 117232);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 99479);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117233);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 99480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'и', s @ ..] => {
							super::push(out, 74985);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'о', 'ф', 'н', 'и', 'о', s @ ..] => {
							super::push(out, 74986);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', 'о', s @ ..] => {
					super::push(out, 46588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21363);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 21364);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 46589);
					match s {
						[] => true,
						['р', 'е', 'н', 'э', s @ ..] => {
							super::push(out, 74987);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 99483);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 117234);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128431);
													match s {
														[] => true,
														['е', s @ ..] => super::t42::n136431(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46590);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 74988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 46591);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', 'г', 'о', s @ ..] => {
							super::push(out, 74989);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 74990);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 99484);
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
						['а', s @ ..] => {
							super::push(out, 74991);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 74992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 21365);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74993);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 74994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'к', 'о', s @ ..] => {
					super::push(out, 46592);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 74995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21366);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 74996);
							match s {
								[] => true,
								['и', 'д', 'о', s @ ..] => {
									super::push(out, 99487);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 74997);
							match s {
								[] => true,
								['и', 'ф', 'о', 'р', 'п', 'о', s @ ..] => {
									super::push(out, 99488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 74998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46593);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 74999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 21367);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21368);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 46594);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75000);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99490);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117235);
											s.is_empty()
										}
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
					super::push(out, 46595);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75001);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 46596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 21369);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46597);
					match s {
						[] => true,
						['л', 'к', s @ ..] => {
							super::push(out, 75002);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 99492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t42::n21485(s, out),
		['э', s @ ..] => {
			super::push(out, 21371);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фика.
pub(crate) fn n5638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21372);
			match s {
				[] => true,
				['р', 'г', 'о', 'п', s @ ..] => {
					super::push(out, 46603);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75018);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99512);
									s.is_empty()
								}
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
			super::push(out, 21373);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хика.
pub(crate) fn n5639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5626);
	s.is_empty()
}

/// Words ending -чика.
pub(crate) fn n5640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5627);
	s.is_empty()
}

/// Words ending -шика.
pub(crate) fn n5641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5628);
	s.is_empty()
}

/// Words ending -щика.
pub(crate) fn n5642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5629);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 21374);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 46604);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99513);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117245);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьика.
pub(crate) fn n5643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5630);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 21375);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46605);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 75020);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99514);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117246);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 128438);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -краснобайка.
pub(crate) fn n129790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128439);
	s.is_empty()
}

/// Words ending --непроливайка.
pub(crate) fn n129791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128440);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135035);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 138780);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 140893);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 142115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нагайка.
pub(crate) fn n75814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75026);
	s.is_empty()
}

/// Words ending -огайка.
pub(crate) fn n47067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46614);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 75027);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99519);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 117251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 75028);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргайка.
pub(crate) fn n47068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46615);
	s.is_empty()
}

/// Words ending -дайка.
pub(crate) fn n21494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21379);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 46616);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75029);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46617);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75030);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99522);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жайка.
pub(crate) fn n21495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21380);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зайка.
pub(crate) fn n21496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21381);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46619);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 46620);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75031);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лайка.
pub(crate) fn n21497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21382);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46621);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 46622);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 75032);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99523);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 75033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46623);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 75034);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 99524);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117254);
									s.is_empty()
								}
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
			super::push(out, 46624);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 46625);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 75035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -майка.
pub(crate) fn n21498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21383);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 46626);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75036);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 99525);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117255);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128441);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135036);
													match s {
														[] => true,
														['п', s @ ..] => super::t42::n140216(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46627);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 75037);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 99526);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117256);
									s.is_empty()
								}
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
			super::push(out, 46628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -найка.
pub(crate) fn n21499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46629);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 75038);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 75039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 46630);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 75040);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 46631);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75041);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 99528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 75042);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 99529);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 99530);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117257);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128442);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 135037);
													match s {
														[] => true,
														['л', s @ ..] => super::t42::n140217(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 75043);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 99531);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117258);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128443);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135038);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пайка.
pub(crate) fn n21500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 46632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			super::push(out, 46633);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75045);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 99532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117259);
									s.is_empty()
								}
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
			super::push(out, 46634);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -райка.
pub(crate) fn n21501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21386);
	s.is_empty()
}

/// Words ending -сайка.
pub(crate) fn n21502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21387);
	s.is_empty()
}

/// Words ending -тайка.
pub(crate) fn n21503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21388);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46635);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 46636);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 75046);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 75047);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 99533);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 75048);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 99534);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117261);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128444);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 135039);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 46637);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 75049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 46638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 75050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 99536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 46639);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 46640);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 46641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -файка.
pub(crate) fn n21504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21389);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 46642);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 75051);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хайка.
pub(crate) fn n21505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21390);
	match s {
		[] => true,
		['н', 'а', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 46643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чайка.
pub(crate) fn n21506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21391);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 46644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шайка.
pub(crate) fn n21507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21392);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 46645);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 75052);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 99537);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117262);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейка.
pub(crate) fn n5645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5632);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21393);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					super::push(out, 46646);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 75053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 46647);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75054);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99539);
									s.is_empty()
								}
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
			super::push(out, 21394);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46648);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 46649);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 75055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 46650);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 75056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46651);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75057);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99540);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 117263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 75058);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 75059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 46652);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75060);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 99541);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117264);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21395);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46653);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 75061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 46654);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75062);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 46655);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75063);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 99543);
									s.is_empty()
								}
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
			super::push(out, 21396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46656);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75064);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', '\u{1}', s @ ..] => {
							super::push(out, 75065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'г', s @ ..] => {
					super::push(out, 46657);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75066);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99545);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 117265);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128446);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 75067);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 75068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 46659);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75069);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 99546);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 117266);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128447);
													match s {
														[] => true,
														['р', s @ ..] => super::t42::n136445(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46660);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75070);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 75071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 21397);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46661);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75072);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 99547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46662);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75073);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99548);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 117267);
											s.is_empty()
										}
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
					super::push(out, 46663);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75074);
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
					super::push(out, 46664);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75075);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46665);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 75076);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99549);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117268);
											s.is_empty()
										}
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
					super::push(out, 46666);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 75077);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99550);
									s.is_empty()
								}
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
			super::push(out, 21398);
			s.is_empty()
		}
		['л', s @ ..] => super::t42::n21515(s, out),
		['м', s @ ..] => super::t42::n21516(s, out),
		['н', s @ ..] => {
			super::push(out, 21401);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75097);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99565);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117276);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 75098);
							match s {
								[] => true,
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 99566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t42::n21518(s, out),
		['р', s @ ..] => super::t42::n21519(s, out),
		['с', s @ ..] => super::t42::n21520(s, out),
		['т', s @ ..] => super::t42::n21521(s, out),
		['ф', s @ ..] => super::t42::n21522(s, out),
		['ч', s @ ..] => super::t42::n21523(s, out),
		['ш', s @ ..] => super::t42::n21524(s, out),
		['щ', s @ ..] => super::t42::n21525(s, out),
		_ => false,
	}
}

/// Words ending -ийка.
pub(crate) fn n5646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5633);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21408);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 21409);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46699);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75119);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 99584);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 117290);
											s.is_empty()
										}
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
					super::push(out, 46700);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21410);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 21411);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 46701);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75121);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 99585);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117291);
											s.is_empty()
										}
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
					super::push(out, 46702);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 75122);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99586);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 117292);
											s.is_empty()
										}
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
					super::push(out, 46703);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46704);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75124);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 99587);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117293);
											s.is_empty()
										}
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
					super::push(out, 46705);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 46706);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75125);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99588);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 117294);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128460);
													match s {
														[] => true,
														['р', s @ ..] => super::t42::n136453(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21412);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 21413);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 21414);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 46709);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75126);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 99589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 75127);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 99590);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117295);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128461);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136454(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46710);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21415);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46712);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 75128);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 75129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 46713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21416);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46714);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75130);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 99592);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117296);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21417);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46715);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 46716);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 75131);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 99593);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 117297);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 128462);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t42::n136455(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 46717);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 46718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 21418);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 46719);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 75133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 46720);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t42::n21537(s, out),
		['с', s @ ..] => {
			super::push(out, 21420);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 21421);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 46728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 21422);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46729);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 75148);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 46730);
					match s {
						[] => true,
						['а', s @ ..] => super::t42::n75937(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ойка.
pub(crate) fn n5647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5634);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21423);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46731);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 46732);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75150);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99609);
									s.is_empty()
								}
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
			super::push(out, 21424);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 46733);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 75151);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 99610);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 117309);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 128467);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 99611);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 117310);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 128468);
													match s {
														[] => true,
														['с', s @ ..] => super::t42::n136458(s, out),
														_ => false,
													}
												}
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
							super::push(out, 75152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 46734);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 46735);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 75153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 21425);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46736);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 46737);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 75154);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 99612);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117311);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128469);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21426);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46738);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 75155);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99613);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117312);
											s.is_empty()
										}
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
					super::push(out, 46739);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 99614);
									s.is_empty()
								}
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
			super::push(out, 21427);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 21428);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 46740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 21429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46741);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 75157);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 99615);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 117313);
											s.is_empty()
										}
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
					super::push(out, 46742);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 46743);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75158);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 75159);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 99616);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 117314);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 117315);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128470);
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													super::push(out, 128471);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', 'ж', s @ ..] => {
							super::push(out, 75160);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99617);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 117316);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128472);
													s.is_empty()
												}
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
								['о', s @ ..] => {
									super::push(out, 99618);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117317);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128473);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 99619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 75161);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 75162);
							s.is_empty()
						}
						['т', 'р', s @ ..] => {
							super::push(out, 75163);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99620);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117318);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128474);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 46744);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 75164);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 99621);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117319);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21430);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46745);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 46746);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 75165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t42::n100701(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t42::n21549(s, out),
		['с', s @ ..] => super::t42::n21550(s, out),
		['т', s @ ..] => super::t42::n21551(s, out),
		['ц', s @ ..] => super::t42::n21552(s, out),
		_ => false,
	}
}

/// Words ending -уйка.
pub(crate) fn n5648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5635);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 21435);
			match s {
				[] => true,
				['р', 'у', 'б', '\u{1}', s @ ..] => {
					super::push(out, 46755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 21436);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46756);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 75173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21437);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 46757);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 75174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21438);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46758);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 75175);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 99633);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 117328);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 99634);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 117329);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 21439);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 21440);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 46759);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 75176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыйка.
pub(crate) fn n5649(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 21441);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21442);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 46761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 75177);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 99635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эйка.
pub(crate) fn n5650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5636);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 21443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юйка.
pub(crate) fn n5651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5637);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 21444);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46762);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 75178);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 99636);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

