use flexion::Path;

/// Words ending -упеть.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 2451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2452);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14396);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14397);
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2453);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7037);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7038);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7039);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14398);
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
					super::push(out, 7040);
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

/// Words ending -ыпеть.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2456);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7042);
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
		_ => false,
	}
}

/// The word поразогреть.
pub(crate) fn n65838(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тогреть.
pub(crate) fn n8523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7051);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22639);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30591);
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

/// Words ending -дреть.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 569);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2466);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2467);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14411);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22640);
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

/// Words ending -ереть.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 570);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2468);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14412);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14413);
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
		['м', s @ ..] => {
			super::push(out, 2469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7054);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7055);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14414);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7056);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14415);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14416);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22643);
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
				['и', s @ ..] => {
					super::push(out, 7058);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14417);
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
					super::push(out, 7059);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7060);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14419);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7062);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14420);
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
		['п', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7063);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7064);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14421);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 22646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14422);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22647);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22648);
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
				['в', s @ ..] => {
					super::push(out, 7065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7066);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14423);
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
					super::push(out, 7067);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14424);
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
					super::push(out, 7068);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14425);
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
					super::push(out, 7069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14426);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 14427);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 14428);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14429);
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
					super::push(out, 7070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14430);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22649);
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
					super::push(out, 7071);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14431);
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
					super::push(out, 7072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7073);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14432);
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
			super::push(out, 2471);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7074);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7075);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14433);
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
					super::push(out, 7076);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14434);
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
		['т', s @ ..] => super::t12::n2633(s, out),
		['х', s @ ..] => {
			super::push(out, 2473);
			match s {
				[] => true,
				['о', s @ ..] => super::t12::n8562(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t12::n2635(s, out),
		_ => false,
	}
}

/// Words ending -зреть.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2475);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2476);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7091);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7092);
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
			super::push(out, 2477);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7093);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14448);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22657);
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
							super::push(out, 14449);
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
				['ц', s @ ..] => {
					super::push(out, 7094);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14450);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30594);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 30595);
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
		['з', s @ ..] => {
			super::push(out, 2478);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7095);
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
		['и', s @ ..] => {
			super::push(out, 2479);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7096);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 14451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7097);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7098);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14452);
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
					super::push(out, 7099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14453);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22658);
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
				['м', s @ ..] => {
					super::push(out, 7100);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14454);
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
					super::push(out, 7101);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7102);
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
			super::push(out, 2481);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2482);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7104);
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

/// Words ending -иреть.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 2483);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7105);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7106);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7107);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14455);
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
		['м', s @ ..] => {
			super::push(out, 2484);
			match s {
				[] => true,
				['с', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7109);
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
		['ш', s @ ..] => {
			super::push(out, 2486);
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

/// Words ending -креть.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2487);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14456);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -мреть.
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ореть.
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 575);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2488);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7110);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7111);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22659);
									s.is_empty()
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
				['б', s @ ..] => {
					super::push(out, 7112);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22660);
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 7113);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 7114);
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							super::push(out, 14457);
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 14458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 7115);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7116);
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
				['с', '\u{1}', s @ ..] => {
					super::push(out, 7117);
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					super::push(out, 7118);
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					super::push(out, 7119);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7120);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14459);
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
		['д', s @ ..] => {
			super::push(out, 2489);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7121);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22661);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22662);
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
			super::push(out, 2490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7122);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7123);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22663);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22664);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22665);
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

/// Words ending -преть.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 576);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2491);
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
		['о', s @ ..] => {
			super::push(out, 2492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
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
				['з', s @ ..] => {
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
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2493);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7124);
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

/// Words ending -треть.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 577);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2494);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14461);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14462);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 14463);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 14464);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22666);
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
			super::push(out, 2495);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14465);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14466);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22667);
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
			super::push(out, 2496);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7127);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22668);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22669);
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
									super::push(out, 22670);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30596);
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
									super::push(out, 22671);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 22672);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22673);
									match s {
										[] => true,
										['р', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36254);
													match s {
														[] => true,
														['п', s @ ..] => super::t12::n76716(s, out),
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
									super::push(out, 22674);
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
												['е', 'н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 30597);
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
									super::push(out, 22675);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22676);
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
								['ы', s @ ..] => {
									super::push(out, 22677);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30598);
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2497);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t12::n8617(s, out),
				['о', s @ ..] => super::t12::n8618(s, out),
				['у', s @ ..] => super::t12::n8619(s, out),
				['ы', s @ ..] => super::t12::n8620(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t12::n2664(s, out),
		_ => false,
	}
}

/// Words ending -уреть.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2499);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7134);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7135);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7136);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7137);
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
		['м', 'х', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыреть.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7138);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7139);
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
				['о', s @ ..] => {
					super::push(out, 7140);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14474);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22680);
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
					super::push(out, 7141);
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

/// The word разбогатеть.
pub(crate) fn n65854(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оробогатеть.
pub(crate) fn n65855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36255);
	match s {
		[] => true,
		['к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обмохнатеть.
pub(crate) fn n65856(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -волосатеть.
pub(crate) fn n19702(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22685);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22686);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22687);
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

/// Words ending -хатеть.
pub(crate) fn n2685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2518);
	match s {
		[] => true,
		['ю', 'р', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14484);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14485);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14486);
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

/// Words ending -ететь.
pub(crate) fn n597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7164);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7165);
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
				['з', s @ ..] => {
					super::push(out, 7166);
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
								['н', '\u{1}', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7167);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7168);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14487);
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

/// Words ending -лтеть.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2520);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14488);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14489);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 14490);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 14491);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22688);
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
							super::push(out, 14492);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22689);
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
							super::push(out, 14493);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22690);
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
			super::push(out, 2521);
			match s {
				[] => true,
				['г', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отеть.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 589);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2522);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7169);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 14494);
							match s {
								[] => true,
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2523);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7170);
					match s {
						[] => true,
						['ы', 'ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14495);
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
		['к', s @ ..] => {
			super::push(out, 2524);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7171);
					match s {
						[] => true,
						['у', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22691);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22692);
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
		['л', s @ ..] => {
			super::push(out, 2525);
			match s {
				[] => true,
				['о', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14496);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14497);
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7172);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7173);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7174);
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
					super::push(out, 7175);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7176);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14498);
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
			super::push(out, 2527);
			match s {
				[] => true,
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14499);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14500);
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
		['х', s @ ..] => {
			super::push(out, 2528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7177);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7178);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14501);
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
					super::push(out, 7179);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7180);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7181);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14502);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14503);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22693);
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
					super::push(out, 7182);
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

/// Words ending -птеть.
pub(crate) fn n600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2529);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14504);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14505);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртеть.
pub(crate) fn n601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2530);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14506);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14507);
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
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14508);
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 14509);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30604);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36256);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76718(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
							super::push(out, 14510);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22694);
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
					super::push(out, 7184);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14511);
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
		_ => false,
	}
}

/// Words ending -стеть.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2531);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7185);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22695);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22696);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22697);
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
							super::push(out, 14513);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30605);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30606);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36257);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76719(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30607);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36258);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76720(s, out),
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
											super::push(out, 30608);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2532);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7186);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22698);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22699);
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
									super::push(out, 22700);
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22701);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30609);
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
					super::push(out, 7187);
					match s {
						[] => true,
						['и', 'в', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22702);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22703);
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
		['л', s @ ..] => {
			super::push(out, 2533);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7188);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22704);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22705);
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
		['о', s @ ..] => {
			super::push(out, 2534);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7189);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22706);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22707);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t12::n2702(s, out),
		['у', s @ ..] => super::t12::n2703(s, out),
		_ => false,
	}
}

/// Words ending -утеть.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 593);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2537);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2538);
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

/// Words ending -хтеть.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2539);
			match s {
				[] => true,
				['р', 'а', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14522);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14523);
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
		['е', 'р', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7194);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7195);
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
		['у', s @ ..] => {
			super::push(out, 2540);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7197);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14525);
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
			super::push(out, 2541);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14526);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14527);
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
		['я', s @ ..] => {
			super::push(out, 2542);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14528);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14529);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14530);
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

/// Words ending -ытеть.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2543);
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

/// Words ending -ютеть.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 596);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7198);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7199);
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

/// Words ending -ятеть.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 2545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word испохабить.
pub(crate) fn n65863(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word надбить.
pub(crate) fn n2765(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -одбить.
pub(crate) fn n2766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2583);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ебить.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2584);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7227);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14552);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30611);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 30612);
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
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7228);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7229);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14554);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30613);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30614);
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
					super::push(out, 7230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14555);
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
		['р', s @ ..] => {
			super::push(out, 2585);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7231);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 14557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22717);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22718);
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
							super::push(out, 14558);
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
						['т', s @ ..] => {
							super::push(out, 14559);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22719);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22720);
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
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22721);
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
									super::push(out, 22722);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30615);
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
					super::push(out, 7232);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14560);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36259);
													match s {
														[] => true,
														['л', s @ ..] => super::t12::n76721(s, out),
														_ => false,
													}
												}
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
							super::push(out, 14561);
							match s {
								[] => true,
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -збить.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2586);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
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
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ибить.
pub(crate) fn n637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2587);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7233);
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
		['ш', s @ ..] => {
			super::push(out, 2588);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7234);
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
				['о', s @ ..] => {
					super::push(out, 7235);
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
							super::push(out, 14562);
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
					super::push(out, 7236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7237);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14563);
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

/// Words ending -лбить.
pub(crate) fn n638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 628);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2589);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14564);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14565);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22724);
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
							super::push(out, 14566);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 14567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 14568);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22725);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30616);
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
									super::push(out, 22726);
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
						['з', s @ ..] => {
							super::push(out, 14569);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22727);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22728);
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
							super::push(out, 14570);
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
							super::push(out, 14571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 14572);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22729);
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
					super::push(out, 7238);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22730);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбить.
pub(crate) fn n639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2590);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14574);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14575);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 14576);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14577);
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
							super::push(out, 14578);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 14579);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22731);
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

/// Words ending -обить.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 630);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2591);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2592);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7239);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7240);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14580);
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
				['н', s @ ..] => {
					super::push(out, 7241);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14581);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30617);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30618);
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
					super::push(out, 7242);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14582);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22732);
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22733);
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
					super::push(out, 7243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7244);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14583);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22734);
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
				['ы', s @ ..] => {
					super::push(out, 7245);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14584);
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
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2593);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14585);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22735);
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
			super::push(out, 2594);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7247);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 14586);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22736);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22737);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30619);
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
				['е', s @ ..] => {
					super::push(out, 7248);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 14587);
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
					super::push(out, 7249);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14588);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14589);
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
			super::push(out, 2595);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14590);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14591);
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
				['з', s @ ..] => {
					super::push(out, 7251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14592);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14593);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14594);
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
						['ы', s @ ..] => {
							super::push(out, 14595);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22738);
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
		['п', s @ ..] => {
			super::push(out, 2596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t12::n2782(s, out),
		['с', s @ ..] => {
			super::push(out, 2598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7262);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7263);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14607);
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
					super::push(out, 7264);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14608);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22743);
									s.is_empty()
								}
								['с', s @ ..] => super::t12::n35013(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбить.
pub(crate) fn n641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 631);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7265);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14609);
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
				['щ', s @ ..] => {
					super::push(out, 7266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14610);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14611);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 14613);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22745);
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
			super::push(out, 2600);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 7267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14614);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14615);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22746);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 22747);
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
							super::push(out, 14616);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22748);
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
							super::push(out, 14617);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7268);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22750);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22751);
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
					super::push(out, 7269);
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

/// Words ending -сбить.
pub(crate) fn n642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
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

/// Words ending -тбить.
pub(crate) fn n643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 633);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2601);
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

/// Words ending -убить.
pub(crate) fn n644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2602);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7270);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7271);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14619);
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
					super::push(out, 7272);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7273);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7274);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14620);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 22752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14621);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14622);
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
					super::push(out, 7276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7277);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14623);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22753);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22754);
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
					super::push(out, 7278);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14624);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22755);
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
		['д', s @ ..] => {
			super::push(out, 2604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7279);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7280);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14625);
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
					super::push(out, 7281);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14626);
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
					super::push(out, 7282);
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
					super::push(out, 7283);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14627);
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
		['з', s @ ..] => {
			super::push(out, 2605);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7284);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14628);
							match s {
								[] => true,
								['а', 'к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2606);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7285);
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
		['л', s @ ..] => {
			super::push(out, 2607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7286);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14629);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22756);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22757);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22758);
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
				['г', s @ ..] => {
					super::push(out, 7287);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14630);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14631);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22759);
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
							super::push(out, 14632);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22760);
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
					super::push(out, 7289);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22761);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22762);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30623);
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
								['и', s @ ..] => {
									super::push(out, 22763);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30624);
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
									super::push(out, 22764);
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
			super::push(out, 2608);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7290);
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
		['р', s @ ..] => super::t12::n2796(s, out),
		_ => false,
	}
}

/// Words ending -ыбить.
pub(crate) fn n645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2610);
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
		['д', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7305);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7306);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14646);
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
		['з', s @ ..] => {
			super::push(out, 2612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2614);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбить.
pub(crate) fn n646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7309);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 7310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7311);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 14647);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22777);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 22778);
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
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7312);
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14649);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22779);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябить.
pub(crate) fn n647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 637);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2616);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14650);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14651);
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
		['р', s @ ..] => {
			super::push(out, 2617);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7314);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7315);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7316);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7317);
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

/// Words ending -авить.
pub(crate) fn n649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 639);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2618);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7318);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22780);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22781);
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
						['н', s @ ..] => {
							super::push(out, 14652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7319);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14653);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 14654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7320);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
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
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14655);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 14656);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22782);
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14657);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22783);
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
					super::push(out, 7321);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 14658);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22784);
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
			super::push(out, 2619);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7323);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22785);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
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
			super::push(out, 2620);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7324);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7325);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2621);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7326);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7327);
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 7328);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7329);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 7330);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', s @ ..] => {
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
				['у', s @ ..] => {
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
					super::push(out, 7331);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t12::n35088(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t12::n2809(s, out),
		['з', s @ ..] => super::t12::n2810(s, out),
		['к', s @ ..] => super::t12::n2811(s, out),
		['л', s @ ..] => super::t12::n2812(s, out),
		['н', s @ ..] => super::t12::n2813(s, out),
		['р', s @ ..] => super::t12::n2814(s, out),
		['с', s @ ..] => super::t12::n2815(s, out),
		['т', s @ ..] => super::t12::n2816(s, out),
		['ч', s @ ..] => super::t12::n2817(s, out),
		['ш', s @ ..] => super::t12::n2818(s, out),
		_ => false,
	}
}

/// The word передешевить.
pub(crate) fn n65898(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word продешевить.
pub(crate) fn n65899(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -трезвить.
pub(crate) fn n20091(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22821);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22822);
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
		['ы', s @ ..] => {
			super::push(out, 22823);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30651);
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

/// The word извить.
pub(crate) fn n8957(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -язвить.
pub(crate) fn n2831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2643);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7358);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7359);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14707);
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
			super::push(out, 7360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 7361);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14708);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
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
		_ => false,
	}
}

/// Words ending -ивить.
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 645);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2644);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7362);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14709);
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7363);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 7364);
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
		['ж', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7365);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7366);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14710);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14711);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22824);
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
					super::push(out, 7368);
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2647);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7369);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14712);
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
					super::push(out, 7370);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14713);
							match s {
								[] => true,
								['а', 'ч', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30652);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30653);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 36265);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76727(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2648);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14714);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14715);
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
							super::push(out, 14716);
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
						['ы', s @ ..] => {
							super::push(out, 14717);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22825);
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
					super::push(out, 7372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 2649);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7373);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22826);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22827);
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
		['т', s @ ..] => {
			super::push(out, 2650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7374);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14718);
							match s {
								[] => true,
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 14719);
							match s {
								[] => true,
								['р', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30654);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36266);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76728(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7375);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2651);
			match s {
				[] => true,
				['е', s @ ..] => super::t12::n8983(s, out),
				['р', s @ ..] => super::t12::n8984(s, out),
				['ь', s @ ..] => super::t12::n8985(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвить.
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2652);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14724);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14725);
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
						['з', s @ ..] => {
							super::push(out, 14726);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 22828);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30660);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14727);
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 14728);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22829);
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

/// Words ending -овить.
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 647);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2653);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2655);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7380);
					match s {
						[] => true,
						['е', 'ц', s @ ..] => {
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
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
		['л', s @ ..] => {
			super::push(out, 2656);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7381);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7382);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 14729);
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
					super::push(out, 7383);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14730);
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
					super::push(out, 7384);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14731);
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
					super::push(out, 7385);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14732);
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
					super::push(out, 7386);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14733);
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
				['л', s @ ..] => {
					super::push(out, 7387);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14734);
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
				['о', s @ ..] => {
					super::push(out, 7388);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14735);
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
					super::push(out, 7389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14736);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14737);
							match s {
								[] => true,
								['н', 'р', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 22830);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 30661);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 30662);
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
						['о', s @ ..] => super::t12::n20145(s, out),
						['у', s @ ..] => {
							super::push(out, 14739);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22837);
									match s {
										[] => true,
										['о', s @ ..] => super::t12::n51685(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t12::n8998(s, out),
				['у', s @ ..] => {
					super::push(out, 7391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t12::n20148(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t12::n9000(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t12::n2845(s, out),
		['п', s @ ..] => super::t12::n2846(s, out),
		['р', s @ ..] => super::t12::n2847(s, out),
		['с', s @ ..] => super::t12::n2848(s, out),
		['т', s @ ..] => super::t13::n2849(s, out),
		_ => false,
	}
}

/// Words ending -рвить.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 648);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2662);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7409);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 7410);
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

/// Words ending -свить.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -твить.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2663);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14754);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14755);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
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
		['и', s @ ..] => {
			super::push(out, 2664);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7411);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14756);
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
		['о', s @ ..] => {
			super::push(out, 2665);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2666);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7413);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22859);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22861);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 22862);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30678);
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
			super::push(out, 2667);
			match s {
				[] => true,
				['в', 'у', 'ч', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7414);
					match s {
						[] => true,
						['в', 'з', 'ы', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', 'а', 'к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['щ', s @ ..] => {
							match s {
								[] => true,
								['е', 'в', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', 'с', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7415);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14757);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22863);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30679);
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
									super::push(out, 22864);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30680);
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
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7416);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14758);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22865);
									match s {
										[] => true,
										['у', 'с', 'о', 'г', s @ ..] => {
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36272);
													match s {
														[] => true,
														['а', 'р', '\u{1}', s @ ..] => super::t13::n76737(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 36273);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76738(s, out),
														_ => false,
													}
												}
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
							super::push(out, 14759);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30681);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30682);
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

/// Words ending -увить.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2668);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2669);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7417);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22866);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22867);
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

/// Words ending -ывить.
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2670);
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

/// Words ending -явить.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2671);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2672);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2673);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7419);
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
		['л', s @ ..] => {
			super::push(out, 2674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7420);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22868);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22869);
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
					super::push(out, 7421);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14760);
							match s {
								[] => true,
								['е', 'ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30683);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30684);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30685);
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
				['к', s @ ..] => {
					super::push(out, 7422);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14761);
							match s {
								[] => true,
								['и', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 7423);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14762);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30686);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 30687);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2675);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 7424);
					match s {
						[] => true,
						['л', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22870);
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
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
			super::push(out, 2676);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7425);
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
		['р', s @ ..] => {
			super::push(out, 2677);
			match s {
				[] => true,
				['д', 'у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14763);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14764);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 14765);
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
				['ы', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14766);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14767);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14768);
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
		['ъ', s @ ..] => {
			super::push(out, 2678);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7426);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7427);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7428);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 2679);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7429);
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

/// Words ending -гадить.
pub(crate) fn n2870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2681);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7434);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7435);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14774);
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
			super::push(out, 7436);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14775);
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
			super::push(out, 7437);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14776);
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
			super::push(out, 7438);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14777);
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
			super::push(out, 7439);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14778);
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
			super::push(out, 7440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14779);
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
			super::push(out, 7441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кадить.
pub(crate) fn n2872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2683);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7443);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7444);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7445);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7446);
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

/// Words ending -ладить.
pub(crate) fn n2873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2684);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7447);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7448);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 7449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14782);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14783);
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
				['з', s @ ..] => {
					super::push(out, 14784);
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
					super::push(out, 14785);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22871);
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 22872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 22873);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30688);
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
					super::push(out, 14787);
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
				['т', s @ ..] => {
					super::push(out, 14788);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 22874);
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
					super::push(out, 14789);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 22875);
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 7450);
			match s {
				[] => true,
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
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7451);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7452);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14790);
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
			super::push(out, 7453);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14791);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14792);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14793);
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
			super::push(out, 7454);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 7456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14794);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 22876);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30689);
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

/// Words ending -мадить.
pub(crate) fn n2874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2685);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7457);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22877);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -радить.
pub(crate) fn n2875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2686);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14796);
					match s {
						[] => true,
						['н', s @ ..] => {
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14797);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 22878);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30690);
									s.is_empty()
								}
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
			super::push(out, 7459);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7460);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 14799);
					match s {
						[] => true,
						['и', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30691);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30692);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 36274);
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

/// Words ending -садить.
pub(crate) fn n2876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2687);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7461);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7462);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 7463);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 7464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7465);
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
			super::push(out, 7466);
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
			super::push(out, 7467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22879);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 22880);
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
				['п', s @ ..] => {
					super::push(out, 14802);
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
				['р', s @ ..] => {
					super::push(out, 14803);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 22881);
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
			super::push(out, 7468);
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
		['у', s @ ..] => {
			super::push(out, 7469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7470);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14804);
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

/// Words ending -чадить.
pub(crate) fn n2877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2688);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7471);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7472);
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
			super::push(out, 7473);
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

/// Words ending -шадить.
pub(crate) fn n2878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2689);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7474);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14805);
					match s {
						[] => true,
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щадить.
pub(crate) fn n2879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2690);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7475);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7476);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14806);
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

/// Words ending -бдить.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2691);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7477);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едить.
pub(crate) fn n666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2692);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7478);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14807);
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
					super::push(out, 7479);
					match s {
						[] => true,
						['п', s @ ..] => {
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
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7480);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2693);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14808);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14809);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22882);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22883);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30693);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30694);
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
						['д', s @ ..] => {
							super::push(out, 14810);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22884);
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
							super::push(out, 14811);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22885);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 22886);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 22887);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30695);
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
									super::push(out, 22888);
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
							super::push(out, 14812);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22889);
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
							super::push(out, 14813);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22890);
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
							super::push(out, 14814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 14815);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22891);
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
		['п', s @ ..] => {
			super::push(out, 2694);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7482);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14816);
							match s {
								[] => true,
								['о', 'л', 'е', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7483);
					s.is_empty()
				}
				['б', s @ ..] => super::t13::n9103(s, out),
				['в', s @ ..] => super::t13::n9104(s, out),
				['д', s @ ..] => {
					super::push(out, 7486);
					match s {
						[] => true,
						['о', s @ ..] => super::t13::n20290(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t13::n9106(s, out),
				['з', s @ ..] => super::t13::n9107(s, out),
				['о', s @ ..] => super::t13::n9108(s, out),
				['п', s @ ..] => super::t13::n9109(s, out),
				['у', s @ ..] => {
					super::push(out, 7491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t13::n20300(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t13::n9111(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t13::n2885(s, out),
		['ц', s @ ..] => super::t13::n2886(s, out),
		_ => false,
	}
}

/// Words ending -ждить.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2698);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14835);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14836);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22904);
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
			super::push(out, 2699);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14837);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14838);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22905);
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
			super::push(out, 2700);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 7501);
					match s {
						[] => true,
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

/// Words ending -здить.
pub(crate) fn n668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2701);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7502);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14840);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 22906);
									s.is_empty()
								}
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
			super::push(out, 2702);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7503);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7504);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 14841);
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
					super::push(out, 7505);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22907);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22908);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30698);
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
				['е', s @ ..] => {
					super::push(out, 7506);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7507);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7508);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14843);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 22909);
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
					super::push(out, 7509);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14844);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 14845);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14846);
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
				['у', s @ ..] => {
					super::push(out, 7510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 7511);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14847);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 14848);
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
						['с', s @ ..] => {
							super::push(out, 14849);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14850);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22910);
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
					super::push(out, 7512);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14851);
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
		['и', s @ ..] => {
			super::push(out, 2703);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7513);
					match s {
						[] => true,
						['а', 'т', 'и', 'п', 'а', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 7514);
					match s {
						[] => true,
						['у', 'м', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22911);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 22912);
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
					super::push(out, 7515);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14852);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14853);
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
						['и', s @ ..] => {
							super::push(out, 14854);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14855);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22913);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22914);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30699);
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
						['с', s @ ..] => {
							super::push(out, 14856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => super::t13::n35416(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t13::n20343(s, out),
						['у', '\u{1}', s @ ..] => super::t13::n20344(s, out),
						['ы', s @ ..] => super::t13::n20345(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t13::n2893(s, out),
		['у', s @ ..] => super::t13::n2894(s, out),
		['ю', s @ ..] => super::t13::n2895(s, out),
		_ => false,
	}
}

/// Words ending -идить.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 2707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7521);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7522);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14862);
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
				['с', s @ ..] => {
					super::push(out, 7523);
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
			super::push(out, 2708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2709);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 2710);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7525);
					match s {
						[] => true,
						['н', 'е', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22927);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22928);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30704);
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

/// Words ending -йдить.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2711);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7526);
					match s {
						[] => true,
						['г', 'п', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22929);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22930);
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
		_ => false,
	}
}

/// Words ending -лдить.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2712);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7527);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14863);
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
				['ф', s @ ..] => {
					super::push(out, 7528);
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
			super::push(out, 2713);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 7529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14864);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14865);
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

/// Words ending -ндить.
pub(crate) fn n672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 662);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2714);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 7531);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14866);
							match s {
								[] => true,
								['у', 'а', 'р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2715);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7532);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14867);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 22931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 14868);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22932);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22933);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30705);
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
									super::push(out, 22934);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30706);
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
		['и', s @ ..] => {
			super::push(out, 2716);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7533);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14869);
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
				['р', s @ ..] => {
					super::push(out, 7534);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14870);
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
			super::push(out, 2717);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14871);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14872);
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
				['л', s @ ..] => {
					super::push(out, 7536);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14873);
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
			super::push(out, 2718);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7537);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22935);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22936);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30707);
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

/// Words ending -одить.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 663);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2719);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7538);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22937);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 22938);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30708);
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
		['в', s @ ..] => {
			super::push(out, 2720);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7539);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7540);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22939);
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22940);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30709);
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
							super::push(out, 14875);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7541);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14876);
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
					super::push(out, 7542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7543);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14878);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14879);
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
					super::push(out, 7544);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 14880);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14881);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22943);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36276);
													s.is_empty()
												}
												['а', s @ ..] => super::t13::n65969(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t13::n20378(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t13::n9171(s, out),
				['и', s @ ..] => super::t13::n9172(s, out),
				['о', s @ ..] => super::t13::n9173(s, out),
				['с', s @ ..] => super::t13::n9174(s, out),
				['т', s @ ..] => super::t13::n9175(s, out),
				['у', s @ ..] => super::t13::n9176(s, out),
				['ы', s @ ..] => super::t13::n9177(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t13::n2910(s, out),
		['к', s @ ..] => super::t13::n2911(s, out),
		['л', s @ ..] => super::t13::n2912(s, out),
		['р', s @ ..] => super::t13::n2913(s, out),
		['х', s @ ..] => super::t13::n2914(s, out),
		_ => false,
	}
}

/// Words ending -рдить.
pub(crate) fn n674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 664);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2726);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7582);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14951);
							match s {
								[] => true,
								['о', 'ф', 'т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2727);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7583);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14952);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23025);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23026);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30756);
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
								['д', s @ ..] => {
									super::push(out, 23027);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30757);
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
									super::push(out, 23028);
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
								['у', s @ ..] => {
									super::push(out, 23029);
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
									super::push(out, 23030);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30758);
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
				['м', s @ ..] => {
					super::push(out, 7584);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14953);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23031);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23032);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30759);
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
				['с', s @ ..] => {
					super::push(out, 7585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14954);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 23033);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 30760);
											match s {
												[] => true,
												['м', 'у', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 23034);
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
							super::push(out, 14956);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23035);
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
			super::push(out, 2728);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7586);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14957);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 23036);
									match s {
										[] => true,
										['е', 'к', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14958);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23037);
									match s {
										[] => true,
										['н', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7587);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14959);
							match s {
								[] => true,
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
		_ => false,
	}
}

/// Words ending -удить.
pub(crate) fn n675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 665);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2729);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2730);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7588);
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
			super::push(out, 2731);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7589);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7590);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7591);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14960);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23038);
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
							super::push(out, 14961);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14962);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23039);
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
				['о', s @ ..] => {
					super::push(out, 7592);
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
				['с', s @ ..] => {
					super::push(out, 7593);
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
			super::push(out, 2732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7594);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7595);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7596);
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
					super::push(out, 7597);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14963);
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
			super::push(out, 2733);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7598);
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
			super::push(out, 2734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7599);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7600);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14964);
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
				['з', s @ ..] => {
					super::push(out, 7601);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14965);
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
				['ы', s @ ..] => {
					super::push(out, 7602);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14966);
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
		['к', s @ ..] => {
			super::push(out, 2735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7603);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7604);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14967);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30761);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30762);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 36295);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76764(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7605);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14968);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30763);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30764);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 36296);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76765(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30765);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 30766);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 36297);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76766(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t13::n2925(s, out),
		['м', s @ ..] => {
			super::push(out, 2737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2738);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7615);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7616);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14982);
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
				['и', s @ ..] => super::t13::n9244(s, out),
				['о', s @ ..] => {
					super::push(out, 7618);
					match s {
						[] => true,
						['п', s @ ..] => super::t13::n20501(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t13::n9246(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t13::n2928(s, out),
		['п', s @ ..] => super::t13::n2929(s, out),
		['р', s @ ..] => super::t13::n2930(s, out),
		['с', s @ ..] => super::t13::n2931(s, out),
		['т', s @ ..] => super::t13::n2932(s, out),
		['х', s @ ..] => super::t13::n2933(s, out),
		['ч', s @ ..] => super::t13::n2934(s, out),
		['ы', s @ ..] => super::t13::n2935(s, out),
		_ => false,
	}
}

/// Words ending -ыдить.
pub(crate) fn n676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 666);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 2747);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15017);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 15018);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15019);
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

/// Words ending -людить.
pub(crate) fn n2937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2748);
	s.is_empty()
}

/// Words ending -ядить.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 668);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2749);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7644);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7645);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23062);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 23063);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30773);
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
									super::push(out, 23064);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23065);
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 36301);
													match s {
														[] => true,
														['н', '\u{1}', s @ ..] => super::t13::n76770(s, out),
														_ => false,
													}
												}
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
							super::push(out, 15021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23066);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23067);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30774);
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
									super::push(out, 23068);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30775);
											match s {
												[] => true,
												['п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76771(s, out),
														['о', 'п', '\u{1}', s @ ..] => super::t13::n76772(s, out),
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
									super::push(out, 23069);
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
				['б', s @ ..] => {
					super::push(out, 7646);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15022);
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
					super::push(out, 7647);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15023);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30776);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36302);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76773(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7648);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 15024);
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
					super::push(out, 7649);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 23070);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 23071);
									match s {
										[] => true,
										['д', 'е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7650);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15025);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23072);
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
					super::push(out, 7651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7652);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15026);
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
					super::push(out, 7653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7654);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15027);
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

/// The word отакелажить.
pub(crate) fn n66041(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -виражить.
pub(crate) fn n20590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15043);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23075);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30779);
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

/// Words ending -тиражить.
pub(crate) fn n20591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15044);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оражить.
pub(crate) fn n9318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7678);
	match s {
		[] => true,
		['д', 'у', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23077);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 23078);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уражить.
pub(crate) fn n9319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7679);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 15045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23079);
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23080);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бжить.
pub(crate) fn n687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2762);
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

/// Words ending -джить.
pub(crate) fn n688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 2763);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7680);
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
		['о', s @ ..] => {
			super::push(out, 2764);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7682);
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

/// Words ending -ежить.
pub(crate) fn n689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2765);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2766);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15046);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 2767);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2768);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7685);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 15048);
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
		['м', s @ ..] => {
			super::push(out, 2769);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7686);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7687);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 7688);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7689);
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
		['н', s @ ..] => {
			super::push(out, 2770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7690);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7691);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 15049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23081);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23082);
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
				['е', s @ ..] => {
					super::push(out, 7692);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15050);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23083);
									match s {
										[] => true,
										['е', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7693);
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
					super::push(out, 7694);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15051);
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
					super::push(out, 7695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15052);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15053);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23084);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30780);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 23085);
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
							super::push(out, 15054);
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
			super::push(out, 2771);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7696);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23086);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23087);
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
					super::push(out, 7697);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15055);
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
					super::push(out, 7698);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23088);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23089);
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
			super::push(out, 2772);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7699);
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

/// Words ending -зжить.
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2773);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7700);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 15056);
							match s {
								[] => true,
								['р', 'д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7701);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23090);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23091);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30781);
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
									super::push(out, 23092);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30782);
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
		['и', s @ ..] => {
			super::push(out, 2774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2775);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15057);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15058);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 15059);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15060);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23093);
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

/// Words ending -ижить.
pub(crate) fn n691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 680);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2776);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15061);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 15062);
							match s {
								[] => true,
								['е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2777);
			match s {
				[] => true,
				['б', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2778);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7704);
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
			super::push(out, 2779);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7705);
					match s {
						[] => true,
						['н', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лжить.
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 681);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2780);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15063);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15064);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23094);
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

/// Words ending -мжить.
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 682);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2781);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7706);
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

/// Words ending -нжить.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 683);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2782);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7707);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожить.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 684);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2783);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 2784);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7708);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7710);
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
			super::push(out, 2785);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7711);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23095);
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
									super::push(out, 23096);
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
			super::push(out, 2786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2787);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7713);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7714);
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
				['б', s @ ..] => {
					super::push(out, 7715);
					match s {
						[] => true,
						['о', s @ ..] => {
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
				['в', s @ ..] => {
					super::push(out, 7716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15066);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23097);
									match s {
										[] => true,
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7717);
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
				['з', s @ ..] => {
					super::push(out, 7718);
					match s {
						[] => true,
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
								['н', '\u{1}', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7719);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15067);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 23098);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30783);
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
						['д', s @ ..] => {
							super::push(out, 15068);
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
						['п', s @ ..] => {
							super::push(out, 15069);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23099);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30784);
											match s {
												[] => true,
												['и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 30785);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 36303);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t13::n76774(s, out),
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
									super::push(out, 23100);
									match s {
										[] => true,
										['а', 'р', s @ ..] => {
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
								_ => false,
							}
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
				['у', s @ ..] => super::t13::n9369(s, out),
				['ы', s @ ..] => super::t13::n9370(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t13::n2978(s, out),
		['н', s @ ..] => super::t13::n2979(s, out),
		['п', s @ ..] => super::t13::n2980(s, out),
		['р', s @ ..] => super::t13::n2981(s, out),
		['т', s @ ..] => super::t13::n2982(s, out),
		_ => false,
	}
}

/// Words ending -ржить.
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 685);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2793);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15080);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15081);
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

/// Words ending -сжить.
pub(crate) fn n697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 686);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2794);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тжить.
pub(crate) fn n698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 687);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2795);
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

/// Words ending -ужить.
pub(crate) fn n699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 688);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2796);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7733);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23119);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 23120);
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
		['л', s @ ..] => {
			super::push(out, 2797);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7734);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15083);
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
					super::push(out, 7735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15084);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15085);
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
							super::push(out, 15086);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23121);
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
							super::push(out, 15087);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23122);
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
							super::push(out, 15088);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 15089);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23123);
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
							super::push(out, 15090);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23124);
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
									super::push(out, 23125);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 15091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 15092);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23126);
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
			super::push(out, 2798);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7736);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15094);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15095);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 23127);
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
							super::push(out, 15096);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23128);
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
							super::push(out, 15097);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23129);
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
						['и', s @ ..] => super::t13::n20688(s, out),
						['о', s @ ..] => super::t13::n20689(s, out),
						['с', s @ ..] => super::t13::n20690(s, out),
						['у', s @ ..] => super::t13::n20691(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t13::n9390(s, out),
				['о', s @ ..] => super::t13::n9391(s, out),
				['п', s @ ..] => super::t13::n9392(s, out),
				['т', s @ ..] => super::t13::n9393(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t13::n2989(s, out),
		_ => false,
	}
}

/// Words ending -ыжить.
pub(crate) fn n700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 689);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2800);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7747);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15117);
							match s {
								[] => true,
								['к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7748);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15118);
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
			super::push(out, 2802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7749);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7750);
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2803);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7751);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15119);
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
					super::push(out, 7752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15120);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15121);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 23146);
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
							super::push(out, 15122);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30800);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 30801);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 30802);
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
						['т', s @ ..] => {
							super::push(out, 15123);
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
				['у', s @ ..] => {
					super::push(out, 7753);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23147);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30803);
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
									super::push(out, 23148);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 23149);
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
			super::push(out, 2804);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15124);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 15125);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15126);
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

/// Words ending -южить.
pub(crate) fn n701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 690);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7754);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7755);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 15127);
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
					super::push(out, 7756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7757);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15128);
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
			super::push(out, 2806);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15129);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15130);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15131);
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 15132);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23150);
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
			super::push(out, 2807);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15133);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 15134);
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
		['ь', s @ ..] => {
			super::push(out, 2808);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15135);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15136);
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

/// Words ending -яжить.
pub(crate) fn n702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 691);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2809);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7759);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 15137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 15138);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30804);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36307);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n76777(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2810);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7760);
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2811);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15139);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 15140);
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
					super::push(out, 7761);
					match s {
						[] => true,
						['о', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2812);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7763);
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
			super::push(out, 2813);
			match s {
				[] => true,
				['у', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15141);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 15142);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 15143);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23151);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 23152);
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
							super::push(out, 15144);
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
		_ => false,
	}
}

/// Words ending -лазить.
pub(crate) fn n3006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7770);
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
			super::push(out, 7771);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15148);
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
			super::push(out, 7772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 7773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15149);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15150);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 23157);
							match s {
								[] => true,
								['е', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15151);
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
			super::push(out, 7774);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7775);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 7776);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15152);
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
			super::push(out, 7777);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 15153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 15154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 7779);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7780);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23159);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23160);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30805);
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

