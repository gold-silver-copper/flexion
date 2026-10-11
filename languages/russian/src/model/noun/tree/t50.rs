use flexion::Path;

/// Words ending -овщина.
pub(crate) fn n50885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50370);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 79156);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 102622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119008);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 129272);
									s.is_empty()
								}
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
			super::push(out, 79157);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 102623);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 79158);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102624);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119010);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 129273);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 135394);
											s.is_empty()
										}
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
					super::push(out, 102625);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 119011);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 129274);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 135395);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 138942);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 102626);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 79159);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102627);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 119013);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 119014);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129275);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 135396);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 138943);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 102628);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 119015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 119016);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129277);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135397);
											s.is_empty()
										}
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
					super::push(out, 102629);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 119017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 102630);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119018);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 129278);
									s.is_empty()
								}
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
			super::push(out, 79160);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102631);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 102632);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 119019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 79161);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102633);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119020);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129279);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135398);
											s.is_empty()
										}
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
					super::push(out, 102634);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129280);
									s.is_empty()
								}
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
			super::push(out, 79162);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102635);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119022);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129281);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135399);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 138944);
													match s {
														[] => true,
														['х', s @ ..] => super::t52::n142420(s, out),
														_ => false,
													}
												}
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
							super::push(out, 119023);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 129282);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135400);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138945);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 102636);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119024);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 102637);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119025);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 129284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 102638);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 119026);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129285);
									match s {
										[] => true,
										['к', s @ ..] => super::t52::n136807(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t52::n103729(s, out),
				['л', s @ ..] => super::t52::n103730(s, out),
				['н', s @ ..] => super::t52::n103731(s, out),
				['о', s @ ..] => super::t52::n103732(s, out),
				['ь', s @ ..] => super::t52::n103733(s, out),
				['я', s @ ..] => super::t52::n103734(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t52::n79993(s, out),
		['м', s @ ..] => super::t52::n79994(s, out),
		['н', s @ ..] => super::t52::n79995(s, out),
		['п', s @ ..] => super::t52::n79996(s, out),
		['р', s @ ..] => super::t52::n79997(s, out),
		['с', s @ ..] => super::t52::n79998(s, out),
		['т', s @ ..] => super::t52::n79999(s, out),
		['ф', s @ ..] => super::t52::n80000(s, out),
		['х', s @ ..] => super::t52::n80001(s, out),
		['ц', s @ ..] => super::t52::n80002(s, out),
		_ => false,
	}
}

/// Words ending -рхангельщина.
pub(crate) fn n140394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бывательщина.
pub(crate) fn n140395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138960);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -любительщина.
pub(crate) fn n140396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138961);
	s.is_empty()
}

/// Words ending -лица-кормилица.
pub(crate) fn n143624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142173);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 142832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -головница.
pub(crate) fn n120674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119412);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 129503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мовница.
pub(crate) fn n81112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80276);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 103339);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 119413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129504);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 129505);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новница.
pub(crate) fn n81113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80277);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 103340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровница.
pub(crate) fn n81114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80278);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 103341);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 119414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 103342);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 119415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129507);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 135514);
									s.is_empty()
								}
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
			super::push(out, 103343);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 119416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховница.
pub(crate) fn n81115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80279);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 103344);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 119417);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 129508);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 135515);
									s.is_empty()
								}
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
			super::push(out, 103345);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увница.
pub(crate) fn n52369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51846);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 80280);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывница.
pub(crate) fn n52370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51847);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 80281);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 103347);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119420);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word прикладница.
pub(crate) fn n140430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138995);
	s.is_empty()
}

/// Words ending -неочередница.
pub(crate) fn n140431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138996);
	s.is_empty()
}

/// Words ending -тпроводница.
pub(crate) fn n136928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135522);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138997);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховодница.
pub(crate) fn n120697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119435);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 129520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сводница.
pub(crate) fn n104455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103364);
	s.is_empty()
}

/// Words ending -годница.
pub(crate) fn n81132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80296);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 103365);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 103366);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119436);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -подница.
pub(crate) fn n81133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80297);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 103367);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 119437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -родница.
pub(crate) fn n81134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80298);
	s.is_empty()
}

/// Words ending -удница.
pub(crate) fn n52377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51854);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 80299);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 103368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80300);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 103369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 103370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдница.
pub(crate) fn n52378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51855);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 80301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядница.
pub(crate) fn n52379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51856);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 80302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80303);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 103372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свояченица.
pub(crate) fn n130881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129529);
	s.is_empty()
}

/// Words ending -шеница.
pub(crate) fn n52389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51866);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 80322);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 80323);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -безобразница.
pub(crate) fn n140433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138998);
	s.is_empty()
}

/// Words ending -кузница.
pub(crate) fn n81197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80361);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 103438);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 119486);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лузница.
pub(crate) fn n81198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80362);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 103439);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яузница.
pub(crate) fn n81199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80363);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 103440);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 119487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзница.
pub(crate) fn n52405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51882);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 80364);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -он-гостиница.
pub(crate) fn n140434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138999);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 141003);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142174);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 142833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ца-разумница.
pub(crate) fn n140435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139000);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141004);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -воспитанница.
pub(crate) fn n140436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139001);
	s.is_empty()
}

/// Words ending -есхлебенница.
pub(crate) fn n140437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139002);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 141005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -венница.
pub(crate) fn n81234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80398);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 103474);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 103475);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 119511);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129563);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 135536);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139003);
											s.is_empty()
										}
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
					super::push(out, 119512);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129564);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 129565);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 129566);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 135538);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 139004);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 141006);
													match s {
														[] => true,
														['о', s @ ..] => super::t52::n143627(s, out),
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
									super::push(out, 135539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 129567);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 135540);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139005);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -луденница.
pub(crate) fn n120777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119514);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 129569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ленница.
pub(crate) fn n81237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80401);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 103479);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 103480);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 119516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 103481);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 103482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 103483);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 119519);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 129571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -менница.
pub(crate) fn n81238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80402);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 103484);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119520);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 129572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 119521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 103485);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 103486);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 103487);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119522);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 129573);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 135541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенница.
pub(crate) fn n81239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80403);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 103488);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 119523);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 129574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренница.
pub(crate) fn n81240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80404);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 103489);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 119524);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 129575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенница.
pub(crate) fn n81241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80405);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 103490);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 103491);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенница.
pub(crate) fn n81242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80406);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 103492);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 119526);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенница.
pub(crate) fn n81243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80407);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 103493);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119527);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 129576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 103494);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 119528);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 129577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инница.
pub(crate) fn n52426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51903);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 80408);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 80409);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 103496);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119529);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 129578);
									s.is_empty()
								}
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
			super::push(out, 80410);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103497);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 103498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 103499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 80411);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103500);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119531);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 129579);
									s.is_empty()
								}
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
			super::push(out, 80412);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103501);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119532);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 129580);
									s.is_empty()
								}
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
			super::push(out, 80413);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103502);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 103503);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119534);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 129581);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135542);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 80414);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 103504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 80415);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 103505);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119535);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 129582);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135543);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 119536);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 129583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онница.
pub(crate) fn n52427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51904);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 80416);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 103506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 80417);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 80418);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 103508);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119537);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 129584);
									s.is_empty()
								}
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
			super::push(out, 80419);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103509);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 119538);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 129585);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135544);
											s.is_empty()
										}
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
					super::push(out, 103510);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 103511);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 103512);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119539);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129586);
									s.is_empty()
								}
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
			super::push(out, 80420);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 103513);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119540);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 103514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 80421);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 80422);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103516);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119542);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 103517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119543);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129589);
									s.is_empty()
								}
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
			super::push(out, 80423);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103518);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119544);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 129590);
									s.is_empty()
								}
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
			super::push(out, 80424);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103519);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 119545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 103520);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 119546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 80425);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 103521);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 119547);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унница.
pub(crate) fn n52428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51905);
	match s {
		[] => true,
		['щ', s @ ..] => {
			super::push(out, 80426);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынница.
pub(crate) fn n52429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51906);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 80427);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янница.
pub(crate) fn n52430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51907);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 80428);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 80429);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103523);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ть-заступница.
pub(crate) fn n140441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139006);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141007);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142177);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -череззерница.
pub(crate) fn n140442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139007);
	s.is_empty()
}

/// Words ending -пустозерница.
pub(crate) fn n140443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139008);
	s.is_empty()
}

/// Words ending -ирница.
pub(crate) fn n52441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51918);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 80458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орница.
pub(crate) fn n52442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51919);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 80459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103557);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103558);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 103559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 80460);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 103560);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119573);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 129600);
									s.is_empty()
								}
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
			super::push(out, 80461);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 80462);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 103562);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119574);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119575);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 119576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 80463);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 103564);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 103565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 80464);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 103566);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129602);
									s.is_empty()
								}
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
			super::push(out, 80465);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 80466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урница.
pub(crate) fn n52443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51920);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 80467);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 103568);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129603);
									s.is_empty()
								}
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
			super::push(out, 80468);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 103569);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119580);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 129604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырница.
pub(crate) fn n52444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51921);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 80469);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 103570);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119581);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129605);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103571);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 119582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 103572);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 80470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрница.
pub(crate) fn n52445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51922);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 80471);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 103573);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119584);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 129606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярница.
pub(crate) fn n52446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51923);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 80472);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 80473);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 103575);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119585);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 129607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкоколосница.
pub(crate) fn n140444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139009);
	s.is_empty()
}

/// Words ending -оллейбусница.
pub(crate) fn n140445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139010);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сокомнатница.
pub(crate) fn n140446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139011);
	s.is_empty()
}

/// Words ending -ца-развратница.
pub(crate) fn n140447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139012);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141009);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142178);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142835);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 143187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тарталетница.
pub(crate) fn n140448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139013);
	s.is_empty()
}

/// Words ending -гаретница.
pub(crate) fn n120876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119613);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 129620);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 135560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кретница.
pub(crate) fn n104708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103617);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 119614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фетница.
pub(crate) fn n81342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80501);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 103618);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итница.
pub(crate) fn n52456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51933);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 80502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'и', s @ ..] => {
			super::push(out, 80503);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 80504);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103621);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 119617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктница.
pub(crate) fn n52457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51934);
	s.is_empty()
}

/// Words ending -отница.
pub(crate) fn n52458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51935);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 80505);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103622);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 80506);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103623);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119619);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129622);
									s.is_empty()
								}
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
			super::push(out, 80507);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103624);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 119620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103625);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 103626);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 80508);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103627);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119623);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 103628);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 80509);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103629);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 119624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 80510);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103630);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119625);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 129626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135561);
											s.is_empty()
										}
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
					super::push(out, 103631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 80511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 80512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103633);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 103634);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119626);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129627);
									s.is_empty()
								}
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
			super::push(out, 80513);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 103635);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119627);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129628);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135562);
											s.is_empty()
										}
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
					super::push(out, 103636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 80514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103637);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртница.
pub(crate) fn n52459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51936);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 80515);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 80516);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 103638);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119629);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 129630);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135563);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стница.
pub(crate) fn n52460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51937);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 80517);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 103639);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 103640);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 119630);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 129631);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135564);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139014);
													match s {
														[] => true,
														['а', s @ ..] => super::t52::n142455(s, out),
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
									super::push(out, 129632);
									s.is_empty()
								}
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
			super::push(out, 80518);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103641);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 103642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119631);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 119632);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 119633);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 129633);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 135565);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139015);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 103643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 80519);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103644);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 80520);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103645);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129634);
									s.is_empty()
								}
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
			super::push(out, 80521);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103646);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 119636);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 129635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 103647);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119637);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129636);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 129637);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135566);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 119638);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129638);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 135567);
											s.is_empty()
										}
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
					super::push(out, 103648);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119639);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129639);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 135568);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139016);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 103649);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119640);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129640);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135569);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139017);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 80522);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 80523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утница.
pub(crate) fn n52461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51938);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 80524);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 80525);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 103652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80526);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 103653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 80527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытница.
pub(crate) fn n52462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51939);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 80528);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119643);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 119644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80529);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятница.
pub(crate) fn n52463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51940);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 80530);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 80531);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'у', '\u{1}', s @ ..] => {
			super::push(out, 80532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -коклюшечница.
pub(crate) fn n140453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139018);
	s.is_empty()
}

/// Words ending -говичница.
pub(crate) fn n120926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119663);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 129649);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 135572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловичница.
pub(crate) fn n120927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119664);
	s.is_empty()
}

/// Words ending -личница.
pub(crate) fn n81392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80551);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 103679);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 119665);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 129650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135573);
									s.is_empty()
								}
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
			super::push(out, 103680);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ничница.
pub(crate) fn n81393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80552);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 103681);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 103682);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119667);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129651);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пичница.
pub(crate) fn n81394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80553);
	s.is_empty()
}

/// Words ending -ричница.
pub(crate) fn n81395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80554);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 103683);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тичница.
pub(crate) fn n81396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80555);
	s.is_empty()
}

/// Words ending -чичница.
pub(crate) fn n81397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80556);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 103684);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119669);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 129652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яичница.
pub(crate) fn n81398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80557);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 103685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -очница.
pub(crate) fn n52470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51947);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 80558);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 103686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 80559);
			match s {
				[] => true,
				['у', 'л', s @ ..] => {
					super::push(out, 103687);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129653);
									s.is_empty()
								}
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
			super::push(out, 80560);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103688);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 80561);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 80562);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103689);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103690);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 119674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 80563);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 80564);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103691);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 103692);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 103693);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 119675);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 119676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 103694);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103695);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 119679);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 119680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 103696);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119681);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 129655);
									s.is_empty()
								}
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
			super::push(out, 80565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103697);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 103698);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119682);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129656);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 135575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 119683);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 119684);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 103699);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 119685);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129658);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135576);
											s.is_empty()
										}
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
						['д', s @ ..] => {
							super::push(out, 119686);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 129659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 119687);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129660);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135577);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 139020);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 103700);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119688);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 129661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 103701);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 80566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103702);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 103703);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 119690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129662);
									s.is_empty()
								}
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
			super::push(out, 80567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103704);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 119691);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 119692);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 119693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 103705);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 103706);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 103707);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119695);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129665);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 135578);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 80568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103708);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119696);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 119697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 103709);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 103710);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119699);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 129667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t52::n104804(s, out),
				['у', s @ ..] => super::t52::n104805(s, out),
				['я', s @ ..] => super::t52::n104806(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t52::n81410(s, out),
		['ш', s @ ..] => super::t52::n81411(s, out),
		_ => false,
	}
}

/// Words ending -учница.
pub(crate) fn n52471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51948);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 80571);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 103722);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 119714);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129674);
									s.is_empty()
								}
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
			super::push(out, 80572);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 80573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80574);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 103724);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 103725);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 119715);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129675);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 135581);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 139023);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 103726);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119716);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103727);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычница.
pub(crate) fn n52472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51949);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 80575);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 103728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 80576);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 103729);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119718);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 129677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючница.
pub(crate) fn n52473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51950);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 80577);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 103730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80578);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 103731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячница.
pub(crate) fn n52474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51951);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 80579);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103732);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 80580);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 103733);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119721);
							match s {
								[] => true,
								['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 129678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -карандашница.
pub(crate) fn n140459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139024);
	s.is_empty()
}

/// Words ending -аушница.
pub(crate) fn n81439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80596);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 103756);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', '\u{1}', s @ ..] => {
			super::push(out, 103757);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лушница.
pub(crate) fn n81440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80597);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 103758);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119740);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 129687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушница.
pub(crate) fn n81442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80599);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 103759);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119741);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 129688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышница.
pub(crate) fn n52480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51957);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 80600);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103760);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80601);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 103761);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119743);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 129689);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135585);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 80602);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 103762);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119744);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 129690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эшница.
pub(crate) fn n52481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51958);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 80603);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 103763);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юшница.
pub(crate) fn n52482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51959);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 80604);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103764);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 119745);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129691);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135586);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['е', s @ ..] => {
					super::push(out, 103765);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 119746);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129692);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135587);
											s.is_empty()
										}
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
					super::push(out, 103766);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сокровищница.
pub(crate) fn n140460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139025);
	s.is_empty()
}

/// Words ending -льница.
pub(crate) fn n52488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51965);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 80611);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 103770);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119750);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 129695);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135590);
											s.is_empty()
										}
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
					super::push(out, 103771);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 119751);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 129696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 103772);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 119752);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 129697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 103773);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 119753);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 129698);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135591);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139026);
													s.is_empty()
												}
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
							super::push(out, 119754);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 119755);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 129700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 103774);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 119756);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 119757);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129702);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 135592);
											s.is_empty()
										}
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
					super::push(out, 103775);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 103776);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119759);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t53::n81456(s, out),
		['и', s @ ..] => {
			super::push(out, 80613);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 103786);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119779);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 119780);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129739);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135620);
											s.is_empty()
										}
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
					super::push(out, 103787);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119781);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 103788);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119782);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129741);
									match s {
										[] => true,
										['ч', '\u{1}', s @ ..] => {
											super::push(out, 135621);
											s.is_empty()
										}
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
					super::push(out, 103789);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 119783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 103790);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119784);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 103791);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 119785);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 103792);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119786);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 103793);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119787);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129745);
									s.is_empty()
								}
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
			super::push(out, 80614);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 103794);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 119788);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129746);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 135622);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 119789);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 129747);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135623);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139055);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t53::n104890(s, out),
				['г', s @ ..] => {
					super::push(out, 103796);
					match s {
						[] => true,
						['и', s @ ..] => super::t53::n121056(s, out),
						['у', s @ ..] => super::t53::n121057(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t53::n104892(s, out),
				['к', s @ ..] => super::t53::n104893(s, out),
				['м', s @ ..] => super::t53::n104894(s, out),
				['т', s @ ..] => super::t53::n104895(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t53::n81459(s, out),
		['ы', s @ ..] => super::t53::n81460(s, out),
		_ => false,
	}
}

/// Words ending --императрица.
pub(crate) fn n140496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139059);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141036);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142196);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 142851);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 143198);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143374);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -конвейерщица.
pub(crate) fn n140500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139063);
	s.is_empty()
}

/// Words ending -омпьютерщица.
pub(crate) fn n140501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139064);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -склеивальщица.
pub(crate) fn n140502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139065);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141040);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ладывальщица.
pub(crate) fn n140503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139066);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141041);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матывальщица.
pub(crate) fn n140504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139067);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141042);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142200);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142853);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зальщица.
pub(crate) fn n105035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 119898);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 129817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', '\u{1}', s @ ..] => {
			super::push(out, 119899);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 119900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кальщица.
pub(crate) fn n105036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103937);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 119901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 129818);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 129819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 129820);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 135656);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139068);
									s.is_empty()
								}
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
			super::push(out, 119902);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 119903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 129821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пальщица.
pub(crate) fn n105038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103939);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 119904);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 129822);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 135657);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 139069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 129823);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 135658);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тальщица.
pub(crate) fn n105040(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'м', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 119905);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 119906);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 129824);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 135659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хальщица.
pub(crate) fn n105041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103941);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 119907);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 129825);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 129826);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 135660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 119908);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 129827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ельщица.
pub(crate) fn n81607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80761);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 103942);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119909);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129828);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 135662);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 129829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'м', s @ ..] => {
					super::push(out, 119910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'н', s @ ..] => {
			super::push(out, 103943);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 119911);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129831);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 135663);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139072);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 103944);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119912);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 129832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135664);
									s.is_empty()
								}
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
			super::push(out, 103945);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 103946);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 103947);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119913);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129833);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 135665);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139073);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['а', s @ ..] => {
					super::push(out, 119914);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 129834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'п', s @ ..] => {
					super::push(out, 119915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 103948);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119916);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129836);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильщица.
pub(crate) fn n81608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80762);
	match s {
		[] => true,
		['д', 'я', 'р', 'п', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 103949);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 103950);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 119917);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 129837);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 135667);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'с', s @ ..] => {
			super::push(out, 103951);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 119918);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 129838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гольщица.
pub(crate) fn n81609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80763);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 103952);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119919);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пульщица.
pub(crate) fn n81610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80764);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 103953);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еяльщица.
pub(crate) fn n81611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80765);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 103954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 103955);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119922);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имик-технолог.
pub(crate) fn n142499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141054);
	s.is_empty()
}

/// Words ending -хник-технолог.
pub(crate) fn n142500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141055);
	s.is_empty()
}

/// Words ending -оном-технолог.
pub(crate) fn n142501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141056);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142204);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пиар-технолог.
pub(crate) fn n142502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141057);
	s.is_empty()
}

/// Words ending -енер-технолог.
pub(crate) fn n142503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141058);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 142205);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивание.
pub(crate) fn n54366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53821);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82005);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 82006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104667);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 104668);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 104669);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 104670);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 82007);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104671);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 104672);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 104673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 82008);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 82009);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 82010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104674);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 120282);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 120283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 120284);
							match s {
								[] => true,
								['о', 'м', 'а', s @ ..] => {
									super::push(out, 130008);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 135762);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 120285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 104675);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 104676);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120286);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 120287);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 130009);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 130010);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135763);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139123);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n142516(s, out),
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
									super::push(out, 130011);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 130012);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 120288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 104677);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 120289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130013);
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
						['р', s @ ..] => {
							super::push(out, 120290);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 130014);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135764);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 120291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 104678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120292);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 120293);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 120294);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130015);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 135765);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 139124);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 130016);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135766);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 135767);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139125);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 104679);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 104680);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 104681);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130017);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 130018);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 135768);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82011);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 104682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t53::n82860(s, out),
		['н', s @ ..] => {
			super::push(out, 82013);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104694);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 104695);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 104696);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 120313);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 120314);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 120315);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 120316);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 120317);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 120318);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 120319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 104697);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 104698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 82014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104699);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 104700);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 104701);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 104702);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 104703);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 104704);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 104705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t53::n82863(s, out),
		['х', s @ ..] => {
			super::push(out, 82016);
			s.is_empty()
		}
		['ч', s @ ..] => super::t53::n82865(s, out),
		['ш', s @ ..] => super::t53::n82866(s, out),
		['щ', s @ ..] => {
			super::push(out, 82019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ование.
pub(crate) fn n54367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53822);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 82020);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 104726);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120356);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 130057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 104727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 120357);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 130058);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 135792);
											s.is_empty()
										}
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
					super::push(out, 104728);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120358);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 130059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 104729);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 120359);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 130060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 104730);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120360);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130061);
									s.is_empty()
								}
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
			super::push(out, 82021);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120361);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t53::n121627(s, out),
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 104731);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120362);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130074);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 135800);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82022);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 104732);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 120363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t53::n82871(s, out),
		['з', s @ ..] => {
			super::push(out, 82024);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104739);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120376);
							match s {
								[] => true,
								['б', 'о', s @ ..] => super::t53::n131437(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 104740);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 104741);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120377);
							match s {
								[] => true,
								['о', s @ ..] => super::t53::n131438(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t53::n82873(s, out),
		['л', s @ ..] => super::t53::n82874(s, out),
		['м', s @ ..] => super::t53::n82875(s, out),
		['н', s @ ..] => super::t53::n82876(s, out),
		['п', s @ ..] => super::t53::n82877(s, out),
		['р', s @ ..] => super::t53::n82878(s, out),
		['с', s @ ..] => super::t53::n82879(s, out),
		['т', s @ ..] => super::t53::n82880(s, out),
		['ф', s @ ..] => super::t53::n82881(s, out),
		['х', s @ ..] => super::t53::n82882(s, out),
		_ => false,
	}
}

/// Words ending -бывание.
pub(crate) fn n82885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82037);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104796);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 104797);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120454);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 130176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 130177);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135878);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 130178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135879);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 135880);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 139203);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 104798);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120455);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 104799);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 104800);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 104801);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 104802);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 104803);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 104804);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вывание.
pub(crate) fn n82886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82038);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104805);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 104806);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120461);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130181);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135881);
									s.is_empty()
								}
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
			super::push(out, 104807);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 104808);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 104809);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120463);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130182);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135882);
									s.is_empty()
								}
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
			super::push(out, 104810);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 120464);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 130183);
							match s {
								[] => true,
								['р', 'з', s @ ..] => {
									super::push(out, 135883);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зывание.
pub(crate) fn n82888(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104811);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 120466);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 120467);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 120468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 104812);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 104813);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 104814);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120470);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135884);
									s.is_empty()
								}
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
			super::push(out, 104815);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 120472);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 120473);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 120474);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135885);
									s.is_empty()
								}
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
			super::push(out, 104816);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 120475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 104817);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 104818);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 104819);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 104820);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лывание.
pub(crate) fn n82889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82040);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104821);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 120479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 104822);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 104823);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120481);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 130192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 120482);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мывание.
pub(crate) fn n82890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82041);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104824);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 120484);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 120485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 104825);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 104826);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120487);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', 'е', s @ ..] => {
			super::push(out, 104827);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 120488);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 104828);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120489);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 130198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 120490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 104829);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 120491);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 120492);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 104830);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 104831);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 104832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120494);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 120495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 104833);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word изнывание.
pub(crate) fn n121766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120497);
	s.is_empty()
}

/// Words ending -пывание.
pub(crate) fn n82892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82043);
	s.is_empty()
}

/// Words ending -рывание.
pub(crate) fn n82893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82044);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104835);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 120498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 120499);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 120500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 120501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 104836);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 104837);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 104838);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 120503);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 120504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 104839);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120505);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 104840);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 120506);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 130204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 104841);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 104842);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 120508);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 104843);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 120509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 104844);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 104845);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 104846);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 104847);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 120511);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сывание.
pub(crate) fn n82894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82045);
	s.is_empty()
}

/// Words ending -тывание.
pub(crate) fn n82895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82046);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 104848);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 104849);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120512);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 130206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 120513);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 120514);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 120515);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 120516);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сть-сказание.
pub(crate) fn n140645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139205);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141135);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 142250);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142879);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -микрорезание.
pub(crate) fn n140646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139206);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -искание.
pub(crate) fn n82955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82102);
	s.is_empty()
}

/// Words ending -ускание.
pub(crate) fn n82956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82103);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 104901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыскание.
pub(crate) fn n82957(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 104902);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 104903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укание.
pub(crate) fn n54408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53861);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 82104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'о', s @ ..] => {
			super::push(out, 82105);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 104905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 82106);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 82107);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 104906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 82108);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 82109);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 82110);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 104907);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 120549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 82111);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 104908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чкание.
pub(crate) fn n54409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53862);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82112);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 104909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 82113);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 104910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкание.
pub(crate) fn n54410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53863);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 82114);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 104911);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыкание.
pub(crate) fn n54411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53864);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 104912);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 104913);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 82115);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 104914);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 120552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 82116);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 104915);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 104916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', '\u{1}', s @ ..] => {
			super::push(out, 82117);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 104917);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 104918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 82118);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 104919);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькание.
pub(crate) fn n54412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53865);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82119);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 104920);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 120554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 82120);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 104921);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120555);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 120556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкание.
pub(crate) fn n54413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53866);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82121);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 104922);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120557);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 130218);
									s.is_empty()
								}
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
			super::push(out, 82122);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 104923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 82123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -якание.
pub(crate) fn n54414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53867);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 82124);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 82125);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 104924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 104925);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 104926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 82126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аз-воспоминание.
pub(crate) fn n140647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139207);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ан-воспоминание.
pub(crate) fn n140648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139208);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 141138);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обработание.
pub(crate) fn n140649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139209);
	s.is_empty()
}

/// The word ябедничание.
pub(crate) fn n140650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139210);
	s.is_empty()
}

/// Words ending -скаредничание.
pub(crate) fn n140651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139211);
	s.is_empty()
}

/// Words ending -сумерничание.
pub(crate) fn n140652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139212);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141139);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word озорничание.
pub(crate) fn n140653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139213);
	s.is_empty()
}

/// Words ending -лодырничание.
pub(crate) fn n140654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139214);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обведение.
pub(crate) fn n121865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120596);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 130241);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -введение.
pub(crate) fn n106100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104989);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 120597);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 130242);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 135904);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 139217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дведение.
pub(crate) fn n106101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104990);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 120598);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 120599);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 130243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еведение.
pub(crate) fn n106102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104991);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 120600);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 130244);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 130245);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 135905);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 139218);
									s.is_empty()
								}
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
			super::push(out, 120601);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 120602);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 120603);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 120604);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 130246);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 135906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 130247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 120605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зведение.
pub(crate) fn n106103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104992);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 120606);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 130248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 120607);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 130249);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иведение.
pub(crate) fn n106104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104993);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 120608);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 130250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оведение.
pub(crate) fn n106105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104994);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 120609);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 120610);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 130251);
					match s {
						[] => true,
						['с', 'с', s @ ..] => {
							super::push(out, 135907);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 139220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', 'и', s @ ..] => {
			super::push(out, 120611);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 120612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 130252);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 130253);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 135908);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 139221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 135909);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 130254);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 135910);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139222);
									s.is_empty()
								}
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
			super::push(out, 120613);
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 130255);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 130256);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 135911);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 139223);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141143);
											s.is_empty()
										}
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
					super::push(out, 130257);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 135912);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 139224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 130258);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 135913);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 139225);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 141144);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142252);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 120614);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 120615);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 130259);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 135914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 130260);
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					super::push(out, 130261);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 135915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 130262);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 135916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 120616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 130263);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 130264);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 135917);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 139226);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 141145);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 142253);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 120617);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 130265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135918);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 135919);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 139227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 130266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 120618);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130267);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 130268);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 135920);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 139228);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 141146);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 120619);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 130269);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 135921);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 139229);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141147);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 120620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тведение.
pub(crate) fn n106106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104995);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 120621);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 130270);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 135922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыведение.
pub(crate) fn n106107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104996);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 120622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -недение.
pub(crate) fn n83069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82210);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 104997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оедение.
pub(crate) fn n83070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82211);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 104998);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 120624);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -редение.
pub(crate) fn n83071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82212);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 104999);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 120625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -седение.
pub(crate) fn n83072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82213);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 105000);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 105001);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 120626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ъедение.
pub(crate) fn n83073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82214);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 105002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 120627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 105003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ждение.
pub(crate) fn n54487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53940);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82215);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 105004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 82216);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					super::push(out, 105005);
					match s {
						[] => true,
						['у', 'д', s @ ..] => {
							super::push(out, 120628);
							match s {
								[] => true,
								['е', 'р', 'п', '-', s @ ..] => {
									super::push(out, 130272);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 135923);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139230);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n142594(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82217);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 105006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 82218);
			match s {
				[] => true,
				['в', 'о', 'т', 'е', 'л', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
					super::push(out, 105007);
					s.is_empty()
				}
				['х', 'с', s @ ..] => {
					super::push(out, 105008);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 120630);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 130273);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135924);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 139231);
													match s {
														[] => true,
														['з', s @ ..] => super::t53::n142595(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82219);
			match s {
				[] => true,
				['е', 'в', 'т', 'у', '-', s @ ..] => {
					super::push(out, 105009);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 120631);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130274);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135925);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139232);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n142596(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идение.
pub(crate) fn n54488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53941);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 82221);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 105010);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 105011);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 105012);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120633);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 130275);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 130276);
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
						['л', s @ ..] => {
							super::push(out, 120634);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 120635);
							s.is_empty()
						}
						['н', 'с', s @ ..] => {
							super::push(out, 120636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130277);
									s.is_empty()
								}
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
			super::push(out, 82222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдение.
pub(crate) fn n54489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82223);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдение.
pub(crate) fn n54490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53943);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 82224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удение.
pub(crate) fn n54491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53944);
	s.is_empty()
}

/// Words ending -юдение.
pub(crate) fn n54492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53945);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 105014);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 105015);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 120637);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 130278);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135926);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139233);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядение.
pub(crate) fn n54493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53946);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 82225);
			match s {
				[] => true,
				['п', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 105016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --предостережение.
pub(crate) fn n140674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139234);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141151);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142257);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 142882);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143209);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143377);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обездвижение.
pub(crate) fn n140675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139235);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141152);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товоздвижение.
pub(crate) fn n140676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139236);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142258);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обезвожение.
pub(crate) fn n140677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139237);
	s.is_empty()
}

/// Words ending -благорожение.
pub(crate) fn n140678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139238);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ужение.
pub(crate) fn n54502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53955);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82236);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105026);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 120648);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 130286);
									match s {
										[] => true,
										['о', 'е', s @ ..] => {
											super::push(out, 135934);
											s.is_empty()
										}
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
					super::push(out, 105027);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 105028);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120649);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130287);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 135935);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139239);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n142601(s, out),
														_ => false,
													}
												}
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
							super::push(out, 120650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжение.
pub(crate) fn n54503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53956);
	match s {
		[] => true,
		['т', 'о', s @ ..] => {
			super::push(out, 82237);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 105029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -южение.
pub(crate) fn n54504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53957);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 82238);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 105030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжение.
pub(crate) fn n54505(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 82239);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 105031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120653);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 130288);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135936);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['\u{1}', s @ ..] => {
					super::push(out, 105032);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 105033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 82240);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word крахмаление.
pub(crate) fn n140680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139240);
	s.is_empty()
}

/// Words ending -обезглавление.
pub(crate) fn n140681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139241);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141156);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word муравление.
pub(crate) fn n137351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135941);
	s.is_empty()
}

/// Words ending -тавление.
pub(crate) fn n106159(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 120667);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130299);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 135942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139242);
									s.is_empty()
								}
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
			super::push(out, 120668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ивление.
pub(crate) fn n83114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82251);
	s.is_empty()
}

/// Words ending -овление.
pub(crate) fn n83115(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 105044);
			match s {
				[] => true,
				['р', 'е', 'ц', 'с', s @ ..] => {
					super::push(out, 120669);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130300);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135943);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139243);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 105045);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 105046);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 120670);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130301);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135944);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139244);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141157);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143711(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твление.
pub(crate) fn n83116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82252);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 105047);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 120671);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 120672);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 130302);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 135945);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 139245);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 141158);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143712(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 120673);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 130303);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 135946);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 139246);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 141159);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143713(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явление.
pub(crate) fn n83117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82253);
	s.is_empty()
}

/// Words ending -еление.
pub(crate) fn n54509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53961);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 82254);
			match s {
				[] => true,
				['е', 'р', 'п', 'о', s @ ..] => {
					super::push(out, 105048);
					match s {
						[] => true,
						['о', 'д', s @ ..] => {
							super::push(out, 120674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130304);
									s.is_empty()
								}
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
			super::push(out, 82255);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105049);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120675);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 130305);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135947);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 139247);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n142606(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82256);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105050);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 120676);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 130306);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 135948);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 139248);
													match s {
														[] => true,
														['н', s @ ..] => super::t53::n142607(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105051);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мление.
pub(crate) fn n54510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53962);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 82258);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 105052);
					match s {
						[] => true,
						['е', 'в', 'у', '-', s @ ..] => {
							super::push(out, 120677);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 130307);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135949);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 139249);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n142608(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82259);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					super::push(out, 105053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оление.
pub(crate) fn n54511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53963);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 82260);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 105054);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 120678);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130308);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135950);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139250);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 105055);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 105056);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 120679);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 130309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135951);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пление.
pub(crate) fn n54512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53964);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 82261);
			match s {
				[] => true,
				['т', 'с', 'с', s @ ..] => {
					super::push(out, 105057);
					match s {
						[] => true,
						['и', 'о', s @ ..] => {
							super::push(out, 120680);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 130310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сление.
pub(crate) fn n54513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53965);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 105058);
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120681);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 120682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 82262);
			match s {
				[] => true,
				['м', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', 'а', s @ ..] => {
							super::push(out, 120683);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 130311);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135952);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139251);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n142609(s, out),
														_ => false,
													}
												}
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
							super::push(out, 120684);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130312);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 135953);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139252);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n142610(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тление.
pub(crate) fn n54514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53966);
	s.is_empty()
}

/// Words ending -шление.
pub(crate) fn n54515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53967);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 82263);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120685);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130313);
									match s {
										[] => true,
										['р', '-', s @ ..] => {
											super::push(out, 135954);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 139253);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n142611(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 139254);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n142612(s, out),
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
									super::push(out, 130314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 120686);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 120687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыление.
pub(crate) fn n54516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53968);
	s.is_empty()
}

/// Words ending -яление.
pub(crate) fn n54517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53969);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 82264);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 82265);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обесценение.
pub(crate) fn n140696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139256);
	s.is_empty()
}

/// Words ending -ма-дополнение.
pub(crate) fn n140697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139257);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 141168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142266);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142887);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 143213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олудополнение.
pub(crate) fn n140698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139258);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -блюмингостроение.
pub(crate) fn n142616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141170);
	s.is_empty()
}

/// Words ending -евоизмерение.
pub(crate) fn n140700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139260);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141171);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 142268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word транжирение.
pub(crate) fn n140701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139261);
	s.is_empty()
}

/// Words ending -агорастворение.
pub(crate) fn n140702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139262);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141172);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 142269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аторение.
pub(crate) fn n106213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105092);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 120709);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130332);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 135969);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урение.
pub(crate) fn n54542(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 82296);
			match s {
				[] => true,
				['е', 'т', 'г', 'е', 'д', '\u{1}', s @ ..] => {
					super::push(out, 105093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 82297);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word обезлесение.
pub(crate) fn n140704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139264);
	s.is_empty()
}

/// Words ending -ичение.
pub(crate) fn n54550(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 105117);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 105118);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 130346);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135979);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 139265);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n142619(s, out),
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
									super::push(out, 130347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'е', s @ ..] => {
					super::push(out, 105119);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 120727);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130348);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135980);
											s.is_empty()
										}
										_ => false,
									}
								}
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
					super::push(out, 105120);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 105121);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 120729);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 130349);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135981);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139266);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янчение.
pub(crate) fn n83187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82316);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 105122);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 120730);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очение.
pub(crate) fn n54552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53998);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 82317);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 105123);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120731);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130350);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135982);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 139267);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82318);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 82319);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105125);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 120732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 105126);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 120733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 105127);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 120734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 105128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 82320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105129);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 120735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 105130);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 120736);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130351);
									match s {
										[] => true,
										['р', 'с', s @ ..] => {
											match s {
												[] => true,
												['о', 'с', 'с', s @ ..] => {
													super::push(out, 139268);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n142620(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 139269);
													s.is_empty()
												}
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
							super::push(out, 120737);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 130352);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 135983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 120738);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130353);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135984);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 120739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учение.
pub(crate) fn n54554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54000);
	match s {
		[] => true,
		['б', '\u{1}', s @ ..] => {
			super::push(out, 82321);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 82322);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 105131);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130354);
									s.is_empty()
								}
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
			super::push(out, 82323);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 82324);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 82325);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 82326);
			match s {
				[] => true,
				['к', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 105133);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычение.
pub(crate) fn n54555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54001);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 82327);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 105134);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 120741);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 130355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючение.
pub(crate) fn n54557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54003);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82328);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 82329);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 105135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 120742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячение.
pub(crate) fn n54558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54004);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 82330);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105136);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 120743);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130356);
									s.is_empty()
								}
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
			super::push(out, 82331);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 82332);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105137);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 120744);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьмо-обращение.
pub(crate) fn n140710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139270);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141175);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142271);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 142888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кт-извещение.
pub(crate) fn n140711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139271);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142272);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -благовещение.
pub(crate) fn n140712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139272);
	s.is_empty()
}

/// Words ending -ервоперемещение.
pub(crate) fn n140713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139273);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141177);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мухоловковые.
pub(crate) fn n140743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139303);
	s.is_empty()
}

/// Words ending -ладанниковые.
pub(crate) fn n140744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139304);
	s.is_empty()
}

/// Words ending -ножгутиковые.
pub(crate) fn n140745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139305);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 141191);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кузнечиковые.
pub(crate) fn n140746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139306);
	s.is_empty()
}

/// Words ending -ьчиковые.
pub(crate) fn n106648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105525);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 120976);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 130480);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 136048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нковые.
pub(crate) fn n55211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54652);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82884);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 105526);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 105527);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 120978);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 130481);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136049);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82885);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 105528);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120979);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 130482);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136050);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82886);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105529);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120980);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 130483);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136051);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82887);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 105530);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120981);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130484);
									s.is_empty()
								}
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
			super::push(out, 82888);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 105531);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120982);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 130485);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136052);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 139307);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 105532);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120983);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 130486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 120984);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130487);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 136053);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соковые.
pub(crate) fn n83763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82889);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 105533);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рковые.
pub(crate) fn n55213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54654);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 82890);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 105534);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сковые.
pub(crate) fn n55214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54655);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 82891);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105535);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 120986);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 130488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 120987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 82892);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105536);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тковые.
pub(crate) fn n55215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54656);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82893);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 105537);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120988);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 130489);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 136054);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 139308);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82894);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 105538);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 120989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 82895);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 105539);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 120990);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 130490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уковые.
pub(crate) fn n55216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54657);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 82896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чковые.
pub(crate) fn n55217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54658);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 82897);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105540);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 120991);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 130491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шковые.
pub(crate) fn n55218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54659);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 82898);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 105541);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 120992);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 130492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 105542);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 120993);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130493);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136055);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82899);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 105543);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 120994);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130494);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136056);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 82900);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105544);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120995);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 130495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яковые.
pub(crate) fn n55219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54660);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 82901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105545);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 120996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кирказоновые.
pub(crate) fn n140752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139312);
	s.is_empty()
}

/// Words ending -филлофоровые.
pub(crate) fn n140753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139313);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -барбарисовые.
pub(crate) fn n140754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139314);
	s.is_empty()
}

/// Words ending -опомицетовые.
pub(crate) fn n140755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139315);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -арнокопытные.
pub(crate) fn n140772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139332);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сопереживанье.
pub(crate) fn n140775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139335);
	s.is_empty()
}

/// Words ending -ыпиванье.
pub(crate) fn n106817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105693);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 121115);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 130589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риванье.
pub(crate) fn n83951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83075);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 105694);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 121116);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 130590);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136110);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 139336);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141205);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чиванье.
pub(crate) fn n83953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83077);
	s.is_empty()
}

/// Words ending -шиванье.
pub(crate) fn n83954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83078);
	s.is_empty()
}

/// Words ending -ованье.
pub(crate) fn n55394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54835);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105695);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121117);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 130591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 105696);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 83079);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121119);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130592);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 136111);
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
								['в', s @ ..] => {
									super::push(out, 130593);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 130594);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 136112);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 136113);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139337);
													s.is_empty()
												}
												_ => false,
											}
										}
										['щ', s @ ..] => {
											super::push(out, 136114);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 139338);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n142652(s, out),
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
									super::push(out, 130595);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 130596);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136115);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 139339);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 130597);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 130598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 105697);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121120);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130599);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 136116);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 83080);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105698);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 105699);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121121);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 130600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 121122);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130601);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 136117);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139340);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 105700);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121123);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 130602);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136118);
											s.is_empty()
										}
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
					super::push(out, 105701);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 105702);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 121124);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130603);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136119);
											s.is_empty()
										}
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
					super::push(out, 105703);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 105704);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121125);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130604);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136120);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105705);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 105706);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121127);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130605);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136121);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 83081);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 83082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105707);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 121128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 121130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 105708);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 121131);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 130607);
									s.is_empty()
								}
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
			super::push(out, 83083);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 105709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t53::n83962(s, out),
		['п', s @ ..] => {
			super::push(out, 83085);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 105714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t53::n83964(s, out),
		['с', s @ ..] => {
			super::push(out, 83087);
			match s {
				[] => true,
				['о', 'т', s @ ..] => super::t53::n106847(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t53::n83966(s, out),
		['ф', s @ ..] => {
			super::push(out, 83089);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 83090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рванье.
pub(crate) fn n55395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54836);
	s.is_empty()
}

/// Words ending -уванье.
pub(crate) fn n55396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54837);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 83091);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 83092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыванье.
pub(crate) fn n55397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54838);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 83093);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105725);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 121150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 105726);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121151);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 130619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 105727);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 121152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 105728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', 'д', s @ ..] => {
			super::push(out, 83094);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105729);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121153);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130620);
									s.is_empty()
								}
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
			super::push(out, 83095);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 121154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 121155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 105731);
					s.is_empty()
				}
				['и', 'р', s @ ..] => {
					super::push(out, 105732);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130622);
									s.is_empty()
								}
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
				['о', s @ ..] => {
					super::push(out, 105733);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121157);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 130623);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136129);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 121158);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 121159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'з', s @ ..] => {
			super::push(out, 83096);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 105734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 121160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 83097);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 105735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 83098);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 105736);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 121161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 105737);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 121162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 105738);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 105739);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 105740);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 105741);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 121165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 83099);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 83100);
			match s {
				[] => true,
				['с', 'о', s @ ..] => {
					super::push(out, 105742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 121166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нуканье.
pub(crate) fn n84033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83147);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 105784);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 121190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 130629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туканье.
pub(crate) fn n84034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83148);
	s.is_empty()
}

/// Words ending -цуканье.
pub(crate) fn n84035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83149);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 105785);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чканье.
pub(crate) fn n55435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54873);
	s.is_empty()
}

/// Words ending -ыканье.
pub(crate) fn n55436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54874);
	match s {
		[] => true,
		['в', 'и', s @ ..] => {
			super::push(out, 83150);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105786);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 83151);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 83152);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105788);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 121192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 105789);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 105790);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 105791);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121194);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 130630);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136132);
											s.is_empty()
										}
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
					super::push(out, 105792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'б', s @ ..] => {
			super::push(out, 83153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', s @ ..] => {
			super::push(out, 83154);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 105794);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 121195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьканье.
pub(crate) fn n55437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54875);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105795);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 105796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 83155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юканье.
pub(crate) fn n55438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54876);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 83156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яканье.
pub(crate) fn n55439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54877);
	s.is_empty()
}

/// Words ending -домовничанье.
pub(crate) fn n140786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139346);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141209);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ябедничанье.
pub(crate) fn n140787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139347);
	s.is_empty()
}

/// Words ending -набожничанье.
pub(crate) fn n140788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139348);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерьезничанье.
pub(crate) fn n140789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139349);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сумерничанье.
pub(crate) fn n140790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зорничанье.
pub(crate) fn n132002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130645);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 136141);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139351);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырничанье.
pub(crate) fn n122496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121223);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 130646);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 136142);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139352);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сничанье.
pub(crate) fn n106973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105845);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 121225);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 130647);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 136143);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 139353);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141214);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьничанье.
pub(crate) fn n106975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105847);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 121226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ричанье.
pub(crate) fn n84108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83220);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 105848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лчанье.
pub(crate) fn n55497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54933);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 83221);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 105849);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 121227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчанье.
pub(crate) fn n55498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54934);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 83222);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 105850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 83223);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 105851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очанье.
pub(crate) fn n55499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54935);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 83224);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105852);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 121228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчанье.
pub(crate) fn n55500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54936);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 83225);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 105853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 83226);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 83227);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 105854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учанье.
pub(crate) fn n55501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54937);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 83228);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 105855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 83229);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 105856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 83230);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 105857);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычанье.
pub(crate) fn n55502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54938);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 83231);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 83232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ячанье.
pub(crate) fn n55503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54939);
	s.is_empty()
}

/// Words ending -кочевряженье.
pub(crate) fn n140795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139355);
	s.is_empty()
}

/// The word картавленье.
pub(crate) fn n140796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139356);
	s.is_empty()
}

/// Words ending -произволенье.
pub(crate) fn n140797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139357);
	s.is_empty()
}

/// The word тараторенье.
pub(crate) fn n140798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139358);
	s.is_empty()
}

/// Words ending -лагоусмотренье.
pub(crate) fn n140799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139359);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 141216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фордыбаченье.
pub(crate) fn n140800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139360);
	s.is_empty()
}

/// Words ending -увековеченье.
pub(crate) fn n140801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139361);
	s.is_empty()
}

/// Words ending -очеловеченье.
pub(crate) fn n140802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139362);
	s.is_empty()
}

/// Words ending -оченье.
pub(crate) fn n55547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54978);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 83287);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 105906);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121271);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130677);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136161);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 139363);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 83288);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 83289);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105908);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121272);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 130678);
									s.is_empty()
								}
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
				['о', s @ ..] => {
					super::push(out, 105909);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 121273);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 130679);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136162);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 139364);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n142663(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 105910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыченье.
pub(crate) fn n55549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54980);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 83294);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 105913);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121274);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 130680);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юченье.
pub(crate) fn n55550(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 83295);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 105914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 83296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105915);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 121275);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яченье.
pub(crate) fn n55551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54981);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 83297);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 105916);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 105917);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 121277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word благовещенье.
pub(crate) fn n140805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139365);
	s.is_empty()
}

/// Words ending -прагреческий.
pub(crate) fn n140837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139397);
	s.is_empty()
}

/// Words ending -рагерманский.
pub(crate) fn n140838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139398);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141232);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-мурманский.
pub(crate) fn n140839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139399);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ь-мурманский.
pub(crate) fn n140840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139400);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141234);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142287);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142889);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143214);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 143379);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ломенский.
pub(crate) fn n122890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121616);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130846);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 136243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инский.
pub(crate) fn n57200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 84429);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 106564);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 121617);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 130847);
									s.is_empty()
								}
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
			super::push(out, 84430);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 106565);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 106566);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 84431);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 106567);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121620);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 130848);
									s.is_empty()
								}
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
			super::push(out, 84432);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 106568);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121621);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 130849);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136244);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 84433);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 106569);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 121622);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130850);
									s.is_empty()
								}
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
			super::push(out, 84434);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 106570);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 121623);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 130851);
									s.is_empty()
								}
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
			super::push(out, 84435);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 106571);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 121624);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130852);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136245);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 139401);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n142681(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 106572);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 121625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 84436);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 106573);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 121626);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130853);
									s.is_empty()
								}
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
			super::push(out, 84437);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 106574);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 84438);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 106575);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121628);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 130854);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 136246);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139402);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n142682(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 84439);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 84440);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 106576);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 84441);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 106577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йнский.
pub(crate) fn n57201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56626);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 84442);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онский.
pub(crate) fn n57202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56627);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 84443);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 84444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унский.
pub(crate) fn n57203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56628);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 84445);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 106578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынский.
pub(crate) fn n57204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56629);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 84446);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 106579);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янский.
pub(crate) fn n57205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56630);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 84447);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 84448);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 106580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 84449);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 106581);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 84450);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 106582);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121632);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -белоозерский.
pub(crate) fn n140843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139403);
	s.is_empty()
}

/// Words ending -нь-каширский.
pub(crate) fn n140844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139404);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -подсоветский.
pub(crate) fn n140845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139405);
	s.is_empty()
}

/// The word фингальский.
pub(crate) fn n140846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139406);
	s.is_empty()
}

/// Words ending -ск-уральский.
pub(crate) fn n140847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139407);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141238);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142290);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еруполномоченный.
pub(crate) fn n142690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141244);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 142293);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -родственничек.
pub(crate) fn n142695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141249);
	s.is_empty()
}

/// Words ending -захребетничек.
pub(crate) fn n142696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141250);
	s.is_empty()
}

/// Words ending -пограничничек.
pub(crate) fn n142697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141251);
	s.is_empty()
}

/// Words ending -полустаночек.
pub(crate) fn n140883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139443);
	s.is_empty()
}

/// Words ending -есмысленочек.
pub(crate) fn n140884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139444);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141252);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теночек.
pub(crate) fn n86099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85193);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 107093);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 121939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 107094);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 121940);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 131044);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136343);
									s.is_empty()
								}
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
			super::push(out, 107095);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 121941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 107096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ченочек.
pub(crate) fn n86100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85194);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 107097);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 121942);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 131046);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136344);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 139445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щеночек.
pub(crate) fn n86101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85195);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 107098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иночек.
pub(crate) fn n58188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57601);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 85196);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 85197);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107100);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 121943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оночек.
pub(crate) fn n58189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57602);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 85198);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 107101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 85199);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107102);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 121944);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131047);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136345);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85200);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107103);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 121945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 85201);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 107104);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уночек.
pub(crate) fn n58190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57603);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 85202);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 107105);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыночек.
pub(crate) fn n58191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57604);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85203);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 85204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юночек.
pub(crate) fn n58192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57605);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 85205);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word выпороточек.
pub(crate) fn n140887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139447);
	s.is_empty()
}

/// The word подросточек.
pub(crate) fn n140888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139448);
	s.is_empty()
}

/// Words ending -промежуточек.
pub(crate) fn n140889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139449);
	s.is_empty()
}

/// Words ending -ог-поисковик.
pub(crate) fn n140891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139451);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141253);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142294);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -расноголовик.
pub(crate) fn n140892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139452);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141254);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ик-промысловик.
pub(crate) fn n140893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139453);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141255);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142295);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -д-белобровик.
pub(crate) fn n140894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139454);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 141256);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142296);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142893);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 143216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -естипенсовик.
pub(crate) fn n140895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139455);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 141257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ук-крестовик.
pub(crate) fn n140896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139456);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141258);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риб-трутовик.
pub(crate) fn n140897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139457);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 141259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -снотолковник.
pub(crate) fn n140907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139467);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лословник.
pub(crate) fn n123355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122080);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 131105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -условник.
pub(crate) fn n108485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107345);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 122081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мовник.
pub(crate) fn n58585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57992);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85566);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107346);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 122082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 107347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85567);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 107348);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 122083);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131106);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136377);
											s.is_empty()
										}
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
					super::push(out, 107349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 85568);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 85569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новник.
pub(crate) fn n58586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57993);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85570);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 85571);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107351);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 122084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85572);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107352);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 107353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85573);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107354);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 85574);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107355);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повник.
pub(crate) fn n58587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57994);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 85575);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 107356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровник.
pub(crate) fn n58588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57995);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85577);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 107357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 85578);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107358);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 85579);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 85580);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совник.
pub(crate) fn n58589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57996);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товник.
pub(crate) fn n58590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57997);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 85582);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 85583);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 85584);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 85585);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 107361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховник.
pub(crate) fn n58591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57998);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85586);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107362);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 85587);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 107363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 85588);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107364);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -човник.
pub(crate) fn n58592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57999);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жук-нарывник.
pub(crate) fn n140908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139468);
	s.is_empty()
}

/// Words ending -д-заповедник.
pub(crate) fn n140910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139470);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141269);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142301);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142895);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 143217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -й-заповедник.
pub(crate) fn n140911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139471);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141270);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 142302);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 142896);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 143218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вагон-ледник.
pub(crate) fn n140912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139472);
	s.is_empty()
}

/// Words ending -чередник.
pub(crate) fn n108539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107399);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 122114);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 131120);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -средник.
pub(crate) fn n86537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85626);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 107400);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 107401);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 122115);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 131121);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136385);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 139473);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 141271);
													match s {
														[] => true,
														['и', s @ ..] => super::t53::n143755(s, out),
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
									super::push(out, 136386);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 139474);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 141272);
													match s {
														[] => true,
														['я', s @ ..] => super::t53::n143756(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -седник.
pub(crate) fn n58612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58019);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 85627);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107402);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ог-подводник.
pub(crate) fn n140915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139475);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141273);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142305);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142898);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 143219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ец-подводник.
pub(crate) fn n140916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139476);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 141274);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142306);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142899);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мотоневодник.
pub(crate) fn n140917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139477);
	s.is_empty()
}

/// Words ending -лубоководник.
pub(crate) fn n140918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139478);
	match s {
		[] => true,
		['г', '-', s @ ..] => {
			super::push(out, 141275);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 142307);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142900);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 143221);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 143380);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роводник.
pub(crate) fn n108552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107412);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 122123);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 131127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 136391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 122124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 131128);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 131129);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 136392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 131130);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 136393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 131131);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 136394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 139479);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141276);
											s.is_empty()
										}
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
					super::push(out, 131132);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 136395);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 139480);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141277);
											s.is_empty()
										}
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
					super::push(out, 131133);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 136396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -годник.
pub(crate) fn n58619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58026);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 85639);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 107414);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 107415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85640);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 85641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 85642);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 107417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изодник.
pub(crate) fn n86554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85643);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 107418);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 122127);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 131135);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 136398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кодник.
pub(crate) fn n58621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58028);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 85644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лодник.
pub(crate) fn n58622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58029);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85645);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 85646);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -модник.
pub(crate) fn n58623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58030);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85647);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107421);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122129);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 131136);
									s.is_empty()
								}
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
			super::push(out, 85648);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -родник.
pub(crate) fn n58624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58031);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 85649);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 85650);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 107423);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 122130);
							match s {
								[] => true,
								['д', 'ж', 'е', 'м', '-', s @ ..] => {
									super::push(out, 131137);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136399);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 139481);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n142724(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85651);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107424);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122131);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131138);
									s.is_empty()
								}
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
			super::push(out, 85652);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107425);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122132);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131139);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122133);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131140);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 136401);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139482);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 107426);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122134);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131141);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136402);
											s.is_empty()
										}
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
					super::push(out, 107427);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122135);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 131142);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136403);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122136);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 122137);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131143);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 136404);
											match s {
												[] => true,
												['е', 'м', '-', s @ ..] => {
													super::push(out, 139483);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n142725(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ходник.
pub(crate) fn n58625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58032);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 85654);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107428);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122138);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 107429);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122139);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131145);
									s.is_empty()
								}
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
			super::push(out, 85655);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 107430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 85656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107431);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 107432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 85657);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 85658);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риятие-смежник.
pub(crate) fn n143763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142311);
	s.is_empty()
}

/// Words ending -читель-смежник.
pub(crate) fn n143764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142312);
	s.is_empty()
}

/// Words ending -раф-художник.
pub(crate) fn n140929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139489);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 141283);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142313);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нер-дорожник.
pub(crate) fn n140930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139490);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141284);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 142314);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 142905);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езнодорожник.
pub(crate) fn n140931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139491);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -автодорожник.
pub(crate) fn n140932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139492);
	s.is_empty()
}

/// Words ending -рана-союзник.
pub(crate) fn n140933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139493);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141286);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сайт-двойник.
pub(crate) fn n140935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139495);
	s.is_empty()
}

/// Words ending -тр-отстойник.
pub(crate) fn n140936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139496);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 141288);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142317);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142906);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 143226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бессимптомник.
pub(crate) fn n142735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141289);
	s.is_empty()
}

/// Words ending -ник-разумник.
pub(crate) fn n140938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139498);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 141290);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142318);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нета-странник.
pub(crate) fn n142738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141292);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -качественник.
pub(crate) fn n140942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139502);
	s.is_empty()
}

/// Words ending -течественник.
pub(crate) fn n140943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139503);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дшественник.
pub(crate) fn n137853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136441);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139504);
			match s {
				[] => true,
				['р', 'п', '-', s @ ..] => {
					super::push(out, 141294);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142321);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 142908);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143227);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143382);
													match s {
														[] => true,
														['л', s @ ..] => super::t53::n144906(s, out),
														_ => false,
													}
												}
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
							super::push(out, 142322);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142909);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 143228);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143383);
													match s {
														[] => true,
														['б', s @ ..] => super::t53::n144907(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тешественник.
pub(crate) fn n140945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139505);
	s.is_empty()
}

/// Words ending -мственник.
pub(crate) fn n123533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122256);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 131212);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 136444);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 139506);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -денник.
pub(crate) fn n58805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58201);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 85896);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 85897);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107627);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 85898);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107628);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 85899);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 107629);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122259);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 131213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женник.
pub(crate) fn n58806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58202);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85900);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107630);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зенник.
pub(crate) fn n58807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58203);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85901);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85902);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 107632);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ленник.
pub(crate) fn n58808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58204);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 85903);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 85904);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107633);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122260);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131214);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136445);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85905);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 85906);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107635);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122261);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 131215);
									s.is_empty()
								}
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
			super::push(out, 85907);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85908);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107637);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122262);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 122263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 85909);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107638);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122264);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131216);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 136446);
											s.is_empty()
										}
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
					super::push(out, 107639);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122265);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 131217);
									s.is_empty()
								}
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
			super::push(out, 85910);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 107640);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 122266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 107641);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 122267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 85911);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 107642);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 122268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -менник.
pub(crate) fn n58809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58205);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85912);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107643);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 85913);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107644);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122270);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131219);
									s.is_empty()
								}
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
			super::push(out, 85914);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107645);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122271);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131220);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 131221);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136447);
											s.is_empty()
										}
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
					super::push(out, 107646);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122272);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131222);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 131223);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136448);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 139507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 131224);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136449);
											s.is_empty()
										}
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
					super::push(out, 107647);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122273);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131225);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136450);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 122274);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131226);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136451);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85915);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107648);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122275);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 122276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 107649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85916);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107650);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85917);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107651);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122278);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131228);
									s.is_empty()
								}
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
			super::push(out, 85918);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107652);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122279);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131229);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 136452);
											s.is_empty()
										}
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
					super::push(out, 107653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 85919);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 107654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122280);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 122281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 85920);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 107655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 85921);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107656);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122282);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненник.
pub(crate) fn n58810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58206);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 85922);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 107657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оенник.
pub(crate) fn n58811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58207);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 85923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 107658);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенник.
pub(crate) fn n58812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58208);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 85924);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 85925);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107659);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122283);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 131231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 107660);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 85926);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 107661);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122285);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131232);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136453);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139508);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренник.
pub(crate) fn n58813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58209);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85927);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107662);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122286);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131233);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136454);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85928);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107663);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 85929);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107664);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 122288);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131234);
									s.is_empty()
								}
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
			super::push(out, 85930);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107665);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 122289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 107666);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 122290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85931);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107667);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122291);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131235);
									s.is_empty()
								}
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
			super::push(out, 85932);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122292);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 122293);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 131236);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136455);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139509);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85933);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107669);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 107670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенник.
pub(crate) fn n58814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58210);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 85934);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 85935);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 85936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 107673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85937);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 85938);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 85939);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107676);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122297);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 131237);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136456);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139510);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенник.
pub(crate) fn n58815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58211);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 85940);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107677);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122298);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131238);
									s.is_empty()
								}
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
			super::push(out, 85941);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107678);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85942);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 85943);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 107680);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122300);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 131239);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136457);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ценник.
pub(crate) fn n58816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58212);
	s.is_empty()
}

/// Words ending -ченник.
pub(crate) fn n58817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58213);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 85944);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенник.
pub(crate) fn n58818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58214);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 85945);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107682);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122301);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131240);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136458);
											s.is_empty()
										}
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
					super::push(out, 107683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85946);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 85947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107685);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенник.
pub(crate) fn n58819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58215);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85948);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107686);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 85949);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 107687);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122304);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131241);
									s.is_empty()
								}
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
			super::push(out, 85950);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107688);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ршинник.
pub(crate) fn n86895(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 107712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 122320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 107713);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 122321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щинник.
pub(crate) fn n58835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58231);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 85981);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 85982);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107716);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122323);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131249);
									s.is_empty()
								}
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
			super::push(out, 85983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107717);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 122324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 107718);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122325);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131251);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136460);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -д-миллионник.
pub(crate) fn n140951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139511);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141296);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142323);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронник.
pub(crate) fn n58844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58240);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86003);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122341);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 131259);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136463);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 86004);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 86005);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122342);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 122343);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 131260);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136464);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 122344);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131261);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 122345);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131262);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136466);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 139512);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n142743(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107740);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 86007);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 107741);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьонник.
pub(crate) fn n58848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58244);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 86010);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 107744);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырехъядерник.
pub(crate) fn n140955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139515);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141298);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142325);
					match s {
						[] => true,
						['ч', '\u{1}', s @ ..] => {
							super::push(out, 142912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word подзаборник.
pub(crate) fn n140956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139516);
	s.is_empty()
}

/// Words ending -ронавирусник.
pub(crate) fn n140957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139517);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рин-советник.
pub(crate) fn n140959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139519);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 141300);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142326);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ассетник.
pub(crate) fn n109105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107956);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 122468);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тетник.
pub(crate) fn n59028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58417);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 86242);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цетник.
pub(crate) fn n59029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58418);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 86243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -четник.
pub(crate) fn n59030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58419);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86244);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 86245);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107959);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шетник.
pub(crate) fn n59031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58420);
	match s {
		[] => true,
		['е', 'р', 'д', s @ ..] => {
			super::push(out, 86246);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 86247);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щий-контрактник.
pub(crate) fn n140960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139520);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141301);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 142327);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытнохоботник.
pub(crate) fn n140961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139521);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141302);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чела-плотник.
pub(crate) fn n140962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139522);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мель-плотник.
pub(crate) fn n140963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139523);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 141304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -егемотник.
pub(crate) fn n123765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122488);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 131320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -смотник.
pub(crate) fn n87200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86279);
	s.is_empty()
}

/// Words ending -хмотник.
pub(crate) fn n87201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86280);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 107989);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -потник.
pub(crate) fn n59053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58441);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 86281);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 86282);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86283);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 107991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротник.
pub(crate) fn n59054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58442);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 86284);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122490);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 122491);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131321);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 122492);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131322);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136489);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122493);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 107993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122494);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сотник.
pub(crate) fn n59055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58443);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 86285);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107994);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 122495);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131325);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 136490);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139524);
													match s {
														[] => true,
														['ж', s @ ..] => super::t53::n142751(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тотник.
pub(crate) fn n59056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58444);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 86286);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107995);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 122496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотник.
pub(crate) fn n59057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58445);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 86287);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107996);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86288);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 107997);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122498);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 131326);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136491);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139525);
													s.is_empty()
												}
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
							super::push(out, 122499);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131327);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136492);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139526);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 107998);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 107999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ист-смертник.
pub(crate) fn n140967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139527);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141306);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142330);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иль-смертник.
pub(crate) fn n140968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139528);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 141307);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142331);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 142916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тво-участник.
pub(crate) fn n140969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139529);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141308);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рд-наместник.
pub(crate) fn n140970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139530);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141309);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хвостник.
pub(crate) fn n109170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108021);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 122519);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 131339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 122520);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 131340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 122521);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 131341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 122522);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 131342);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 136501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пакостник.
pub(crate) fn n123800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122523);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 131343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ностник.
pub(crate) fn n87229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86308);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 108023);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 122524);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 131344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -постник.
pub(crate) fn n87230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86309);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 108024);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 122525);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 131345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ростник.
pub(crate) fn n87231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86310);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 108025);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 122526);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 131346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 108026);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рстник.
pub(crate) fn n59065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58453);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 86311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -устник.
pub(crate) fn n59066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58454);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86312);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 108027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 86313);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108028);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 122528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ород-спутник.
pub(crate) fn n140971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139531);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 141310);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еатр-спутник.
pub(crate) fn n140972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139532);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -белокопытник.
pub(crate) fn n140973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139533);
	s.is_empty()
}

/// Words ending -еб-тетеревятник.
pub(crate) fn n140974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139534);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141312);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142334);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еб-перепелятник.
pub(crate) fn n140975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139535);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141313);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142335);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --памятник.
pub(crate) fn n123826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122549);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 131356);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 136509);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 139536);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 141314);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 142336);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 131357);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 136510);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 139537);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 141315);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142337);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 131358);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 136511);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 139538);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 141316);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142338);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 131359);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136512);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 139539);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 141317);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142339);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 142920);
													match s {
														[] => true,
														['н', s @ ..] => super::t53::n144682(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нятник.
pub(crate) fn n59089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58477);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', 'о', '\u{1}', s @ ..] => {
					super::push(out, 108055);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 108056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86344);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пятник.
pub(crate) fn n59090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58478);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 86345);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 86346);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108058);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рятник.
pub(crate) fn n59091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58479);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 86347);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108059);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 122552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86348);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 108060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сятник.
pub(crate) fn n59092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58480);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 86349);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108061);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 86350);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', s @ ..] => {
			super::push(out, 86351);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108063);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86352);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108064);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122556);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', '\u{1}', s @ ..] => {
					super::push(out, 108065);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 108066);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122557);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утятник.
pub(crate) fn n87275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86353);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 108067);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -богородичник.
pub(crate) fn n140983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139543);
	s.is_empty()
}

/// Words ending -ничник.
pub(crate) fn n59130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58518);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86422);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122606);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131388);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136523);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 122607);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131389);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136524);
											s.is_empty()
										}
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
					super::push(out, 108140);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 86423);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108141);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86424);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 108142);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122610);
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
					super::push(out, 108143);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 108144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 86425);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108145);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122612);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131390);
									s.is_empty()
								}
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
			super::push(out, 86426);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108146);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122613);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131391);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136525);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 139544);
													match s {
														[] => true,
														['л', s @ ..] => super::t53::n142765(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86427);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108147);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 122614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86428);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108148);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 86429);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108149);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122616);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131392);
									s.is_empty()
								}
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
			super::push(out, 86430);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 108150);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 86431);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108151);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 122618);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 108152);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122619);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчичник.
pub(crate) fn n87363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86440);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 108160);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 122621);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щичник.
pub(crate) fn n59135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58523);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 86441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стролодочник.
pub(crate) fn n140985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139545);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лочник.
pub(crate) fn n59144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58532);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86456);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108183);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122638);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131404);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136530);
											s.is_empty()
										}
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
					super::push(out, 108184);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 108185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 86457);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 108186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86458);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108187);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 108188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 108189);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122640);
							match s {
								[] => true,
								['с', 'х', s @ ..] => {
									super::push(out, 131405);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136531);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139546);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n142767(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86459);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 86460);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 108191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86461);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 108193);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122642);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131406);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136532);
											s.is_empty()
										}
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
					super::push(out, 108194);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122643);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131407);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136533);
											s.is_empty()
										}
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
					super::push(out, 108195);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122644);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131408);
									s.is_empty()
								}
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
			super::push(out, 86462);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108196);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 108197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 86463);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 108198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочник.
pub(crate) fn n59145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58533);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86464);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 108199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86465);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 86466);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108201);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122648);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131409);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136534);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 86467);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 108202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 86468);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

