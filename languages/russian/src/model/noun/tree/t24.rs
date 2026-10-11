use flexion::Path;

/// Words ending -тес.
pub(crate) fn n3539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3528);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15643);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38305);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66433);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38306);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66434);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112252);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15644);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38307);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66435);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92531);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112253);
											s.is_empty()
										}
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
					super::push(out, 38308);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92532);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112254);
											s.is_empty()
										}
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
					super::push(out, 38309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15645);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15646);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38311);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66437);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92533);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 66438);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92534);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112256);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15647);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38312);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66439);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 66440);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92536);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112257);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125035);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38313);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15648);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38314);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 66442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92537);
									s.is_empty()
								}
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
			super::push(out, 15649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уес.
pub(crate) fn n3540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3529);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15650);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15651);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38316);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фес.
pub(crate) fn n3541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3530);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15652);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 15653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цес.
pub(crate) fn n3542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3531);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15654);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38318);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66444);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92538);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112258);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125036);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134102(s, out),
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
									super::push(out, 92539);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112259);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125037);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134103(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15655);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38319);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чес.
pub(crate) fn n3543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3532);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15656);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15657);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38320);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66446);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92540);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112260);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шес.
pub(crate) fn n3544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3533);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 15659);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьес.
pub(crate) fn n3545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3534);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -милгравис.
pub(crate) fn n126336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125038);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 132735);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 137274);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -им-сервис.
pub(crate) fn n126337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125039);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 132736);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 137275);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 139980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гендиадис.
pub(crate) fn n126338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125040);
	s.is_empty()
}

/// Words ending -ежис.
pub(crate) fn n15762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15677);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38341);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 66461);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 92546);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112264);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 125041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зис.
pub(crate) fn n3552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3541);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15678);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38342);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38343);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66462);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92547);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112265);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125042);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15679);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38344);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92548);
									s.is_empty()
								}
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
			super::push(out, 15680);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38345);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38346);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66465);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92549);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 66466);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['а', 'т', s @ ..] => {
											super::push(out, 112266);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125043);
													match s {
														[] => true,
														['м', s @ ..] => super::t38::n134106(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 112267);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['и', 'ц', s @ ..] => {
													super::push(out, 125044);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134107(s, out),
														_ => false,
													}
												}
												['л', s @ ..] => {
													super::push(out, 125045);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n134108(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 125046);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 125047);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134109(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38347);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15681);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15682);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38349);
					match s {
						[] => true,
						['о', 'п', 'а', '\u{1}', s @ ..] => {
							super::push(out, 66468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'е', s @ ..] => {
					super::push(out, 38350);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кис.
pub(crate) fn n3553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3542);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15683);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38351);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66470);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 66471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15684);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лис.
pub(crate) fn n3554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3543);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15685);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15686);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38353);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38354);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 38355);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66472);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92551);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112268);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125048);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134110(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15687);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38358);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15688);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38359);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15689);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 38361);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 66477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 38362);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 15690);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38363);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66479);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92552);
									s.is_empty()
								}
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
			super::push(out, 15691);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38364);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66480);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92553);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112269);
											s.is_empty()
										}
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
					super::push(out, 38365);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66481);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38366);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66482);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92555);
									s.is_empty()
								}
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
			super::push(out, 15692);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15693);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38368);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66483);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92556);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112270);
											s.is_empty()
										}
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
					super::push(out, 38369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38370);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 66485);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['а', s @ ..] => {
							super::push(out, 66486);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92558);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112271);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125049);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 92559);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112272);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 125050);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n134111(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t38::n67182(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15694);
			s.is_empty()
		}
		['у', s @ ..] => super::t38::n15780(s, out),
		['ф', s @ ..] => super::t38::n15781(s, out),
		['э', s @ ..] => super::t38::n15782(s, out),
		_ => false,
	}
}

/// Words ending -мис.
pub(crate) fn n3555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3544);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15698);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38375);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 38376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15699);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38377);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66493);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 92565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 38378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38379);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66495);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92566);
									s.is_empty()
								}
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
			super::push(out, 15700);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38380);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66496);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92567);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112281);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125058);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134116(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15701);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38381);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66497);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92568);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112282);
											s.is_empty()
										}
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
					super::push(out, 38382);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нис.
pub(crate) fn n3556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3545);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15702);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15703);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38384);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38385);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38386);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 66500);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 92570);
									s.is_empty()
								}
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
			super::push(out, 15704);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38387);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66501);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92571);
									s.is_empty()
								}
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
			super::push(out, 15705);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38388);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 66503);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92573);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112283);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125059);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15706);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38389);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66504);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 38390);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 66505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38391);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38392);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66507);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92575);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112284);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15707);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38393);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66508);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 66509);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92576);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112285);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125060);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n134117(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15708);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15709);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38395);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 15710);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38396);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пис.
pub(crate) fn n3558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3547);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15712);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38398);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15713);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38399);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66514);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92577);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112286);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125061);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n134118(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15714);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38400);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15715);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38401);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66516);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92578);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112287);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15716);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38402);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66517);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92579);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112288);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15717);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38403);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38404);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 38405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 15718);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38406);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рис.
pub(crate) fn n3559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3548);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15719);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15720);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38407);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66520);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92580);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112289);
											s.is_empty()
										}
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
					super::push(out, 38408);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66521);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92581);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 92582);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112290);
											s.is_empty()
										}
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
					super::push(out, 38409);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66522);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 66523);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 92583);
									s.is_empty()
								}
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
			super::push(out, 15721);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38410);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38411);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92584);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 66525);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92585);
									s.is_empty()
								}
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
			super::push(out, 15722);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38412);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38413);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15723);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38414);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66527);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92586);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112291);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125062);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134119(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 112292);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125063);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134120(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15724);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 38416);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15725);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15726);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38418);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15727);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38419);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38420);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66533);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 66534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38421);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38422);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66536);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92589);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112293);
											s.is_empty()
										}
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
					super::push(out, 38423);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66537);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92590);
									s.is_empty()
								}
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
			super::push(out, 15729);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сис.
pub(crate) fn n3560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3549);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15730);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38425);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66539);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92591);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112294);
											s.is_empty()
										}
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
					super::push(out, 38426);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66540);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92592);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112295);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125064);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134121(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15731);
			match s {
				[] => true,
				['м', 'и', s @ ..] => {
					super::push(out, 38427);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92593);
									s.is_empty()
								}
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
			super::push(out, 15732);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15733);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66543);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 66544);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92594);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112296);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125065);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 92595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38429);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15734);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 38431);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66545);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 92596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112297);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15735);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38432);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66546);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92597);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112298);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125066);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134122(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15736);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66547);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 66548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92599);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112299);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125067);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 92600);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112300);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125068);
													s.is_empty()
												}
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
							super::push(out, 66549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38433);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66550);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92601);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112301);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125069);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38434);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15737);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38435);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 38436);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 66552);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92602);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 112302);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15738);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38437);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тис.
pub(crate) fn n3561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3550);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15739);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15740);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38439);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66555);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92604);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112303);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125070);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n134123(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15741);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38441);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66557);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92605);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112304);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125071);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15742);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38442);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15743);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38443);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66559);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92606);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112305);
											s.is_empty()
										}
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
					super::push(out, 38444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38445);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15745);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 15746);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -луис.
pub(crate) fn n15832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15747);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фис.
pub(crate) fn n3563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3552);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15748);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38450);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38451);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66560);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92607);
									s.is_empty()
								}
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
			super::push(out, 15749);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15750);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38452);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 38453);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66561);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92608);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112306);
											s.is_empty()
										}
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
					super::push(out, 38454);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 38455);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66562);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92609);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112307);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 125072);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хис.
pub(crate) fn n3564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3553);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15751);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38456);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цис.
pub(crate) fn n3565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3554);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15752);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38457);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шис.
pub(crate) fn n3566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3555);
	s.is_empty()
}

/// Words ending -юис.
pub(crate) fn n3567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3556);
	s.is_empty()
}

/// Words ending -рихоплакс.
pub(crate) fn n126372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125074);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 132755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кантопанакс.
pub(crate) fn n126373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125075);
	s.is_empty()
}

/// Words ending -котринакс.
pub(crate) fn n126374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125076);
	s.is_empty()
}

/// Words ending -пакс.
pub(crate) fn n15848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15762);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 66578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 66579);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92622);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 112314);
									s.is_empty()
								}
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
			super::push(out, 38468);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 66580);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 92623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ракс.
pub(crate) fn n15849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15763);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 38470);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 66582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 38471);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 66583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сакс.
pub(crate) fn n15850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15764);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38472);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 38473);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 66584);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 92626);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112315);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125077);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132756);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 112316);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125078);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -факс.
pub(crate) fn n15852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15766);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 38475);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 66586);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92627);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 112317);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125079);
											s.is_empty()
										}
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
					super::push(out, 66587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -екс.
pub(crate) fn n3572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3561);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15767);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15768);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15769);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38477);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66590);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 66591);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92628);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112318);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125080);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38478);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66592);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92629);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112319);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38479);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15771);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38480);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66593);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 66594);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92630);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112320);
											s.is_empty()
										}
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
					super::push(out, 38482);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38483);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66595);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92631);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112321);
											s.is_empty()
										}
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
					super::push(out, 38484);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66596);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92632);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112322);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125081);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134126(s, out),
														_ => false,
													}
												}
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
							super::push(out, 66597);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92633);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112323);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125082);
													match s {
														[] => true,
														['у', s @ ..] => super::t38::n134127(s, out),
														_ => false,
													}
												}
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
							super::push(out, 66598);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92634);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112324);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125083);
													match s {
														[] => true,
														['г', s @ ..] => super::t38::n134128(s, out),
														_ => false,
													}
												}
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
							super::push(out, 66599);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92635);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112325);
											match s {
												[] => true,
												['-', s @ ..] => super::t38::n126382(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t38::n93633(s, out),
								_ => false,
							}
						}
						['у', 'р', s @ ..] => super::t38::n67295(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t38::n38855(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t38::n38856(s, out),
				['е', s @ ..] => super::t38::n38857(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t38::n15859(s, out),
		['р', s @ ..] => super::t38::n15860(s, out),
		['с', s @ ..] => super::t38::n15861(s, out),
		['т', s @ ..] => super::t38::n15862(s, out),
		['ф', s @ ..] => super::t38::n15863(s, out),
		['ц', s @ ..] => super::t38::n15864(s, out),
		_ => false,
	}
}

/// Words ending -икс.
pub(crate) fn n3573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3562);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15779);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15780);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38506);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38507);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66620);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92653);
									s.is_empty()
								}
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
			super::push(out, 15781);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 15782);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38508);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38509);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66622);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92655);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112332);
											s.is_empty()
										}
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
					super::push(out, 38510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15783);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38511);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66623);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 66624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'п', s @ ..] => {
					super::push(out, 38512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38513);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 38514);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15784);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38515);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66628);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 66629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66630);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92656);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112333);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125088);
													s.is_empty()
												}
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
							super::push(out, 66631);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92657);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112334);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125089);
													s.is_empty()
												}
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
							super::push(out, 66632);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92658);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112335);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125090);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15785);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38517);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66633);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92659);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112336);
											s.is_empty()
										}
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
					super::push(out, 38518);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66634);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92660);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 66635);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92661);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112338);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125091);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38519);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38520);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66636);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92662);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112339);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 92663);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 92664);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112340);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125093);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134132(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15787);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38521);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66637);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t38::n15874(s, out),
		['ч', s @ ..] => {
			super::push(out, 15789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нкс.
pub(crate) fn n3574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3563);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15790);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38531);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66656);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 66657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38532);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окс.
pub(crate) fn n3575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3564);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15791);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38533);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66659);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92677);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112348);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125095);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134133(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15792);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15793);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38534);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38535);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66661);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38536);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66662);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92679);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112349);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15794);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15795);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38537);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66663);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92680);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112350);
											s.is_empty()
										}
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
					super::push(out, 38538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15796);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38539);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38540);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66666);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркс.
pub(crate) fn n3576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3565);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15797);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15798);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38542);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66667);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92683);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112351);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125096);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укс.
pub(crate) fn n3577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3566);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15799);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 15800);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 15801);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкс.
pub(crate) fn n3578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3567);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15802);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38544);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 38545);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66668);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38546);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -якс.
pub(crate) fn n3579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3568);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15803);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -даугавпилс.
pub(crate) fn n134134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132765);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -спиричуэлс.
pub(crate) fn n134135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132766);
	s.is_empty()
}

/// Words ending -африкаанс.
pub(crate) fn n126397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125099);
	s.is_empty()
}

/// The word аффорданс.
pub(crate) fn n126398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125100);
	s.is_empty()
}

/// Words ending -перфоманс.
pub(crate) fn n126399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125101);
	s.is_empty()
}

/// Words ending -ерформанс.
pub(crate) fn n126400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125102);
	s.is_empty()
}

/// Words ending --де-франс.
pub(crate) fn n126401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125103);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 132767);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енс.
pub(crate) fn n3593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3582);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15836);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15837);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15838);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38576);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38577);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66688);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92698);
									s.is_empty()
								}
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
			super::push(out, 15839);
			match s {
				[] => true,
				['-', 'и', 'т', 'у', 'б', '\u{1}', s @ ..] => {
					super::push(out, 38578);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 38579);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66689);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92699);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112360);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15840);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15841);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38581);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66690);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92700);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112361);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125104);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134137(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38582);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66691);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92701);
									s.is_empty()
								}
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
			super::push(out, 15842);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38583);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15843);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15844);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38585);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66693);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92702);
									s.is_empty()
								}
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
			super::push(out, 15845);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38586);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66694);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92703);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112362);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 92704);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112363);
											s.is_empty()
										}
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
					super::push(out, 38587);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66695);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38588);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66696);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38589);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66697);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92707);
									s.is_empty()
								}
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
			super::push(out, 15846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38590);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66698);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92708);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112364);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125105);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134138(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38591);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66699);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92709);
									s.is_empty()
								}
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
			super::push(out, 15847);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 15848);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 15849);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38593);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word принс.
pub(crate) fn n67397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66701);
	s.is_empty()
}

/// Words ending -онс.
pub(crate) fn n3595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3584);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15853);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15854);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 38598);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66702);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнс.
pub(crate) fn n3596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3585);
	s.is_empty()
}

/// Words ending -унс.
pub(crate) fn n3597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3586);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 15855);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -энс.
pub(crate) fn n3598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3587);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янс.
pub(crate) fn n3599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3588);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 15857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олективос.
pub(crate) fn n126404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125106);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 132770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вадос.
pub(crate) fn n38988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38616);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 66712);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 92718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 112369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -надос.
pub(crate) fn n38989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38617);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 66713);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 92719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -падос.
pub(crate) fn n38990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38618);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 66714);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 92720);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здос.
pub(crate) fn n15961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15874);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 38619);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 66715);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идос.
pub(crate) fn n15962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15875);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 38620);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 66716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 38621);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 38622);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 66717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йдос.
pub(crate) fn n15963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15876);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 38623);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66718);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдос.
pub(crate) fn n15964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15877);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 38624);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66719);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндос.
pub(crate) fn n15965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15878);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38625);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66720);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 66721);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92723);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 112370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 66722);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 92724);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 112371);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125107);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 92725);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 112372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'п', '\u{1}', s @ ..] => {
			super::push(out, 38626);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 38627);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 66723);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одос.
pub(crate) fn n15966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15879);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 38628);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 66724);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 92726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38629);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 66725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 38630);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 66726);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 92727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удос.
pub(crate) fn n15967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15880);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 38631);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 66727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 92728);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 112373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иос.
pub(crate) fn n3606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3595);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15881);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38632);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66728);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92729);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112374);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125108);
													s.is_empty()
												}
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
							super::push(out, 66729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15882);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38633);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66730);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92730);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112375);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15883);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38634);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15884);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38635);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66732);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кос.
pub(crate) fn n3607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3596);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15885);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38636);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38637);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38638);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66733);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 66734);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92733);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112376);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15886);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15887);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15888);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38639);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66735);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92734);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 66736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92735);
									s.is_empty()
								}
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
			super::push(out, 15889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38640);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 38641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66737);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92736);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112377);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125109);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38643);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66739);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 66740);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92738);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112378);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 66741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38644);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66742);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92739);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112379);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125110);
													match s {
														[] => true,
														['г', s @ ..] => super::t38::n134140(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38645);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66743);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92740);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112380);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125111);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134141(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15890);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38646);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66744);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92741);
									s.is_empty()
								}
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
			super::push(out, 15891);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38647);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66745);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92742);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112381);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125112);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38648);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92743);
									s.is_empty()
								}
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
			super::push(out, 15892);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38649);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38650);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 38651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15893);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38652);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66747);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92744);
									s.is_empty()
								}
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
			super::push(out, 15894);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38653);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38654);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92745);
									s.is_empty()
								}
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
			super::push(out, 15895);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15896);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 15897);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38656);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лос.
pub(crate) fn n3608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3597);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15898);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38657);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15899);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38659);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92746);
									s.is_empty()
								}
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
			super::push(out, 15900);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66752);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 66753);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92747);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112382);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125113);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15901);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38661);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66754);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92748);
									s.is_empty()
								}
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
			super::push(out, 15902);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38662);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38663);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15903);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38664);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 38665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66756);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 66757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15904);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38667);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66758);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38668);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66759);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92750);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112383);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15905);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66760);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 66761);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92751);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112384);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125114);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n134142(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 38671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15906);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38673);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15907);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38674);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66765);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92752);
									s.is_empty()
								}
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
			super::push(out, 15908);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 15909);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38676);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мос.
pub(crate) fn n3609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3598);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38677);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 38678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15911);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15912);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38679);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 38680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15913);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 38681);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66769);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92753);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 112385);
											s.is_empty()
										}
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
					super::push(out, 38682);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66770);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 92754);
									s.is_empty()
								}
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
			super::push(out, 15914);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15915);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38683);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66771);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92755);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112386);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125115);
													match s {
														[] => true,
														['ж', s @ ..] => super::t38::n134143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15916);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нос.
pub(crate) fn n3610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3599);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15917);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15918);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 38686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 38687);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 38688);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66773);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 92756);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112387);
											s.is_empty()
										}
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
					super::push(out, 38689);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 38690);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66774);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92757);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112388);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15919);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15920);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15921);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38692);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15922);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38693);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66776);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92758);
									s.is_empty()
								}
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
			super::push(out, 15923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38694);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 38695);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15924);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38697);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 66779);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92759);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112389);
											s.is_empty()
										}
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
					super::push(out, 38698);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66780);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92760);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112390);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125116);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134144(s, out),
														_ => false,
													}
												}
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
							super::push(out, 66781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38699);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66782);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 66783);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92762);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112391);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125117);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15925);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38700);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66784);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 38701);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66785);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 66786);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t38::n39074(s, out),
				['д', s @ ..] => super::t38::n39075(s, out),
				['к', s @ ..] => super::t38::n39076(s, out),
				['л', s @ ..] => super::t38::n39077(s, out),
				['м', s @ ..] => super::t38::n39078(s, out),
				['п', 'м', s @ ..] => super::t38::n39079(s, out),
				['р', s @ ..] => super::t38::n39080(s, out),
				['т', s @ ..] => super::t38::n39081(s, out),
				['х', s @ ..] => super::t38::n39082(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t38::n16013(s, out),
		['р', s @ ..] => super::t38::n16014(s, out),
		['с', s @ ..] => super::t38::n16015(s, out),
		['т', s @ ..] => super::t38::n16016(s, out),
		['у', s @ ..] => super::t38::n16017(s, out),
		['х', s @ ..] => super::t38::n16018(s, out),
		['ы', s @ ..] => super::t38::n16019(s, out),
		_ => false,
	}
}

/// Words ending -пос.
pub(crate) fn n3611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3600);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 15933);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38717);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15934);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38718);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15935);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38719);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15936);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38720);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66819);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92787);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 112402);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рос.
pub(crate) fn n3612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3601);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15938);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15939);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38722);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 38723);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 38724);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92788);
									s.is_empty()
								}
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
			super::push(out, 15940);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38725);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66822);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92789);
									s.is_empty()
								}
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
			super::push(out, 15941);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38726);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15942);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66824);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 92790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38728);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66825);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38729);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66826);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92792);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112403);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125123);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38730);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 66828);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92794);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112404);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125124);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38731);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66829);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92795);
									s.is_empty()
								}
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
			super::push(out, 15943);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38733);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38734);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66832);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92796);
									s.is_empty()
								}
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
			super::push(out, 15944);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38735);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92797);
									s.is_empty()
								}
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
			super::push(out, 15946);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38736);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66834);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92798);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 112405);
											s.is_empty()
										}
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
					super::push(out, 38737);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66835);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92799);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112406);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125125);
													match s {
														[] => true,
														['в', s @ ..] => super::t38::n134146(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38738);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66836);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92800);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112407);
											s.is_empty()
										}
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
					super::push(out, 38739);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66837);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38740);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66838);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92802);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112408);
											s.is_empty()
										}
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
					super::push(out, 38741);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66839);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 66840);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 66841);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38742);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 38743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66842);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 66843);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t38::n16034(s, out),
		['т', s @ ..] => super::t38::n16035(s, out),
		['у', s @ ..] => super::t38::n16036(s, out),
		['ф', s @ ..] => super::t38::n16037(s, out),
		['э', s @ ..] => super::t38::n16038(s, out),
		_ => false,
	}
}

/// Words ending -сос.
pub(crate) fn n3613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15952);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15953);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15954);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38758);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 66855);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92808);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112410);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125127);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134147(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38759);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 66856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92809);
									s.is_empty()
								}
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
			super::push(out, 15955);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15956);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38760);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92810);
									s.is_empty()
								}
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
			super::push(out, 15957);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38761);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 38762);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66859);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92811);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112411);
											s.is_empty()
										}
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
					super::push(out, 38763);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66860);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92812);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112412);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125128);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134148(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38764);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66861);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92813);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112413);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125129);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38765);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 66862);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38766);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66863);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92815);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 112414);
											s.is_empty()
										}
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
					super::push(out, 38767);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38768);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66865);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92816);
									s.is_empty()
								}
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
			super::push(out, 15958);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38769);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66866);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92817);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112415);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125130);
													match s {
														[] => true,
														['п', s @ ..] => super::t38::n134149(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38770);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тос.
pub(crate) fn n3614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3603);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15960);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38771);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66868);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92818);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 92819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38772);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66869);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92820);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112416);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15961);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38773);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15962);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38774);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66871);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92821);
									s.is_empty()
								}
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
			super::push(out, 15963);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15964);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38776);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 38777);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15965);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38778);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66873);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 92822);
									s.is_empty()
								}
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
			super::push(out, 15966);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38779);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15967);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38780);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66875);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92823);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112417);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15968);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 15969);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38781);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66876);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фос.
pub(crate) fn n3615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3604);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15970);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38782);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 66877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38784);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15971);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38785);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38787);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15972);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38788);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 38789);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38790);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хос.
pub(crate) fn n3616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15973);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15974);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чос.
pub(crate) fn n3617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15975);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьос.
pub(crate) fn n3618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3607);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15976);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38794);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66886);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92826);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112418);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 125131);
													match s {
														[] => true,
														['ь', s @ ..] => super::t38::n134150(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15977);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38795);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66887);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92827);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112419);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125132);
													s.is_empty()
												}
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
							super::push(out, 66888);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92828);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112420);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --энд-чипс.
pub(crate) fn n126431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125133);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 132782);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 137298);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 139991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -топс.
pub(crate) fn n16085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15998);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38817);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 66902);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 92836);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 112425);
									s.is_empty()
								}
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
			super::push(out, 38818);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 66903);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упс.
pub(crate) fn n3624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3613);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16000);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 16001);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 16002);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38820);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омегаверс.
pub(crate) fn n126432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125134);
	s.is_empty()
}

/// Words ending -ильдеперс.
pub(crate) fn n126433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125135);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 132783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тхорс.
pub(crate) fn n39213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38841);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 66918);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92845);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 112430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урс.
pub(crate) fn n3629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3618);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16029);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 16030);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 38842);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 38843);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66920);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92846);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112431);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 66921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38844);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38845);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38846);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66924);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92847);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112432);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125136);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n134153(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 125137);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134154(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 112433);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 112434);
											s.is_empty()
										}
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
					super::push(out, 38847);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 38848);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66926);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92848);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 66927);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 92849);
									s.is_empty()
								}
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
			super::push(out, 16031);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38849);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эрс.
pub(crate) fn n3630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3619);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 16033);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38850);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66929);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92850);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112435);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125138);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n134155(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word блюграсс.
pub(crate) fn n126437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125139);
	s.is_empty()
}

/// Words ending -кросс.
pub(crate) fn n39237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38865);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 66940);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 92859);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 112442);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 125140);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132787);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137301);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n141435(s, out),
														_ => false,
													}
												}
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
							super::push(out, 112443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 66941);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 92860);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 112444);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лоросс.
pub(crate) fn n67640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66944);
	s.is_empty()
}

/// Words ending -норосс.
pub(crate) fn n67641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66945);
	s.is_empty()
}

/// Words ending -усс.
pub(crate) fn n3635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16051);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38868);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66946);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 66947);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92863);
									s.is_empty()
								}
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
			super::push(out, 16052);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 16053);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16054);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38869);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 38870);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нта-клаус.
pub(crate) fn n126440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -робус.
pub(crate) fn n39269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38897);
	s.is_empty()
}

/// Words ending -тобус.
pub(crate) fn n39270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38898);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 66965);
			match s {
				[] => true,
				['а', '-', s @ ..] => {
					super::push(out, 92874);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 112453);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 125143);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 132789);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 137302);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n141436(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбус.
pub(crate) fn n16163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16075);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38899);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 66966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 92875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убус.
pub(crate) fn n16164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16076);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 38900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вус.
pub(crate) fn n3645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3634);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16077);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 38901);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гус.
pub(crate) fn n3646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3635);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16078);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38902);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66967);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92876);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112454);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125144);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38903);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66968);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92877);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112455);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125145);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16079);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16080);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38905);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 38906);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66969);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92878);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112456);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16081);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38907);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 66970);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92879);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112457);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 125146);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134159(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38908);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66971);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92880);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112458);
											s.is_empty()
										}
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
					super::push(out, 38909);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66972);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 66973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92882);
									s.is_empty()
								}
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
			super::push(out, 16082);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38910);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дус.
pub(crate) fn n3647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16084);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38912);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38913);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16085);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38914);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66976);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92883);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112459);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125147);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38915);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66977);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92884);
									s.is_empty()
								}
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
			super::push(out, 16086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38916);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38917);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 38918);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92885);
									s.is_empty()
								}
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
			super::push(out, 16087);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38919);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66980);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92886);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112460);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 125148);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38921);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16089);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еус.
pub(crate) fn n3648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3637);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 16090);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38923);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16091);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38924);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 66984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жус.
pub(crate) fn n3649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3638);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 16092);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38925);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зус.
pub(crate) fn n3650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3639);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16093);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16094);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38927);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66986);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38928);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16095);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38929);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66988);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92888);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112461);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16096);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38930);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иус.
pub(crate) fn n3651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3640);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38931);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 16098);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38932);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16099);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38933);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 66992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 38934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16100);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 38935);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66993);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92889);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112462);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16101);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38936);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16102);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66995);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92890);
									s.is_empty()
								}
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
			super::push(out, 16103);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38938);
					match s {
						[] => true,
						['в', 'и', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92891);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112463);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125149);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134160(s, out),
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
									super::push(out, 92892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 66996);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 66997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38939);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92893);
									s.is_empty()
								}
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
			super::push(out, 16104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38940);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66999);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92894);
									s.is_empty()
								}
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
			super::push(out, 16105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кус.
pub(crate) fn n3652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3641);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16106);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16107);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38942);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 38943);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38944);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92895);
									s.is_empty()
								}
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
			super::push(out, 16108);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38945);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67001);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92896);
									s.is_empty()
								}
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
			super::push(out, 16109);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38946);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38947);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16110);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38948);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67004);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92897);
									s.is_empty()
								}
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
			super::push(out, 16111);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38949);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67005);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38950);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38951);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67007);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16112);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16113);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67008);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 67009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67010);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 67011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16114);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38954);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67012);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 67013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38955);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 67015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16115);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16116);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38957);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38958);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 38959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16117);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лус.
pub(crate) fn n3653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16118);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16119);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38961);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38963);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67019);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 67020);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92904);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112464);
											s.is_empty()
										}
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
					super::push(out, 38964);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 38965);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67021);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92905);
									s.is_empty()
								}
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
			super::push(out, 16120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16121);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38967);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16122);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38968);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 38969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 38970);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67024);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92906);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112465);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125150);
													match s {
														[] => true,
														['г', s @ ..] => super::t38::n134161(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38971);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67025);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 92907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мус.
pub(crate) fn n3654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3643);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16123);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38972);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16124);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38973);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 38974);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67028);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92908);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112466);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125151);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134162(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 112467);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125152);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134163(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38975);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 38976);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 38977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16125);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38978);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16126);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38979);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38980);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67030);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92909);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112468);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125153);
													s.is_empty()
												}
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
							super::push(out, 67031);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92910);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112469);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 125154);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16127);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38981);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67032);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92911);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112470);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125155);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134164(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16128);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нус.
pub(crate) fn n3655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3644);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16129);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38983);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38984);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67033);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92912);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112471);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16130);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38985);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38986);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 38987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67036);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 67037);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 92913);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112472);
											s.is_empty()
										}
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
					super::push(out, 38989);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67038);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92914);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 112473);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125156);
													match s {
														[] => true,
														['п', s @ ..] => super::t38::n134165(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67039);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92915);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112474);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125157);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38990);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67040);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92916);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112475);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125158);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38991);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67041);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92917);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112476);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 67042);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92918);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112477);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 92919);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112478);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125159);
													s.is_empty()
												}
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
							super::push(out, 67043);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92920);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112479);
											s.is_empty()
										}
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
					super::push(out, 38992);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67044);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 67045);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 38993);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67046);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92922);
									s.is_empty()
								}
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
			super::push(out, 16132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38994);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16133);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38995);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 67048);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92923);
									s.is_empty()
								}
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
			super::push(out, 16134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16135);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38997);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 38998);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 38999);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16136);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39001);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 16137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 16138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39003);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 39004);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67050);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оус.
pub(crate) fn n3656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3645);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 16139);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39005);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92925);
									s.is_empty()
								}
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
			super::push(out, 16140);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 16141);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39006);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пус.
pub(crate) fn n3657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3646);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16142);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16143);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39009);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							super::push(out, 67055);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92926);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112480);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125160);
													match s {
														[] => true,
														['г', s @ ..] => super::t38::n134166(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16144);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39010);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67056);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92927);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112481);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125161);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16145);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39011);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67057);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92928);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112482);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125162);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134167(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39012);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39013);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67059);
							match s {
								[] => true,
								['-', 'к', s @ ..] => {
									super::push(out, 92929);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112483);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125163);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n134168(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['а', 'и', 'в', 'а', '\u{1}', s @ ..] => {
									super::push(out, 92930);
									s.is_empty()
								}
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
			super::push(out, 16147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рус.
pub(crate) fn n3658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3647);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16148);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39015);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67060);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92931);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112484);
											s.is_empty()
										}
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
					super::push(out, 39016);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39017);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67062);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 16149);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39019);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 39020);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 39021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 39022);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67066);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 92933);
									s.is_empty()
								}
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
			super::push(out, 16150);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39023);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67067);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 92934);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112485);
											s.is_empty()
										}
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
					super::push(out, 39024);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67068);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92935);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112486);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125164);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134169(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16151);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67069);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 67070);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92936);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112487);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125165);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134170(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16152);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39026);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67072);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92937);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112488);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16153);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39027);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67073);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92938);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112489);
											s.is_empty()
										}
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
					super::push(out, 39028);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67074);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92939);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112490);
											s.is_empty()
										}
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
					super::push(out, 39029);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67075);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 67076);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 92940);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											super::push(out, 112491);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 67077);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39030);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67078);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 92942);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 112492);
											s.is_empty()
										}
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
					super::push(out, 39031);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67079);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39032);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 39033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16154);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16155);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39034);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 67080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39035);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 67081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16156);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39036);
					s.is_empty()
				}
				['а', s @ ..] => super::t38::n39411(s, out),
				['м', s @ ..] => super::t38::n39412(s, out),
				['р', s @ ..] => super::t38::n39413(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 16157);
			match s {
				[] => true,
				['ъ', s @ ..] => super::t38::n39414(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сус.
pub(crate) fn n3659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3648);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16158);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 39041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39042);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 39043);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16160);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39045);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16161);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39047);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67089);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92948);
									s.is_empty()
								}
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
			super::push(out, 16162);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 39048);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16163);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тус.
pub(crate) fn n3660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3649);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16164);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39050);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39051);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39052);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16165);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16166);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39053);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67093);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 67094);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39054);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39055);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67096);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92952);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112496);
											s.is_empty()
										}
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
					super::push(out, 39056);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39057);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39058);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67099);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 67100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39059);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67101);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92953);
									s.is_empty()
								}
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
			super::push(out, 16169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39060);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67103);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92954);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112497);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 67104);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92955);
									s.is_empty()
								}
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
			super::push(out, 16170);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 16171);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39062);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39063);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67106);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 67107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 16173);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39064);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фус.
pub(crate) fn n3661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3650);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16174);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39065);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67109);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92958);
									s.is_empty()
								}
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
			super::push(out, 16175);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 39066);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92959);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112498);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125167);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134171(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16176);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16177);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 39068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 16178);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хус.
pub(crate) fn n3662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3651);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16179);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -парацельс.
pub(crate) fn n126466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125168);
	s.is_empty()
}

/// Words ending -одактилюс.
pub(crate) fn n126467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125169);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 132803);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скрофулюс.
pub(crate) fn n126468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125170);
	s.is_empty()
}

/// Words ending -фейк-ньюс.
pub(crate) fn n126469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125171);
	s.is_empty()
}

/// Words ending --самотряс.
pub(crate) fn n126470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125172);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 132804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 137308);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139997);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141589);
									match s {
										[] => true,
										['ш', 'о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 142519);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбат.
pub(crate) fn n16316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16227);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 39125);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 67163);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 92995);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 112518);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125173);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39126);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 67164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 39127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67165);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сбат.
pub(crate) fn n16317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16228);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39128);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 67166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 39129);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 39130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ябат.
pub(crate) fn n16321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16232);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ват.
pub(crate) fn n3687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3676);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39132);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67167);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 92996);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112519);
											s.is_empty()
										}
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
					super::push(out, 39133);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67168);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92997);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112520);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16233);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39135);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67169);
							match s {
								[] => true,
								['а', 'е', s @ ..] => {
									super::push(out, 92998);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112521);
											s.is_empty()
										}
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
					super::push(out, 39136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39137);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39138);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67170);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92999);
									s.is_empty()
								}
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
			super::push(out, 16234);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 16235);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39140);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39141);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39142);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67172);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93000);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112522);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125174);
													s.is_empty()
												}
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
							super::push(out, 67173);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 93001);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 112523);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125175);
													match s {
														[] => true,
														['г', s @ ..] => super::t38::n134174(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67174);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93002);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112524);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125176);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гат.
pub(crate) fn n3688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16236);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39143);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39144);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 39145);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 67175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93003);
									s.is_empty()
								}
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
			super::push(out, 16237);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39146);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67176);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39147);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67177);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39148);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67178);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 67179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93007);
									s.is_empty()
								}
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
			super::push(out, 16238);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39149);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67180);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112525);
											s.is_empty()
										}
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
					super::push(out, 39150);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 67181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39151);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67182);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 67183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16240);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39152);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16241);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39153);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дат.
pub(crate) fn n3689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3678);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16242);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 16243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39154);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67186);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93009);
									s.is_empty()
								}
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
			super::push(out, 16244);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39155);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16245);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39156);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 39157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 39158);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 39159);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67191);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93010);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112526);
											s.is_empty()
										}
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
					super::push(out, 39160);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39161);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 39162);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16246);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39163);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 67195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39164);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67196);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93011);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 112527);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16247);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39165);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67197);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 93012);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112528);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125177);
													match s {
														[] => true,
														['ы', s @ ..] => super::t38::n134175(s, out),
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
									super::push(out, 93013);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112529);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125178);
													match s {
														[] => true,
														['у', s @ ..] => super::t38::n134176(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16248);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39166);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16249);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39167);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16250);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39169);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67199);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93014);
									s.is_empty()
								}
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
			super::push(out, 16251);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39170);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еат.
pub(crate) fn n3690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3679);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16252);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39171);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 67201);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93015);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112530);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125179);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
					super::push(out, 39172);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 39173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16253);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16254);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39174);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67203);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93016);
									s.is_empty()
								}
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
			super::push(out, 16255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жат.
pub(crate) fn n3691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3680);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16256);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16257);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16258);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 16259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зат.
pub(crate) fn n3692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3681);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16260);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39176);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 39177);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 39178);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67205);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93018);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112531);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125180);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39179);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93019);
									s.is_empty()
								}
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
			super::push(out, 16261);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39180);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67207);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'а', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 67210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39182);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67211);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 93022);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 93023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 67212);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93024);
									s.is_empty()
								}
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
			super::push(out, 16262);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39183);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67213);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 67214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16263);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39184);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93025);
									s.is_empty()
								}
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
			super::push(out, 16264);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67216);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93026);
									s.is_empty()
								}
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
			super::push(out, 16265);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39186);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иат.
pub(crate) fn n3693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3682);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 16266);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39187);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16267);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39188);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67219);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93027);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112532);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125181);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16268);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39189);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 67221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 39190);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67222);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 93029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39191);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67223);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 93030);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112533);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16269);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39192);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67224);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93031);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112534);
											s.is_empty()
										}
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
					super::push(out, 39193);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 67225);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 93032);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112535);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16270);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39194);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67226);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93033);
									s.is_empty()
								}
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
			super::push(out, 16271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39195);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67227);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 67228);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 67229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16273);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39197);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67231);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93035);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112536);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16274);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39198);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67232);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 93036);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112537);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125182);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'е', s @ ..] => {
					super::push(out, 39199);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67233);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93037);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112538);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кат.
pub(crate) fn n3694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3683);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16275);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16276);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39200);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 39201);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67234);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93038);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112539);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 67235);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 93039);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112540);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125183);
													match s {
														[] => true,
														['в', s @ ..] => super::t38::n134178(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16277);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39202);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 67236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93040);
									s.is_empty()
								}
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
			super::push(out, 16278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39203);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 67237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93041);
									s.is_empty()
								}
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
			super::push(out, 16279);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39204);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39205);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39206);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67238);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 67239);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93042);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112541);
											s.is_empty()
										}
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
					super::push(out, 39207);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67240);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93043);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112542);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125184);
													match s {
														[] => true,
														['м', s @ ..] => super::t38::n134179(s, out),
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
									super::push(out, 93044);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112543);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125185);
													match s {
														[] => true,
														['р', '-', s @ ..] => super::t38::n134180(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16280);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39208);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67241);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 39209);
					s.is_empty()
				}
				['л', 'б', s @ ..] => {
					super::push(out, 39210);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67242);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93046);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112544);
											s.is_empty()
										}
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
					super::push(out, 39211);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67243);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93047);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 67244);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39212);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39213);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67246);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 93049);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112545);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16281);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16282);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16284);
			match s {
				[] => true,
				['в', s @ ..] => super::t38::n39591(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лат.
pub(crate) fn n3695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3684);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16285);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39216);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 39217);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67248);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93050);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112546);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125186);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134181(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16286);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39218);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67249);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39219);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16287);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 39220);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67251);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 93052);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112547);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 93053);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112548);
											s.is_empty()
										}
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
					super::push(out, 39221);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67252);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 39222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16288);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16289);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39223);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39224);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67255);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93055);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112549);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125187);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16290);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16291);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16292);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67256);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 67257);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 67258);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 67259);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 39226);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39227);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67260);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 67261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39228);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39229);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67262);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93059);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112550);
											s.is_empty()
										}
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
					super::push(out, 39230);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67263);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93060);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112551);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16293);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39231);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мат.
pub(crate) fn n3696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3685);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16294);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39232);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 39233);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67265);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39234);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39235);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39236);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67268);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 67269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39237);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67270);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93064);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112552);
											s.is_empty()
										}
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
					super::push(out, 39238);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39239);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16295);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 16296);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39240);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67273);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39241);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67274);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93066);
									s.is_empty()
								}
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
			super::push(out, 16297);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39242);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39243);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67276);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93067);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112553);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125188);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134182(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39244);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 16298);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39245);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67278);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112554);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16299);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39246);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67279);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93069);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112555);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 67280);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93070);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 112556);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 67281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16300);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39247);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67282);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 67283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 39248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39249);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67285);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93071);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 93072);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112557);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 67286);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93073);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112558);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 67287);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 67288);
							match s {
								[] => true,
								['о', s @ ..] => super::t38::n94072(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t38::n67987(s, out),
						['с', s @ ..] => super::t38::n67988(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t38::n39626(s, out),
				['т', s @ ..] => super::t38::n39627(s, out),
				['х', s @ ..] => super::t38::n39628(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t38::n16395(s, out),
		['о', s @ ..] => super::t38::n16396(s, out),
		['р', s @ ..] => super::t38::n16397(s, out),
		['т', s @ ..] => {
			super::push(out, 16304);
			s.is_empty()
		}
		['х', s @ ..] => super::t38::n16399(s, out),
		['ш', s @ ..] => super::t38::n16400(s, out),
		_ => false,
	}
}

/// Words ending -нат.
pub(crate) fn n3697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3686);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16307);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16308);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39264);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 39265);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67306);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93089);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 67307);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93090);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112567);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125194);
													s.is_empty()
												}
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
							super::push(out, 67308);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93091);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112568);
											s.is_empty()
										}
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
					super::push(out, 39266);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67309);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 93092);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 93093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 67310);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93094);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112569);
											s.is_empty()
										}
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
					super::push(out, 39267);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39268);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67311);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39269);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39270);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39271);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67313);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 93097);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112570);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125195);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134186(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39272);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67314);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 39273);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67315);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93099);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112571);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 67316);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93100);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112572);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125196);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134187(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67317);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93101);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112573);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 67318);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93102);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112574);
											s.is_empty()
										}
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
					super::push(out, 39274);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67319);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 67320);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93104);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112575);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125197);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16309);
			match s {
				[] => true,
				['а', s @ ..] => super::t38::n39651(s, out),
				['и', s @ ..] => super::t38::n39652(s, out),
				['о', s @ ..] => {
					super::push(out, 39277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t38::n16404(s, out),
		['и', s @ ..] => super::t38::n16405(s, out),
		['н', s @ ..] => super::t38::n16406(s, out),
		['о', s @ ..] => super::t38::n16407(s, out),
		['р', s @ ..] => super::t38::n16408(s, out),
		['у', s @ ..] => super::t38::n16409(s, out),
		_ => false,
	}
}

/// Words ending -оат.
pub(crate) fn n3698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3687);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 16316);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16317);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16318);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 39305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67347);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 67348);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93127);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112590);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16319);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пат.
pub(crate) fn n3699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3688);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16320);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16321);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16322);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39307);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67350);
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
				['т', s @ ..] => {
					super::push(out, 39308);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 39309);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67351);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93128);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112591);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16323);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16324);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39310);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39311);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67352);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93129);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112592);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125206);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16325);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16326);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16327);
			s.is_empty()
		}
		_ => false,
	}
}

