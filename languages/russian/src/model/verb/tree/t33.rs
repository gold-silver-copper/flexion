use flexion::Path;

/// Words ending -рмироваться.
pub(crate) fn n68647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36840);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41876);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 41877);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43357);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43643);
											match s {
												[] => true,
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
									super::push(out, 43358);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43644);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 43645);
											match s {
												[] => true,
												['а', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41878);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43359);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43646);
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
					super::push(out, 41879);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42840);
							match s {
								[] => true,
								['о', 'л', 'х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42841);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42842);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43360);
									match s {
										[] => true,
										['р', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умироваться.
pub(crate) fn n68648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36841);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 40098);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41881);
					match s {
						[] => true,
						['к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 40099);
			match s {
				[] => true,
				['к', 'а', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фмироваться.
pub(crate) fn n68649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36842);
	match s {
		[] => true,
		['и', 'р', 'а', 'г', 'о', 'л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьмироваться.
pub(crate) fn n68650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36843);
	match s {
		[] => true,
		['л', 'и', 'ф', 'о', 'р', 'к', 'и', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юмироваться.
pub(crate) fn n68651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36844);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 40100);
			match s {
				[] => true,
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 40101);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41882);
					match s {
						[] => true,
						['о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инироваться.
pub(crate) fn n68655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36848);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40113);
			match s {
				[] => true,
				['м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42853);
							match s {
								[] => true,
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
		['г', 'а', 'в', 'н', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 40114);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41889);
					match s {
						[] => true,
						['р', 'е', 'т', 'е', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41890);
					match s {
						[] => true,
						['о', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43363);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43364);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'а', 'г', 'а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 40115);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41891);
					match s {
						[] => true,
						['и', 'ц', 'с', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 42854);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 41893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42855);
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
				['и', s @ ..] => {
					super::push(out, 41894);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42856);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 43365);
									match s {
										[] => true,
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
							super::push(out, 42857);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['н', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'и', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 41895);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43366);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 41896);
					match s {
						[] => true,
						['е', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'т', 'и', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 41897);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43367);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 43368);
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
		['т', s @ ..] => {
			super::push(out, 40116);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42858);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43369);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 43647);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'ь', 'л', 'и', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 40117);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 40118);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41899);
					match s {
						[] => true,
						['а', 'в', s @ ..] => super::t33::n87448(s, out),
						_ => false,
					}
				}
				['ь', s @ ..] => super::t33::n84399(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онироваться.
pub(crate) fn n68658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36850);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 40121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40122);
			match s {
				[] => true,
				['с', 'н', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42859);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43371);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 43372);
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
						['и', s @ ..] => {
							super::push(out, 42860);
							match s {
								[] => true,
								['д', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'о', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43650);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43769);
													match s {
														[] => true,
														['е', 'п', '\u{1}', s @ ..] => super::t33::n90290(s, out),
														_ => false,
													}
												}
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
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43373);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 43374);
									match s {
										[] => true,
										['л', 'л', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 43375);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43651);
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
						['ю', s @ ..] => {
							super::push(out, 42861);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 43376);
									match s {
										[] => true,
										['о', 'в', 'э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 40123);
			match s {
				[] => true,
				['а', 'т', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'ш', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'р', 'а', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 40124);
			match s {
				[] => true,
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'н', 'а', 'р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40125);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42862);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43377);
									match s {
										[] => true,
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
							super::push(out, 42863);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43378);
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
				['д', s @ ..] => {
					super::push(out, 41903);
					match s {
						[] => true,
						['у', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41904);
					match s {
						[] => true,
						['а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 40126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42864);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43379);
									match s {
										[] => true,
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
							super::push(out, 42865);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42866);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43380);
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
		['ш', 'р', 'о', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтернироваться.
pub(crate) fn n87467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42867);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 43381);
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

/// Words ending -орнироваться.
pub(crate) fn n78612(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унироваться.
pub(crate) fn n68660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36852);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 40128);
			match s {
				[] => true,
				['а', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отксерокопироваться.
pub(crate) fn n90155(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -альпироваться.
pub(crate) fn n84449(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 42875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульпироваться.
pub(crate) fn n84450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41912);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отпрепарироваться.
pub(crate) fn n90156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43770);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word скооперироваться.
pub(crate) fn n90157(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ирироваться.
pub(crate) fn n68680(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 41921);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43385);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43656);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
		_ => false,
	}
}

/// Words ending -орироваться.
pub(crate) fn n68681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36869);
	match s {
		[] => true,
		['б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'з', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'л', 'е', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 40153);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42881);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43386);
									match s {
										[] => true,
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
		['л', 'х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'г', 'и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41922);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42882);
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
		['п', 'р', 'о', 'к', 'н', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['и', 'н', 'о', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трироваться.
pub(crate) fn n68682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36870);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40154);
			match s {
				[] => true,
				['м', 'о', 'н', 'о', 'р', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42883);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42884);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42885);
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
		['и', s @ ..] => {
			super::push(out, 40155);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 40156);
			match s {
				[] => true,
				['е', 'ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42886);
							s.is_empty()
						}
						['н', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43387);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43388);
									match s {
										[] => true,
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
							super::push(out, 42887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 42888);
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41923);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'в', 'к', 'е', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42889);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43657);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43658);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43771);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90292(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 43659);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'и', 'м', 'д', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'о', 'м', 'е', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42890);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43389);
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
				['у', 'р', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ю', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 43390);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 43660);
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
						['р', 'е', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -урироваться.
pub(crate) fn n68683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36871);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 40157);
			match s {
				[] => true,
				['и', 'ф', 'н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'т', 'в', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'н', 'е', 'ц', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 40158);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41924);
					match s {
						[] => true,
						['н', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'у', 'р', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 42891);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -ифрироваться.
pub(crate) fn n78678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40159);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 41925);
			match s {
				[] => true,
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -офрироваться.
pub(crate) fn n78679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40160);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 42892);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 43391);
							match s {
								[] => true,
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

/// Words ending -эрироваться.
pub(crate) fn n68685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36873);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 41926);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word карбюрироваться.
pub(crate) fn n68686(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word разбалансироваться.
pub(crate) fn n90162(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word дисбалансироваться.
pub(crate) fn n90163(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -нансироваться.
pub(crate) fn n84518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41932);
	match s {
		[] => true,
		['и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 43664);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43772);
									match s {
										[] => true,
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

/// Words ending -юансироваться.
pub(crate) fn n84519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41933);
	match s {
		[] => true,
		['н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енсироваться.
pub(crate) fn n78690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40168);
	match s {
		[] => true,
		['д', 'н', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 42900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'м', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 42901);
					match s {
						[] => true,
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

/// Words ending -онсироваться.
pub(crate) fn n78691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40169);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 41934);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 41935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42902);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 42903);
					match s {
						[] => true,
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

/// Words ending -рсироваться.
pub(crate) fn n68692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36878);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ссироваться.
pub(crate) fn n68693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36879);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40170);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41936);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42904);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43397);
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
				['л', s @ ..] => {
					super::push(out, 41937);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42905);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43398);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 41938);
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
				['п', s @ ..] => {
					super::push(out, 41939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41940);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40171);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43399);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43400);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 43665);
											match s {
												[] => true,
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
									super::push(out, 43401);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43666);
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
								['ы', s @ ..] => {
									super::push(out, 43402);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 43667);
											match s {
												[] => true,
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
							super::push(out, 42908);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43403);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 43404);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 43668);
											match s {
												[] => true,
												['э', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40172);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 41942);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 43405);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 43669);
											match s {
												[] => true,
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
					super::push(out, 41943);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42909);
							match s {
								[] => true,
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
			super::push(out, 40173);
			match s {
				[] => true,
				['д', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 40174);
			match s {
				[] => true,
				['к', 'с', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усироваться.
pub(crate) fn n68694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36880);
	match s {
		[] => true,
		['к', 'о', 'ф', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 41944);
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
		_ => false,
	}
}

/// The word эмульсироваться.
pub(crate) fn n68695(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word продюсироваться.
pub(crate) fn n68696(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -петироваться.
pub(crate) fn n78716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40184);
	match s {
		[] => true,
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 42915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 42916);
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

/// Words ending -ретироваться.
pub(crate) fn n78717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40185);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 41955);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 41956);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 43409);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 41957);
			match s {
				[] => true,
				['п', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'е', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43410);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43773);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43804);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 42917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итироваться.
pub(crate) fn n68700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36884);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 40186);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41959);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42918);
							match s {
								[] => true,
								['к', 'с', 'и', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'и', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 40187);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'о', 'р', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'б', 'а', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 40188);
			match s {
				[] => true,
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 41960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 41961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 40189);
			match s {
				[] => true,
				['а', 'р', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ь', 'л', 'у', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 40190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41962);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 41963);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41964);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42919);
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

/// Words ending -ктироваться.
pub(crate) fn n68702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36885);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 41965);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43412);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 43413);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 43414);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 43415);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 43416);
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
				['л', s @ ..] => {
					super::push(out, 41966);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42920);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 42921);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 43417);
									match s {
										[] => true,
										['о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 41967);
					match s {
						[] => true,
						['м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41968);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43418);
									match s {
										[] => true,
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
			super::push(out, 40192);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 41969);
					match s {
						[] => true,
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 41970);
					match s {
						[] => true,
						['п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 42922);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43419);
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
							super::push(out, 42923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'с', 'н', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43420);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43670);
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
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43421);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43671);
											match s {
												[] => true,
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
					super::push(out, 41971);
					match s {
						[] => true,
						['р', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43422);
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41972);
					match s {
						[] => true,
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 41973);
					match s {
						[] => true,
						['ф', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 41974);
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
		['н', s @ ..] => {
			super::push(out, 40193);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41975);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43423);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43424);
									match s {
										[] => true,
										['р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'р', 'т', 'с', 'н', 'и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41976);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42924);
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

/// Words ending -нтироваться.
pub(crate) fn n68703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36886);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40194);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41977);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 43672);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 43774);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90295(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 43673);
											match s {
												[] => true,
												['а', 'р', 'т', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90296(s, out),
														['е', 'р', '\u{1}', s @ ..] => super::t33::n90297(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41978);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42925);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41979);
					match s {
						[] => true,
						['н', 'и', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 41980);
					match s {
						[] => true,
						['е', 'р', 'п', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 41981);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43425);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43674);
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
									super::push(out, 43426);
									match s {
										[] => true,
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 43427);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43675);
											s.is_empty()
										}
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
							super::push(out, 42926);
							match s {
								[] => true,
								['н', 'р', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 42927);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43428);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43676);
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
								['и', s @ ..] => {
									super::push(out, 43429);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 42928);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 43430);
									match s {
										[] => true,
										['п', 'м', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43677);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 43678);
											match s {
												[] => true,
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
							super::push(out, 42929);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43431);
									match s {
										[] => true,
										['е', 'п', 'с', 'к', 'э', 'а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 42930);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43432);
									match s {
										[] => true,
										['ф', '\u{1}', s @ ..] => super::t33::n89862(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t33::n87641(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t33::n84601(s, out),
				['ц', s @ ..] => super::t33::n84602(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t33::n78731(s, out),
		['у', s @ ..] => super::t33::n78732(s, out),
		_ => false,
	}
}

/// Words ending -отироваться.
pub(crate) fn n68704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36887);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40197);
			match s {
				[] => true,
				['а', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 40198);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'ь', 'л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'ж', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 40199);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 40200);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['й', 'о', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 40201);
			match s {
				[] => true,
				['и', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 41987);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43434);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 43435);
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
				['ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'а', 'к', 'с', 'э', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 40202);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'а', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 40203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птироваться.
pub(crate) fn n68705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36888);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40204);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41988);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43436);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43683);
											match s {
												[] => true,
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
									super::push(out, 43437);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43684);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 43438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43685);
											s.is_empty()
										}
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
					super::push(out, 41989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40205);
			match s {
				[] => true,
				['с', 'и', 'т', 'н', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40206);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртироваться.
pub(crate) fn n68706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36889);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40207);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41990);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42938);
							match s {
								[] => true,
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 41991);
					match s {
						[] => true,
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
			super::push(out, 40208);
			match s {
				[] => true,
				['в', 'н', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42940);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43686);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43775);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90298(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 43687);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43776);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90299(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 40209);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41992);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 41993);
					match s {
						[] => true,
						['с', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41994);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42941);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 43439);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43688);
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 42942);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 42943);
							match s {
								[] => true,
								['к', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'а', 'р', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42944);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
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

/// Words ending -стироваться.
pub(crate) fn n68707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36890);
	match s {
		[] => true,
		['а', 'л', 'л', 'а', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 40210);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43440);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43689);
											match s {
												[] => true,
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
				['ж', 'д', 'й', 'а', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42945);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43441);
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
				['ф', 'и', 'н', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40211);
			match s {
				[] => true,
				['н', 'м', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'с', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'г', 'а', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 42946);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43442);
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
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', 'е', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42947);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43443);
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
				['р', 'к', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 40212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41996);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 41997);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42948);
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

/// Words ending -утироваться.
pub(crate) fn n68708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40213);
			match s {
				[] => true,
				['к', 'о', s @ ..] => {
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
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
		['б', s @ ..] => {
			super::push(out, 40214);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41998);
					match s {
						[] => true,
						['р', 'т', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 40215);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 40216);
			match s {
				[] => true,
				['м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 42949);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43444);
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
		['о', s @ ..] => {
			super::push(out, 40217);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 41999);
					match s {
						[] => true,
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'м', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'к', 'е', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтироваться.
pub(crate) fn n68709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36892);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42950);
							match s {
								[] => true,
								['к', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 42951);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 43445);
									match s {
										[] => true,
										['а', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 43777);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t33::n90300(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
						['з', s @ ..] => {
							super::push(out, 42952);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43446);
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
						['п', 'а', 'т', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43447);
									match s {
										[] => true,
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
							super::push(out, 42953);
							match s {
								[] => true,
								['н', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43690);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43778);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t33::n90301(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютироваться.
pub(crate) fn n68710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36893);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42000);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42954);
							match s {
								[] => true,
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
					super::push(out, 42001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40218);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 40219);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42002);
					match s {
						[] => true,
						['р', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word стенографироваться.
pub(crate) fn n90195(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -астенографироваться.
pub(crate) fn n90196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43779);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 43805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word литографироваться.
pub(crate) fn n90197(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word гектографироваться.
pub(crate) fn n90198(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -фотографироваться.
pub(crate) fn n90199(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43806);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43807);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43808);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 43809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word картографироваться.
pub(crate) fn n90200(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -фицироваться.
pub(crate) fn n78784(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42011);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'д', '\u{1}', s @ ..] => {
					super::push(out, 42965);
					s.is_empty()
				}
				['л', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42012);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 42966);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'а', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 42967);
					match s {
						[] => true,
						['а', 'в', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 43780);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 43810);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t33::n90345(s, out),
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
									super::push(out, 43695);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 43781);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'у', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'м', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 42968);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 43452);
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
				['н', s @ ..] => {
					super::push(out, 42969);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', 'м', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43453);
							match s {
								[] => true,
								['к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43782);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43811);
													match s {
														[] => true,
														['е', 'п', '\u{1}', s @ ..] => super::t33::n90346(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 43696);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 43697);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43783);
											match s {
												[] => true,
												['л', 'э', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t33::n87726(s, out),
				['т', s @ ..] => super::t33::n87727(s, out),
				['ц', s @ ..] => {
					super::push(out, 42971);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43460);
							match s {
								[] => true,
								['п', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 43461);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 43705);
									match s {
										[] => true,
										['а', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t33::n84684(s, out),
		['о', s @ ..] => {
			super::push(out, 42014);
			match s {
				[] => true,
				['и', 'д', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'п', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'и', 'к', '\u{1}', s @ ..] => super::t33::n87732(s, out),
				_ => false,
			}
		}
		['ф', 'а', '\u{1}', s @ ..] => super::t33::n84686(s, out),
		_ => false,
	}
}

/// Words ending -оцироваться.
pub(crate) fn n68726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36901);
	match s {
		[] => true,
		['в', 'о', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 42017);
					match s {
						[] => true,
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'и', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 42973);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43463);
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
		_ => false,
	}
}

/// Words ending -уцироваться.
pub(crate) fn n68728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36902);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сформировываться.
pub(crate) fn n87830(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -нировываться.
pub(crate) fn n79115(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'л', 'п', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'т', s @ ..] => {
			match s {
				[] => true,
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
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

/// Words ending -пировываться.
pub(crate) fn n79116(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'р', 'д', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'к', 'ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'у', 'р', 'г', s @ ..] => {
			match s {
				[] => true,
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

/// Words ending -сировываться.
pub(crate) fn n79117(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', 'у', 'б', s @ ..] => {
			match s {
				[] => true,
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'е', 'р', 'д', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тировываться.
pub(crate) fn n79118(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'л', 'л', 'а', 'б', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'в', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
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

/// Words ending -воровываться.
pub(crate) fn n68985(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тровываться.
pub(crate) fn n68986(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', 'т', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'е', 'к', 'р', 'о', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', 'л', 'и', 'ф', s @ ..] => {
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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

/// Words ending -уровываться.
pub(crate) fn n68987(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'ш', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ифровываться.
pub(crate) fn n68988(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ц', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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

/// The word заблагодушествовать.
pub(crate) fn n90222(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поблагодушествовать.
pub(crate) fn n90223(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word помалодушествовать.
pub(crate) fn n90224(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word денационализировать.
pub(crate) fn n90324(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ренационализировать.
pub(crate) fn n90325(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word докапитализировать.
pub(crate) fn n90326(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -наимпровизироваться.
pub(crate) fn n90260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43796);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -заидеологизироваться.
pub(crate) fn n90261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43797);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лизироваться.
pub(crate) fn n78470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40040);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 41814);
			match s {
				[] => true,
				['б', 'о', 'л', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', 'е', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 42737);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 43592);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 42738);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43293);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 43294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 42739);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['а', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'е', 'в', 'д', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'т', 'с', 'у', 'д', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ц', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43593);
									s.is_empty()
								}
								['и', 'н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'а', 'т', 'с', 'б', 'у', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 43594);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 43748);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 43798);
													match s {
														[] => true,
														['м', 'о', 'к', '\u{1}', s @ ..] => super::t33::n90332(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', 'т', 'и', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 42740);
					match s {
						[] => true,
						['и', 'д', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 42741);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43595);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 42742);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43596);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43749);
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
						['г', 'и', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 43296);
							match s {
								[] => true,
								['г', s @ ..] => super::t33::n89661(s, out),
								['м', s @ ..] => super::t33::n89662(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t33::n88774(s, out),
						_ => false,
					}
				}
				['п', 'и', 'ц', 'и', 'н', 'у', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => super::t33::n87209(s, out),
				['с', s @ ..] => super::t33::n87210(s, out),
				['т', s @ ..] => super::t33::n87211(s, out),
				['у', s @ ..] => super::t33::n87212(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t33::n84218(s, out),
		['и', s @ ..] => super::t33::n84219(s, out),
		['л', s @ ..] => super::t33::n84220(s, out),
		['о', s @ ..] => super::t33::n84221(s, out),
		_ => false,
	}
}

/// Words ending -мизироваться.
pub(crate) fn n78471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40041);
	match s {
		[] => true,
		['а', 'н', 'и', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43309);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 43608);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 42748);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 43310);
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
				['т', s @ ..] => {
					super::push(out, 42749);
					match s {
						[] => true,
						['и', 'г', 'е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 42750);
					match s {
						[] => true,
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
			match s {
				[] => true,
				['н', 'о', 'к', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42751);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43311);
							match s {
								[] => true,
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
			super::push(out, 41819);
			match s {
				[] => true,
				['и', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'г', 'л', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низироваться.
pub(crate) fn n78472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40042);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41820);
			match s {
				[] => true,
				['б', 'р', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ь', 'л', 'а', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 42752);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 43312);
							match s {
								[] => true,
								['с', 'и', 'р', 'х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43313);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43609);
									match s {
										[] => true,
										['е', 'м', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'у', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 42753);
					match s {
						[] => true,
						['о', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'м', 'а', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 42754);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 43314);
							match s {
								[] => true,
								['э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', 'е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'л', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41821);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42755);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', 'о', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'н', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42756);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 43315);
							match s {
								[] => true,
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
					super::push(out, 42757);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 43316);
							match s {
								[] => true,
								['ю', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'в', 'у', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'с', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42758);
					match s {
						[] => true,
						['р', 'к', 'у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43610);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43758);
											match s {
												[] => true,
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
				['л', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 42759);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43317);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 43611);
									match s {
										[] => true,
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
				['т', 'а', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'ж', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41822);
			match s {
				[] => true,
				['б', 'р', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 42760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ц', 'ю', 'л', 'о', 'в', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 42761);
					match s {
						[] => true,
						['б', 'а', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'д', '\u{1}', s @ ..] => {
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
				['м', s @ ..] => {
					super::push(out, 42762);
					match s {
						[] => true,
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'а', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 42763);
					match s {
						[] => true,
						['х', 'н', 'и', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43612);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43759);
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
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t33::n88835(s, out),
						['о', 'к', '\u{1}', s @ ..] => super::t33::n88836(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t33::n87256(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t33::n84230(s, out),
		['у', 'м', 'м', 'и', '\u{1}', s @ ..] => super::t33::n84231(s, out),
		['х', 'е', 'т', 'и', 'л', 'о', 'п', '\u{1}', s @ ..] => super::t33::n84232(s, out),
		['я', 'в', 'а', 'л', 'с', '\u{1}', s @ ..] => super::t33::n84233(s, out),
		_ => false,
	}
}

/// Words ending -ризироваться.
pub(crate) fn n78475(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41824);
			match s {
				[] => true,
				['б', 'р', 'е', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', 'ь', 'л', 'у', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43321);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 43616);
									match s {
										[] => true,
										['о', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'е', 'г', 'е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 42765);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43322);
							match s {
								[] => true,
								['к', 'у', 'н', 'е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 42766);
					match s {
						[] => true,
						['е', 'л', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'л', 'и', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'е', 'в', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41825);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42767);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 43323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', 'ь', 'л', 'у', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['й', 'е', 'в', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'р', 'е', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42768);
					match s {
						[] => true,
						['е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ю', 'ь', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
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
				['г', 'е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 42769);
					match s {
						[] => true,
						['р', 'е', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43617);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43760);
											match s {
												[] => true,
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
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 43324);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43618);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', 'а', 'р', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41826);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42770);
					match s {
						[] => true,
						['л', 'л', 'е', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 41827);
			match s {
				[] => true,
				['л', 'у', s @ ..] => {
					match s {
						[] => true,
						['к', 'е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизироваться.
pub(crate) fn n78477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40043);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41828);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43619);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 43325);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 43620);
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
				['к', s @ ..] => {
					super::push(out, 42771);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43326);
							match s {
								[] => true,
								['л', 'и', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 42772);
					match s {
						[] => true,
						['а', 'р', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 43327);
							match s {
								[] => true,
								['е', 'л', 'ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 43328);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['а', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'и', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'л', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43621);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', 'а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43329);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 43622);
									match s {
										[] => true,
										['с', 'к', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 43623);
									match s {
										[] => true,
										['а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['х', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 43624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'о', 'ф', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42773);
					match s {
						[] => true,
						['к', 'о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 43625);
									match s {
										[] => true,
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'ю', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43761);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43802);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90338(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 43762);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43803);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90339(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', 'с', 'о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42774);
					match s {
						[] => true,
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 42775);
					match s {
						[] => true,
						['р', 'е', 'н', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'р', 'е', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 43330);
							match s {
								[] => true,
								['з', s @ ..] => super::t33::n89730(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => super::t33::n88872(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t33::n87283(s, out),
				['р', s @ ..] => super::t33::n87284(s, out),
				['т', s @ ..] => super::t33::n87285(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t33::n84241(s, out),
		['н', s @ ..] => super::t33::n84242(s, out),
		['о', s @ ..] => super::t33::n84243(s, out),
		['р', s @ ..] => super::t33::n84244(s, out),
		['у', s @ ..] => super::t33::n84245(s, out),
		['э', s @ ..] => super::t33::n84246(s, out),
		['ю', s @ ..] => super::t33::n84247(s, out),
		_ => false,
	}
}

/// Words ending -вакцинироваться.
pub(crate) fn n87448(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 43648);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43768);
							match s {
								[] => true,
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
			super::push(out, 43370);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43649);
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

/// Words ending -ьцинироваться.
pub(crate) fn n84399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41900);
	match s {
		[] => true,
		['л', 'а', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word перепозиционироваться.
pub(crate) fn n90290(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word зарегистрироваться.
pub(crate) fn n90292(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word переинтерпретироваться.
pub(crate) fn n90340(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word эксплантироваться.
pub(crate) fn n90295(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word трансплантироваться.
pub(crate) fn n90296(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ретрансплантироваться.
pub(crate) fn n90297(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ферментироваться.
pub(crate) fn n89862(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ументироваться.
pub(crate) fn n87641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42932);
	match s {
		[] => true,
		['г', 'р', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тентироваться.
pub(crate) fn n84601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41982);
	match s {
		[] => true,
		['а', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -центироваться.
pub(crate) fn n84602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41983);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 42933);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43679);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онтироваться.
pub(crate) fn n78731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40195);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 41984);
			match s {
				[] => true,
				['с', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 42934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 43680);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 43681);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 43682);
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
				['з', s @ ..] => {
					super::push(out, 42935);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 42936);
					match s {
						[] => true,
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
			super::push(out, 41985);
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41986);
			match s {
				[] => true,
				['ф', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унтироваться.
pub(crate) fn n78732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40196);
	match s {
		[] => true,
		['ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word реконвертироваться.
pub(crate) fn n90298(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отконвертироваться.
pub(crate) fn n90299(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заасфальтироваться.
pub(crate) fn n90300(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word проконсультироваться.
pub(crate) fn n90301(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word переквалифицироваться.
pub(crate) fn n90345(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перетарифицироваться.
pub(crate) fn n90346(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -сифицироваться.
pub(crate) fn n87726(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 43454);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43698);
					match s {
						[] => true,
						['в', 'и', 'д', '\u{1}', s @ ..] => {
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
				['а', s @ ..] => {
					super::push(out, 43699);
					match s {
						[] => true,
						['л', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43812);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43817);
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
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 43700);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43784);
							match s {
								[] => true,
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
			super::push(out, 43455);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43785);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 43813);
									match s {
										[] => true,
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
		['ь', 'л', 'а', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тифицироваться.
pub(crate) fn n87727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42970);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43456);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 43457);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43701);
					match s {
						[] => true,
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
			super::push(out, 43458);
			match s {
				[] => true,
				['а', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'е', s @ ..] => {
			match s {
				[] => true,
				['д', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 43702);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 43786);
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
				_ => false,
			}
		}
		['о', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 43459);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43704);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43787);
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
				['и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нфицироваться.
pub(crate) fn n84684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42013);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 43462);
					match s {
						[] => true,
						['е', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43788);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43814);
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
		_ => false,
	}
}

/// The word кинофицироваться.
pub(crate) fn n87732(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word аффицироваться.
pub(crate) fn n84686(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word коммерциализироваться.
pub(crate) fn n90332(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -гинализироваться.
pub(crate) fn n89661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43597);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 43750);
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

/// Words ending -минализироваться.
pub(crate) fn n89662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43598);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43751);
			match s {
				[] => true,
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43752);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43799);
					match s {
						[] => true,
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онализироваться.
pub(crate) fn n88774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43297);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['г', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'с', 'е', 'ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'а', 'т', 'с', 'б', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'т', 'и', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рализироваться.
pub(crate) fn n87209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42743);
	match s {
		[] => true,
		['а', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43599);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43753);
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
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['а', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['й', 'е', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сализироваться.
pub(crate) fn n87210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42744);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 43298);
			match s {
				[] => true,
				['е', 'в', 'и', 'н', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тализироваться.
pub(crate) fn n87211(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43299);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 43600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43754);
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
		['е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 43300);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'о', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уализироваться.
pub(crate) fn n87212(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', 'и', 'в', 'и', 'д', 'н', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'и', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43601);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'л', 'л', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'е', 'ц', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 43602);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43755);
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

/// Words ending -глизироваться.
pub(crate) fn n84218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41815);
	s.is_empty()
}

/// Words ending -илизироваться.
pub(crate) fn n84219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41816);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43301);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43756);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43800);
											match s {
												[] => true,
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
									super::push(out, 43757);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 43302);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43603);
							match s {
								[] => true,
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
					super::push(out, 43303);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43604);
							match s {
								[] => true,
								['н', 'е', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43801);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43816);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90353(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43304);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 43605);
							match s {
								[] => true,
								['о', 'т', 'в', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 42745);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43305);
					match s {
						[] => true,
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

/// Words ending -ллизироваться.
pub(crate) fn n84220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41817);
	match s {
		[] => true,
		['а', 'т', 'с', 'и', 'р', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 43306);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 43606);
							match s {
								[] => true,
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

/// Words ending -олизироваться.
pub(crate) fn n84221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41818);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43307);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 43607);
							match s {
								[] => true,
								['е', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'п', 'и', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', 'м', 'и', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 42746);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43308);
					match s {
						[] => true,
						['к', 'л', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'о', 'н', 'о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 42747);
			match s {
				[] => true,
				['в', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'к', 'а', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word тонизироваться.
pub(crate) fn n88835(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word котонизироваться.
pub(crate) fn n88836(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -фонизироваться.
pub(crate) fn n87256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42764);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43318);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43613);
					match s {
						[] => true,
						['е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 43319);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43614);
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

/// Words ending -рнизироваться.
pub(crate) fn n84230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41823);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', 'о', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 43320);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43615);
							match s {
								[] => true,
								['е', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word иммунизироваться.
pub(crate) fn n84231(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word политехнизироваться.
pub(crate) fn n84232(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word славянизироваться.
pub(crate) fn n84233(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word забюрократизироваться.
pub(crate) fn n90338(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word дебюрократизироваться.
pub(crate) fn n90339(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -загерметизироваться.
pub(crate) fn n89730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43626);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -згерметизироваться.
pub(crate) fn n88872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43331);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43627);
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

/// Words ending -нетизироваться.
pub(crate) fn n87283(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 43332);
			match s {
				[] => true,
				['а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 43333);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43628);
					match s {
						[] => true,
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
			super::push(out, 43334);
			match s {
				[] => true,
				['е', 'б', 'и', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретизироваться.
pub(crate) fn n87284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42776);
	match s {
		[] => true,
		['к', 'н', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43629);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43763);
							match s {
								[] => true,
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

/// Words ending -тетизироваться.
pub(crate) fn n87285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42777);
	match s {
		[] => true,
		['с', 'э', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -итизироваться.
pub(crate) fn n84241(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 42778);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43335);
					match s {
						[] => true,
						['м', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 42779);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 43630);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43764);
									match s {
										[] => true,
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
		['ф', s @ ..] => {
			super::push(out, 42780);
			match s {
				[] => true,
				['а', 'р', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтизироваться.
pub(crate) fn n84242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41829);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43336);
					match s {
						[] => true,
						['т', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43631);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43632);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43765);
									match s {
										[] => true,
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

/// Words ending -отизироваться.
pub(crate) fn n84243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41830);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 42781);
			match s {
				[] => true,
				['о', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'р', 'а', 'н', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43337);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43633);
							match s {
								[] => true,
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
		['н', 'п', 'и', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 43338);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43634);
							match s {
								[] => true,
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

/// Words ending -ртизироваться.
pub(crate) fn n84244(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 43339);
					match s {
						[] => true,
						['н', 'а', 'т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 43340);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43635);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 42782);
			match s {
				[] => true,
				['м', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 43636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'с', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утизироваться.
pub(crate) fn n84245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41831);
	match s {
		[] => true,
		['р', 'ш', 'р', 'а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -этизироваться.
pub(crate) fn n84246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41832);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 42783);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -ютизироваться.
pub(crate) fn n84247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41833);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 42784);
			match s {
				[] => true,
				['о', 'с', 'б', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word десенсибилизироваться.
pub(crate) fn n90353(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

