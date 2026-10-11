use flexion::Path;

/// Words ending -тство.
pub(crate) fn n35614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35273);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63357);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 63358);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90216);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 90217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 63359);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 63360);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 90218);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 110766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 63361);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90219);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 110767);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124165);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132215);
											s.is_empty()
										}
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
					super::push(out, 90220);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 110768);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124166);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 124167);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132216);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136976);
													s.is_empty()
												}
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
							super::push(out, 110769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 63362);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 90221);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110770);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124168);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 132217);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 136977);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141259(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90222);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 90223);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110771);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124169);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132218);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136978);
													match s {
														[] => true,
														['д', s @ ..] => super::t51::n141260(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90224);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110772);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124170);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132219);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136979);
													match s {
														[] => true,
														['д', s @ ..] => super::t51::n141261(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63363);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 63364);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90225);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 110773);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124171);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132220);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136980);
													match s {
														[] => true,
														['к', s @ ..] => super::t51::n141262(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63365);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 90226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110774);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124172);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132221);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136981);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уство.
pub(crate) fn n35615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35274);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 63366);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 90227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фство.
pub(crate) fn n35616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35275);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63367);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 90228);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 110775);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 124173);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 132222);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136982);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n141263(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63368);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 90229);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьство.
pub(crate) fn n35617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35276);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 63369);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90230);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 110776);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 124174);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132223);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 136983);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 132224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 110777);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124175);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132225);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 110778);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 110779);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 110780);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 110781);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 110782);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124177);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132226);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 110783);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124178);
									match s {
										[] => true,
										['а', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136984);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 136985);
													match s {
														[] => true,
														['ь', s @ ..] => super::t51::n141264(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90231);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110784);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124179);
									match s {
										[] => true,
										['к', s @ ..] => super::t51::n133589(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => super::t51::n111962(s, out),
						['т', s @ ..] => super::t51::n111963(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t51::n91200(s, out),
				['о', s @ ..] => super::t51::n91201(s, out),
				['у', s @ ..] => super::t51::n91202(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яство.
pub(crate) fn n35618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35277);
	s.is_empty()
}

/// Words ending -паровозишко.
pub(crate) fn n138422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137006);
	s.is_empty()
}

/// Words ending -ялишко.
pub(crate) fn n64296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63634);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 90419);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 110895);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мишко.
pub(crate) fn n35929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35588);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 63635);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63636);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 90421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 63637);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 90422);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110896);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124232);
									s.is_empty()
								}
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
			super::push(out, 63638);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90423);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110897);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124233);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нишко.
pub(crate) fn n35930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35589);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63639);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 90424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 63640);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 90425);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124234);
									s.is_empty()
								}
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
			super::push(out, 63641);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 90426);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124235);
									s.is_empty()
								}
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
			super::push(out, 63642);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 90427);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110900);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 124236);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 132266);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137007);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63643);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90428);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 110901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63644);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 90429);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110902);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 124237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пишко.
pub(crate) fn n35931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35590);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 63645);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 90430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ришко.
pub(crate) fn n35932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35591);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63646);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90431);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110903);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 110904);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124239);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132267);
											s.is_empty()
										}
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
					super::push(out, 90432);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110905);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 124240);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132268);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 137008);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63647);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90433);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110906);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124241);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132269);
											s.is_empty()
										}
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
					super::push(out, 90434);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 63648);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 90435);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110908);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124242);
									s.is_empty()
								}
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
			super::push(out, 63649);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90436);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 110909);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124243);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132270);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 137009);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63650);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90437);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110910);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 124244);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132271);
											s.is_empty()
										}
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
					super::push(out, 90438);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 90439);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110912);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 90440);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 110913);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 110914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сишко.
pub(crate) fn n35933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35592);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 63651);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 63652);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90442);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 90443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63653);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 90444);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110915);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124246);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 132272);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тишко.
pub(crate) fn n35934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35593);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63654);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90445);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 110916);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124247);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132273);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 137010);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63655);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90446);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 63656);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 90447);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110918);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124248);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 132274);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137011);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90448);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110919);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124249);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132275);
											s.is_empty()
										}
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
					super::push(out, 90449);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110920);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124250);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132276);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 137012);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63658);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 90450);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110921);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 124251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 90451);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110922);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 124252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132277);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 110923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 90452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 63659);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90453);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 110924);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124253);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132278);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137013);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63660);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90454);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 110925);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 124254);
									s.is_empty()
								}
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
			super::push(out, 63661);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90455);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110926);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уишко.
pub(crate) fn n35935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35594);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 63662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чишко.
pub(crate) fn n35936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35595);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63663);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 90456);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 90457);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 110928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 63664);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 90458);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110929);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124256);
									s.is_empty()
								}
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
			super::push(out, 63665);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 90459);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110930);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124257);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132279);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 137014);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141277(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90460);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124258);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132280);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 63666);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 63667);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90462);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 110932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 63668);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110933);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 124259);
									s.is_empty()
								}
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
			super::push(out, 63669);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 90464);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 110934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124260);
									s.is_empty()
								}
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
			super::push(out, 63670);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90465);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 90466);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 63671);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 90467);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щишко.
pub(crate) fn n35937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35596);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63672);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90468);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 110937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьишко.
pub(crate) fn n35938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35597);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 63673);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90469);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110938);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124261);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 132281);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 137015);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63674);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 90470);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 63675);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90471);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 110940);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124262);
									s.is_empty()
								}
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
			super::push(out, 63676);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90472);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110941);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124263);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132282);
											s.is_empty()
										}
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
					super::push(out, 90473);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110942);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124264);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132283);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 63677);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90474);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110943);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 90475);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 110944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 63678);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 90476);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ундер.
pub(crate) fn n37144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36799);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 64710);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91161);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 111320);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124424);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 132346);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 137041);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 91162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 64711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 64712);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 91164);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 111322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 64713);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 91165);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трансбордер.
pub(crate) fn n138458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137042);
	s.is_empty()
}

/// The word газгольдер.
pub(crate) fn n138459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137043);
	s.is_empty()
}

/// Words ending -таск-менеджер.
pub(crate) fn n141291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139850);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141497);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тайм-менеджер.
pub(crate) fn n141292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139851);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141498);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word кеш-менеджер.
pub(crate) fn n141293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139852);
	s.is_empty()
}

/// Words ending -радиомаркер.
pub(crate) fn n138468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137052);
	s.is_empty()
}

/// Words ending -ран-штабелер.
pub(crate) fn n141296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139855);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142456);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word банк-дилер.
pub(crate) fn n138472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137056);
	s.is_empty()
}

/// Words ending -амбервиллер.
pub(crate) fn n138473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137057);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етка-киллер.
pub(crate) fn n138474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137058);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139857);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суперкиллер.
pub(crate) fn n138475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137059);
	s.is_empty()
}

/// Words ending -ок-н-роллер.
pub(crate) fn n138476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137060);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139858);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -буллер.
pub(crate) fn n65626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64956);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 91350);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 91351);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 111442);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 124499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-контролер.
pub(crate) fn n138477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137061);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139859);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141502);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142458);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142981);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 143264);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-гастролер.
pub(crate) fn n138478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137062);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139860);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 141503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -четчик-таймер.
pub(crate) fn n142950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141504);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дада-высотомер.
pub(crate) fn n138480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137064);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139862);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141505);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -е-трансформер.
pub(crate) fn n138481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137065);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 139863);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141506);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вник-инженер.
pub(crate) fn n141305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139864);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141507);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142463);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирал-инженер.
pub(crate) fn n141306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139865);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 141508);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 142464);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ик-дизайнер.
pub(crate) fn n138486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137070);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139866);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 141509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 139867);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141510);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word мейнтейнер.
pub(crate) fn n138487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137071);
	s.is_empty()
}

/// Words ending -гада-пионер.
pub(crate) fn n138488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137072);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139868);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141511);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кондиционер.
pub(crate) fn n138489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137073);
	s.is_empty()
}

/// Words ending -р-селекционер.
pub(crate) fn n138490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137074);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139869);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 141512);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142465);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142985);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ст-революционер.
pub(crate) fn n138491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137075);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139870);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142466);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 142986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коммонер.
pub(crate) fn n112705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111523);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 124552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ронер.
pub(crate) fn n37425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37075);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 65095);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 91457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонер.
pub(crate) fn n37426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37076);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 65096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -д-девелопер.
pub(crate) fn n138494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137078);
	s.is_empty()
}

/// Words ending -агон-хоппер.
pub(crate) fn n138495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137079);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трейнхоппер.
pub(crate) fn n138496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137080);
	s.is_empty()
}

/// Words ending -рейк-дансер.
pub(crate) fn n138499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137083);
	s.is_empty()
}

/// Words ending -онкордансер.
pub(crate) fn n138500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137084);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 139872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -россдрессер.
pub(crate) fn n138501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137085);
	s.is_empty()
}

/// Words ending -нк-андеррайтер.
pub(crate) fn n138502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137086);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139873);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 141514);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стритфайтер.
pub(crate) fn n138503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137087);
	s.is_empty()
}

/// Words ending -еликобактер.
pub(crate) fn n138504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137088);
	s.is_empty()
}

/// Words ending -ацетобактер.
pub(crate) fn n138505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137089);
	s.is_empty()
}

/// Words ending -опикстартер.
pub(crate) fn n138506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137090);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139874);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -автостартер.
pub(crate) fn n138507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137091);
	s.is_empty()
}

/// Words ending -р-конвертер.
pub(crate) fn n138508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137092);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139875);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141515);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142467);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ес-репортер.
pub(crate) fn n138509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137093);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139876);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 141516);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142468);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 142988);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 143266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -импортер.
pub(crate) fn n112827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111644);
	s.is_empty()
}

/// Words ending -ппортер.
pub(crate) fn n92629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91656);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 111645);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 124617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 132456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спортер.
pub(crate) fn n92630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91657);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 111646);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 124618);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 132457);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137094);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 139877);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141517);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n143921(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 111647);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124619);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132458);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сортер.
pub(crate) fn n66007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65333);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 91658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ранд-мастер.
pub(crate) fn n138512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137096);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 139878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ер-ломастер.
pub(crate) fn n138513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137097);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139879);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141518);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142470);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 142990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -томастер.
pub(crate) fn n112842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111659);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 124631);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 132465);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмастер.
pub(crate) fn n92644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91671);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 111660);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 124632);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -настер.
pub(crate) fn n66015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65341);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 91672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -растер.
pub(crate) fn n66016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65342);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 91673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 111661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фастер.
pub(crate) fn n66017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65343);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 91674);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 111662);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124633);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 132466);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 137098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бстер.
pub(crate) fn n37587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37236);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 65344);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гстер.
pub(crate) fn n37588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37237);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65345);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91676);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 111663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 65346);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91677);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 111664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 65347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 91678);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 111665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дстер.
pub(crate) fn n37589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37238);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 65348);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 91679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 65349);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91680);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -естер.
pub(crate) fn n37590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37239);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 65350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 65351);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91682);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111666);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 124634);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132467);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65352);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 65353);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 91683);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 91684);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111668);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 91685);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 65354);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 91686);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -истер.
pub(crate) fn n37591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37240);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 65355);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 91687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 65356);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 65357);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 91688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 91689);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 91690);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 111672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 65358);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 91691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 65359);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91692);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111673);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 124636);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132468);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91693);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йстер.
pub(crate) fn n37592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37241);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 65361);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91694);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 91695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кстер.
pub(crate) fn n37593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37242);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 65362);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 65363);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 91696);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мстер.
pub(crate) fn n37594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нстер.
pub(crate) fn n37595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37244);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65365);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91697);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111676);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 124638);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132469);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65366);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 91698);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111677);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124639);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 132470);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остер.
pub(crate) fn n37596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37245);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 65367);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 91699);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 111678);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124640);
									s.is_empty()
								}
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
			super::push(out, 65368);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 91700);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 91701);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111679);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124641);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132471);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137099);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 65369);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 65370);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 65371);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91702);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пстер.
pub(crate) fn n37597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37246);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 65372);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 91703);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рстер.
pub(crate) fn n37598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37247);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65373);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 91704);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устер.
pub(crate) fn n37599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37248);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65374);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 91705);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111680);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124642);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 132472);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьстер.
pub(crate) fn n37600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37249);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 65376);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 91706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эстер.
pub(crate) fn n37601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37250);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 65377);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 65378);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91707);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 111681);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юстер.
pub(crate) fn n37602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37251);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 65379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 91708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -термокаутер.
pub(crate) fn n138516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137100);
	s.is_empty()
}

/// Words ending -истрибьютер.
pub(crate) fn n138517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137101);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 139880);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ернет-серфер.
pub(crate) fn n141322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139881);
	match s {
		[] => true,
		['т', 'н', 'и', '\u{1}', s @ ..] => {
			super::push(out, 141519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ребля-индор.
pub(crate) fn n138532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137116);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 139888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ран-экскаватор.
pub(crate) fn n138539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137123);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 139893);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141523);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ер-элеватор.
pub(crate) fn n138540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137124);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 139894);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 141524);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142473);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word рекультиватор.
pub(crate) fn n138541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137125);
	s.is_empty()
}

/// Words ending -консерватор.
pub(crate) fn n138542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137126);
	s.is_empty()
}

/// Words ending -лигатор.
pub(crate) fn n93003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92025);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 111861);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124724);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мигатор.
pub(crate) fn n93004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92026);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 111862);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 124725);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тигатор.
pub(crate) fn n93005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92027);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 111863);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 124726);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 132517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгатор.
pub(crate) fn n66534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65856);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92028);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 111864);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огатор.
pub(crate) fn n66535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65857);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 92029);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 111865);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьгатор.
pub(crate) fn n66536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65858);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 92030);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 111866);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 124729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -датор.
pub(crate) fn n38096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37739);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 92031);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'а', s @ ..] => {
					super::push(out, 92032);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 111868);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124730);
									s.is_empty()
								}
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
			super::push(out, 65859);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92033);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еатор.
pub(crate) fn n38097(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 65860);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 65861);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 92035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -затор.
pub(crate) fn n38098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37740);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 65862);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 65863);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 92036);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111870);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 124731);
									s.is_empty()
								}
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
			super::push(out, 65864);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 92037);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 111871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 65865);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 92038);
					match s {
						[] => true,
						['и', 'т', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132518);
											s.is_empty()
										}
										['е', s @ ..] => super::t51::n133881(s, out),
										_ => false,
									}
								}
								['я', s @ ..] => super::t51::n126020(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t51::n113057(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t51::n93017(s, out),
				['л', s @ ..] => super::t51::n93018(s, out),
				['м', s @ ..] => super::t51::n93019(s, out),
				['н', s @ ..] => super::t51::n93020(s, out),
				['р', s @ ..] => super::t51::n93021(s, out),
				['т', s @ ..] => super::t51::n93022(s, out),
				['х', s @ ..] => super::t51::n93023(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 65866);
			s.is_empty()
		}
		['о', s @ ..] => super::t51::n66546(s, out),
		['у', s @ ..] => super::t51::n66547(s, out),
		_ => false,
	}
}

/// Words ending -иатор.
pub(crate) fn n38099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37741);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 65869);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 65870);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92046);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111888);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124776);
									s.is_empty()
								}
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
			super::push(out, 65871);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111889);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 124777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 111890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'м', 'о', s @ ..] => {
					super::push(out, 92047);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 65872);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 92048);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 65873);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92049);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 111893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 92050);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 111894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 65874);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 92051);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 65875);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92053);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 111896);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 92054);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111897);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 92055);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111898);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катор.
pub(crate) fn n38100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37742);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 65876);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 92056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 65877);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 92057);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111899);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124781);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132568);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 111900);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124782);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 132569);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 137167);
													match s {
														[] => true,
														['и', s @ ..] => super::t51::n141356(s, out),
														_ => false,
													}
												}
												['ь', s @ ..] => {
													super::push(out, 137168);
													match s {
														[] => true,
														['л', s @ ..] => super::t51::n141357(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 132570);
											s.is_empty()
										}
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
					super::push(out, 92058);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 111901);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124783);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 132571);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t51::n113095(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 92059);
					s.is_empty()
				}
				['н', s @ ..] => super::t51::n93041(s, out),
				['р', 'б', s @ ..] => super::t51::n93042(s, out),
				['с', s @ ..] => {
					super::push(out, 92060);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 92061);
					s.is_empty()
				}
				['ф', s @ ..] => super::t51::n93045(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t51::n66559(s, out),
		_ => false,
	}
}

/// Words ending -латор.
pub(crate) fn n38101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37743);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65879);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 92065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матор.
pub(crate) fn n38102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37744);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65880);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 92066);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 111912);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124801);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132590);
											s.is_empty()
										}
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
					super::push(out, 92067);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111913);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124802);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 132591);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 137187);
													match s {
														[] => true,
														['ц', s @ ..] => super::t51::n141369(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 65881);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92068);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111914);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 124803);
									s.is_empty()
								}
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
			super::push(out, 65882);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92069);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111915);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124804);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132592);
											s.is_empty()
										}
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
					super::push(out, 92070);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111917);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124805);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 132593);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 111918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 65883);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92071);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111919);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 124806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 92072);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 65884);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 92073);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 111921);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124807);
									s.is_empty()
								}
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
			super::push(out, 65885);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111922);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124808);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 132594);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 137188);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n141370(s, out),
														_ => false,
													}
												}
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
							super::push(out, 111923);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124809);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 132595);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 137189);
													match s {
														[] => true,
														['ь', s @ ..] => super::t51::n141371(s, out),
														_ => false,
													}
												}
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
							super::push(out, 111924);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124810);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132596);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137190);
													match s {
														[] => true,
														['т', s @ ..] => super::t51::n141372(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 65886);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92074);
					match s {
						[] => true,
						['к', s @ ..] => super::t51::n113118(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натор.
pub(crate) fn n38103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37745);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65887);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 65888);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92075);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111926);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124812);
									s.is_empty()
								}
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
			super::push(out, 65889);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 92076);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111927);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124813);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132597);
											s.is_empty()
										}
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
					super::push(out, 92077);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111928);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124814);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132598);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137191);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 92078);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111929);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 124815);
									match s {
										[] => true,
										['к', 'э', '-', s @ ..] => {
											super::push(out, 132599);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 137192);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141373(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 92079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 65890);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 92080);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 92081);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 92082);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111930);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124816);
									match s {
										[] => true,
										['о', 'к', '-', s @ ..] => {
											super::push(out, 132600);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137193);
													match s {
														[] => true,
														['т', s @ ..] => super::t51::n141374(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 92083);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124817);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 124818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 92084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111932);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 111933);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124820);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132601);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 137194);
													s.is_empty()
												}
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
							super::push(out, 111934);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 124821);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 124822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 111935);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124823);
									match s {
										[] => true,
										['т', s @ ..] => super::t51::n133969(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', 'л', 'л', 'и', '\u{1}', s @ ..] => {
							super::push(out, 111936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t51::n93070(s, out),
				['с', s @ ..] => {
					super::push(out, 92086);
					s.is_empty()
				}
				['т', s @ ..] => super::t51::n93072(s, out),
				['у', s @ ..] => super::t51::n93073(s, out),
				['х', s @ ..] => super::t51::n93074(s, out),
				['ц', 'к', 'а', 'в', s @ ..] => super::t51::n93075(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t51::n66572(s, out),
		['р', s @ ..] => super::t51::n66573(s, out),
		_ => false,
	}
}

/// Words ending -патор.
pub(crate) fn n38104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37746);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 65893);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 92094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92095);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111945);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 92096);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 111946);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратор.
pub(crate) fn n38105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37747);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 65894);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124830);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 132604);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 124831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 111947);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124832);
									s.is_empty()
								}
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
			super::push(out, 65895);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92097);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 111948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 65896);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92098);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111949);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124833);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132605);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65897);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92099);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111950);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124834);
									s.is_empty()
								}
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
			super::push(out, 65898);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 65899);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 92100);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 92101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 111952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 92102);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111953);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124835);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 132606);
											match s {
												[] => true,
												['е', 'р', s @ ..] => super::t51::n138617(s, out),
												_ => false,
											}
										}
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
					super::push(out, 92103);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111954);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124836);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132607);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t51::n93091(s, out),
				['н', s @ ..] => super::t51::n93092(s, out),
				['п', s @ ..] => super::t51::n93093(s, out),
				['т', s @ ..] => super::t51::n93094(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t51::n66582(s, out),
		['о', s @ ..] => super::t51::n66583(s, out),
		['р', s @ ..] => super::t51::n66584(s, out),
		['т', s @ ..] => super::t51::n66585(s, out),
		['у', s @ ..] => super::t51::n66586(s, out),
		['ф', s @ ..] => super::t51::n66587(s, out),
		['э', s @ ..] => super::t51::n66588(s, out),
		['ю', s @ ..] => super::t51::n66589(s, out),
		_ => false,
	}
}

/// Words ending -сатор.
pub(crate) fn n38106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37748);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92129);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 92130);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 111985);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124864);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 132629);
											s.is_empty()
										}
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
					super::push(out, 92131);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 111986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 65907);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 65908);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92133);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111987);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'д', s @ ..] => {
					super::push(out, 92134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 65909);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92135);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 111988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -татор.
pub(crate) fn n38107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37749);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92136);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111989);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 124866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 92137);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111990);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124867);
									s.is_empty()
								}
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
			super::push(out, 65910);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 92138);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111991);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 111992);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 92139);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 92140);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 92141);
					s.is_empty()
				}
				['м', 'и', 'о', s @ ..] => {
					super::push(out, 92142);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111993);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124870);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132630);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 137209);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 92143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 65911);
			match s {
				[] => true,
				['е', 'с', 'с', s @ ..] => {
					super::push(out, 92144);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111994);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 92145);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 111995);
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
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111996);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124872);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 132631);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 111997);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124873);
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
						['и', 'р', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124874);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 124875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 111998);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124876);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132632);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 124877);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132633);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137210);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 124878);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132634);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 137211);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 124879);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132635);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137212);
													match s {
														[] => true,
														['д', s @ ..] => super::t51::n141387(s, out),
														_ => false,
													}
												}
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
							super::push(out, 111999);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124880);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132636);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['н', s @ ..] => {
					super::push(out, 92146);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 92147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 65912);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 65913);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 92148);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 112001);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124881);
									s.is_empty()
								}
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
				['\u{1}', s @ ..] => {
					super::push(out, 92149);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 92150);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 112002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 92151);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 112003);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124882);
									s.is_empty()
								}
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
			super::push(out, 65914);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 92152);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 112004);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124883);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 132637);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уатор.
pub(crate) fn n38108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37750);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 65915);
			match s {
				[] => true,
				['а', 'в', 'э', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 112005);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 124884);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 132638);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 137213);
													match s {
														[] => true,
														['б', s @ ..] => super::t51::n141388(s, out),
														_ => false,
													}
												}
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
							super::push(out, 112006);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124885);
									s.is_empty()
								}
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
			super::push(out, 65916);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92153);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112007);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92154);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 92155);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 112008);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 124887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фатор.
pub(crate) fn n38109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37751);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 65917);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92156);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 112009);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шатор.
pub(crate) fn n38110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37752);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 65918);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92157);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 112010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -деоредактор.
pub(crate) fn n138634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137215);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -диоредактор.
pub(crate) fn n138635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137216);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 139950);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зиаттрактор.
pub(crate) fn n138636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137217);
	s.is_empty()
}

/// Words ending -езус-фактор.
pub(crate) fn n138637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137218);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ектор.
pub(crate) fn n38126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37768);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 65946);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 92186);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112031);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124902);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 132651);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 137219);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141393(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 92187);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 92188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 65947);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 92189);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 112032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 92190);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 112033);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124903);
									s.is_empty()
								}
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
			super::push(out, 65948);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 92191);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 112034);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124904);
									s.is_empty()
								}
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
			super::push(out, 65949);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 92192);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 92193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 92194);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 92195);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 92196);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 124905);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 124906);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132652);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137220);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141394(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 92197);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 112037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 65950);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 92198);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112038);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124907);
									s.is_empty()
								}
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
			super::push(out, 65951);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 92199);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 112039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 65952);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 92200);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112040);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124908);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 132653);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 137221);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141395(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['б', 'ы', s @ ..] => {
											super::push(out, 132654);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137222);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141396(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 132655);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 92201);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 92202);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112041);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124909);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 132656);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t51::n93195(s, out),
				['о', s @ ..] => {
					super::push(out, 92204);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 112042);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t51::n93197(s, out),
				['э', s @ ..] => super::t51::n93198(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t51::n66645(s, out),
		['т', s @ ..] => super::t51::n66646(s, out),
		['ф', s @ ..] => super::t51::n66647(s, out),
		['ъ', s @ ..] => {
			super::push(out, 65957);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 92220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нстриктор.
pub(crate) fn n126218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124922);
	s.is_empty()
}

/// Words ending -нктор.
pub(crate) fn n38128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37770);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 65962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -октор.
pub(crate) fn n38129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37771);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 65963);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92222);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 112061);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124923);
									s.is_empty()
								}
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
			super::push(out, 65964);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 92223);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уктор.
pub(crate) fn n38130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37772);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 65965);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 92224);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 112062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 92225);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 112063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 92226);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112064);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132671);
											s.is_empty()
										}
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
					super::push(out, 92227);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 112065);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124925);
									s.is_empty()
								}
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
			super::push(out, 65966);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 92228);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124926);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 132672);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137233);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 124927);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 132673);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 137234);
													match s {
														[] => true,
														['с', s @ ..] => super::t51::n141401(s, out),
														['т', s @ ..] => super::t51::n141402(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 132674);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137235);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эктор.
pub(crate) fn n38131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37773);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 65967);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ассимилятор.
pub(crate) fn n138655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137236);
	s.is_empty()
}

/// Words ending -ра-вентилятор.
pub(crate) fn n138656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137237);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139962);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141570);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 142508);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 143021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ик-ретранслятор.
pub(crate) fn n138657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137238);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139963);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141571);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 142509);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улятор.
pub(crate) fn n66695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66004);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 92256);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 112083);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 132680);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137239);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 139964);
													match s {
														[] => true,
														['л', s @ ..] => super::t51::n143018(s, out),
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
									super::push(out, 132681);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 137240);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 139965);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n143019(s, out),
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
									super::push(out, 132682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 112084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 92257);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 112085);
					match s {
						[] => true,
						['м', 'е', s @ ..] => {
							super::push(out, 124941);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132683);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137241);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139966);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n143020(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 92258);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 112086);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124942);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 132684);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137242);
											s.is_empty()
										}
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
					super::push(out, 112087);
					match s {
						[] => true,
						['л', 'а', 'к', '-', s @ ..] => {
							super::push(out, 124943);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132685);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137243);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 139967);
													match s {
														[] => true,
														['л', s @ ..] => super::t51::n143021(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 92259);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 112088);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 112089);
					match s {
						[] => true,
						['к', 'к', 'а', 'о', 'р', s @ ..] => {
							super::push(out, 124944);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132686);
									match s {
										[] => true,
										['и', 'г', '\u{1}', s @ ..] => {
											super::push(out, 137244);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 92260);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 112090);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 124945);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137245);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 137246);
											match s {
												[] => true,
												['а', s @ ..] => super::t51::n141409(s, out),
												['т', s @ ..] => super::t51::n141410(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вискозиметр.
pub(crate) fn n138668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137249);
	s.is_empty()
}

/// Words ending -ниметр.
pub(crate) fn n66743(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'л', 'п', 'о', s @ ..] => {
			super::push(out, 92296);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 112116);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 124962);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 132693);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 137250);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 92297);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 112117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риметр.
pub(crate) fn n66744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66051);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92298);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 92299);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 92300);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 112118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 92301);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 112119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -симетр.
pub(crate) fn n66745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66052);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 92302);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 92303);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 92304);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 112120);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124963);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 132694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиметр.
pub(crate) fn n66746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66053);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92305);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 112121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 124964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 92306);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 112122);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124965);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 132695);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137251);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 92307);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 112123);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 124966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 92308);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 112124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 124967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 92309);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 112125);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лметр.
pub(crate) fn n38208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37847);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 66055);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92311);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 112127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мметр.
pub(crate) fn n38209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37848);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 66056);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 66057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ометр.
pub(crate) fn n38210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37849);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 66058);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 92312);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 112128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 66059);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 66060);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 66061);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 92314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 92315);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 112131);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124969);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132696);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137252);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'и', s @ ..] => {
			super::push(out, 66062);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 92316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 112132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124970);
									s.is_empty()
								}
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
			super::push(out, 66063);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 92317);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 92318);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 112133);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124971);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 132697);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 137253);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141413(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 92319);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 112134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 66064);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 92320);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 112135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 92321);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112136);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124973);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132698);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137254);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 66065);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 92322);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 112137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 92323);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 112138);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 124974);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 124975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 112139);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 112140);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124976);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 112141);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 112142);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124978);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132699);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137255);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 66066);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 92324);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 112143);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124979);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 132700);
											s.is_empty()
										}
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
					super::push(out, 92325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 66067);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['м', 'о', 'м', s @ ..] => {
							super::push(out, 112144);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 124980);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 132701);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137256);
													match s {
														[] => true,
														['т', s @ ..] => super::t51::n141414(s, out),
														_ => false,
													}
												}
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
							super::push(out, 112145);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 92326);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 92327);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112146);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124982);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132702);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					super::push(out, 92328);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 112147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t51::n66761(s, out),
		['с', s @ ..] => super::t51::n66762(s, out),
		['т', s @ ..] => super::t51::n66763(s, out),
		['х', s @ ..] => super::t51::n66764(s, out),
		_ => false,
	}
}

/// Words ending -рметр.
pub(crate) fn n38211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37850);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 66072);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 92340);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 112156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сметр.
pub(crate) fn n38212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37851);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 66073);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 92341);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 112157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 66074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92342);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 112158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тметр.
pub(crate) fn n38213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37852);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 66075);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92343);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 112159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 66076);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92344);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112160);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132708);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 132709);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уметр.
pub(crate) fn n38214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37853);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 66077);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 92345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кула-пилонос.
pub(crate) fn n141429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139988);
	s.is_empty()
}

/// Words ending -вод-автомат.
pub(crate) fn n138736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137317);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140002);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 141592);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тие-автомат.
pub(crate) fn n138737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137318);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 140003);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 141593);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142521);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нок-автомат.
pub(crate) fn n138738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137319);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140004);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141594);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оз-комбинат.
pub(crate) fn n138740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137321);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 140006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ола-интернат.
pub(crate) fn n138741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137322);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140007);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 141596);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дом-интернат.
pub(crate) fn n138742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137323);
	s.is_empty()
}

/// Words ending -вец-кастрат.
pub(crate) fn n138743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137324);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140008);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вер-самолет.
pub(crate) fn n138752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137333);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140016);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлессмахерит.
pub(crate) fn n141464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140023);
	s.is_empty()
}

/// Words ending -альк-хлорит.
pub(crate) fn n138770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137351);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140024);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word уолл-стрит.
pub(crate) fn n138771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137352);
	s.is_empty()
}

/// Words ending -орд-лейтенант.
pub(crate) fn n138779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137360);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140029);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141608);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -банк-гарант.
pub(crate) fn n138780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137361);
	s.is_empty()
}

/// Words ending -курант.
pub(crate) fn n68778(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 93664);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 112906);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 125356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 112907);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 125357);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132895);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137362);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -турант.
pub(crate) fn n68779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68069);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 93665);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 112908);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 125358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иконгестант.
pub(crate) fn n138782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137363);
	s.is_empty()
}

/// Words ending -ьтант.
pub(crate) fn n40454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40052);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 68099);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 112928);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125371);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 112929);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 125372);
									match s {
										[] => true,
										['о', 'к', '-', s @ ..] => {
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137364);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141471(s, out),
														['е', s @ ..] => super::t51::n141472(s, out),
														['о', s @ ..] => super::t51::n141473(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 137365);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141474(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 137366);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141475(s, out),
														['е', s @ ..] => super::t51::n141476(s, out),
														_ => false,
													}
												}
												['ц', s @ ..] => {
													super::push(out, 137367);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141477(s, out),
														_ => false,
													}
												}
												['ч', s @ ..] => {
													super::push(out, 137368);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141478(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютант.
pub(crate) fn n40455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40053);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 68100);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 93692);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 112930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 68101);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93693);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112931);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125373);
									s.is_empty()
								}
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
			super::push(out, 68102);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 93694);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 112932);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 125374);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132901);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 137369);
													match s {
														[] => true,
														['и', s @ ..] => super::t51::n141479(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -питан-регент.
pub(crate) fn n141480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140039);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -принц-регент.
pub(crate) fn n141481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140040);
	s.is_empty()
}

/// Words ending -истр-президент.
pub(crate) fn n138792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137373);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 140041);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 141619);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 142536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пересидент.
pub(crate) fn n138793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137374);
	s.is_empty()
}

/// Words ending -к-корреспондент.
pub(crate) fn n134282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132912);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137375);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140042);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-корреспондент.
pub(crate) fn n134283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132913);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137376);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140043);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 141621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -токорреспондент.
pub(crate) fn n134284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132914);
	s.is_empty()
}

/// Words ending -рдент.
pub(crate) fn n40481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40078);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 68136);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 93727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удент.
pub(crate) fn n40482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40079);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 68137);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 93728);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арк-эмбиент.
pub(crate) fn n138796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137377);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 140044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ютер-клиент.
pub(crate) fn n138797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137378);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 140045);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-апартамент.
pub(crate) fn n138798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137379);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 140046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одепартамент.
pub(crate) fn n138799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137380);
	s.is_empty()
}

/// Words ending -сдепартамент.
pub(crate) fn n138800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137381);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140047);
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					super::push(out, 141623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -естамент.
pub(crate) fn n114195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112988);
	s.is_empty()
}

/// Words ending -остамент.
pub(crate) fn n114196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112989);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 125416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гмент.
pub(crate) fn n40499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40096);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68157);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 68158);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 68159);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 93758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дмент.
pub(crate) fn n40500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40097);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 68160);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 93759);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 112990);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емент.
pub(crate) fn n40501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40098);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 68161);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 93760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 93761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112992);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 112993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 68162);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 68163);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 68164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жмент.
pub(crate) fn n40502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40099);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 68165);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 93762);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -змент.
pub(crate) fn n40503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40100);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 68166);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 93763);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 112995);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имент.
pub(crate) fn n40504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40101);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 68167);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 93764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 68168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ймент.
pub(crate) fn n40505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40102);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 68169);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 93765);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 112997);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 125419);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132924);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лмент.
pub(crate) fn n40506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40103);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 68170);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 93766);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 112998);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 125420);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 132925);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ммент.
pub(crate) fn n40507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40104);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 68171);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 93767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 112999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нмент.
pub(crate) fn n40508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40105);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 68172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93768);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 93769);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 68173);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93770);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 113001);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125421);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 132926);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137382);
													match s {
														[] => true,
														['и', s @ ..] => super::t51::n141489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омент.
pub(crate) fn n40509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40106);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 68174);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пмент.
pub(crate) fn n40510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40107);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 68175);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 93771);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113002);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 125422);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132927);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137383);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 68176);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93772);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113003);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125423);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмент.
pub(crate) fn n40511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40108);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 68177);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 93773);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -смент.
pub(crate) fn n40512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40109);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68178);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93774);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113004);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125424);
									s.is_empty()
								}
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
			super::push(out, 68179);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 93775);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 113005);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125425);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 132929);
											s.is_empty()
										}
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
					super::push(out, 93776);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113006);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125426);
									s.is_empty()
								}
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
			super::push(out, 68180);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 93777);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 93778);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113007);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125427);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 132930);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 68181);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 93779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 68182);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 93780);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 68183);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 93781);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113009);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тмент.
pub(crate) fn n40513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68184);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 68185);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 68186);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 68187);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 93782);
					match s {
						[] => true,
						['о', 'п', 'п', 'а', '\u{1}', s @ ..] => {
							super::push(out, 113010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 68188);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 68189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -умент.
pub(crate) fn n40514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68190);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 93783);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113011);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 125429);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132931);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 68191);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 68192);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 93784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 68193);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 93785);
					match s {
						[] => true,
						['с', 'н', 'и', 'о', s @ ..] => {
							super::push(out, 113012);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 125430);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132932);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137384);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141490(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чмент.
pub(crate) fn n40515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40112);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68194);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 93786);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 68195);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 93787);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113014);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шмент.
pub(crate) fn n40516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40113);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 68196);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 93788);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113015);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125432);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132933);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьмент.
pub(crate) fn n40517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40114);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 68197);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 93789);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113016);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125433);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132934);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рь-референт.
pub(crate) fn n138804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140050);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141625);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -банк-эмитент.
pub(crate) fn n138805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137386);
	s.is_empty()
}

/// Words ending -акула-молот.
pub(crate) fn n138811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137392);
	s.is_empty()
}

/// Words ending -изель-молот.
pub(crate) fn n138812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137393);
	s.is_empty()
}

/// Words ending -ист-эксперт.
pub(crate) fn n138813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137394);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141627);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинг-концерт.
pub(crate) fn n138814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137395);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 140055);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 141628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильм-концерт.
pub(crate) fn n138815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137396);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 140056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ольтранспорт.
pub(crate) fn n141498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140057);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 141629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142540);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 143036);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 143291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инц-консорт.
pub(crate) fn n138820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137401);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140058);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141630);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ийца-садист.
pub(crate) fn n138825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137406);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 140062);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 141634);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енер-методист.
pub(crate) fn n138826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137407);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140063);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word фоторезист.
pub(crate) fn n138827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137408);
	s.is_empty()
}

/// Words ending -т-криминалист.
pub(crate) fn n138828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137409);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140064);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141636);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142544);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 143037);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 143292);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -да-финалист.
pub(crate) fn n138829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137410);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 140065);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141637);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 142545);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-интернационалист.
pub(crate) fn n138830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137411);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 140066);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141638);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 142546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word орнаменталист.
pub(crate) fn n138831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137412);
	s.is_empty()
}

/// Words ending -илист.
pub(crate) fn n40893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40484);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 68551);
			match s {
				[] => true,
				['с', '-', s @ ..] => {
					super::push(out, 94056);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113199);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125529);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 132986);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 137413);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141508(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клист.
pub(crate) fn n40894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40485);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 68552);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94057);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 94058);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 68553);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 94059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 68554);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94060);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллист.
pub(crate) fn n40895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40486);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 68555);
			match s {
				[] => true,
				['т', 'е', 'м', '-', s @ ..] => {
					super::push(out, 94061);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 113203);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125531);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 132987);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137414);
													match s {
														[] => true,
														['б', s @ ..] => super::t51::n141509(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олист.
pub(crate) fn n40896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40487);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 68556);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 68557);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 68558);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 68559);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94062);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113204);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125532);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132988);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 68560);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94063);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113205);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125533);
									match s {
										[] => true,
										['м', '-', s @ ..] => {
											super::push(out, 132989);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137415);
													match s {
														[] => true,
														['в', s @ ..] => super::t51::n141510(s, out),
														_ => false,
													}
												}
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
							super::push(out, 113206);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125534);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132990);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137416);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 94064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 68561);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 94065);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113207);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125535);
									s.is_empty()
								}
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
			super::push(out, 68562);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94066);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113208);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лулист.
pub(crate) fn n69284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68563);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 94067);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 113209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нер-программист.
pub(crate) fn n138836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137417);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140070);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 141642);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 142550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь-непрограммист.
pub(crate) fn n138837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137418);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140071);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141643);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енер-экономист.
pub(crate) fn n138838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137419);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 140072);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 141644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ль-маринист.
pub(crate) fn n138839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137420);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140073);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141645);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -автомашинист.
pub(crate) fn n138840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137421);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 140074);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word постмодернист.
pub(crate) fn n138841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137422);
	s.is_empty()
}

/// Words ending -нер-прочнист.
pub(crate) fn n138842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137423);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140075);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 141646);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рный-парашютист.
pub(crate) fn n138843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137424);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140076);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 141647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ограф.
pub(crate) fn n41362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40951);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 68923);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 68924);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94301);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125604);
									s.is_empty()
								}
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
			super::push(out, 68925);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 68926);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 94303);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 94304);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 94305);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 94306);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113342);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 125605);
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
							super::push(out, 113343);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 113344);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 125606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 113345);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 125607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 94307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 68927);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					super::push(out, 94308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 68928);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94309);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 94310);
					s.is_empty()
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 113348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', 'о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 125609);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 125610);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133020);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137437);
													s.is_empty()
												}
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
							super::push(out, 113349);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 125611);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133021);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137438);
													s.is_empty()
												}
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
							super::push(out, 113350);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125612);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 133022);
											s.is_empty()
										}
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
						['б', s @ ..] => {
							super::push(out, 113351);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125613);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133023);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 113352);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 94311);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113353);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 113354);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125615);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133024);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 137439);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 68929);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 94312);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113355);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125616);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133025);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'и', 'ц', s @ ..] => {
					super::push(out, 94313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 68930);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94314);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 94315);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113356);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 113357);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125618);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133026);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 113358);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125619);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133027);
											s.is_empty()
										}
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
					super::push(out, 94316);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t51::n114567(s, out),
						['и', s @ ..] => super::t51::n114568(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t51::n95334(s, out),
				['у', s @ ..] => super::t51::n95335(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t51::n69655(s, out),
		['н', s @ ..] => super::t51::n69656(s, out),
		['п', s @ ..] => super::t51::n69657(s, out),
		['р', s @ ..] => super::t51::n69658(s, out),
		['с', 'к', s @ ..] => super::t51::n69659(s, out),
		['т', s @ ..] => super::t51::n69660(s, out),
		['ф', s @ ..] => super::t51::n69661(s, out),
		['х', s @ ..] => super::t51::n69662(s, out),
		_ => false,
	}
}

/// Words ending -спевец.
pub(crate) fn n69968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 94542);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 113489);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ревец.
pub(crate) fn n41735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41320);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69244);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 94543);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113490);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125703);
									s.is_empty()
								}
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
			super::push(out, 69245);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 94544);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69246);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 94545);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 113492);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125704);
									s.is_empty()
								}
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
			super::push(out, 69247);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 94546);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113493);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 94547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113494);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -севец.
pub(crate) fn n41736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41321);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 69248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тевец.
pub(crate) fn n41737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41322);
	s.is_empty()
}

/// Words ending -уевец.
pub(crate) fn n41738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41323);
	s.is_empty()
}

/// Words ending -цевец.
pub(crate) fn n41739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41324);
	s.is_empty()
}

/// Words ending -чевец.
pub(crate) fn n41740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41325);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69249);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94548);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113495);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 94549);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113496);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 94550);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113497);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125709);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133067);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69250);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94551);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шевец.
pub(crate) fn n41741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41326);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69251);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94552);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113499);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125710);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 133068);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щевец.
pub(crate) fn n41742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41327);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 69252);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 69253);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94553);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 113500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьевец.
pub(crate) fn n41743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41328);
	s.is_empty()
}

/// Words ending -яевец.
pub(crate) fn n41744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41329);
	s.is_empty()
}

/// Words ending -вислинговец.
pub(crate) fn n138875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137456);
	s.is_empty()
}

/// Words ending -ьетконговец.
pub(crate) fn n138876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137457);
	s.is_empty()
}

/// Words ending -алькаидовец.
pub(crate) fn n138877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137458);
	s.is_empty()
}

/// The word мордовец.
pub(crate) fn n127024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125724);
	s.is_empty()
}

/// Words ending -удовец.
pub(crate) fn n70003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69278);
	s.is_empty()
}

/// Words ending -ядовец.
pub(crate) fn n70004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69279);
	s.is_empty()
}

/// Words ending -зовец.
pub(crate) fn n41755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69281);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69282);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94576);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113518);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125725);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133078);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69283);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 69284);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94577);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113519);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йовец.
pub(crate) fn n41756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41341);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковец.
pub(crate) fn n41757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41342);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69286);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94579);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113520);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 94580);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113521);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 125728);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 133079);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 113522);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125729);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133080);
											s.is_empty()
										}
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
					super::push(out, 94581);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113523);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 94582);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 94583);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69287);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94584);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 94585);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113525);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 125731);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133081);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69288);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 94586);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113526);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 113527);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125732);
									s.is_empty()
								}
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
			super::push(out, 69289);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94587);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113528);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125733);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133082);
											s.is_empty()
										}
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
					super::push(out, 94588);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 94589);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113530);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 94590);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 94591);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 113532);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125735);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133083);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 137459);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 125736);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133084);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137460);
													s.is_empty()
												}
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
							super::push(out, 113533);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125737);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 133085);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137461);
													match s {
														[] => true,
														['б', s @ ..] => super::t51::n141526(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69290);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94592);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 113534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 69291);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94593);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 94594);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113536);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125738);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133086);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 113537);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 125739);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133087);
											s.is_empty()
										}
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
					super::push(out, 94595);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 69292);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94596);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113539);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125740);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 133088);
											match s {
												[] => true,
												['а', s @ ..] => super::t51::n138881(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t51::n70018(s, out),
		['р', s @ ..] => super::t51::n70019(s, out),
		['с', s @ ..] => super::t51::n70020(s, out),
		['у', s @ ..] => super::t51::n70021(s, out),
		['ь', s @ ..] => super::t51::n70022(s, out),
		['э', s @ ..] => {
			super::push(out, 69298);
			s.is_empty()
		}
		['ю', s @ ..] => super::t51::n70024(s, out),
		['я', s @ ..] => {
			super::push(out, 69300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ловец.
pub(crate) fn n41758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41343);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69301);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 69302);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 69303);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 94609);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113551);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125748);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 133094);
											s.is_empty()
										}
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
					super::push(out, 94610);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113552);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 94611);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113553);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125750);
									s.is_empty()
								}
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
			super::push(out, 69304);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 94612);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113554);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125751);
									s.is_empty()
								}
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
			super::push(out, 69305);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 94613);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113555);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125752);
									match s {
										[] => true,
										['п', '-', s @ ..] => {
											super::push(out, 133095);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 137464);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141528(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69306);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 94614);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 113556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69307);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 94616);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113558);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125753);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133096);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137465);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 94617);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 94618);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113559);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125754);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 133097);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137466);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69308);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 69309);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69310);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94620);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113561);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125755);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 113562);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 125756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 113563);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 113564);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 125758);
									s.is_empty()
								}
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
			super::push(out, 69311);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 94621);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69312);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94622);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113566);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125759);
									s.is_empty()
								}
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
			super::push(out, 69313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мовец.
pub(crate) fn n41759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41344);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 69314);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 69315);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 94624);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113567);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125760);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133099);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137467);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новец.
pub(crate) fn n41760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41345);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69316);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94625);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 94626);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 113569);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125761);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133100);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137468);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 94627);
					match s {
						[] => true,
						['е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 113570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 94628);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 113571);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125762);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133101);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 137469);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141529(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69317);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94629);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113572);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125763);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133102);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69318);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94630);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 94631);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113573);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 125764);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133103);
											s.is_empty()
										}
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
					super::push(out, 94632);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113574);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 94633);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 94634);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113575);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125766);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 133104);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69319);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 94636);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113577);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125767);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 133105);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137470);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141530(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69320);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 69321);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94638);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125768);
									s.is_empty()
								}
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
			super::push(out, 69322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -повец.
pub(crate) fn n41761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41346);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69323);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 69324);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					super::push(out, 94639);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 94640);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113579);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 125769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133106);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69325);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 69326);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 94641);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 94642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113580);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 113581);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125770);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133107);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровец.
pub(crate) fn n41762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41347);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69328);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 94643);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 94644);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113582);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125771);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133108);
											s.is_empty()
										}
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
					super::push(out, 94645);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 113583);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125772);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 133109);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137471);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 94646);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113584);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125773);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133110);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69329);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94647);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113585);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125774);
									s.is_empty()
								}
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
			super::push(out, 69330);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 94648);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69331);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94649);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113587);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 94650);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113588);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125776);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133111);
											s.is_empty()
										}
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
					super::push(out, 94651);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125777);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133112);
											s.is_empty()
										}
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
					super::push(out, 94652);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113590);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125778);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 133113);
											s.is_empty()
										}
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
					super::push(out, 94653);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113591);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 94654);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113592);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125780);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133114);
											s.is_empty()
										}
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
					super::push(out, 94655);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 113593);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125781);
									s.is_empty()
								}
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
			super::push(out, 69332);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 94656);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113594);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125782);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133115);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 137472);
													match s {
														[] => true,
														['в', s @ ..] => super::t51::n141531(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69333);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94657);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125783);
									s.is_empty()
								}
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
			super::push(out, 69334);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 69335);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94658);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113596);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125784);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133116);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137473);
													match s {
														[] => true,
														['х', s @ ..] => super::t51::n141532(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69336);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94659);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113597);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125785);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133117);
											s.is_empty()
										}
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
					super::push(out, 94660);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113598);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 94661);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113599);
							match s {
								[] => true,
								['н', s @ ..] => super::t51::n127087(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => super::t51::n114811(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t51::n70062(s, out),
		['ы', s @ ..] => super::t51::n70063(s, out),
		['э', s @ ..] => super::t51::n70064(s, out),
		['ю', s @ ..] => super::t51::n70065(s, out),
		_ => false,
	}
}

/// Words ending -совец.
pub(crate) fn n41763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41348);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69341);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 69342);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 69343);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 94667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69344);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94668);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113604);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125789);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133119);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товец.
pub(crate) fn n41764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41349);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69346);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 69347);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 69348);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94670);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 94671);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113606);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125790);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133120);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69349);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94672);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113607);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125791);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133121);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69350);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94673);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113608);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125792);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133122);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69351);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94674);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113609);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125793);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133123);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137474);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фовец.
pub(crate) fn n41765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41350);
	s.is_empty()
}

/// Words ending -ховец.
pub(crate) fn n41766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41351);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 69352);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69353);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94676);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113610);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125794);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133124);
											s.is_empty()
										}
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
					super::push(out, 94677);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113611);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125795);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133125);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69354);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94678);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125796);
									s.is_empty()
								}
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
			super::push(out, 69355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шовец.
pub(crate) fn n41767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41352);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69356);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94679);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 94680);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 94681);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69357);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94682);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 69358);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94683);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бутербродец.
pub(crate) fn n138895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137476);
	s.is_empty()
}

/// Words ending -роходец.
pub(crate) fn n95742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94721);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 125816);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 125817);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 125818);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 133136);
							match s {
								[] => true,
								['р', 'е', 'п', '-', s @ ..] => {
									super::push(out, 137477);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 140092);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 141651);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n144008(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сходец.
pub(crate) fn n70119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 94722);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 113643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -выходец.
pub(crate) fn n95744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94723);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 113644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -билефельдец.
pub(crate) fn n138897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137478);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 140093);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word нефтеделец.
pub(crate) fn n138903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137484);
	s.is_empty()
}

/// Words ending -льшеземелец.
pub(crate) fn n138904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137485);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140097);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 141652);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лец-кормилец.
pub(crate) fn n138905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137486);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 140098);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -высокополец.
pub(crate) fn n138906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137487);
	s.is_empty()
}

/// Words ending -ставрополец.
pub(crate) fn n138907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137488);
	s.is_empty()
}

/// Words ending -борнхольмец.
pub(crate) fn n138908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137489);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 140099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -воин-афганец.
pub(crate) fn n141541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140100);
	s.is_empty()
}

/// Words ending -тор-республиканец.
pub(crate) fn n141542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140101);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141654);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142557);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еч-кладенец.
pub(crate) fn n138912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137493);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140102);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141655);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удженец.
pub(crate) fn n95977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94956);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 113796);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 125900);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133172);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еженец.
pub(crate) fn n70380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69649);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 94957);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 113797);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 125901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 94958);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 94959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иженец.
pub(crate) fn n70381(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 94960);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 113798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 94961);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 113799);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 125902);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133173);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рженец.
pub(crate) fn n70383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69651);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 94962);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 113800);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 125903);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133174);
									s.is_empty()
								}
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
			super::push(out, 94963);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 113801);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яженец.
pub(crate) fn n70385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69653);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 94965);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 113802);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зенец.
pub(crate) fn n41964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41548);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 69654);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94966);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йенец.
pub(crate) fn n41965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41549);
	s.is_empty()
}

/// Words ending -кенец.
pub(crate) fn n41966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41550);
	s.is_empty()
}

/// Words ending -ленец.
pub(crate) fn n41967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41551);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69655);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 69656);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 69657);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 94968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69658);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 69659);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94970);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125905);
									s.is_empty()
								}
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
			super::push(out, 69660);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 94971);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113807);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125906);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133175);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137496);
													match s {
														[] => true,
														['в', s @ ..] => super::t51::n141544(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -менец.
pub(crate) fn n41968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41552);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113808);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 125907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 94973);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 113809);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125908);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 133176);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69661);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 94974);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 94975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69662);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 94976);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69663);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94977);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 69664);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 94978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненец.
pub(crate) fn n41969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41553);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 69665);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 94979);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 69666);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 94980);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113813);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125909);
									s.is_empty()
								}
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
			super::push(out, 69667);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94981);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113814);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125910);
									s.is_empty()
								}
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
			super::push(out, 69668);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94982);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 94983);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 94984);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69669);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 94985);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113816);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125911);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133177);
											s.is_empty()
										}
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
					super::push(out, 94986);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенец.
pub(crate) fn n41970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41554);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69670);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 94987);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69671);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94988);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113819);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125912);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 133178);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137497);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренец.
pub(crate) fn n41971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41555);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69672);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 94989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 69673);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 69674);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 94990);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 113822);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125914);
									s.is_empty()
								}
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
			super::push(out, 69675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сенец.
pub(crate) fn n41972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41556);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69676);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 94991);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 94992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69677);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 94993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69678);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 94994);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 113824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 69679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 94995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенец.
pub(crate) fn n41973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41557);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 69680);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 94996);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113825);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 125915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 94997);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113826);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 125916);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133179);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 137498);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69681);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 69682);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94998);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фенец.
pub(crate) fn n41974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41558);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69683);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 94999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хенец.
pub(crate) fn n41975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41559);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69684);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 69685);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 95001);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченец.
pub(crate) fn n41976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41560);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69686);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 95002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69687);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95003);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113829);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 125917);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 133180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 113830);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125918);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133181);
											s.is_empty()
										}
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
					super::push(out, 95004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 69688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69689);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95006);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113831);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенец.
pub(crate) fn n41977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41561);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69690);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95007);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 95008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69691);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 69692);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95010);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125920);
									s.is_empty()
								}
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
			super::push(out, 69693);
			s.is_empty()
		}
		['ы', 'в', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95011);
					s.is_empty()
				}
				['з', 'о', 'в', '\u{1}', s @ ..] => {
					super::push(out, 95012);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенец.
pub(crate) fn n41978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41562);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69694);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95013);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113833);
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
				['в', s @ ..] => {
					super::push(out, 95014);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113834);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 125921);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133182);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 137499);
													match s {
														[] => true,
														['б', s @ ..] => super::t51::n141545(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 95015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69695);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 95016);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113835);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 125922);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133183);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 137500);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69696);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьенец.
pub(crate) fn n41979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41563);
	s.is_empty()
}

/// Words ending -аргинец.
pub(crate) fn n96062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95041);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 113851);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 125933);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -виргинец.
pub(crate) fn n115064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113852);
	s.is_empty()
}

/// Words ending -угинец.
pub(crate) fn n70452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69714);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 95043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -динец.
pub(crate) fn n41986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41570);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69715);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 95044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95045);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 95046);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 95047);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 69716);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95048);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 113856);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125934);
									s.is_empty()
								}
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
			super::push(out, 69717);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 95049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 69718);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95050);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 69719);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95051);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 95052);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 95053);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 95054);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 113860);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125935);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 133188);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 137501);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69720);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95055);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 95056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69721);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95057);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 95058);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113862);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 125936);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 133189);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137502);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69722);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 69723);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 69724);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 69725);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95062);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еинец.
pub(crate) fn n41987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41571);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 69726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жинец.
pub(crate) fn n41988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41572);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 69727);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 69728);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 95063);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 95064);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зинец.
pub(crate) fn n41989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41573);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95065);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 95066);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113866);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 125937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 95067);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 113867);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 125938);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 133190);
											s.is_empty()
										}
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
					super::push(out, 95068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 69729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95069);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 69730);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 95070);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 113870);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125939);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133191);
											s.is_empty()
										}
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
					super::push(out, 95071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69731);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69732);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95073);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69733);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 69734);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95074);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кинец.
pub(crate) fn n41990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69735);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95075);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 95076);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69736);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 95077);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 95078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 95080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69737);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 69738);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69739);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 69740);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95083);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 95084);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113878);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125940);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133192);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137503);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

