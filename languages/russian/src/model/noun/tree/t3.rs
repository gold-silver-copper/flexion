use flexion::Path;

/// Words ending -те.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1562);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8544);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27024);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54328);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82599);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 105326);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8545);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 8546);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27025);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54329);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 82600);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 54330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 27026);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 54331);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82601);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 105327);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8548);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27027);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54332);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 82602);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 105328);
													match s {
														[] => true,
														['е', s @ ..] => super::t14::n122131(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8549);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1563);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8550);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27029);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 54333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 8551);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27030);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 54334);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82603);
											s.is_empty()
										}
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
					super::push(out, 8553);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27031);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 54335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 8554);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27032);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54336);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 82604);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1564);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8556);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27034);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54337);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 82605);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27035);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 54338);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 82606);
											s.is_empty()
										}
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
					super::push(out, 8557);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27036);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8558);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27037);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 54340);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 82607);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 105329);
													match s {
														[] => true,
														['д', s @ ..] => super::t14::n122132(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27038);
							s.is_empty()
						}
						['д', s @ ..] => super::t14::n27226(s, out),
						['к', s @ ..] => {
							super::push(out, 27040);
							match s {
								[] => true,
								['и', s @ ..] => super::t14::n54901(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t14::n8586(s, out),
				['о', s @ ..] => super::t14::n8587(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t14::n1570(s, out),
		['р', s @ ..] => super::t14::n1571(s, out),
		['с', s @ ..] => super::t14::n1572(s, out),
		['т', s @ ..] => super::t14::n1573(s, out),
		['у', s @ ..] => super::t14::n1574(s, out),
		['ш', s @ ..] => super::t14::n1575(s, out),
		['ь', s @ ..] => super::t14::n1576(s, out),
		['э', s @ ..] => super::t14::n1577(s, out),
		['ю', s @ ..] => super::t14::n1578(s, out),
		_ => false,
	}
}

/// Words ending -фе.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8574);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8575);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27056);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8576);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27057);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54355);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 54356);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82618);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 105337);
													match s {
														[] => true,
														['ч', s @ ..] => super::t14::n122137(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27058);
							match s {
								[] => true,
								['с', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 54357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8577);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27059);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 54358);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 82619);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1576);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27060);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 54359);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 82620);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 105338);
													match s {
														[] => true,
														['к', s @ ..] => super::t14::n122138(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8579);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27061);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 54360);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 82621);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 105339);
													match s {
														[] => true,
														['-', s @ ..] => super::t14::n122139(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1577);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8580);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27062);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54361);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 82622);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 105340);
													match s {
														[] => true,
														['л', s @ ..] => super::t14::n122140(s, out),
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
									super::push(out, 54362);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 82623);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 105341);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 54363);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82624);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 105342);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1578);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8581);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8582);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 27064);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 54364);
									s.is_empty()
								}
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
			super::push(out, 1579);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8583);
					match s {
						[] => true,
						['в', s @ ..] => super::t14::n27252(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хе.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8585);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27066);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54367);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82627);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1582);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8586);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 27067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1583);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 8587);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27068);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -це.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1584);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8588);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 27069);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 54369);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 82628);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 105345);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n122142(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27070);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54370);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 82629);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 105346);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1585);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8589);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 27071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8590);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27072);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 54371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 8591);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27073);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 54372);
									s.is_empty()
								}
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
			super::push(out, 1586);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 8592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1587);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27075);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54373);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82630);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 105347);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n122143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8594);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 8595);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27076);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54374);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 82631);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 105348);
													match s {
														[] => true,
														['н', s @ ..] => super::t14::n122144(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27077);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54375);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 82632);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 105349);
													match s {
														[] => true,
														['л', s @ ..] => super::t14::n122145(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 82633);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 105350);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n122146(s, out),
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
									super::push(out, 54376);
									s.is_empty()
								}
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
			super::push(out, 1588);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 8596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8597);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27079);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54377);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 82634);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', 'у', 'г', '\u{1}', s @ ..] => {
							super::push(out, 27080);
							s.is_empty()
						}
						['т', s @ ..] => super::t14::n27268(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t14::n8624(s, out),
				['о', s @ ..] => super::t14::n8625(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t14::n1594(s, out),
		['т', s @ ..] => super::t14::n1595(s, out),
		['ь', s @ ..] => super::t14::n1596(s, out),
		_ => false,
	}
}

/// Words ending -че.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8606);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27097);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 54408);
									s.is_empty()
								}
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
			super::push(out, 1594);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8608);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27099);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54409);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82660);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1595);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8609);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27100);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54410);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 82661);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 105362);
													s.is_empty()
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 82662);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 105363);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1596);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1597);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8611);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27101);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 54411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8612);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ше.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1598);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8613);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8614);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8615);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 27103);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54412);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 82663);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 105364);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8616);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8617);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27104);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 54413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27105);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54414);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 82664);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1599);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8618);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8619);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27107);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54415);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 82665);
											s.is_empty()
										}
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
					super::push(out, 8620);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1600);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8621);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27109);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 54416);
									s.is_empty()
								}
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
			super::push(out, 1601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8622);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27110);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54417);
									s.is_empty()
								}
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
			super::push(out, 1602);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8623);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8624);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 8625);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27112);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 54418);
									s.is_empty()
								}
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
			super::push(out, 1603);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8626);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27113);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 54419);
									s.is_empty()
								}
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
			super::push(out, 1604);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8627);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 8628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ще.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['и', s @ ..] => super::t14::n1609(s, out),
		_ => false,
	}
}

/// Words ending -ые.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1606);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8649);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27219);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54621);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 82852);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 105485);
													s.is_empty()
												}
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
							super::push(out, 27220);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 54622);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82853);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 105486);
													match s {
														[] => true,
														['с', s @ ..] => super::t14::n122212(s, out),
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
									super::push(out, 54623);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 82854);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 105487);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n122213(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t15::n1611(s, out),
		['л', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8653);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27243);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 54691);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 82944);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 105585);
													match s {
														[] => true,
														['р', s @ ..] => super::t15::n122299(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8654);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27244);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54692);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 82945);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 105586);
													match s {
														[] => true,
														['о', s @ ..] => super::t15::n122300(s, out),
														_ => false,
													}
												}
												['к', s @ ..] => {
													super::push(out, 105587);
													match s {
														[] => true,
														['у', s @ ..] => super::t15::n122301(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 105588);
													match s {
														[] => true,
														['н', s @ ..] => super::t15::n122302(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 82946);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 105589);
													match s {
														[] => true,
														['д', s @ ..] => super::t15::n122303(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1609);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8655);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27245);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t15::n1614(s, out),
		['р', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8670);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27274);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54730);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 82986);
											match s {
												[] => true,
												['с', s @ ..] => super::t15::n106754(s, out),
												_ => false,
											}
										}
										['ч', s @ ..] => super::t15::n83862(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t15::n1616(s, out),
		['т', s @ ..] => super::t15::n1617(s, out),
		_ => false,
	}
}

/// Words ending -ье.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8676);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27281);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27282);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 54738);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82995);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 105637);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8677);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27283);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 54739);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 54740);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 82996);
											s.is_empty()
										}
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
					super::push(out, 8678);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27284);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54741);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 82997);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 105638);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 54742);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 82998);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8679);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 8680);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 8681);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8682);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27288);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54743);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 82999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 27289);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27290);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 54744);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 83000);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', 'н', s @ ..] => {
									super::push(out, 54745);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 83001);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 105639);
													match s {
														[] => true,
														['р', s @ ..] => super::t15::n122355(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27291);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54746);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83002);
											s.is_empty()
										}
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
					super::push(out, 8683);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27292);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 54747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 27293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8684);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27294);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 27295);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 27296);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54748);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83003);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27297);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 54749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t15::n8711(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t15::n1620(s, out),
		['ж', s @ ..] => super::t15::n1621(s, out),
		['з', s @ ..] => {
			super::push(out, 1618);
			match s {
				[] => true,
				['а', s @ ..] => super::t15::n8724(s, out),
				['е', s @ ..] => {
					super::push(out, 8699);
					match s {
						[] => true,
						['б', s @ ..] => super::t15::n27517(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t15::n1623(s, out),
		['л', s @ ..] => super::t15::n1624(s, out),
		['м', s @ ..] => super::t15::n1625(s, out),
		['н', s @ ..] => super::t15::n1626(s, out),
		['п', s @ ..] => super::t15::n1627(s, out),
		['р', s @ ..] => super::t15::n1628(s, out),
		['с', s @ ..] => super::t15::n1629(s, out),
		['т', s @ ..] => super::t15::n1630(s, out),
		['ф', s @ ..] => super::t15::n1631(s, out),
		['ч', s @ ..] => super::t15::n1632(s, out),
		['ш', s @ ..] => super::t15::n1633(s, out),
		_ => false,
	}
}

/// The word бабаж.
pub(crate) fn n55704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55133);
	s.is_empty()
}

/// Words ending -важ.
pub(crate) fn n1635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1631);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8759);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8760);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27540);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гаж.
pub(crate) fn n1636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8761);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27541);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27542);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -даж.
pub(crate) fn n1637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1633);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 8762);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27543);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27544);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55137);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27545);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55138);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 55139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8763);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27546);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заж.
pub(crate) fn n1638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1634);
	s.is_empty()
}

/// Words ending -каж.
pub(crate) fn n1639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1635);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лаж.
pub(crate) fn n1640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1636);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8765);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8766);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27548);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55142);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27549);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8767);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -маж.
pub(crate) fn n1641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1637);
	s.is_empty()
}

/// Words ending -наж.
pub(crate) fn n1642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1638);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8768);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					super::push(out, 27550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55145);
							s.is_empty()
						}
						['о', 'н', 'р', 'е', 'з', '\u{1}', s @ ..] => {
							super::push(out, 55146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8769);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 8770);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8771);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27553);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27554);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27555);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55147);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83413);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105977);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8772);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27556);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паж.
pub(crate) fn n1643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1639);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8773);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 8774);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27557);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55149);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83415);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105978);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121308);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8775);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27558);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 55150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55151);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 55152);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83416);
									s.is_empty()
								}
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
			super::push(out, 8776);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27561);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55153);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83417);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105979);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 121309);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8778);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27562);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55154);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раж.
pub(crate) fn n1644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1640);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8779);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8780);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27564);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8781);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8782);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27567);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55155);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83419);
									s.is_empty()
								}
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
			super::push(out, 8783);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8784);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8785);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27570);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 27571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27572);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8787);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27573);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83420);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 83421);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105980);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121310);
													match s {
														[] => true,
														['к', s @ ..] => super::t15::n132053(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 105981);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121311);
													match s {
														[] => true,
														['р', 'х', 'о', s @ ..] => super::t15::n132054(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27574);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55158);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83422);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 55159);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 55160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55161);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 55162);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83423);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105983);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8788);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27576);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 27577);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55163);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 27578);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55164);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -саж.
pub(crate) fn n1645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1641);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8789);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27579);
					match s {
						[] => true,
						['р', 'е', 'в', '-', s @ ..] => {
							super::push(out, 55165);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83426);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105984);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121312);
													match s {
														[] => true,
														['ц', s @ ..] => super::t15::n132055(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8790);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27580);
					match s {
						[] => true,
						['к', '-', s @ ..] => {
							super::push(out, 55166);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83427);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 105985);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121313);
													match s {
														[] => true,
														['а', s @ ..] => super::t15::n132056(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -таж.
pub(crate) fn n1646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8792);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8793);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8794);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'а', s @ ..] => {
			super::push(out, 8795);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 27584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8796);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 55168);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 55169);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 55170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55171);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 55172);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83431);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105986);
											s.is_empty()
										}
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
					super::push(out, 27585);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 55173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27586);
					match s {
						[] => true,
						['м', 'о', s @ ..] => {
							match s {
								[] => true,
								['и', 'д', 'у', s @ ..] => {
									super::push(out, 83432);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105987);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121314);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 83433);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105988);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8797);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8798);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27587);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55174);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 105989);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121315);
													match s {
														[] => true,
														['а', s @ ..] => super::t15::n132057(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 105990);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8799);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 8800);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27589);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8801);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27590);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 27591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 8802);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27592);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55177);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83434);
									s.is_empty()
								}
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
			super::push(out, 8803);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27593);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 27594);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55178);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27595);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55179);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 27596);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55180);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83437);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 105991);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фаж.
pub(crate) fn n1647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1643);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 8804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хаж.
pub(crate) fn n1648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1644);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8805);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бж.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дж.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8806);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8808);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27600);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27601);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27602);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1647);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8810);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 8811);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8812);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27603);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55184);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 83438);
											s.is_empty()
										}
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
					super::push(out, 8813);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 27604);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8814);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27605);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 55186);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83439);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 55187);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83440);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27606);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55188);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 83441);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 105992);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8815);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27607);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55189);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 83442);
											s.is_empty()
										}
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
					super::push(out, 8816);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27608);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55190);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 83443);
											s.is_empty()
										}
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
					super::push(out, 8817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1648);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8818);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8819);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8820);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27610);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 55191);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 83444);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 105993);
													match s {
														[] => true,
														['р', s @ ..] => super::t15::n122590(s, out),
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
									super::push(out, 55192);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83445);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 105994);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 55193);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83446);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 105995);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 55194);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83447);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 105996);
													s.is_empty()
												}
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
							super::push(out, 27611);
							match s {
								[] => true,
								['к', s @ ..] => super::t15::n55767(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27612);
							match s {
								[] => true,
								['п', s @ ..] => super::t15::n55768(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t15::n27806(s, out),
						['э', s @ ..] => super::t15::n27807(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t15::n1653(s, out),
		['л', s @ ..] => super::t15::n1654(s, out),
		['н', s @ ..] => super::t15::n1655(s, out),
		['р', s @ ..] => super::t15::n1656(s, out),
		['у', s @ ..] => super::t15::n1657(s, out),
		['э', s @ ..] => super::t15::n1658(s, out),
		_ => false,
	}
}

/// Words ending -еж.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1655);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8833);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27627);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8834);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27628);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55207);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 83454);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8836);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27629);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55208);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 83455);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1659);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8837);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8838);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27631);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55209);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83456);
											s.is_empty()
										}
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
					super::push(out, 8839);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8840);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27634);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 55211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 27635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8842);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8843);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8845);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27638);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8846);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8847);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27640);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55214);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83457);
											s.is_empty()
										}
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
					super::push(out, 8848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1661);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8849);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8850);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27642);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55215);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 83458);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106001);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8851);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1662);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8852);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8853);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27645);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8854);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27646);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 55217);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 83459);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 106002);
													match s {
														[] => true,
														['б', s @ ..] => super::t15::n122593(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8855);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27647);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 55218);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83460);
											s.is_empty()
										}
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
					super::push(out, 8856);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27648);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55219);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 83461);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106003);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8857);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27649);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55220);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t15::n84354(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t15::n1667(s, out),
		['р', s @ ..] => super::t15::n1668(s, out),
		['с', s @ ..] => super::t15::n1669(s, out),
		['т', s @ ..] => super::t15::n1670(s, out),
		['ч', s @ ..] => super::t15::n1671(s, out),
		['ь', s @ ..] => super::t15::n1672(s, out),
		_ => false,
	}
}

/// Words ending -иж.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8876);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1670);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8877);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8878);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27671);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55235);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83470);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8879);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27672);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 55236);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 83471);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1672);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8880);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55237);
									s.is_empty()
								}
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
			super::push(out, 1673);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8881);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8882);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8883);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27676);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8884);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27677);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8885);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27678);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55240);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 83472);
											s.is_empty()
										}
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
					super::push(out, 8886);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27679);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55241);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 83473);
											s.is_empty()
										}
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
					super::push(out, 8887);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27680);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55242);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 83474);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8888);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8889);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27682);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 55244);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83475);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1676);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мж.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1677);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27683);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27684);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55245);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83476);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106006);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нж.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1678);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8892);
					match s {
						[] => true,
						['о', 'д', 'р', 'е', 'л', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 27685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27686);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55246);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 83477);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106007);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8894);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8895);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 8896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8897);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ож.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1683);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8899);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27689);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 55247);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83478);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106008);
													s.is_empty()
												}
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
							super::push(out, 27690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 8900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1685);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8901);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8902);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 8903);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27692);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 55248);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83479);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1687);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8904);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27693);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55249);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83480);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106009);
													match s {
														[] => true,
														['е', s @ ..] => super::t15::n122594(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рж.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1688);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 8905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8906);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8908);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 8909);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27697);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55251);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83481);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 106010);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1690);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8910);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8911);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уж.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1691);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8912);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8913);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27699);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55252);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 83482);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 106011);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8914);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1696);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8917);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 8918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27700);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 55253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыж.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8921);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яж.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1702);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8923);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8924);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27702);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 8925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1704);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8927);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1705);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8929);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8930);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27704);
							match s {
								[] => true,
								['п', 'у', 'с', '\u{1}', s @ ..] => {
									super::push(out, 55255);
									s.is_empty()
								}
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
			super::push(out, 1706);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word хадаз.
pub(crate) fn n55829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55257);
	s.is_empty()
}

/// Words ending -жаз.
pub(crate) fn n1715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1711);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 8937);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 27709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заз.
pub(crate) fn n1716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1712);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8938);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27710);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 27711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иаз.
pub(crate) fn n1717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1713);
	match s {
		[] => true,
		['б', 'е', 'м', 'а', '\u{1}', s @ ..] => {
			super::push(out, 8939);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 8940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -каз.
pub(crate) fn n1718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8941);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27712);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 55259);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83484);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106012);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121321);
													match s {
														[] => true,
														['л', s @ ..] => super::t15::n132061(s, out),
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
									super::push(out, 83485);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106013);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121322);
													match s {
														[] => true,
														['у', s @ ..] => super::t15::n132062(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8942);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8943);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27715);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 55261);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 83486);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106014);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 121323);
													match s {
														[] => true,
														['к', s @ ..] => super::t15::n132063(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8944);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 8945);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лаз.
pub(crate) fn n1719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1715);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 8947);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27717);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 55263);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83487);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106015);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8948);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 8949);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27719);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27720);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55264);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83488);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106016);
											s.is_empty()
										}
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
					super::push(out, 27721);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55265);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83489);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106017);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 55266);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83490);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106018);
											s.is_empty()
										}
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
					super::push(out, 27722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 8950);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27723);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8951);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27724);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55269);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83491);
									s.is_empty()
								}
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
			super::push(out, 8952);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 8953);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27725);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8954);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27726);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55271);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55272);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27728);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'а', s @ ..] => {
			super::push(out, 8955);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 27729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8956);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маз.
pub(crate) fn n1720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1716);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8957);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 27732);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 27733);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 27734);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'т', s @ ..] => {
			super::push(out, 8958);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27735);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55277);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83495);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106019);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8959);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27736);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 55278);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 83496);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106020);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121324);
													match s {
														[] => true,
														['г', s @ ..] => super::t15::n132064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8960);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27737);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55279);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83497);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наз.
pub(crate) fn n1721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1717);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8961);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27738);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 55280);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83498);
									s.is_empty()
								}
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
			super::push(out, 8962);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27739);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 27740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 8963);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27741);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55281);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паз.
pub(crate) fn n1722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1718);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8964);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8965);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раз.
pub(crate) fn n1723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8966);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8967);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27743);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55282);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 55283);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 55284);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83500);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106021);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55285);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 55286);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83502);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106022);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121325);
													s.is_empty()
												}
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
							super::push(out, 55287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 8968);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 8969);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 8970);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27744);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55288);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27745);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55289);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83504);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106023);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8971);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27746);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55290);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8972);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27748);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 55291);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83506);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 83507);
									s.is_empty()
								}
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
			super::push(out, 8973);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27749);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 27750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 8974);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27751);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55292);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таз.
pub(crate) fn n1724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1720);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8975);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8976);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 8977);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27752);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55293);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83509);
									s.is_empty()
								}
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
			super::push(out, 8978);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27753);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83510);
									s.is_empty()
								}
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
			super::push(out, 8979);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уаз.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1721);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 8980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хаз.
pub(crate) fn n1726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1722);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 8981);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 8982);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8983);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бз.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1723);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дз.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1724);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8985);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27759);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55295);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83511);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 106024);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1725);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8986);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27760);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55296);
									s.is_empty()
								}
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
			super::push(out, 1726);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8987);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27761);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8988);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27762);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55298);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83512);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ез.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1727);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 1728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1729);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1730);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1731);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8991);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27763);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 55299);
									s.is_empty()
								}
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
			super::push(out, 1732);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8992);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27764);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55300);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 83513);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106025);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n122600(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8993);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8994);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27766);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27767);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55302);
									s.is_empty()
								}
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
			super::push(out, 1733);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8995);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 27768);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55303);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83514);
											s.is_empty()
										}
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
					super::push(out, 8996);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 8997);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27770);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55305);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83515);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1734);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8998);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55306);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83516);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106026);
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
										['и', s @ ..] => {
											super::push(out, 83517);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 106027);
													match s {
														[] => true,
														['а', s @ ..] => super::t15::n122601(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 83518);
											s.is_empty()
										}
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
					super::push(out, 8999);
					match s {
						[] => true,
						['к', 'е', s @ ..] => {
							super::push(out, 27771);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27772);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55308);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83519);
											s.is_empty()
										}
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
					super::push(out, 9001);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 27773);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55309);
									match s {
										[] => true,
										['м', '\u{1}', s @ ..] => {
											super::push(out, 83520);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9002);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 9003);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9004);
					s.is_empty()
				}
				['е', s @ ..] => super::t15::n9031(s, out),
				['з', s @ ..] => {
					super::push(out, 9005);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 9006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t15::n27970(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t15::n9034(s, out),
				['п', s @ ..] => super::t15::n9035(s, out),
				['у', s @ ..] => super::t15::n9036(s, out),
				['ы', s @ ..] => super::t15::n9037(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t15::n1740(s, out),
		['ь', s @ ..] => super::t15::n1741(s, out),
		_ => false,
	}
}

/// Words ending -зз.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	s.is_empty()
}

/// Words ending -из.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1738);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9014);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27793);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55326);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 83532);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1739);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9015);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9016);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9017);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27795);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 55327);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83533);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27796);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55328);
									s.is_empty()
								}
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
			super::push(out, 1740);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9018);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27797);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 55329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9019);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27798);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55330);
									s.is_empty()
								}
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
			super::push(out, 1741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9020);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9021);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27799);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55331);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 83534);
											s.is_empty()
										}
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
					super::push(out, 9022);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1742);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9023);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27801);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9024);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27802);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55333);
									s.is_empty()
								}
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
			super::push(out, 1743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1744);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9026);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27803);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 55334);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83535);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1745);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9027);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27804);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55335);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83536);
											s.is_empty()
										}
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
					super::push(out, 9028);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27805);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27806);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55337);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 83537);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106038);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9029);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 27807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1746);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 9030);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27808);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55338);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83538);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106039);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9031);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27809);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t15::n55913(s, out),
								_ => false,
							}
						}
						['к', s @ ..] => super::t15::n28005(s, out),
						['н', s @ ..] => super::t15::n28006(s, out),
						['т', s @ ..] => super::t15::n28007(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t15::n9059(s, out),
				['к', s @ ..] => super::t15::n9060(s, out),
				['о', s @ ..] => super::t15::n9061(s, out),
				['р', s @ ..] => super::t15::n9062(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t15::n1751(s, out),
		['н', s @ ..] => super::t15::n1752(s, out),
		['р', s @ ..] => super::t15::n1753(s, out),
		['т', s @ ..] => super::t15::n1754(s, out),
		['у', s @ ..] => super::t15::n1755(s, out),
		['ф', s @ ..] => super::t15::n1756(s, out),
		['ц', s @ ..] => super::t15::n1757(s, out),
		['ш', s @ ..] => super::t15::n1758(s, out),
		_ => false,
	}
}

/// Words ending -йз.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 164);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лз.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9066);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27855);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55368);
									s.is_empty()
								}
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
			super::push(out, 1757);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9067);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 27856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9068);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27857);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 55369);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83553);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106048);
													match s {
														[] => true,
														['о', s @ ..] => super::t15::n122616(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нз.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оз.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9070);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9071);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27858);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 27859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9072);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27860);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55371);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83554);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106049);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9073);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9074);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27862);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 55372);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83555);
											s.is_empty()
										}
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
					super::push(out, 9075);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27863);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27864);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 27865);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55373);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 83556);
											match s {
												[] => true,
												['о', 'м', '-', s @ ..] => {
													super::push(out, 106050);
													match s {
														[] => true,
														['ь', s @ ..] => super::t15::n122617(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27866);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55374);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 83557);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 106051);
													match s {
														[] => true,
														['т', s @ ..] => super::t15::n122618(s, out),
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
									super::push(out, 55375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 27867);
							match s {
								[] => true,
								['у', s @ ..] => super::t15::n55949(s, out),
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27868);
							match s {
								[] => true,
								['е', s @ ..] => super::t15::n55950(s, out),
								['п', s @ ..] => super::t15::n55951(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t15::n28063(s, out),
						['т', s @ ..] => super::t15::n28064(s, out),
						['х', s @ ..] => super::t15::n28065(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t15::n9103(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t15::n1765(s, out),
		['д', s @ ..] => super::t15::n1766(s, out),
		['и', s @ ..] => super::t15::n1767(s, out),
		['к', s @ ..] => super::t15::n1768(s, out),
		['л', s @ ..] => super::t15::n1769(s, out),
		['м', s @ ..] => super::t15::n1770(s, out),
		['н', s @ ..] => super::t15::n1771(s, out),
		['р', s @ ..] => super::t15::n1772(s, out),
		['т', s @ ..] => super::t15::n1773(s, out),
		['у', s @ ..] => super::t15::n1774(s, out),
		['ф', s @ ..] => super::t15::n1775(s, out),
		['х', s @ ..] => super::t15::n1776(s, out),
		_ => false,
	}
}

/// Words ending -рз.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1773);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1774);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9107);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27896);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55407);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83582);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1775);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9108);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27897);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 55408);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 83583);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 106067);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1776);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9109);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 27898);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55409);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83584);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9110);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уз.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9111);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27900);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27901);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55412);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27902);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55413);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 83585);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106068);
													match s {
														[] => true,
														['е', s @ ..] => super::t15::n122632(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9112);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 9113);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 9114);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27904);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55414);
									s.is_empty()
								}
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
			super::push(out, 1779);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9115);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27905);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55415);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83586);
											s.is_empty()
										}
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
					super::push(out, 9116);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27907);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 27908);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55416);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83587);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 106069);
													s.is_empty()
												}
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
							super::push(out, 27909);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55417);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83588);
											s.is_empty()
										}
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
					super::push(out, 9119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1782);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9120);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27911);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 55418);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83589);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 55419);
									s.is_empty()
								}
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
			super::push(out, 1783);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 1784);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1785);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1786);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9121);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1787);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9123);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27914);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55420);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83590);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1788);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9124);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27915);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55421);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 83591);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1789);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9125);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 9126);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9128);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 27917);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9129);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27918);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55423);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83592);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1790);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9130);
					s.is_empty()
				}
				['в', s @ ..] => super::t15::n9158(s, out),
				['н', s @ ..] => super::t15::n9159(s, out),
				['р', s @ ..] => super::t15::n9160(s, out),
				['у', s @ ..] => super::t15::n9161(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t15::n1796(s, out),
		['х', s @ ..] => super::t15::n1797(s, out),
		['ц', s @ ..] => super::t15::n1798(s, out),
		_ => false,
	}
}

/// Words ending -ыз.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['б', 'а', s @ ..] => {
			super::push(out, 1795);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9139);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9140);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27927);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55431);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83596);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'г', 'о', s @ ..] => {
			super::push(out, 1796);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9141);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27928);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55432);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83597);
											s.is_empty()
										}
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
					super::push(out, 9142);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27929);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 55433);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83598);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьз.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1797);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9143);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27930);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 55434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эз.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1798);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1799);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юз.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1800);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9146);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 27932);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55435);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83599);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106070);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1801);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9147);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27933);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55436);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 83600);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 106071);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n122633(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яз.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1802);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1803);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9150);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27934);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55437);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83601);
											s.is_empty()
										}
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
					super::push(out, 9151);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9152);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27936);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55438);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 27937);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55439);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83602);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 27938);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27939);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55441);
									s.is_empty()
								}
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
			super::push(out, 1805);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1806);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1807);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27941);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-би.
pub(crate) fn n9187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9160);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 27946);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 55445);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аби.
pub(crate) fn n1820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1814);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9161);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9162);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 27947);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55446);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83604);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106072);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9163);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27948);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55447);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83605);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106073);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9164);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27949);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 55448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9165);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27950);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55449);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83606);
									s.is_empty()
								}
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
			super::push(out, 9166);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27951);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 27952);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55450);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83607);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106074);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27953);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 55451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27954);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 9169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бби.
pub(crate) fn n1821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1815);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9170);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9171);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9172);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27957);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 27958);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55453);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83608);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106075);
											s.is_empty()
										}
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
					super::push(out, 27959);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 27960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гби.
pub(crate) fn n1822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1816);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9173);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -беби.
pub(crate) fn n9201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9174);
	s.is_empty()
}

/// Words ending -жби.
pub(crate) fn n1824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1818);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9175);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зби.
pub(crate) fn n1825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1819);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9177);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иби.
pub(crate) fn n1826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1820);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9180);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27963);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55455);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83609);
									s.is_empty()
								}
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
			super::push(out, 9181);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 9182);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27964);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йби.
pub(crate) fn n1827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1821);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9183);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кби.
pub(crate) fn n1828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1822);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 9184);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лби.
pub(crate) fn n1829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1823);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9185);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27967);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9186);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мби.
pub(crate) fn n1830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1824);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9187);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27970);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9188);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 27972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9189);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27973);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 27974);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27975);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9190);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9191);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9192);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9193);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нби.
pub(crate) fn n1831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1825);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9194);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9195);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27980);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оби.
pub(crate) fn n1832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1826);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9196);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27981);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55460);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 83610);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 106076);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121360);
													match s {
														[] => true,
														['е', s @ ..] => super::t15::n132075(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9197);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9198);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 27982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9199);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27983);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9200);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 27984);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55462);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83611);
									s.is_empty()
								}
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
			super::push(out, 9201);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27985);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55463);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рби.
pub(crate) fn n1833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1827);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9202);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27987);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 27988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9203);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27989);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55465);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83613);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106077);
											s.is_empty()
										}
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
					super::push(out, 27990);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 27991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9204);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9205);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сби.
pub(crate) fn n1834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1828);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9207);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9208);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9209);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27995);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27996);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55468);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83614);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 55469);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 83615);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106078);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121361);
													match s {
														[] => true,
														['д', s @ ..] => super::t15::n132076(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9210);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27997);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тби.
pub(crate) fn n1835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1829);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уби.
pub(crate) fn n1836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1830);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9212);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9213);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27998);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чби.
pub(crate) fn n1837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1831);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9214);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шби.
pub(crate) fn n1838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1832);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9215);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 9216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьби.
pub(crate) fn n1839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1833);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9217);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28000);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55472);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яби.
pub(crate) fn n1840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1834);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9218);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ви.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9219);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28002);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 55473);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83617);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1836);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9220);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28003);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9221);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28004);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 55475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9222);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28005);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28006);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9223);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28007);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 55478);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83618);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106079);
													s.is_empty()
												}
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
							super::push(out, 28008);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 55479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9224);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28009);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 55480);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 83619);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1837);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9225);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1838);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9226);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28011);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55481);
									s.is_empty()
								}
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
			super::push(out, 1839);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9227);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28012);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83620);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 28013);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55483);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 83621);
											s.is_empty()
										}
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
					super::push(out, 9228);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28014);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55484);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83622);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 106080);
													match s {
														[] => true,
														['-', s @ ..] => super::t15::n122636(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1840);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9229);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 9230);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28015);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55485);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 83623);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 106081);
													match s {
														[] => true,
														['к', s @ ..] => super::t15::n122637(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9231);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 9232);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28016);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55486);
									s.is_empty()
								}
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
			super::push(out, 1841);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9233);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 28017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9234);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 28018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55487);
									s.is_empty()
								}
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
			super::push(out, 1842);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9235);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28019);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55488);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 83624);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 106082);
													match s {
														[] => true,
														['р', s @ ..] => super::t15::n122638(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1843);
			match s {
				[] => true,
				['а', s @ ..] => super::t15::n9263(s, out),
				['ы', s @ ..] => super::t15::n9264(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t15::n1850(s, out),
		['э', s @ ..] => super::t15::n1851(s, out),
		_ => false,
	}
}

/// Words ending -ги.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1846);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9240);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28023);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 55491);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83625);
											s.is_empty()
										}
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
					super::push(out, 9241);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9242);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28025);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55492);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83626);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106083);
													match s {
														[] => true,
														['в', s @ ..] => super::t15::n122639(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9243);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1847);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9244);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9245);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 9247);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28030);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1848);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9248);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28032);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55493);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 83627);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 106084);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9249);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9250);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28034);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 9251);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28035);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55495);
									s.is_empty()
								}
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
			super::push(out, 1849);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9252);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28036);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 55496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28037);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55497);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83628);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106085);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1850);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9254);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28039);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55498);
									s.is_empty()
								}
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
			super::push(out, 1851);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9255);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28040);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55499);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 28041);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 55500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28042);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55501);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 83629);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 106086);
													match s {
														[] => true,
														['а', s @ ..] => super::t15::n122640(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9256);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28043);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28044);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55502);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 83630);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106087);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 55503);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83631);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t15::n1858(s, out),
		['р', s @ ..] => super::t15::n1859(s, out),
		['у', s @ ..] => super::t15::n1860(s, out),
		['ь', s @ ..] => {
			super::push(out, 1855);
			match s {
				[] => true,
				['н', s @ ..] => super::t15::n9293(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ди.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1856);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9267);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9268);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9270);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 9271);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 28061);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55517);
									s.is_empty()
								}
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
			super::push(out, 1857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9272);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28062);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9273);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28064);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 55518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9274);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9275);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 28066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 9276);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28067);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1858);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9278);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28070);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55519);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83641);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106093);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9279);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28071);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 28072);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28073);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28074);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55523);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 83642);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106094);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1859);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9282);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28075);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9283);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28076);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 55525);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 83643);
											s.is_empty()
										}
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
					super::push(out, 9284);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9286);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28078);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55526);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83644);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106095);
													s.is_empty()
												}
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
							super::push(out, 28079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9287);
					match s {
						[] => true,
						['б', s @ ..] => super::t15::n28274(s, out),
						['в', s @ ..] => {
							super::push(out, 28081);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 28082);
							s.is_empty()
						}
						['к', s @ ..] => super::t15::n28277(s, out),
						['р', s @ ..] => {
							super::push(out, 28084);
							match s {
								[] => true,
								['б', s @ ..] => super::t15::n56102(s, out),
								_ => false,
							}
						}
						['ф', s @ ..] => super::t15::n28279(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t15::n9315(s, out),
				['у', s @ ..] => super::t15::n9316(s, out),
				['я', s @ ..] => super::t15::n9317(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t15::n1867(s, out),
		['р', s @ ..] => super::t15::n1868(s, out),
		['у', s @ ..] => super::t15::n1869(s, out),
		['э', s @ ..] => super::t15::n1870(s, out),
		['ю', s @ ..] => super::t15::n1871(s, out),
		_ => false,
	}
}

/// Words ending -еи.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9308);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28101);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55541);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 83655);
											s.is_empty()
										}
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
					super::push(out, 9309);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28102);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55542);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83656);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1867);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9310);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28103);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55543);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 83657);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106102);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жи.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1868);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9311);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28104);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55544);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 83658);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 55545);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83659);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 28105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9312);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28106);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55546);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83660);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28107);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 28108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9313);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28109);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 28110);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 55548);
									s.is_empty()
								}
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
			super::push(out, 1869);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28111);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 55549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83661);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1870);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9315);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28112);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28113);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55550);
									s.is_empty()
								}
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
			super::push(out, 1871);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28114);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55551);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83662);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106103);
													match s {
														[] => true,
														['п', s @ ..] => super::t15::n122647(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1872);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1873);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 9318);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 28115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1874);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9319);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9320);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зи.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1875);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9321);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 9322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9324);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28120);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55552);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 83663);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106104);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9325);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28121);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 55553);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83664);
											s.is_empty()
										}
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
					super::push(out, 9326);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28122);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55554);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 83665);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106105);
													s.is_empty()
												}
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
							super::push(out, 28123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 9327);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28124);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55555);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83666);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106106);
													match s {
														[] => true,
														['э', s @ ..] => super::t15::n122648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1876);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9328);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28125);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55556);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 83667);
											s.is_empty()
										}
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
					super::push(out, 9329);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28126);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55557);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83668);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 106107);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9330);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 28127);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9331);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28128);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 28129);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9332);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28130);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 55561);
									s.is_empty()
								}
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
			super::push(out, 1877);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9333);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28131);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 55562);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 83669);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28132);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55563);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 83670);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1878);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9334);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9335);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28134);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55564);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 83671);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 106108);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9336);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28135);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 55565);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83672);
											s.is_empty()
										}
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
					super::push(out, 9337);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28136);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55566);
									s.is_empty()
								}
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
			super::push(out, 1880);
			match s {
				[] => true,
				['а', s @ ..] => super::t15::n9365(s, out),
				['е', s @ ..] => super::t15::n9366(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t15::n1887(s, out),
		['н', s @ ..] => super::t15::n1888(s, out),
		['о', s @ ..] => super::t15::n1889(s, out),
		['у', s @ ..] => super::t15::n1890(s, out),
		['ц', s @ ..] => super::t15::n1891(s, out),
		['ю', s @ ..] => super::t15::n1892(s, out),
		['я', s @ ..] => super::t15::n1893(s, out),
		_ => false,
	}
}

/// Words ending -ии.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9351);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28151);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55576);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83677);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106110);
													match s {
														[] => true,
														['о', s @ ..] => super::t15::n122650(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9352);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28152);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55577);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83678);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 106111);
													match s {
														[] => true,
														['е', s @ ..] => super::t15::n122651(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1889);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9353);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28153);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 55578);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83679);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1890);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9354);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28154);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55579);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 83680);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106112);
													match s {
														[] => true,
														['о', s @ ..] => super::t15::n122652(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1891);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9355);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28155);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55580);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83681);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106113);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9356);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28156);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55581);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 83682);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1892);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9357);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28157);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28158);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55582);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 83683);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106114);
													match s {
														[] => true,
														['о', s @ ..] => super::t15::n122653(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 83684);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 106115);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1893);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9358);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28159);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 55583);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 83685);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 106116);
													match s {
														[] => true,
														['п', s @ ..] => super::t15::n122654(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1894);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9359);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28160);
							match s {
								[] => true,
								['р', s @ ..] => super::t15::n56157(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t15::n9387(s, out),
				['у', s @ ..] => {
					super::push(out, 9361);
					match s {
						[] => true,
						['к', s @ ..] => super::t15::n28356(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t15::n1901(s, out),
		['с', s @ ..] => super::t15::n1902(s, out),
		['х', s @ ..] => super::t15::n1903(s, out),
		['ц', s @ ..] => super::t15::n1904(s, out),
		_ => false,
	}
}

/// Words ending -йи.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9368);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28169);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ки.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28170);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28171);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55595);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 83695);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 106125);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9370);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28172);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83696);
											s.is_empty()
										}
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
					super::push(out, 9371);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28173);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 55597);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 83697);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106126);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9372);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 9373);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28174);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 55598);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 83698);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 106127);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9374);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28175);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55599);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83699);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 106128);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9375);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28176);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 55600);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83700);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 106129);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9376);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9377);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28178);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 55602);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83701);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 106130);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 55603);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83702);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 106131);
													s.is_empty()
												}
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
							super::push(out, 28179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28180);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9378);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28181);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55606);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83703);
											s.is_empty()
										}
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
					super::push(out, 9379);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 9380);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28182);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 55607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9381);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28183);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 55608);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83704);
											s.is_empty()
										}
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
					super::push(out, 9382);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28184);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55609);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 83705);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106132);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t15::n1907(s, out),
		['в', s @ ..] => super::t15::n1908(s, out),
		['д', s @ ..] => super::t15::n1909(s, out),
		['е', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9396);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28211);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55638);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 83726);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t15::n1911(s, out),
		['з', s @ ..] => super::t15::n1912(s, out),
		['и', s @ ..] => super::t15::n1913(s, out),
		['й', s @ ..] => super::t15::n1914(s, out),
		['к', s @ ..] => super::t15::n1915(s, out),
		['л', s @ ..] => super::t15::n1916(s, out),
		['м', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['е', s @ ..] => super::t15::n9453(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t15::n1918(s, out),
		['о', s @ ..] => super::t15::n1919(s, out),
		['п', s @ ..] => super::t15::n1920(s, out),
		['р', s @ ..] => super::t15::n1921(s, out),
		['с', s @ ..] => super::t15::n1922(s, out),
		['т', s @ ..] => super::t15::n1923(s, out),
		['у', s @ ..] => super::t15::n1924(s, out),
		['ч', s @ ..] => super::t15::n1925(s, out),
		['ш', s @ ..] => super::t15::n1926(s, out),
		['ь', s @ ..] => super::t15::n1927(s, out),
		['ю', s @ ..] => super::t15::n1928(s, out),
		['я', s @ ..] => super::t15::n1929(s, out),
		_ => false,
	}
}

/// Words ending -ли.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28422);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9495);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28424);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55836);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 83878);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 106231);
													s.is_empty()
												}
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
							super::push(out, 28425);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55837);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83879);
											s.is_empty()
										}
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
					super::push(out, 9496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28426);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 28427);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28428);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 55839);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 83880);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28429);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28430);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55841);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83881);
											s.is_empty()
										}
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
					super::push(out, 9497);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28431);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55842);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 83882);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 106232);
													match s {
														[] => true,
														['ц', s @ ..] => super::t15::n122715(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 83883);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106233);
													match s {
														[] => true,
														['а', s @ ..] => super::t15::n122716(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9498);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28432);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28433);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9499);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28434);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55845);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83884);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28435);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 55846);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 83885);
											s.is_empty()
										}
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
					super::push(out, 9500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9501);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28437);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28438);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55847);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 83886);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28439);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55848);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 83887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 55849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9502);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28440);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 55850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t15::n9531(s, out),
				['т', s @ ..] => super::t15::n9532(s, out),
				['х', s @ ..] => super::t15::n9533(s, out),
				['ш', s @ ..] => super::t15::n9534(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t15::n1931(s, out),
		['г', s @ ..] => super::t15::n1932(s, out),
		['д', s @ ..] => super::t15::n1933(s, out),
		['е', s @ ..] => super::t15::n1934(s, out),
		['з', s @ ..] => super::t15::n1935(s, out),
		['и', s @ ..] => super::t15::n1936(s, out),
		['л', s @ ..] => super::t15::n1937(s, out),
		['м', s @ ..] => super::t15::n1938(s, out),
		['о', s @ ..] => super::t15::n1939(s, out),
		['п', s @ ..] => super::t15::n1940(s, out),
		['р', s @ ..] => super::t15::n1941(s, out),
		['с', s @ ..] => super::t15::n1942(s, out),
		['т', s @ ..] => super::t15::n1943(s, out),
		['у', s @ ..] => super::t15::n1944(s, out),
		['ф', s @ ..] => super::t15::n1945(s, out),
		['ш', s @ ..] => super::t15::n1946(s, out),
		['ы', s @ ..] => super::t15::n1947(s, out),
		['э', s @ ..] => {
			super::push(out, 1942);
			s.is_empty()
		}
		['ю', s @ ..] => super::t15::n1949(s, out),
		['я', s @ ..] => super::t15::n1950(s, out),
		_ => false,
	}
}

/// Words ending -ми.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9576);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9577);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28529);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55916);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 83929);
											s.is_empty()
										}
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
					super::push(out, 9578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28530);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28531);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55917);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83930);
											s.is_empty()
										}
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
					super::push(out, 9579);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28532);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9580);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28533);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55919);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 83931);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106260);
													s.is_empty()
												}
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
							super::push(out, 28534);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 55920);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 83932);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 106261);
													match s {
														[] => true,
														['м', s @ ..] => super::t15::n122733(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9581);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28535);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 55921);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 83933);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106262);
													match s {
														[] => true,
														['р', s @ ..] => super::t15::n122734(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9582);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9583);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28537);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55922);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 83934);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106263);
													match s {
														[] => true,
														['ь', s @ ..] => super::t15::n122735(s, out),
														_ => false,
													}
												}
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
							super::push(out, 28538);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 55923);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 83935);
											s.is_empty()
										}
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
					super::push(out, 9584);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28539);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 55924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 28540);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55925);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 83937);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 106264);
													match s {
														[] => true,
														['а', s @ ..] => super::t15::n122736(s, out),
														_ => false,
													}
												}
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
							super::push(out, 28541);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 55926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9585);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28542);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55927);
									s.is_empty()
								}
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
			super::push(out, 1946);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9586);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28543);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 55928);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83938);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t15::n1953(s, out),
		['и', s @ ..] => super::t15::n1954(s, out),
		['к', s @ ..] => super::t16::n1955(s, out),
		['м', s @ ..] => super::t16::n1956(s, out),
		['о', s @ ..] => super::t16::n1957(s, out),
		['р', s @ ..] => super::t16::n1958(s, out),
		['с', s @ ..] => super::t16::n1959(s, out),
		['у', s @ ..] => super::t16::n1960(s, out),
		['х', s @ ..] => super::t16::n1961(s, out),
		['ш', s @ ..] => super::t16::n1962(s, out),
		['я', s @ ..] => super::t16::n1963(s, out),
		_ => false,
	}
}

/// Words ending -ни.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1958);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9616);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28574);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55949);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83948);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28575);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55950);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 83949);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106268);
													match s {
														[] => true,
														['н', s @ ..] => super::t16::n122738(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9617);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28576);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 55951);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 83950);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 106269);
													s.is_empty()
												}
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
							super::push(out, 28577);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 55952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 9619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9620);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28580);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 55953);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 83951);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106270);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 55954);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 83952);
											s.is_empty()
										}
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
					super::push(out, 9621);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28581);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 55955);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 83953);
											s.is_empty()
										}
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
					super::push(out, 9622);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28582);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 55956);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 83954);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106271);
													s.is_empty()
												}
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
							super::push(out, 28583);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 55957);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 83955);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106272);
													match s {
														[] => true,
														['б', s @ ..] => super::t16::n122739(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9623);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28584);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 55958);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 83956);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 106273);
													match s {
														[] => true,
														['а', s @ ..] => super::t16::n122740(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t16::n9651(s, out),
				['с', s @ ..] => super::t16::n9652(s, out),
				['т', s @ ..] => super::t16::n9653(s, out),
				['ф', s @ ..] => super::t16::n9654(s, out),
				['х', s @ ..] => super::t16::n9655(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t16::n1965(s, out),
		['г', s @ ..] => super::t16::n1966(s, out),
		['д', s @ ..] => super::t16::n1967(s, out),
		['е', s @ ..] => super::t16::n1968(s, out),
		['ж', s @ ..] => super::t16::n1969(s, out),
		['з', s @ ..] => super::t16::n1970(s, out),
		['и', s @ ..] => super::t16::n1971(s, out),
		['к', s @ ..] => super::t16::n1972(s, out),
		['л', s @ ..] => super::t16::n1973(s, out),
		['н', s @ ..] => super::t16::n1974(s, out),
		['о', s @ ..] => super::t16::n1975(s, out),
		['р', s @ ..] => super::t16::n1976(s, out),
		['с', s @ ..] => super::t16::n1977(s, out),
		['т', s @ ..] => super::t16::n1978(s, out),
		['у', s @ ..] => super::t16::n1979(s, out),
		['ч', s @ ..] => super::t16::n1980(s, out),
		['ш', s @ ..] => super::t16::n1981(s, out),
		['ь', s @ ..] => super::t16::n1982(s, out),
		['ю', s @ ..] => super::t16::n1983(s, out),
		['я', s @ ..] => super::t16::n1984(s, out),
		_ => false,
	}
}

/// Words ending -ои.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1979);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9691);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56028);
									s.is_empty()
								}
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
			super::push(out, 1981);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9693);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1983);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9694);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

