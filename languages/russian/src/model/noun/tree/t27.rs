use flexion::Path;

/// Words ending -юды.
pub(crate) fn n4484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4473);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18443);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42319);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яды.
pub(crate) fn n4485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4474);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18444);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42320);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -байтслезы.
pub(crate) fn n127423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126123);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 133272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -разносолы.
pub(crate) fn n127424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126124);
	s.is_empty()
}

/// Words ending -урбангулы.
pub(crate) fn n127425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126125);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 133273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кулы.
pub(crate) fn n18584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18480);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 42356);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 70497);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пялы.
pub(crate) fn n18585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18481);
	s.is_empty()
}

/// Words ending -итнагдимы.
pub(crate) fn n127426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126126);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 133274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-обоймы.
pub(crate) fn n127427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126127);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 133275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ор-органы.
pub(crate) fn n127428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126128);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 133276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бадрижаны.
pub(crate) fn n127429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126129);
	s.is_empty()
}

/// Words ending -полстраны.
pub(crate) fn n127430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126130);
	s.is_empty()
}

/// Words ending -ины.
pub(crate) fn n4520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4509);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18504);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42377);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70511);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 95648);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 114239);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126131);
													match s {
														[] => true,
														['о', s @ ..] => super::t39::n134647(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18505);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42378);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70512);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95649);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 114240);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114241);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126132);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 114242);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126133);
													s.is_empty()
												}
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
							super::push(out, 70513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18506);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42379);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70514);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95650);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114243);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126134);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18507);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 42380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 42381);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70516);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95651);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114244);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126135);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18508);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42382);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70517);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 95652);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114245);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126136);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42383);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70518);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95653);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114246);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 126137);
													match s {
														[] => true,
														['и', s @ ..] => super::t39::n134648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18509);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42384);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70519);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95654);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 114247);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 126138);
													match s {
														[] => true,
														['т', s @ ..] => super::t39::n134649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18510);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18511);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 42386);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18512);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18513);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 18514);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лоны.
pub(crate) fn n18621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18517);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42391);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 70522);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95656);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114248);
									s.is_empty()
								}
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
			super::push(out, 42392);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70523);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 95657);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 114249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роны.
pub(crate) fn n18623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18519);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42393);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70524);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95658);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 114250);
									s.is_empty()
								}
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
			super::push(out, 42394);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70525);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95659);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114251);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126139);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 42395);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 70526);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95660);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соны.
pub(crate) fn n18624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18520);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42396);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 42397);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70528);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95662);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рны.
pub(crate) fn n4523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4512);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18521);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 42398);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70529);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95663);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 114254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уны.
pub(crate) fn n4524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4513);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18522);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 42399);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70530);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95664);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хны.
pub(crate) fn n4525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4514);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18523);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яны.
pub(crate) fn n4526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4515);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18524);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 18525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-европы.
pub(crate) fn n127440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126140);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 133280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ползалупы.
pub(crate) fn n127441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126141);
	s.is_empty()
}

/// Words ending -кодоллары.
pub(crate) fn n127442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126142);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 133281);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 137545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тартарары.
pub(crate) fn n127443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126143);
	s.is_empty()
}

/// Words ending -шантары.
pub(crate) fn n96704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95678);
	s.is_empty()
}

/// Words ending -уары.
pub(crate) fn n18647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18543);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 42417);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 42418);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70546);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чары.
pub(crate) fn n18648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18544);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 42419);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шары.
pub(crate) fn n18649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18545);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 42420);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 70547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бры.
pub(crate) fn n4534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4523);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18546);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 42421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 18547);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 42422);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70548);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вры.
pub(crate) fn n4535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18548);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 42423);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70549);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гры.
pub(crate) fn n4536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4525);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18549);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18550);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 42425);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70550);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95682);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114267);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дры.
pub(crate) fn n4537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4526);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18551);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еры.
pub(crate) fn n4538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4527);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18552);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 18553);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42427);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70551);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 95683);
									s.is_empty()
								}
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
			super::push(out, 18554);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 42428);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70552);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 95684);
									s.is_empty()
								}
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
			super::push(out, 18555);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18556);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 42430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 18557);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42431);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 42432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18558);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42433);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70554);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 95685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иры.
pub(crate) fn n4539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4528);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 18559);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42434);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70555);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 95686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кры.
pub(crate) fn n4540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4529);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18560);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 42435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мры.
pub(crate) fn n4541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4530);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18561);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42436);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оры.
pub(crate) fn n4542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4531);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18562);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42437);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70556);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 95687);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 114268);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 126144);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42438);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70557);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 95688);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 114269);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 126145);
													match s {
														[] => true,
														['е', s @ ..] => super::t39::n134652(s, out),
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
									super::push(out, 95689);
									s.is_empty()
								}
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
			super::push(out, 18563);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42439);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18564);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18566);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18567);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42443);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70559);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95690);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114270);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42444);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 70560);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95691);
									match s {
										[] => true,
										['ъ', s @ ..] => {
											super::push(out, 114271);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126146);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18569);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 18570);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тры.
pub(crate) fn n4543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4532);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 18571);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42445);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18572);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42446);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уры.
pub(crate) fn n4544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4533);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					super::push(out, 42447);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 42448);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70563);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 95692);
									s.is_empty()
								}
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
			super::push(out, 18573);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42449);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 42450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хры.
pub(crate) fn n4545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4534);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18574);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18575);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42452);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 70565);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 95693);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114272);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 126147);
													match s {
														[] => true,
														['у', s @ ..] => super::t39::n134653(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юры.
pub(crate) fn n4546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4535);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 18576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -март-часы.
pub(crate) fn n127448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126148);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 133284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -укрыниксы.
pub(crate) fn n127449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126149);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 133285);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инсы.
pub(crate) fn n18700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18595);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 42476);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 70579);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 70580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 42477);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 70581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 42478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осы.
pub(crate) fn n4554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4543);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18596);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42479);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 70582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18597);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 18598);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42480);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18599);
			match s {
				[] => true,
				['б', 'а', 'б', '\u{1}', s @ ..] => {
					super::push(out, 42481);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 42482);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18600);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 42483);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 70585);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95703);
									s.is_empty()
								}
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
			super::push(out, 18601);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42484);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42485);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсы.
pub(crate) fn n4555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4544);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18602);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42486);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70588);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 42487);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 70589);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42488);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70590);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95706);
									s.is_empty()
								}
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
			super::push(out, 18603);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42489);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 42490);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссы.
pub(crate) fn n4556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4545);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18604);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42491);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70592);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95707);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114275);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126150);
													match s {
														[] => true,
														['и', s @ ..] => super::t39::n134656(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -айтсы.
pub(crate) fn n42914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42492);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 70593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усы.
pub(crate) fn n4558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4547);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18606);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18607);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 42493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 18608);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 18609);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42494);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70594);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95708);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114276);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 126151);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134657(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 42495);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 42496);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьсы.
pub(crate) fn n4559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4548);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18610);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42497);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясы.
pub(crate) fn n4560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4549);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18611);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42498);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 70597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олпланеты.
pub(crate) fn n127452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126152);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 133288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фараониты.
pub(crate) fn n127453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126153);
	s.is_empty()
}

/// Words ending -оты.
pub(crate) fn n4566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4555);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18630);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42520);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70617);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95715);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 114280);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18631);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42521);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70618);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95716);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 114281);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 126154);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18633);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42523);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70619);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95717);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114282);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18634);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 18635);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42524);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70620);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 95718);
									s.is_empty()
								}
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
			super::push(out, 18636);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42525);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70621);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95719);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114283);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18637);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 18638);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42526);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70622);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 95720);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114284);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рты.
pub(crate) fn n4567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4556);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18639);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42527);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 42528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18640);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 42530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 18641);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 42531);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70624);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95721);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сты.
pub(crate) fn n4568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4557);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18642);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 42532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18643);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42533);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70625);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 95722);
									s.is_empty()
								}
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
			super::push(out, 18644);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ватты.
pub(crate) fn n42957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42535);
	s.is_empty()
}

/// Words ending -отты.
pub(crate) fn n18751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18646);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 42536);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уты.
pub(crate) fn n4570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4559);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18647);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 18648);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42537);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 70628);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95724);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114286);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126155);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134659(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --луковицы.
pub(crate) fn n127456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126156);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 133290);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лстраницы.
pub(crate) fn n127457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126157);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133291);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 137548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анцы.
pub(crate) fn n18769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18664);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 42554);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 70642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 42555);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 42556);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70644);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 95733);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114293);
									s.is_empty()
								}
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
			super::push(out, 42557);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70645);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 95734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 70646);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95735);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 114294);
									s.is_empty()
								}
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
			super::push(out, 42558);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 70647);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енцы.
pub(crate) fn n18770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18665);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 42559);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инцы.
pub(crate) fn n18771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18666);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 42560);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70648);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онцы.
pub(crate) fn n18772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18667);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 42561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 42562);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 70650);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пцы.
pub(crate) fn n4583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4572);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18668);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 42563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рцы.
pub(crate) fn n4584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4573);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18669);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42564);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 70651);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95737);
									s.is_empty()
								}
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
			super::push(out, 18670);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42565);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70652);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сцы.
pub(crate) fn n4585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18671);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42566);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тцы.
pub(crate) fn n4586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4575);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18672);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42567);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70654);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95739);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114295);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126158);
													match s {
														[] => true,
														['п', s @ ..] => super::t39::n134662(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18673);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьцы.
pub(crate) fn n4587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4576);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42569);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70656);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95740);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 114296);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 126159);
													match s {
														[] => true,
														['п', s @ ..] => super::t39::n134663(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 42570);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70657);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 95741);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 114297);
											s.is_empty()
										}
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
					super::push(out, 42571);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70658);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркменбашы.
pub(crate) fn n127460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126160);
	s.is_empty()
}

/// Words ending -жецерковь.
pub(crate) fn n127461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126161);
	s.is_empty()
}

/// Words ending -ь-морковь.
pub(crate) fn n127462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126162);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 133294);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137549);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 140124);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 141665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word споцервь.
pub(crate) fn n127463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126163);
	s.is_empty()
}

/// Words ending -етвь.
pub(crate) fn n18811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18706);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 42602);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 70679);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 70680);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывь.
pub(crate) fn n4605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4594);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18708);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42604);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70681);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95757);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114305);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явь.
pub(crate) fn n4606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4595);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18709);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42605);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илплощадь.
pub(crate) fn n127464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126164);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 133295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анплощадь.
pub(crate) fn n127465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126165);
	s.is_empty()
}

/// Words ending --исповедь.
pub(crate) fn n127466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126166);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 133296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 137550);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140125);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 141666);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142563);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ледь.
pub(crate) fn n18825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18720);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42618);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 70693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'ж', 'о', '\u{1}', s @ ..] => {
			super::push(out, 42619);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 42620);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 70694);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95766);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -медь.
pub(crate) fn n18826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18721);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42621);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42622);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 70696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 95767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 70697);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 95768);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -редь.
pub(crate) fn n18829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18724);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42625);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 70698);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42626);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70699);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -седь.
pub(crate) fn n18830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18725);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 42627);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70700);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ждь.
pub(crate) fn n4609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4598);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18726);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42628);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 42629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здь.
pub(crate) fn n4610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4599);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18727);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42630);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 70702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18728);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42631);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42632);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95772);
									s.is_empty()
								}
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
			super::push(out, 18729);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42633);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70705);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 95773);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114312);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126167);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идь.
pub(crate) fn n4611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4600);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 18730);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42634);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70706);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95774);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114313);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18731);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одь.
pub(crate) fn n4612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4601);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18732);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42636);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 70707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42637);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70708);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95775);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114314);
											s.is_empty()
										}
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
					super::push(out, 42638);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18733);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42639);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70710);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95776);
									s.is_empty()
								}
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
			super::push(out, 18734);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42640);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70711);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95777);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114315);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126168);
													s.is_empty()
												}
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
							super::push(out, 70712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 18735);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70713);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 95778);
									s.is_empty()
								}
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
			super::push(out, 18736);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42642);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70714);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 95779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42643);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70715);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 95780);
									s.is_empty()
								}
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
			super::push(out, 18737);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42644);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70716);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдь.
pub(crate) fn n4613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4602);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18738);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42645);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 42646);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 42647);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 42648);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удь.
pub(crate) fn n4614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4603);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18739);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42649);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70719);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 95783);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114316);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 126169);
													match s {
														[] => true,
														['к', s @ ..] => super::t39::n134667(s, out),
														['ч', s @ ..] => super::t39::n134668(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18740);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 18741);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42650);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 70720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 18742);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 42651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18743);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 18744);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70721);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 70722);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95784);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114317);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 126170);
													s.is_empty()
												}
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
							super::push(out, 70723);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 95785);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114318);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126171);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18745);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 18746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыдь.
pub(crate) fn n4615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4604);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 18747);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдь.
pub(crate) fn n4616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4605);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42655);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдь.
pub(crate) fn n4617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4606);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18749);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42656);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 42657);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядь.
pub(crate) fn n4618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4607);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18750);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42658);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70727);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95787);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 114319);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 126172);
													match s {
														[] => true,
														['п', s @ ..] => super::t39::n134669(s, out),
														_ => false,
													}
												}
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
							super::push(out, 70728);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95788);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114320);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126173);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42659);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70729);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95789);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114321);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126174);
													match s {
														[] => true,
														['н', s @ ..] => super::t39::n134670(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 42660);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70730);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 70731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 42661);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70732);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95790);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114322);
											s.is_empty()
										}
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
					super::push(out, 42662);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42663);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70734);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95791);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114323);
											s.is_empty()
										}
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
					super::push(out, 42664);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 70735);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95792);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114324);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18751);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42665);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70736);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 95793);
									s.is_empty()
								}
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
			super::push(out, 18752);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 18753);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42666);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70737);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42667);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42668);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70738);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95795);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язь.
pub(crate) fn n4631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4620);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18776);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 18777);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 18778);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42703);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 70766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 42704);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70767);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 42705);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70768);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95809);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114331);
											s.is_empty()
										}
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
					super::push(out, 42706);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70769);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 42707);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70770);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95811);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42708);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70771);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95812);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114332);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126176);
													s.is_empty()
												}
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
							super::push(out, 70772);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 42709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70773);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 70774);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95814);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 95815);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114333);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 126177);
													match s {
														[] => true,
														['з', s @ ..] => super::t39::n134672(s, out),
														_ => false,
													}
												}
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
							super::push(out, 70775);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 70776);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 95816);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 114334);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126178);
													match s {
														[] => true,
														['ф', s @ ..] => super::t39::n134673(s, out),
														_ => false,
													}
												}
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
							super::push(out, 70777);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95817);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 114335);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126179);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134674(s, out),
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
									super::push(out, 95818);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 114336);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42710);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70778);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95819);
									s.is_empty()
								}
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
			super::push(out, 18780);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42711);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18781);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42712);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 42713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18782);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42714);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 42715);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 70781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 42716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18783);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42717);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70782);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аль.
pub(crate) fn n4632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4621);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18784);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42718);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70783);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 70784);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95822);
									s.is_empty()
								}
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
			super::push(out, 18785);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42719);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42720);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 70786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 42721);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70787);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 70788);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42722);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 42723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18787);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42724);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 42725);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70792);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95827);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114337);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126180);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42726);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95828);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 95829);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114338);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126181);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42727);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70794);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 70795);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95831);
									s.is_empty()
								}
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
			super::push(out, 18788);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42728);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70796);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 70797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 42729);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 70798);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 70799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 42730);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 70800);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95833);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114339);
											s.is_empty()
										}
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
					super::push(out, 42731);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70801);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95834);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114340);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 70802);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95835);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114341);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 126182);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42732);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70803);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 70804);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95837);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114342);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 70805);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 70806);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95839);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114343);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126183);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42733);
					match s {
						[] => true,
						['е', s @ ..] => super::t39::n71555(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t39::n43156(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t39::n18894(s, out),
		['ж', s @ ..] => super::t39::n18895(s, out),
		['з', s @ ..] => super::t39::n18896(s, out),
		['и', s @ ..] => super::t39::n18897(s, out),
		['к', s @ ..] => super::t39::n18898(s, out),
		['м', s @ ..] => super::t39::n18899(s, out),
		['н', s @ ..] => super::t39::n18900(s, out),
		['п', s @ ..] => super::t39::n18901(s, out),
		['р', s @ ..] => super::t39::n18902(s, out),
		['с', s @ ..] => super::t39::n18903(s, out),
		['т', s @ ..] => super::t39::n18904(s, out),
		['у', s @ ..] => super::t39::n18905(s, out),
		['ф', s @ ..] => super::t39::n18906(s, out),
		['х', s @ ..] => super::t39::n18907(s, out),
		['ц', s @ ..] => super::t39::n18908(s, out),
		['ч', s @ ..] => super::t39::n18909(s, out),
		['ш', s @ ..] => super::t39::n18910(s, out),
		['щ', s @ ..] => super::t39::n18911(s, out),
		_ => false,
	}
}

/// Words ending -дирижабль.
pub(crate) fn n127504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126204);
	s.is_empty()
}

/// Words ending -ратикабль.
pub(crate) fn n127505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126205);
	s.is_empty()
}

/// Words ending -актикабль.
pub(crate) fn n127506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126206);
	s.is_empty()
}

/// Words ending -артикабль.
pub(crate) fn n127507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126207);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 133313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оннетабль.
pub(crate) fn n127508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126208);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 133314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фешенебль.
pub(crate) fn n127509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126209);
	s.is_empty()
}

/// Words ending -констебль.
pub(crate) fn n127510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126210);
	s.is_empty()
}

/// Words ending -ик-ансамбль.
pub(crate) fn n115607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114395);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 126211);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 133315);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 137557);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 140130);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141669);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142564);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обль.
pub(crate) fn n18915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18810);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 42807);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70907);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 95923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42808);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70908);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убль.
pub(crate) fn n18916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18811);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 42809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 42810);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42811);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбль.
pub(crate) fn n18917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18812);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 42812);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70912);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 95925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вль.
pub(crate) fn n4634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4623);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18813);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42813);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70913);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 95926);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 70914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42814);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 70915);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 95927);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114397);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гль.
pub(crate) fn n4635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4624);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18814);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18815);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18816);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42817);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 70917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дль.
pub(crate) fn n4636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4625);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18817);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42818);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18818);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42819);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70919);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 95928);
									s.is_empty()
								}
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
			super::push(out, 18819);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 42820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18820);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 42821);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70920);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95929);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 114398);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126212);
													match s {
														[] => true,
														['л', s @ ..] => super::t39::n134686(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18821);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42822);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70921);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 95930);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114399);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126213);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ель.
pub(crate) fn n4637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4626);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42823);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 42824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70922);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 70923);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 70924);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 70925);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 95933);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114401);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 70926);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95934);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114402);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126214);
													s.is_empty()
												}
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
							super::push(out, 70927);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 95935);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114403);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 114404);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 126215);
													s.is_empty()
												}
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
							super::push(out, 70928);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95936);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114405);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126216);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70929);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 70930);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 95937);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 114406);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126217);
													match s {
														[] => true,
														['а', s @ ..] => super::t39::n134687(s, out),
														_ => false,
													}
												}
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
							super::push(out, 70931);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 70932);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95939);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 114407);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126218);
													match s {
														[] => true,
														['ф', s @ ..] => super::t39::n134688(s, out),
														_ => false,
													}
												}
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
							super::push(out, 70933);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95940);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114408);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 126219);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134689(s, out),
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
									super::push(out, 95941);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t39::n115621(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => super::t39::n71682(s, out),
						['х', s @ ..] => super::t39::n71683(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t39::n43249(s, out),
				['м', s @ ..] => super::t39::n43250(s, out),
				['о', s @ ..] => super::t39::n43251(s, out),
				['т', s @ ..] => super::t39::n43252(s, out),
				['у', s @ ..] => super::t39::n43253(s, out),
				['ы', s @ ..] => super::t39::n43254(s, out),
				['ю', s @ ..] => super::t39::n43255(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t39::n18928(s, out),
		['г', s @ ..] => super::t39::n18929(s, out),
		['д', s @ ..] => super::t39::n18930(s, out),
		['ж', s @ ..] => super::t39::n18931(s, out),
		['з', s @ ..] => super::t39::n18932(s, out),
		['и', s @ ..] => super::t39::n18933(s, out),
		['к', s @ ..] => super::t39::n18934(s, out),
		['л', s @ ..] => super::t39::n18935(s, out),
		['м', s @ ..] => super::t40::n18936(s, out),
		['н', s @ ..] => super::t40::n18937(s, out),
		['о', s @ ..] => super::t40::n18938(s, out),
		['п', s @ ..] => super::t40::n18939(s, out),
		['р', s @ ..] => super::t40::n18940(s, out),
		['с', s @ ..] => super::t40::n18941(s, out),
		['т', s @ ..] => super::t40::n18942(s, out),
		['ф', s @ ..] => super::t40::n18943(s, out),
		['х', s @ ..] => super::t40::n18944(s, out),
		['ц', s @ ..] => super::t40::n18945(s, out),
		['ч', s @ ..] => super::t40::n18946(s, out),
		['ш', s @ ..] => super::t40::n18947(s, out),
		['щ', s @ ..] => super::t40::n18948(s, out),
		_ => false,
	}
}

/// Words ending -иль.
pub(crate) fn n4638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4627);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18844);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42968);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71215);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42969);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 71216);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96311);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 114804);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 96312);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114805);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 71217);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96313);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114806);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18845);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71218);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 71219);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 42971);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42972);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 71221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42973);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 71222);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 96316);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 71223);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 42974);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71224);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96318);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 114807);
											s.is_empty()
										}
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
					super::push(out, 42975);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71225);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96319);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114808);
											s.is_empty()
										}
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
					super::push(out, 42976);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 42977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 42978);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 42979);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42980);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 42981);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 42982);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71230);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 71231);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96320);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114809);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18848);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42983);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 42984);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71233);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96321);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42985);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 71234);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 96322);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114810);
											s.is_empty()
										}
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
					super::push(out, 42986);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71235);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114811);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18849);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42987);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96324);
									s.is_empty()
								}
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
			super::push(out, 18850);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42988);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96325);
									s.is_empty()
								}
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
			super::push(out, 18851);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42989);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 42990);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71239);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 96327);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114812);
											s.is_empty()
										}
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
					super::push(out, 42991);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t40::n18957(s, out),
		['м', s @ ..] => super::t40::n18958(s, out),
		['н', s @ ..] => super::t40::n18959(s, out),
		['о', s @ ..] => super::t40::n18960(s, out),
		['п', s @ ..] => super::t40::n18961(s, out),
		['р', s @ ..] => super::t40::n18962(s, out),
		['с', s @ ..] => super::t40::n18963(s, out),
		['т', s @ ..] => super::t40::n18964(s, out),
		['ф', s @ ..] => super::t40::n18965(s, out),
		['х', s @ ..] => super::t40::n18966(s, out),
		['ц', s @ ..] => super::t40::n18967(s, out),
		_ => false,
	}
}

/// Words ending -йль.
pub(crate) fn n4639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4628);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18863);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43022);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71282);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 96362);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114835);
											s.is_empty()
										}
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
					super::push(out, 43023);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71283);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 96363);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114836);
											s.is_empty()
										}
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
					super::push(out, 43024);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71284);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96364);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 114837);
											s.is_empty()
										}
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
					super::push(out, 43025);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71285);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 43026);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 71286);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96366);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 114838);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18864);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43027);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71287);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96367);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 114839);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 126642);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t40::n135051(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43028);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71288);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96368);
									match s {
										[] => true,
										['к', '-', s @ ..] => {
											super::push(out, 114840);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126643);
													match s {
														[] => true,
														['ь', s @ ..] => super::t40::n135052(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18865);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43029);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71289);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кль.
pub(crate) fn n4640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4629);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18866);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43030);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 43031);
					match s {
						[] => true,
						['к', 'е', 'п', 'с', '-', s @ ..] => {
							super::push(out, 71290);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96370);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 114841);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126644);
													match s {
														[] => true,
														['и', s @ ..] => super::t40::n135053(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18867);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18868);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лль.
pub(crate) fn n4641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4630);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18869);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43033);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71291);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96371);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114842);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 126645);
													match s {
														[] => true,
														['с', s @ ..] => super::t40::n135054(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18870);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43034);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 43035);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18871);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43037);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18872);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 43038);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мль.
pub(crate) fn n4642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4631);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18873);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43039);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18874);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43040);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 71295);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96372);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114843);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 126646);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n135055(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нль.
pub(crate) fn n4643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18875);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43041);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оль.
pub(crate) fn n4644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4633);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18876);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43042);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 43043);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 71297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43044);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71298);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96373);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 96374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43045);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96375);
									s.is_empty()
								}
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
			super::push(out, 18877);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43046);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43047);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71300);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43048);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 71301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43049);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71302);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 96377);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114844);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126647);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43050);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 71303);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 71304);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96379);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 71305);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 96380);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 114846);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126648);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n135056(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18878);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43052);
					match s {
						[] => true,
						['а', 'р', 'и', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 96381);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 96382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 43053);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71307);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 43054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18879);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43055);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71308);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 96384);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114847);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18880);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43056);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43057);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 71309);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 71310);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 96386);
									s.is_empty()
								}
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
			super::push(out, 18881);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71311);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 96387);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114848);
											s.is_empty()
										}
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
					super::push(out, 43059);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71312);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96388);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114849);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 71313);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96389);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t40::n72063(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t40::n43483(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t40::n18987(s, out),
		['к', s @ ..] => super::t40::n18988(s, out),
		['м', s @ ..] => super::t40::n18989(s, out),
		['н', s @ ..] => super::t40::n18990(s, out),
		['п', s @ ..] => super::t40::n18991(s, out),
		['р', s @ ..] => super::t40::n18992(s, out),
		['с', s @ ..] => super::t40::n18993(s, out),
		['т', s @ ..] => super::t40::n18994(s, out),
		['у', s @ ..] => super::t40::n18995(s, out),
		['ф', s @ ..] => super::t40::n18996(s, out),
		['х', s @ ..] => super::t40::n18997(s, out),
		['ь', s @ ..] => super::t40::n18998(s, out),
		_ => false,
	}
}

/// Words ending -пль.
pub(crate) fn n4645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4634);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18894);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71379);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 96454);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 114890);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126669);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43108);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71380);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96455);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114891);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18895);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 43109);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 71381);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96456);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 114892);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 126670);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18896);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18897);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 43111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рль.
pub(crate) fn n4646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4635);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18898);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 43112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сль.
pub(crate) fn n4647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18899);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43113);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71383);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96457);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114893);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18900);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43114);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71384);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 96458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 71385);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96459);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114894);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 126671);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 114895);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 126672);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 96460);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 96461);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114896);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 126673);
													match s {
														[] => true,
														['п', s @ ..] => super::t40::n135066(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18901);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 43115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18902);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43116);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71386);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тль.
pub(crate) fn n4648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4637);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18903);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43117);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71387);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96463);
									s.is_empty()
								}
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
			super::push(out, 18904);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43118);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 71388);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96464);
									s.is_empty()
								}
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
			super::push(out, 18905);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43119);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71389);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96465);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114897);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126674);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18906);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43120);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71390);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уль.
pub(crate) fn n4649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18907);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43121);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 43122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 18908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43123);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 43124);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71393);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96467);
									s.is_empty()
								}
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
			super::push(out, 18909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43125);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43126);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43127);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 71395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 43128);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 71396);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43129);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71397);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96470);
									s.is_empty()
								}
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
			super::push(out, 18910);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43130);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 71398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18911);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43132);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 43133);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71399);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 96471);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114898);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126675);
													match s {
														[] => true,
														['и', s @ ..] => super::t40::n135067(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43134);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71400);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96472);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114899);
											s.is_empty()
										}
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
					super::push(out, 43135);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71401);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96473);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 71402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 43136);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71403);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 96474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 71404);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96475);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114901);
											s.is_empty()
										}
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
					super::push(out, 43137);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 43138);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71406);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96476);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114902);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 126676);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18913);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18914);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18915);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 43141);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71408);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96477);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 114903);
											match s {
												[] => true,
												['е', 'м', '\u{1}', s @ ..] => {
													super::push(out, 126677);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18916);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43142);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71409);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 96478);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 114904);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126678);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18917);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43143);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71410);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 96479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71411);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114905);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t40::n43568(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t40::n19023(s, out),
		_ => false,
	}
}

/// Words ending -ыль.
pub(crate) fn n4650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4639);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43148);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 43149);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43150);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71415);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43151);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71416);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 71417);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96485);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114907);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 126679);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43152);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71418);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114908);
											s.is_empty()
										}
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
					super::push(out, 43153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 18920);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43154);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18921);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71420);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 96487);
									s.is_empty()
								}
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
			super::push(out, 18922);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43156);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18923);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 18924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43158);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43159);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18925);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18926);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18927);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43162);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 71423);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 43163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71424);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 71425);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96489);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114909);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 71426);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43164);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 43165);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 71428);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эль.
pub(crate) fn n4651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4640);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18928);
			s.is_empty()
		}
		['-', 'л', 'й', 'э', 'п', '\u{1}', s @ ..] => {
			super::push(out, 18929);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18930);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43166);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43167);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 43168);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 43169);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 43170);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18931);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43171);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 43172);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 71436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96493);
									match s {
										[] => true,
										['г', '\u{1}', s @ ..] => {
											super::push(out, 114910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 71437);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96494);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114911);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18932);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18933);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43174);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 71438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18934);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 43175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43176);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71440);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 43177);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71441);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96496);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114912);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18935);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43178);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 71442);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96497);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114913);
											s.is_empty()
										}
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
					super::push(out, 43179);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71443);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96498);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114914);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юль.
pub(crate) fn n4652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4641);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18936);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43180);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71444);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96499);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114915);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 126680);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43181);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71445);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 96500);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 114916);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18938);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43183);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71446);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96501);
									s.is_empty()
								}
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
			super::push(out, 18939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18940);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 18941);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43185);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18943);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43187);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71448);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96502);
									s.is_empty()
								}
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
			super::push(out, 18944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яль.
pub(crate) fn n4653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4642);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43188);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 71449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43189);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96503);
									s.is_empty()
								}
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
			super::push(out, 18946);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43190);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 71451);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 96504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ахичевань.
pub(crate) fn n127994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126681);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 133690);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьвань.
pub(crate) fn n43631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43208);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 71464);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 96512);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 114919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гань.
pub(crate) fn n19065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18960);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43209);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43210);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 71466);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 71467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71468);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дань.
pub(crate) fn n19066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18961);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43212);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зань.
pub(crate) fn n19067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18962);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43214);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71470);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кань.
pub(crate) fn n19068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18963);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43215);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 43216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лань.
pub(crate) fn n19069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18964);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43217);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 43218);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мань.
pub(crate) fn n19070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18965);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43219);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43220);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 71473);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нань.
pub(crate) fn n19071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18966);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 43222);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71474);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96515);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114920);
									s.is_empty()
								}
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
			super::push(out, 43223);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71475);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43224);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 71476);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оань.
pub(crate) fn n19072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18967);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43225);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 71477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пань.
pub(crate) fn n19073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18968);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43226);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 71478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43227);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71479);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 43228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71480);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 96520);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 114921);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 126682);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 133691);
													match s {
														[] => true,
														['ф', s @ ..] => super::t40::n139259(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43229);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 71481);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 71482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43230);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71483);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 96521);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114922);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126683);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133692);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рань.
pub(crate) fn n19074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18969);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43231);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 71484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 43232);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 43233);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43234);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 71485);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тань.
pub(crate) fn n19075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18970);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43235);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71486);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 96523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 43236);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71487);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 96524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43237);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71488);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96525);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 114923);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126684);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 126685);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 133693);
													match s {
														[] => true,
														['р', s @ ..] => super::t40::n139260(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 71489);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96526);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 114924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уань.
pub(crate) fn n19076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18971);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 43238);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 71490);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 96527);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114925);
									s.is_empty()
								}
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
			super::push(out, 43239);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 71491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фань.
pub(crate) fn n19077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18972);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 43240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71492);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96528);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 114926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хань.
pub(crate) fn n19078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18973);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43241);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 43242);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71493);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 96529);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114927);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126686);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43243);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 71494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 43244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шань.
pub(crate) fn n19079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18974);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 43245);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 71496);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96530);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 114928);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 126687);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юань.
pub(crate) fn n19080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18975);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43246);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 43247);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 71497);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 96531);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 114929);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 126688);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 133694);
													match s {
														[] => true,
														['т', s @ ..] => super::t40::n139261(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гнь.
pub(crate) fn n4661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4650);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43248);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ень.
pub(crate) fn n4662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4651);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18977);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43249);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 71498);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96532);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114930);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 114931);
											s.is_empty()
										}
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
					super::push(out, 43250);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71499);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96533);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 71500);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96534);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114933);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126689);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t40::n135073(s, out),
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
									super::push(out, 96535);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114934);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 71501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43251);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71502);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96537);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114935);
											s.is_empty()
										}
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
					super::push(out, 43252);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71503);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43253);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71504);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96539);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 71505);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96540);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 96541);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 114937);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126690);
													s.is_empty()
												}
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
							super::push(out, 71506);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96542);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114938);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126691);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18978);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43254);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71507);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 71508);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43255);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 71509);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 71510);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 71511);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 96547);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114939);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126692);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 126693);
													match s {
														[] => true,
														['ф', '\u{1}', s @ ..] => super::t40::n135074(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43256);
					s.is_empty()
				}
				['и', s @ ..] => super::t40::n43680(s, out),
				['о', s @ ..] => super::t40::n43681(s, out),
				['р', s @ ..] => {
					super::push(out, 43259);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71519);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 96554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t40::n43683(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t40::n19084(s, out),
		['ж', s @ ..] => super::t40::n19085(s, out),
		['з', s @ ..] => super::t40::n19086(s, out),
		['л', s @ ..] => super::t40::n19087(s, out),
		['м', s @ ..] => super::t40::n19088(s, out),
		['п', s @ ..] => super::t40::n19089(s, out),
		['р', s @ ..] => super::t40::n19090(s, out),
		['с', s @ ..] => super::t40::n19091(s, out),
		['т', s @ ..] => super::t40::n19092(s, out),
		['ч', s @ ..] => super::t40::n19093(s, out),
		['ш', s @ ..] => super::t40::n19094(s, out),
		_ => false,
	}
}

/// Words ending -знь.
pub(crate) fn n4663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4652);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18990);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18991);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18992);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 43338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 18993);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43339);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43340);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							super::push(out, 71649);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инь.
pub(crate) fn n4664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4653);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 18994);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 43341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43342);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43343);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 71650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 43344);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71651);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 43345);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71652);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43346);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 71653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 43347);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71654);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 71655);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96689);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115022);
											s.is_empty()
										}
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
					super::push(out, 43348);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71656);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43349);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71657);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 71658);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96691);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 115023);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126733);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18996);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43351);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43352);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71659);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96692);
									s.is_empty()
								}
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
			super::push(out, 18998);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 43353);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 71660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 18999);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43354);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 71661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 19000);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43355);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71662);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онь.
pub(crate) fn n4665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4654);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19001);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 19002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43356);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 71663);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96694);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115024);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 126734);
													match s {
														[] => true,
														['с', s @ ..] => super::t40::n135092(s, out),
														_ => false,
													}
												}
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
							super::push(out, 71664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 71665);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96696);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115025);
											s.is_empty()
										}
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
					super::push(out, 43357);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71666);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96697);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 115026);
											s.is_empty()
										}
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
					super::push(out, 43358);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96698);
									s.is_empty()
								}
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
			super::push(out, 19003);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43359);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43360);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43361);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71669);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96699);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 115027);
											s.is_empty()
										}
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
					super::push(out, 43362);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71670);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43363);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71671);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96701);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115028);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19005);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43364);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 71672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43365);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 71673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43366);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71674);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96702);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 115029);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 96703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 71675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43367);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71676);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 96704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115030);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43368);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 43369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71678);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96705);
									s.is_empty()
								}
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
			super::push(out, 19007);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19008);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43371);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43372);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 71679);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 71680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96706);
									s.is_empty()
								}
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
			super::push(out, 19009);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19010);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 43373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19011);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43374);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 71681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнь.
pub(crate) fn n4666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4655);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19012);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43375);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 43376);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 43377);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71682);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96707);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115031);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126735);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снь.
pub(crate) fn n4667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4656);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19013);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19014);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унь.
pub(crate) fn n4668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4657);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 19015);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 43380);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19016);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19017);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43382);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43383);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71685);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 43384);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71686);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 96709);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 115032);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19018);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43385);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71687);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 96710);
									s.is_empty()
								}
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
			super::push(out, 19019);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43386);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19020);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43387);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 19021);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 19022);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 43388);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71690);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96711);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 115033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынь.
pub(crate) fn n4669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4658);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19023);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43389);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71691);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 96712);
									s.is_empty()
								}
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
			super::push(out, 19024);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43390);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 71692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19025);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43391);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19026);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 43392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19027);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19028);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43394);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 43395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71696);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 71697);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96713);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115034);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126736);
													s.is_empty()
												}
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
							super::push(out, 71698);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96714);
									s.is_empty()
								}
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
			super::push(out, 19029);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -энь.
pub(crate) fn n4670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4659);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 19030);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 43397);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71699);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 96715);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 115035);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 126737);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнь.
pub(crate) fn n4671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4660);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19031);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 19032);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43398);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 71700);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 96716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янь.
pub(crate) fn n4672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4661);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19033);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43399);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71701);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96717);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 115036);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19035);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 43401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19036);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 43402);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 71703);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96718);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115037);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 126738);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19037);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43403);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 71704);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 96719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 43404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -переступь.
pub(crate) fn n128052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126739);
	s.is_empty()
}

/// Words ending -арь.
pub(crate) fn n4678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4667);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19051);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43422);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71718);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43423);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 43424);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71719);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 96726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43425);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43426);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 71721);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 71722);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 71723);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96727);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 71724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 43427);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 43428);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 71726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19052);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43429);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71727);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43430);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71728);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 96730);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 71729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 43431);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 71730);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 96731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 43432);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71731);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 71732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43433);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71733);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96733);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 115041);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126740);
													match s {
														[] => true,
														['е', s @ ..] => super::t40::n135093(s, out),
														_ => false,
													}
												}
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
							super::push(out, 71734);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96734);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 115042);
											s.is_empty()
										}
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
					super::push(out, 43434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71735);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 71736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96735);
									s.is_empty()
								}
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
			super::push(out, 19053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43435);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71737);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 71738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43437);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71739);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 96736);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 115043);
											s.is_empty()
										}
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
					super::push(out, 43438);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43439);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71741);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43440);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71742);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 96739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43442);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71744);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 43443);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 71745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t40::n19159(s, out),
		['з', s @ ..] => super::t40::n19160(s, out),
		['к', s @ ..] => super::t40::n19161(s, out),
		['л', s @ ..] => {
			super::push(out, 19057);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t40::n43888(s, out),
				['е', s @ ..] => super::t40::n43889(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t40::n19163(s, out),
		['н', s @ ..] => super::t40::n19164(s, out),
		['п', s @ ..] => super::t40::n19165(s, out),
		['р', s @ ..] => super::t40::n19166(s, out),
		['с', s @ ..] => super::t40::n19167(s, out),
		['т', s @ ..] => super::t40::n19168(s, out),
		['ф', s @ ..] => super::t40::n19169(s, out),
		['х', s @ ..] => super::t40::n19170(s, out),
		['ц', s @ ..] => super::t40::n19171(s, out),
		['ч', s @ ..] => super::t40::n19172(s, out),
		['ш', s @ ..] => super::t40::n19173(s, out),
		_ => false,
	}
}

/// The word недрочабрь.
pub(crate) fn n128064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126751);
	s.is_empty()
}

/// Words ending -ермодверь.
pub(crate) fn n128065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126752);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 133722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тицезверь.
pub(crate) fn n128066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126753);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 133723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -демлагерь.
pub(crate) fn n128067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126754);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 133724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льдъегерь.
pub(crate) fn n128068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126755);
	s.is_empty()
}

/// Words ending -серь.
pub(crate) fn n19181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19076);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -терь.
pub(crate) fn n19182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19077);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43522);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 71850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96814);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 96815);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115071);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 126756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 96816);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 115072);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 126757);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 133725);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43523);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71851);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96817);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115073);
									s.is_empty()
								}
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
			super::push(out, 43524);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71852);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 96818);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115074);
									s.is_empty()
								}
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
			super::push(out, 43525);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71853);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71854);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 43526);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 71855);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 96821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 43527);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 71856);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херь.
pub(crate) fn n19183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19078);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43528);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черь.
pub(crate) fn n19184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19079);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 43529);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 71857);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щерь.
pub(crate) fn n19185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19080);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 43530);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 71858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирь.
pub(crate) fn n4681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4670);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19081);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43531);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43532);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19082);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43533);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71861);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96823);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115076);
											s.is_empty()
										}
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
					super::push(out, 43534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19083);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43535);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71862);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 43536);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71863);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 96825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43537);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 71864);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96826);
									s.is_empty()
								}
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
			super::push(out, 19084);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43538);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 71865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19085);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43539);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71866);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96827);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115077);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19086);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43540);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 71867);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 71868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 43541);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 71869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19087);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43542);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 71870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 43543);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 71871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 19088);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43544);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71872);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96828);
									s.is_empty()
								}
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
			super::push(out, 19089);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43545);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71873);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96829);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115078);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орь.
pub(crate) fn n4682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4671);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19090);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43546);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19091);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 43547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19092);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19093);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43550);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 71877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43551);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 71878);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 43553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43554);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 43555);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 71879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прь.
pub(crate) fn n4683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4672);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19096);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43556);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ррь.
pub(crate) fn n4684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4673);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 19097);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 43557);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урь.
pub(crate) fn n4685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4674);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19098);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43558);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71880);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43560);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19100);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43561);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71882);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96833);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115079);
											s.is_empty()
										}
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
					super::push(out, 43562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хрь.
pub(crate) fn n4686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4675);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19101);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырь.
pub(crate) fn n4687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4676);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19102);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19103);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43565);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71883);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43566);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71884);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96835);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 115080);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126758);
													s.is_empty()
												}
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
							super::push(out, 71885);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 96836);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115081);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 96837);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 96838);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115082);
											s.is_empty()
										}
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
					super::push(out, 43567);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71886);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96839);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 115083);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 71887);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 71888);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43568);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 71889);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96842);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115084);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 71890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43569);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 71891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43570);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71892);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 71893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43571);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 71894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19105);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 43572);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 43573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19106);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 43574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19107);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43575);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 71895);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96844);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 115085);
											s.is_empty()
										}
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
					super::push(out, 43576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71896);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 71897);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 71898);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 71899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19108);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43577);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 43578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 71900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43579);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71901);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 43580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19109);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43581);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 71902);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 71903);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96846);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 115086);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 71904);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96847);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115087);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 71905);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96848);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 115088);
											s.is_empty()
										}
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
					super::push(out, 43582);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 43583);
					match s {
						[] => true,
						['а', s @ ..] => super::t40::n72657(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t40::n44008(s, out),
				['ш', s @ ..] => {
					super::push(out, 43585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => super::t40::n19215(s, out),
		_ => false,
	}
}

/// Words ending -ярь.
pub(crate) fn n4688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4677);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19111);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19112);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 43587);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71910);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 96855);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115093);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19113);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43588);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окись.
pub(crate) fn n44030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43606);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 71927);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 96861);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 115097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126763);
									s.is_empty()
								}
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
			super::push(out, 71928);
			s.is_empty()
		}
		['х', 'е', 'р', s @ ..] => {
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 96862);
					s.is_empty()
				}
				['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 96863);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пись.
pub(crate) fn n19230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19125);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43607);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 71929);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 43608);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 71930);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 96865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 71931);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 96866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 43609);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 71932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 96867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 71933);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96868);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115098);
									s.is_empty()
								}
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
			super::push(out, 43610);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43611);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71935);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96869);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 115099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 71936);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 71937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 96870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 71938);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 96871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 71939);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 96872);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 96873);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 96874);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115100);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126764);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 133728);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71940);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 96875);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115101);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 96876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 71941);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 96877);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 115102);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 115103);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 126765);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 133729);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n139280(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43612);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 71942);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 96878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 43613);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 71943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рись.
pub(crate) fn n19231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19126);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 43614);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 43615);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 71944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ось.
pub(crate) fn n4692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4681);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19127);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 19128);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19129);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43617);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19131);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43619);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 71946);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 96879);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 115104);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19132);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43620);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19133);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43621);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71948);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96880);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усь.
pub(crate) fn n4693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4682);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 19134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43623);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 71949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19136);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 43624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43625);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43626);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71950);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96881);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 115105);
											s.is_empty()
										}
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
					super::push(out, 43627);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 43628);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71952);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 43629);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 71953);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысь.
pub(crate) fn n4694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4683);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19138);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19140);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43631);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 43632);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71954);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 96885);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115106);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126766);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясь.
pub(crate) fn n4695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4684);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 19141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -н-кровать.
pub(crate) fn n128080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126767);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 133730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -о-кровать.
pub(crate) fn n128081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126768);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 133731);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 137857);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140321);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141796);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 142654);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -благодать.
pub(crate) fn n128082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126769);
	s.is_empty()
}

/// Words ending -лева-мать.
pub(crate) fn n128083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126770);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133732);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 137858);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гиня-мать.
pub(crate) fn n128084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126771);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133733);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 137859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еть.
pub(crate) fn n4697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4686);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43648);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43649);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 71965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 43650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19154);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 71966);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 71967);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96891);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115111);
											s.is_empty()
										}
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
					super::push(out, 43652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43653);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 71968);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 71969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19156);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43654);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 71970);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 96892);
									s.is_empty()
								}
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
			super::push(out, 19157);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43655);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 71971);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96893);
									s.is_empty()
								}
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
			super::push(out, 19158);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43656);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 43657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19159);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 43658);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71972);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 96894);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115112);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126772);
													match s {
														[] => true,
														['н', s @ ..] => super::t40::n135112(s, out),
														_ => false,
													}
												}
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
							super::push(out, 71973);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96895);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 115113);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126773);
													match s {
														[] => true,
														['р', s @ ..] => super::t40::n135113(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 126774);
													match s {
														[] => true,
														['е', s @ ..] => super::t40::n135114(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43659);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71974);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 96896);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 115114);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 126775);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43660);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 71975);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 96897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 43661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71976);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43662);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71977);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96899);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 115115);
											s.is_empty()
										}
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
					super::push(out, 43663);
					match s {
						[] => true,
						['р', s @ ..] => super::t40::n72730(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t40::n44088(s, out),
				['т', s @ ..] => super::t40::n44089(s, out),
				['ц', s @ ..] => super::t40::n44090(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t40::n19265(s, out),
		_ => false,
	}
}

/// Words ending -ить.
pub(crate) fn n4698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4687);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19161);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43670);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 71988);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96911);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115121);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19163);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43671);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 71989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19164);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43672);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лть.
pub(crate) fn n4699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4688);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19165);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 43673);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 71990);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 96912);
									s.is_empty()
								}
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
			super::push(out, 19166);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43674);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оть.
pub(crate) fn n4700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4689);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19168);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43676);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 71991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43677);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 71992);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 71993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19169);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43678);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 43679);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71995);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 71996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19170);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43680);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 71997);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96914);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 115122);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126781);
													s.is_empty()
												}
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
							super::push(out, 71998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 43681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43682);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 71999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96915);
									s.is_empty()
								}
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
			super::push(out, 19172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43683);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43684);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 72001);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43685);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72002);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 72003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19173);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43686);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43687);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72005);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96917);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', 'а', 'п', '\u{1}', s @ ..] => {
							super::push(out, 72006);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19174);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43688);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72007);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43689);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 43690);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72009);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рть.
pub(crate) fn n4701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4690);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19175);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43691);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72010);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 96920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72011);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 96921);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 115124);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126782);
													match s {
														[] => true,
														['к', s @ ..] => super::t40::n135116(s, out),
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
									super::push(out, 96922);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115125);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 126783);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 96923);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115126);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 126784);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t40::n135117(s, out),
														_ => false,
													}
												}
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
							super::push(out, 72012);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 96924);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 115127);
											s.is_empty()
										}
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
					super::push(out, 43692);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 43693);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96925);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 96926);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 115128);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 126785);
													match s {
														[] => true,
														['п', s @ ..] => super::t40::n135118(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72014);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 72015);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 43695);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72016);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 96928);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 115129);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19176);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19177);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43697);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сть.
pub(crate) fn n4702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4691);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19178);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 43698);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43699);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 96930);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 115130);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 126786);
													s.is_empty()
												}
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
							super::push(out, 72020);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96931);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 115131);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126787);
													match s {
														[] => true,
														['м', s @ ..] => super::t40::n135119(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', 'о', 'с', '\u{1}', s @ ..] => {
									super::push(out, 96932);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 96933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 72021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43700);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 43701);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 43702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72023);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 72024);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 96934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72025);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 96935);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 96936);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115132);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126788);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43703);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 72026);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 96937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 43704);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72027);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96938);
									match s {
										[] => true,
										['с', 'д', s @ ..] => {
											super::push(out, 115133);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126789);
													match s {
														[] => true,
														['м', s @ ..] => super::t40::n135120(s, out),
														_ => false,
													}
												}
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
							super::push(out, 72028);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 72029);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 96940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 72030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 96941);
									s.is_empty()
								}
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
			super::push(out, 19179);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43705);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72031);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 96942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72032);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 96943);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115134);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 126790);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 96944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 43706);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 43707);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43708);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72034);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 72035);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 72036);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 96945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43709);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 43710);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72037);
							match s {
								[] => true,
								['е', s @ ..] => super::t40::n97981(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t40::n44135(s, out),
				['т', s @ ..] => {
					super::push(out, 43712);
					s.is_empty()
				}
				['ч', s @ ..] => super::t40::n44137(s, out),
				['ш', s @ ..] => super::t40::n44138(s, out),
				['щ', s @ ..] => super::t40::n44139(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t40::n19285(s, out),
		['л', s @ ..] => super::t40::n19286(s, out),
		['о', s @ ..] => super::t40::n19287(s, out),
		['р', s @ ..] => super::t40::n19288(s, out),
		['у', s @ ..] => super::t40::n19289(s, out),
		['ы', s @ ..] => super::t40::n19290(s, out),
		['ю', s @ ..] => super::t40::n19291(s, out),
		['я', s @ ..] => super::t40::n19292(s, out),
		_ => false,
	}
}

/// Words ending -уть.
pub(crate) fn n4703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4692);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 19188);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19189);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 19190);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43743);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 72154);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 43744);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72155);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97197);
									s.is_empty()
								}
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
			super::push(out, 19191);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19192);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 19193);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43746);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фть.
pub(crate) fn n4704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4693);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19194);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19195);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43748);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72156);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 97198);
									s.is_empty()
								}
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
			super::push(out, 19196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыть.
pub(crate) fn n4705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4694);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19197);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43749);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19198);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19199);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43750);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72158);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97199);
									s.is_empty()
								}
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
			super::push(out, 19200);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19201);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43752);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72159);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97200);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 115630);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19202);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19203);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 43754);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72160);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лють.
pub(crate) fn n19309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19204);
	s.is_empty()
}

/// Words ending -ять.
pub(crate) fn n4707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4696);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 19205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19206);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43756);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43757);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72162);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 97202);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 115631);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127458);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19207);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43758);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72163);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлебопечь.
pub(crate) fn n128805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127459);
	s.is_empty()
}

/// Words ending -ектропечь.
pub(crate) fn n128806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127460);
	s.is_empty()
}

/// Words ending -заволочь.
pub(crate) fn n116884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115637);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 127461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сволочь.
pub(crate) fn n98253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97213);
	s.is_empty()
}

/// Words ending -толочь.
pub(crate) fn n72930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72176);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 97214);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115639);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 127462);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134477);
									s.is_empty()
								}
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
			super::push(out, 97215);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 115640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слочь.
pub(crate) fn n44196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43772);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 72177);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97216);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 115641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочь.
pub(crate) fn n19324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19219);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43773);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 72178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43774);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 72179);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ночь.
pub(crate) fn n19325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19220);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43775);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 43776);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72180);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 97217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115642);
									s.is_empty()
								}
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
			super::push(out, 43777);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 72181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97218);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 115643);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 127463);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчь.
pub(crate) fn n4714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4703);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19221);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учь.
pub(crate) fn n4715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4704);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19222);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терпомощь.
pub(crate) fn n128810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127464);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 134478);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 138504);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сихпомощь.
pub(crate) fn n128811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127465);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134479);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 138505);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суйфэньхэ.
pub(crate) fn n128813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127467);
	s.is_empty()
}

/// Words ending -юй-мао-цю.
pub(crate) fn n128814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127468);
	s.is_empty()
}

/// Words ending -юй-мао-чю.
pub(crate) fn n128815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127469);
	s.is_empty()
}

/// Words ending -ра-майнью.
pub(crate) fn n128816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127470);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 134480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овая.
pub(crate) fn n19429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19324);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 43860);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 72232);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 97243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 43861);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72233);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97244);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115659);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 127471);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43862);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 72234);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 97245);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 115660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 72235);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97246);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115661);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 127472);
											s.is_empty()
										}
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
					super::push(out, 72236);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97247);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115662);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 72237);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 72238);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97248);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 115663);
									s.is_empty()
								}
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
			super::push(out, 43863);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72239);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 43864);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 72240);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 97250);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115664);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 127473);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 134481);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72241);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97251);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 115665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 97252);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 115666);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 127474);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 43865);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 72242);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 97253);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 115667);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 127475);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134482);
													match s {
														[] => true,
														['б', s @ ..] => super::t40::n139941(s, out),
														_ => false,
													}
												}
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
							super::push(out, 97254);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 115668);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 127476);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134483);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 134484);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n139942(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 72243);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 72244);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 97255);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 115669);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 127477);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 134485);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 43867);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 72245);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 97256);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 115670);
									s.is_empty()
								}
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
			super::push(out, 43868);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 72246);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 97257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 72247);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 97258);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 115671);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t40::n73002(s, out),
				['у', s @ ..] => {
					super::push(out, 72249);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 97261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t40::n44293(s, out),
		['т', s @ ..] => super::t40::n44294(s, out),
		_ => false,
	}
}

/// Words ending -костлявая.
pub(crate) fn n128827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127481);
	s.is_empty()
}

/// Words ending -жая.
pub(crate) fn n4797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4786);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 19330);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43876);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72260);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97271);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115676);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 72261);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97272);
									s.is_empty()
								}
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
			super::push(out, 19331);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 43877);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72262);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97273);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115677);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

