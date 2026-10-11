use flexion::Path;

/// Words ending -бниться.
pub(crate) fn n13570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10970);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 19204);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33452);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33453);
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

/// Words ending -вниться.
pub(crate) fn n13571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10971);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19205);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27146);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33454);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19206);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33455);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33456);
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
				['р', s @ ..] => {
					super::push(out, 27149);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 33457);
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

/// Words ending -гниться.
pub(crate) fn n13572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10972);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 19207);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27150);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 27152);
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

/// Words ending -дниться.
pub(crate) fn n13573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10973);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19208);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27153);
					match s {
						[] => true,
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
			super::push(out, 19209);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 33458);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'с', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27154);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37728);
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
				['о', s @ ..] => {
					super::push(out, 27155);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19210);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27156);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['\u{1}', s @ ..] => {
							super::push(out, 33459);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37729);
									s.is_empty()
								}
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
			super::push(out, 19211);
			match s {
				[] => true,
				['р', 'т', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ениться.
pub(crate) fn n13574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10974);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19212);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33461);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 33462);
							match s {
								[] => true,
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
					super::push(out, 27158);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37730);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37731);
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
									super::push(out, 37732);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40513);
											match s {
												[] => true,
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
					super::push(out, 27159);
					match s {
						[] => true,
						['б', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19213);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27160);
					match s {
						[] => true,
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37733);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37734);
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
				['р', 'е', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33463);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33464);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19214);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27161);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37735);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 37736);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37737);
									match s {
										[] => true,
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
					super::push(out, 27162);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33465);
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
		['ж', s @ ..] => {
			super::push(out, 19215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27163);
					s.is_empty()
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27164);
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
		['з', s @ ..] => {
			super::push(out, 19216);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27165);
					match s {
						[] => true,
						['к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27166);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27167);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 27168);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33466);
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
				['е', s @ ..] => {
					super::push(out, 27169);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33467);
							match s {
								[] => true,
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => super::t27::n59128(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t27::n43673(s, out),
				['и', s @ ..] => {
					super::push(out, 27171);
					match s {
						[] => true,
						['р', s @ ..] => super::t27::n59131(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t27::n43675(s, out),
				['о', s @ ..] => super::t27::n43676(s, out),
				['п', s @ ..] => super::t27::n43677(s, out),
				['ч', s @ ..] => super::t27::n43678(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t27::n27054(s, out),
		['п', s @ ..] => super::t27::n27055(s, out),
		['р', s @ ..] => super::t27::n27056(s, out),
		['с', s @ ..] => super::t27::n27057(s, out),
		['т', s @ ..] => super::t27::n27058(s, out),
		['ц', s @ ..] => super::t27::n27059(s, out),
		['ч', s @ ..] => super::t27::n27060(s, out),
		['щ', s @ ..] => super::t27::n27061(s, out),
		_ => false,
	}
}

/// Words ending -жниться.
pub(crate) fn n13575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10975);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19226);
			match s {
				[] => true,
				['л', 'в', 'у', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 19227);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27202);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27203);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37759);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40526);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37760);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40527);
											match s {
												[] => true,
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

/// Words ending -зниться.
pub(crate) fn n13576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10976);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19228);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33496);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33497);
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
				['л', s @ ..] => {
					super::push(out, 27205);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33498);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40528);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 40529);
											match s {
												[] => true,
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
					super::push(out, 27206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33499);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 33500);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37761);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37762);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 37763);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37764);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37765);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40530);
											match s {
												[] => true,
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
		['о', s @ ..] => {
			super::push(out, 19229);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27207);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33501);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33502);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37766);
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
		['я', s @ ..] => {
			super::push(out, 19230);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27208);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37767);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37768);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 37769);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37770);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37771);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40531);
											match s {
												[] => true,
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

/// Words ending -иниться.
pub(crate) fn n13577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10977);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27209);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 27210);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27211);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27212);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33503);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37772);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', 'а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 42105);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87934(s, out),
														_ => false,
													}
												}
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
							super::push(out, 33504);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33505);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37773);
									match s {
										[] => true,
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
		['г', s @ ..] => {
			super::push(out, 19232);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27213);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37774);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19233);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33506);
							s.is_empty()
						}
						['о', 'с', s @ ..] => {
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
								['с', s @ ..] => {
									super::push(out, 37775);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40532);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 42106);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87935(s, out),
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 33507);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['ж', s @ ..] => {
			super::push(out, 19234);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27215);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37776);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37777);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37778);
									match s {
										[] => true,
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
			super::push(out, 19235);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27216);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33508);
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
		['к', s @ ..] => {
			super::push(out, 19236);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27217);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37779);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37780);
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
			super::push(out, 19237);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27218);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33509);
							match s {
								[] => true,
								['ф', 'а', 'н', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27219);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33510);
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
				['д', s @ ..] => {
					super::push(out, 27220);
					match s {
						[] => true,
						['у', s @ ..] => super::t27::n59198(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t27::n43728(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t27::n27074(s, out),
		['т', s @ ..] => super::t27::n27075(s, out),
		['ф', s @ ..] => super::t27::n27076(s, out),
		['ч', s @ ..] => super::t27::n27077(s, out),
		['щ', s @ ..] => super::t27::n27078(s, out),
		_ => false,
	}
}

/// Words ending -лниться.
pub(crate) fn n13578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10978);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19243);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33527);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33528);
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
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
							super::push(out, 33529);
							match s {
								[] => true,
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
							super::push(out, 33530);
							match s {
								[] => true,
								['и', s @ ..] => {
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
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33531);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37791);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мниться.
pub(crate) fn n13579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10979);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19244);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19245);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33532);
							s.is_empty()
						}
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19246);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19247);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27246);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33533);
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
				['п', s @ ..] => {
					super::push(out, 27247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33534);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33535);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33536);
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
							super::push(out, 33537);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 33538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27248);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 33539);
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
		['у', s @ ..] => {
			super::push(out, 19248);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27249);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ониться.
pub(crate) fn n13580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10980);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19249);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33540);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 33541);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 40535);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 40536);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 42107);
													match s {
														[] => true,
														['а', 'р', '\u{1}', s @ ..] => super::t27::n87936(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33542);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33543);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37793);
									match s {
										[] => true,
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
		['д', s @ ..] => {
			super::push(out, 19250);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27250);
					match s {
						[] => true,
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27251);
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
		['ж', s @ ..] => {
			super::push(out, 19251);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27252);
					match s {
						[] => true,
						['п', 'с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19252);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27253);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33544);
							match s {
								[] => true,
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
		['к', s @ ..] => {
			super::push(out, 19253);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27254);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19254);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33545);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33546);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 37794);
									match s {
										[] => true,
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
							super::push(out, 33547);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37795);
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
							super::push(out, 33548);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37796);
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
							super::push(out, 33549);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37797);
									match s {
										[] => true,
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
							super::push(out, 33550);
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
							super::push(out, 33551);
							match s {
								[] => true,
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
					super::push(out, 27256);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33552);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37798);
									match s {
										[] => true,
										['д', 'о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42108);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 42109);
													match s {
														[] => true,
														['н', '\u{1}', s @ ..] => super::t27::n87937(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t27::n59274(s, out),
						['с', s @ ..] => {
							super::push(out, 33554);
							match s {
								[] => true,
								['о', s @ ..] => super::t27::n72011(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t27::n43766(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t27::n27091(s, out),
		['п', s @ ..] => super::t27::n27092(s, out),
		['р', s @ ..] => super::t27::n27093(s, out),
		['с', s @ ..] => super::t27::n27094(s, out),
		['т', s @ ..] => super::t27::n27095(s, out),
		['ф', s @ ..] => super::t27::n27096(s, out),
		_ => false,
	}
}

/// Words ending -пниться.
pub(crate) fn n13581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10981);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19261);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33572);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33573);
							match s {
								[] => true,
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
			super::push(out, 19262);
			match s {
				[] => true,
				['р', 'к', 'у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -рниться.
pub(crate) fn n13582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10982);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19263);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27273);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33574);
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
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33575);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 27275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33576);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33577);
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
							super::push(out, 33578);
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
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33579);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37823);
									match s {
										[] => true,
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

/// Words ending -сниться.
pub(crate) fn n13583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10983);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19264);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19265);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33580);
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33581);
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
							super::push(out, 33582);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33583);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37824);
									match s {
										[] => true,
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
			super::push(out, 19266);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27276);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27277);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33584);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33585);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37825);
									match s {
										[] => true,
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
			super::push(out, 19267);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33586);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
		['я', s @ ..] => {
			super::push(out, 19268);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27278);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27279);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33587);
							match s {
								[] => true,
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
					super::push(out, 27280);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 27281);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37826);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27282);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33588);
							match s {
								[] => true,
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

/// Words ending -тниться.
pub(crate) fn n13584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10984);
	match s {
		[] => true,
		['а', 'л', 'б', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19269);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33589);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 33590);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'м', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 19270);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27283);
					match s {
						[] => true,
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

/// Words ending -униться.
pub(crate) fn n13585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10985);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19271);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27284);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33591);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27285);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37827);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37828);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37829);
									match s {
										[] => true,
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
			super::push(out, 19273);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27286);
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
		['р', s @ ..] => {
			super::push(out, 19274);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27287);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37830);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чниться.
pub(crate) fn n13586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10986);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19275);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27288);
					match s {
						[] => true,
						['з', 'о', 'р', 'п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19276);
			s.is_empty()
		}
		['у', 'т', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шниться.
pub(crate) fn n13587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10987);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19277);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27289);
					match s {
						[] => true,
						['о', 'д', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19278);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27290);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33592);
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
				['т', s @ ..] => {
					super::push(out, 27291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33593);
							match s {
								[] => true,
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

/// Words ending -ыниться.
pub(crate) fn n13588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10988);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19279);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27292);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37831);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37832);
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
		['т', s @ ..] => {
			super::push(out, 19280);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27293);
					match s {
						[] => true,
						['у', 'п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юниться.
pub(crate) fn n13589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10989);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19281);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33594);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33595);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 33596);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33597);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 37833);
									match s {
										[] => true,
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
		['н', s @ ..] => {
			super::push(out, 19282);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27295);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33598);
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
			super::push(out, 19283);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27296);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37834);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37835);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 37836);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40547);
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
									super::push(out, 37837);
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

/// Words ending -яниться.
pub(crate) fn n13590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10990);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19284);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27297);
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
		['в', s @ ..] => {
			super::push(out, 19285);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27298);
					match s {
						[] => true,
						['л', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27299);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33599);
							match s {
								[] => true,
								['к', 'с', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19286);
			match s {
				[] => true,
				['к', 'е', 'т', 'с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19287);
			match s {
				[] => true,
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33600);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33601);
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
							super::push(out, 33602);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37838);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40548);
											match s {
												[] => true,
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
							super::push(out, 33603);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33604);
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
		['р', s @ ..] => {
			super::push(out, 19288);
			match s {
				[] => true,
				['г', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33605);
							match s {
								[] => true,
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
					super::push(out, 27300);
					match s {
						[] => true,
						['в', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19289);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27301);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
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
		['ъ', s @ ..] => {
			super::push(out, 19290);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27302);
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
		['ь', s @ ..] => {
			super::push(out, 19291);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33606);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33607);
							match s {
								[] => true,
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
					super::push(out, 27304);
					match s {
						[] => true,
						['с', 'е', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word забеспокоиться.
pub(crate) fn n85383(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word обеспокоиться.
pub(crate) fn n85384(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word самоуспокоиться.
pub(crate) fn n85385(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поуспокоиться.
pub(crate) fn n85386(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -реслоиться.
pub(crate) fn n59389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33613);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word прослоиться.
pub(crate) fn n43847(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word расслоиться.
pub(crate) fn n43848(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отслоиться.
pub(crate) fn n43849(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ноиться.
pub(crate) fn n13595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10995);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 19305);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27318);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поиться.
pub(crate) fn n13596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10996);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19306);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19307);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19308);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19309);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19310);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19311);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27319);
					match s {
						[] => true,
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

/// Words ending -роиться.
pub(crate) fn n13597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10997);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19312);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19313);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27320);
					match s {
						[] => true,
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
			super::push(out, 19314);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27321);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33614);
							match s {
								[] => true,
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
					super::push(out, 27322);
					match s {
						[] => true,
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
			super::push(out, 19315);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27323);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 27324);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 27326);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27327);
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
				['ы', s @ ..] => {
					super::push(out, 27328);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33615);
							match s {
								[] => true,
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
			super::push(out, 19316);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27329);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33617);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33618);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 37840);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['м', 'а', 'с', '\u{1}', s @ ..] => {
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
									super::push(out, 37841);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40552);
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
							super::push(out, 33619);
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
							super::push(out, 33620);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 33621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
										['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'у', 'р', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33622);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37842);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19318);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27332);
					match s {
						[] => true,
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

/// Words ending -тоиться.
pub(crate) fn n13598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10998);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 19319);
			match s {
				[] => true,
				['о', 'д', 'у', '\u{1}', s @ ..] => {
					super::push(out, 27333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щепиться.
pub(crate) fn n27168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19329);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27344);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 27345);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27346);
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

/// Words ending -ипиться.
pub(crate) fn n13604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11004);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19330);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 19331);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27348);
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

/// Words ending -йпиться.
pub(crate) fn n13605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11005);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19332);
			match s {
				[] => true,
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лпиться.
pub(crate) fn n13606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11006);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19333);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33632);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33633);
							match s {
								[] => true,
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

/// Words ending -опиться.
pub(crate) fn n13607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11007);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19334);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 19335);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27349);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33634);
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
			super::push(out, 19336);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19337);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27351);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27352);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27353);
					match s {
						[] => true,
						['у', 'к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27354);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27355);
					match s {
						[] => true,
						['н', s @ ..] => {
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37843);
									s.is_empty()
								}
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
			super::push(out, 19339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19340);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27358);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37844);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37845);
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
				['к', s @ ..] => {
					super::push(out, 27359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33637);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33638);
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
				['о', s @ ..] => {
					super::push(out, 27360);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37846);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37847);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37848);
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
				['п', s @ ..] => {
					super::push(out, 27361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27362);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33641);
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
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27363);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27364);
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
					super::push(out, 27365);
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
					super::push(out, 27366);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 27367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27368);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33642);
							match s {
								[] => true,
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

/// Words ending -спиться.
pub(crate) fn n13608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11008);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19342);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19343);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27369);
					match s {
						[] => true,
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
			super::push(out, 19344);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отпиться.
pub(crate) fn n27184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19345);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -упиться.
pub(crate) fn n13610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11010);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19346);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 19347);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27370);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 33643);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37849);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42110);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 42111);
													match s {
														[] => true,
														['р', s @ ..] => super::t27::n87938(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33644);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33645);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37850);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 37851);
									match s {
										[] => true,
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
							super::push(out, 33646);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33647);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37852);
									match s {
										[] => true,
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
					super::push(out, 27373);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27375);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33648);
							match s {
								[] => true,
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
		['л', s @ ..] => {
			super::push(out, 19348);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27376);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27377);
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
				['б', s @ ..] => {
					super::push(out, 27378);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 27379);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33649);
							match s {
								[] => true,
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
					super::push(out, 27380);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27381);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27383);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27384);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33651);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37853);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37854);
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
		['п', s @ ..] => {
			super::push(out, 19349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27385);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33652);
							match s {
								[] => true,
								['т', 'н', 'о', 'к', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27386);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27387);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['т', s @ ..] => super::t27::n27190(s, out),
		_ => false,
	}
}

/// Words ending -ыпиться.
pub(crate) fn n13611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11011);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19353);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27392);
					match s {
						[] => true,
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

/// Words ending -япиться.
pub(crate) fn n13612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11012);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19354);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 27393);
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

/// Words ending -вариться.
pub(crate) fn n27194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19355);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27394);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27395);
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
			super::push(out, 27396);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 27397);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27398);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33658);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37856);
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
					super::push(out, 33659);
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
			super::push(out, 27399);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33660);
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
			super::push(out, 27400);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27401);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 27402);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 33661);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27403);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33662);
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
				['м', s @ ..] => {
					super::push(out, 33663);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37857);
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
				['п', s @ ..] => {
					super::push(out, 33664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33665);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37858);
							match s {
								[] => true,
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
					super::push(out, 33666);
					match s {
						[] => true,
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
		['с', s @ ..] => {
			super::push(out, 27404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27405);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27406);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27407);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33669);
					match s {
						[] => true,
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

/// Words ending -дариться.
pub(crate) fn n27196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19357);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27408);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27409);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33670);
					match s {
						[] => true,
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
			super::push(out, 27410);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 27411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33671);
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
			super::push(out, 27412);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33672);
					match s {
						[] => true,
						['и', 'к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40553);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40554);
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
		['о', s @ ..] => {
			super::push(out, 27413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 33673);
					match s {
						[] => true,
						['а', 'л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40555);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 40556);
									match s {
										[] => true,
										['о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 40557);
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
				['п', s @ ..] => {
					super::push(out, 33674);
					match s {
						[] => true,
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
			super::push(out, 27414);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33675);
					match s {
						[] => true,
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
			super::push(out, 27415);
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

/// Words ending -жариться.
pub(crate) fn n27197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27416);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27417);
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
		['з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27418);
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
			super::push(out, 27419);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33676);
					match s {
						[] => true,
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

/// The word обзариться.
pub(crate) fn n44008(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word взариться.
pub(crate) fn n44009(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ззариться.
pub(crate) fn n44010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27422);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33677);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37859);
					match s {
						[] => true,
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

/// Words ending -озариться.
pub(crate) fn n44011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27423);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33678);
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

/// Words ending -иариться.
pub(crate) fn n27199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19360);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 27424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33679);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33680);
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

/// Words ending -мариться.
pub(crate) fn n27200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19361);
	match s {
		[] => true,
		['х', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -париться.
pub(crate) fn n27201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19362);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27425);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27426);
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
		['е', s @ ..] => {
			super::push(out, 27427);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27428);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27429);
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
			super::push(out, 27430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33681);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 33682);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 33683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33684);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37860);
							match s {
								[] => true,
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
			super::push(out, 27431);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 27433);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33686);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 33688);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37861);
							match s {
								[] => true,
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
			super::push(out, 27434);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33689);
					match s {
						[] => true,
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

/// Words ending -тариться.
pub(crate) fn n27202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19363);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27435);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27436);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33690);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33691);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37862);
							match s {
								[] => true,
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
			super::push(out, 27438);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33692);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37863);
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
		['ы', s @ ..] => {
			super::push(out, 27439);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37864);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37865);
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
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37866);
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

/// Words ending -уариться.
pub(crate) fn n27203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19364);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 27440);
			match s {
				[] => true,
				['а', 'с', 'к', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37867);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37868);
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
		_ => false,
	}
}

/// Words ending -хариться.
pub(crate) fn n27204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19365);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27441);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37869);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 37870);
							match s {
								[] => true,
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
			super::push(out, 27442);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37871);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37872);
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
		['у', s @ ..] => {
			super::push(out, 27443);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37873);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37874);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40558);
									match s {
										[] => true,
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
							super::push(out, 37875);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40559);
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
		_ => false,
	}
}

/// Words ending -цариться.
pub(crate) fn n27205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19366);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шариться.
pub(crate) fn n27206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19367);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27445);
			s.is_empty()
		}
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27446);
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

/// Words ending -бриться.
pub(crate) fn n13614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19368);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19369);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27448);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 33696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37876);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
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
				['ф', s @ ..] => {
					super::push(out, 27449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33697);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33698);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33699);
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
				['ш', s @ ..] => {
					super::push(out, 27450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33700);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 33701);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37877);
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
			super::push(out, 19370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27451);
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
			super::push(out, 19371);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33702);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40560);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37879);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40561);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 40562);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 40563);
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
											super::push(out, 40564);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 42112);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87939(s, out),
														_ => false,
													}
												}
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
							super::push(out, 33703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37880);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37881);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37882);
									match s {
										[] => true,
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
			super::push(out, 19372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27453);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 27454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33704);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 33705);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 37883);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27456);
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
			super::push(out, 19373);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19374);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27457);
					match s {
						[] => true,
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
			super::push(out, 19375);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33706);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33707);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => super::t27::n72235(s, out),
								['н', '\u{1}', s @ ..] => super::t27::n72236(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => super::t27::n59604(s, out),
						['ы', s @ ..] => super::t27::n59605(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t27::n27215(s, out),
		_ => false,
	}
}

/// Words ending -вриться.
pub(crate) fn n13615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11015);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19377);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27460);
					match s {
						[] => true,
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

/// Words ending -гриться.
pub(crate) fn n13616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11016);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19378);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27461);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27462);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 27463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33710);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33711);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 33712);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33713);
							match s {
								[] => true,
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
					super::push(out, 27464);
					match s {
						[] => true,
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
			super::push(out, 19379);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33714);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 33715);
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

/// Words ending -дриться.
pub(crate) fn n13617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11017);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19380);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27466);
					match s {
						[] => true,
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
			super::push(out, 19381);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27467);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33716);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33717);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37887);
									match s {
										[] => true,
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
					super::push(out, 27468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33718);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 33719);
							match s {
								[] => true,
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
					super::push(out, 27469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33720);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33721);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37888);
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
		['з', s @ ..] => {
			super::push(out, 19382);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27470);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27471);
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
		['н', s @ ..] => {
			super::push(out, 19383);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27472);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 33722);
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
				['е', s @ ..] => {
					super::push(out, 27473);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19384);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33724);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
							super::push(out, 33725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19385);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 27476);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 33727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33728);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33729);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33730);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33731);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37889);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40565);
											match s {
												[] => true,
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

/// Words ending -ериться.
pub(crate) fn n13618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11018);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19386);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27478);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 33732);
							match s {
								[] => true,
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
		['б', s @ ..] => {
			super::push(out, 19387);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27479);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33733);
							match s {
								[] => true,
								['а', 'ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19388);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27480);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 27481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33735);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 33736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33738);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37890);
									match s {
										[] => true,
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
					super::push(out, 27483);
					match s {
						[] => true,
						['д', s @ ..] => {
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
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', s @ ..] => {
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
					super::push(out, 27484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27485);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33739);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40566);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 40567);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 40568);
											match s {
												[] => true,
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
					super::push(out, 27486);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27487);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33740);
							match s {
								[] => true,
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
			super::push(out, 19389);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27488);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33741);
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
		['з', s @ ..] => {
			super::push(out, 19390);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27489);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37891);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 37892);
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
		['к', s @ ..] => {
			super::push(out, 19391);
			match s {
				[] => true,
				['ш', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19392);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27490);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27491);
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
								['з', 'о', 'в', '\u{1}', s @ ..] => {
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
				['е', s @ ..] => {
					super::push(out, 27492);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33742);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 37893);
									match s {
										[] => true,
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
					super::push(out, 27493);
					match s {
						[] => true,
						['а', 'р', 'о', 'с', '\u{1}', s @ ..] => super::t27::n59663(s, out),
						['и', s @ ..] => super::t27::n59664(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t27::n44089(s, out),
				['о', s @ ..] => super::t27::n44090(s, out),
				['с', '\u{1}', s @ ..] => super::t27::n44091(s, out),
				['т', 'о', '\u{1}', s @ ..] => super::t27::n44092(s, out),
				['у', s @ ..] => super::t27::n44093(s, out),
				['ь', s @ ..] => super::t27::n44094(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t27::n27232(s, out),
		['п', s @ ..] => super::t27::n27233(s, out),
		['с', s @ ..] => super::t27::n27234(s, out),
		['т', s @ ..] => super::t27::n27235(s, out),
		['х', s @ ..] => super::t27::n27236(s, out),
		['ч', s @ ..] => super::t27::n27237(s, out),
		['щ', s @ ..] => super::t27::n27238(s, out),
		['ь', s @ ..] => super::t27::n27239(s, out),
		_ => false,
	}
}

/// Words ending -зриться.
pub(crate) fn n13619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11019);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 19401);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19402);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27518);
					match s {
						[] => true,
						['о', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ириться.
pub(crate) fn n13620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11020);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19404);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27519);
					match s {
						[] => true,
						['у', 'м', s @ ..] => {
							match s {
								[] => true,
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19405);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27520);
					match s {
						[] => true,
						['е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27521);
					match s {
						[] => true,
						['а', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37908);
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 27522);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33755);
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
			super::push(out, 19406);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27523);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27524);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33756);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 37909);
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
					super::push(out, 27525);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27526);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 33758);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 37910);
									match s {
										[] => true,
										['т', 'у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 27527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
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
		['ш', s @ ..] => {
			super::push(out, 19407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27528);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27529);
					match s {
						[] => true,
						['б', 'е', 'д', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33759);
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
				['у', s @ ..] => {
					super::push(out, 27531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -криться.
pub(crate) fn n13621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11021);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19408);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19409);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ориться.
pub(crate) fn n13622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11022);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19410);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27533);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33762);
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
					super::push(out, 27534);
					match s {
						[] => true,
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
			super::push(out, 19411);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27535);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 33763);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37911);
									match s {
										[] => true,
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
					super::push(out, 27536);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 33764);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37912);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37913);
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
								['з', 'а', 'р', s @ ..] => {
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
									super::push(out, 37914);
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
												['е', s @ ..] => {
													match s {
														[] => true,
														['н', '\u{1}', s @ ..] => super::t27::n87940(s, out),
														['р', 'е', 'п', '\u{1}', s @ ..] => super::t27::n87941(s, out),
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
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 37915);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40570);
											match s {
												[] => true,
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
							super::push(out, 33765);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37916);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40571);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 40572);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 40573);
											match s {
												[] => true,
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
					super::push(out, 27537);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33766);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37917);
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
				['т', s @ ..] => super::t27::n44142(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27539);
					match s {
						[] => true,
						['з', s @ ..] => super::t27::n59723(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27540);
					match s {
						[] => true,
						['а', 'п', 'ш', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t27::n27252(s, out),
		['й', s @ ..] => super::t27::n27253(s, out),
		['к', s @ ..] => super::t27::n27254(s, out),
		['м', s @ ..] => super::t27::n27255(s, out),
		['н', s @ ..] => {
			super::push(out, 19417);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => super::t27::n44164(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t27::n27257(s, out),
		['с', s @ ..] => super::t27::n27258(s, out),
		['т', s @ ..] => super::t27::n27259(s, out),
		['ф', s @ ..] => super::t27::n27260(s, out),
		['х', s @ ..] => super::t27::n27261(s, out),
		['ш', s @ ..] => super::t27::n27262(s, out),
		_ => false,
	}
}

/// Words ending -триться.
pub(crate) fn n13623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11023);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19424);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27572);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33803);
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
						['ы', s @ ..] => {
							super::push(out, 33804);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37935);
									s.is_empty()
								}
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
			super::push(out, 19425);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 27573);
					match s {
						[] => true,
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19426);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27574);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37936);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37937);
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
		['с', s @ ..] => {
			super::push(out, 19427);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37938);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37939);
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
								['у', s @ ..] => {
									super::push(out, 37940);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37941);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40580);
											match s {
												[] => true,
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
							super::push(out, 33806);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37942);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40581);
											match s {
												[] => true,
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
					super::push(out, 27575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33807);
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
							super::push(out, 33808);
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
								_ => false,
							}
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33809);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37943);
									match s {
										[] => true,
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
					super::push(out, 27576);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37944);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 37945);
									match s {
										[] => true,
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

/// Words ending -уриться.
pub(crate) fn n13624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11024);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27577);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27578);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33810);
							match s {
								[] => true,
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
					super::push(out, 27579);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33811);
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
					super::push(out, 27580);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33812);
							match s {
								[] => true,
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
		['г', s @ ..] => {
			super::push(out, 19429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27581);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33813);
							match s {
								[] => true,
								['а', 'б', 'а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19430);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19431);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27582);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27583);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27584);
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
			super::push(out, 19432);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27585);
					match s {
						[] => true,
						['л', 'г', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19433);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27586);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27587);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 33815);
							match s {
								[] => true,
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
					super::push(out, 27588);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33816);
							match s {
								[] => true,
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
					super::push(out, 27589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27590);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33817);
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
					super::push(out, 27591);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33818);
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
					super::push(out, 27592);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33819);
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
					super::push(out, 27593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 33820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33821);
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
					super::push(out, 27594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33822);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33823);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33824);
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
				['у', s @ ..] => {
					super::push(out, 27595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 27596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33825);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37946);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 37947);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40582);
											match s {
												[] => true,
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
					super::push(out, 27597);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33827);
							match s {
								[] => true,
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
			super::push(out, 19434);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33828);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t27::n44217(s, out),
				['х', s @ ..] => super::t27::n44218(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t27::n27274(s, out),
		['п', s @ ..] => super::t27::n27275(s, out),
		['т', s @ ..] => super::t27::n27276(s, out),
		['ч', s @ ..] => super::t27::n27277(s, out),
		['ш', s @ ..] => super::t27::n27278(s, out),
		['щ', s @ ..] => super::t27::n27279(s, out),
		_ => false,
	}
}

/// Words ending -фриться.
pub(crate) fn n13625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11025);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19441);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33836);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33837);
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
		_ => false,
	}
}

/// Words ending -хриться.
pub(crate) fn n13626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11026);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19442);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27612);
					match s {
						[] => true,
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
			super::push(out, 19443);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33838);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33839);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37953);
									match s {
										[] => true,
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
							super::push(out, 33840);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37954);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40587);
											match s {
												[] => true,
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
									super::push(out, 37955);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40588);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 27613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33841);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 37956);
									match s {
										[] => true,
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

/// Words ending -щриться.
pub(crate) fn n13627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11027);
	match s {
		[] => true,
		['е', 'п', 'с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19445);
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

/// Words ending -ыриться.
pub(crate) fn n13628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11028);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19446);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27614);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33842);
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
		['з', s @ ..] => {
			super::push(out, 19447);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27615);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 27616);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37957);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37958);
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
								['с', s @ ..] => {
									super::push(out, 37959);
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
		['п', s @ ..] => {
			super::push(out, 19448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27617);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27618);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37960);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37961);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27619);
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
			super::push(out, 19449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27620);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33843);
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
					super::push(out, 27621);
					match s {
						[] => true,
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
			super::push(out, 19450);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37962);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37963);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 37964);
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
						['ч', s @ ..] => {
							super::push(out, 33844);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юриться.
pub(crate) fn n13629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11029);
	match s {
		[] => true,
		['к', 'и', 'н', 'а', 'м', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яриться.
pub(crate) fn n13630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11030);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19451);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 19452);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27622);
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
			super::push(out, 19453);
			match s {
				[] => true,
				['б', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19454);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 27623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33845);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
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
		['ъ', s @ ..] => {
			super::push(out, 19455);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27624);
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

/// Words ending -паситься.
pub(crate) fn n27303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19461);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
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

/// Words ending -раситься.
pub(crate) fn n27304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19462);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 27632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33853);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33854);
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
					super::push(out, 33855);
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
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 33856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 33857);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еситься.
pub(crate) fn n13632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11032);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19463);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27633);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27634);
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
					super::push(out, 27635);
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
				['о', s @ ..] => {
					super::push(out, 27636);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27637);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27638);
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
								['а', 'з', '\u{1}', s @ ..] => {
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
					super::push(out, 27639);
					match s {
						[] => true,
						['д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'в', 'а', 'р', 'у', '\u{1}', s @ ..] => {
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
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27640);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33858);
							match s {
								[] => true,
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
		['л', s @ ..] => {
			super::push(out, 19465);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27641);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33859);
							match s {
								[] => true,
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
					super::push(out, 27642);
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
		['м', s @ ..] => {
			super::push(out, 19466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27643);
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
				['в', '\u{1}', s @ ..] => {
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
					super::push(out, 27644);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27645);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33860);
							match s {
								[] => true,
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
			super::push(out, 19467);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27646);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33861);
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

/// Words ending -кситься.
pub(crate) fn n13633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11033);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19468);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27647);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33862);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33863);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 37967);
									match s {
										[] => true,
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
			super::push(out, 19469);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33864);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33865);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37968);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 37969);
									match s {
										[] => true,
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
							super::push(out, 33866);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37970);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37971);
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
		_ => false,
	}
}

/// Words ending -оситься.
pub(crate) fn n13634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11034);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27648);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27649);
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
					super::push(out, 27650);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27652);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 27653);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33867);
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
					super::push(out, 27654);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 33868);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 33869);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 33870);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37972);
									match s {
										[] => true,
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
					super::push(out, 27655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33871);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33872);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33873);
							match s {
								[] => true,
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
					super::push(out, 27656);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 27657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27658);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33874);
							match s {
								[] => true,
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
		['л', s @ ..] => {
			super::push(out, 19471);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27659);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 33875);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37973);
									match s {
										[] => true,
										['д', 'з', 'и', 'п', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', 'с', 'о', 'р', 'п', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 33876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37974);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37975);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37976);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 37977);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37978);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40590);
											match s {
												[] => true,
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
		['н', s @ ..] => {
			super::push(out, 19472);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27660);
					match s {
						[] => true,
						['з', s @ ..] => {
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
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 27661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37979);
									match s {
										[] => true,
										['п', s @ ..] => super::t27::n80748(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t27::n44318(s, out),
				['д', s @ ..] => super::t27::n44319(s, out),
				['е', 'р', s @ ..] => super::t27::n44320(s, out),
				['з', s @ ..] => super::t27::n44321(s, out),
				['и', 'р', 'п', '\u{1}', s @ ..] => super::t27::n44322(s, out),
				['о', s @ ..] => super::t27::n44323(s, out),
				['р', s @ ..] => super::t27::n44324(s, out),
				['с', s @ ..] => super::t27::n44325(s, out),
				['т', s @ ..] => super::t27::n44326(s, out),
				['у', s @ ..] => super::t27::n44327(s, out),
				['ы', s @ ..] => super::t27::n44328(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n27315(s, out),
		['с', s @ ..] => super::t27::n27316(s, out),
		_ => false,
	}
}

/// Words ending -рситься.
pub(crate) fn n13635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11035);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19475);
			match s {
				[] => true,
				['м', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19476);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33896);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33897);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33898);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33899);
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

/// Words ending -уситься.
pub(crate) fn n13636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11036);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19477);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27677);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33900);
							match s {
								[] => true,
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
					super::push(out, 27678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27679);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33901);
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
					super::push(out, 27680);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33902);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 33903);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37994);
									match s {
										[] => true,
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
					super::push(out, 27681);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27682);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33904);
							match s {
								[] => true,
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
					super::push(out, 27683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27684);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33907);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33908);
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
				['т', s @ ..] => {
					super::push(out, 27685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33909);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33910);
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
						['е', s @ ..] => {
							super::push(out, 33911);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37995);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 40598);
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33912);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33913);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37996);
									match s {
										[] => true,
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

/// Words ending -ыситься.
pub(crate) fn n13637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11037);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19479);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27686);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 27687);
					match s {
						[] => true,
						['о', 'в', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 27688);
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
			super::push(out, 19480);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33914);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33915);
							match s {
								[] => true,
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

/// Words ending -эситься.
pub(crate) fn n13638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11038);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 19481);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 27689);
					match s {
						[] => true,
						['с', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ватиться.
pub(crate) fn n27325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19483);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27691);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33918);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38001);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42121);
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 42122);
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
		['х', s @ ..] => {
			super::push(out, 27692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'с', '\u{1}', s @ ..] => {
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
					super::push(out, 33919);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38002);
							match s {
								[] => true,
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

/// Words ending -гатиться.
pub(crate) fn n27326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19484);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27693);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27694);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38003);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38004);
							match s {
								[] => true,
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
					super::push(out, 33921);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катиться.
pub(crate) fn n27327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19485);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27695);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27696);
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
			super::push(out, 27697);
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
			super::push(out, 27698);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
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
			super::push(out, 27699);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33922);
					match s {
						[] => true,
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

/// Words ending -латиться.
pub(crate) fn n27328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19486);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27700);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33923);
					match s {
						[] => true,
						['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 27701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33924);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 38006);
							match s {
								[] => true,
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
			super::push(out, 27702);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33926);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33927);
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
					super::push(out, 33928);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 33929);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38007);
							match s {
								[] => true,
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
			super::push(out, 27703);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33930);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 38008);
							match s {
								[] => true,
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
			super::push(out, 27704);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33931);
					match s {
						[] => true,
						['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матиться.
pub(crate) fn n27329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19487);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27705);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 27706);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38009);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 38010);
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
		['х', s @ ..] => {
			super::push(out, 27707);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38011);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 38012);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 27708);
			match s {
				[] => true,
				['о', 'л', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38013);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натиться.
pub(crate) fn n27330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19488);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 27709);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38014);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 38015);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40599);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 38016);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40600);
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
		_ => false,
	}
}

/// Words ending -патиться.
pub(crate) fn n27331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19489);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27710);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38017);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38018);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38019);
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
		_ => false,
	}
}

/// Words ending -ратиться.
pub(crate) fn n27332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19490);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27711);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33932);
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
		['в', s @ ..] => {
			super::push(out, 27712);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33933);
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
				['о', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 33934);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40601);
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
		['к', s @ ..] => {
			super::push(out, 27713);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33935);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38020);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 40602);
									match s {
										[] => true,
										['б', 'а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 38021);
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
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33936);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33937);
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
							super::push(out, 38022);
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 38023);
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
						['ы', 'д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -хатиться.
pub(crate) fn n27333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19491);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27715);
			match s {
				[] => true,
				['с', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27716);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33938);
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
		['ю', s @ ..] => {
			super::push(out, 27717);
			match s {
				[] => true,
				['р', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чатиться.
pub(crate) fn n27334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19492);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27718);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27719);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33939);
					match s {
						[] => true,
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

/// Words ending -гтиться.
pub(crate) fn n13640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11040);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19493);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27720);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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

/// Words ending -етиться.
pub(crate) fn n13641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11041);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19494);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33940);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33941);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38024);
									match s {
										[] => true,
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
							super::push(out, 33942);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 33943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38025);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 38026);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 40603);
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
								['р', s @ ..] => {
									super::push(out, 38027);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40604);
											match s {
												[] => true,
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
									super::push(out, 38028);
									match s {
										[] => true,
										['л', 'о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33944);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33945);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38029);
									match s {
										[] => true,
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
				['ц', s @ ..] => {
					super::push(out, 27722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33946);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33947);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33948);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38030);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40605);
											match s {
												[] => true,
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
									super::push(out, 38031);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 40606);
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
						['ы', s @ ..] => {
							super::push(out, 33949);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38032);
									match s {
										[] => true,
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
			super::push(out, 19495);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27723);
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
		['м', s @ ..] => {
			super::push(out, 19496);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27724);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27725);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 19497);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27726);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 33950);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38033);
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
				['к', s @ ..] => {
					super::push(out, 27727);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33951);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33952);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33953);
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
				['с', s @ ..] => {
					super::push(out, 27729);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27730);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19498);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27731);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => super::t27::n60094(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t28::n27341(s, out),
		['ф', s @ ..] => super::t28::n27342(s, out),
		['ш', s @ ..] => super::t28::n27343(s, out),
		_ => false,
	}
}

/// Words ending -ититься.
pub(crate) fn n13642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11042);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19502);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27735);
					match s {
						[] => true,
						['л', 'о', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38039);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 38040);
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
			super::push(out, 19503);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27736);
					match s {
						[] => true,
						['н', 'о', 'м', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['н', s @ ..] => {
			super::push(out, 19504);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27737);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38041);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
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
		['х', s @ ..] => {
			super::push(out, 19505);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27738);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 33958);
							match s {
								[] => true,
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
					super::push(out, 27739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33959);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38042);
									match s {
										[] => true,
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
							super::push(out, 33960);
							match s {
								[] => true,
								['в', s @ ..] => {
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
				['у', s @ ..] => {
					super::push(out, 27740);
					match s {
						[] => true,
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
			super::push(out, 19506);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27741);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтиться.
pub(crate) fn n13643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11043);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19507);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27742);
					match s {
						[] => true,
						['п', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтиться.
pub(crate) fn n13644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11044);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19508);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 27743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33961);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33962);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 33963);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 33964);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 33965);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38043);
									match s {
										[] => true,
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
			super::push(out, 19509);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27744);
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

/// Words ending -нтиться.
pub(crate) fn n13645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11045);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19510);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27745);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 38044);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 38045);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 38046);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40609);
											match s {
												[] => true,
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
		['е', s @ ..] => {
			super::push(out, 19511);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27746);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33966);
							match s {
								[] => true,
								['л', 'л', 'е', 'т', 'н', 'ы', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 19512);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33967);
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
							super::push(out, 33968);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33969);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33970);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38047);
									match s {
										[] => true,
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
					super::push(out, 27748);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33971);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38048);
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

/// Words ending -отиться.
pub(crate) fn n13646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11046);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19513);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27749);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33972);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38049);
									s.is_empty()
								}
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38050);
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
							super::push(out, 33973);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38051);
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
		['в', s @ ..] => {
			super::push(out, 19514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27750);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33974);
							match s {
								[] => true,
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
		['г', s @ ..] => {
			super::push(out, 19515);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27751);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33975);
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
				['л', s @ ..] => {
					super::push(out, 27752);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33976);
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
				['я', s @ ..] => {
					super::push(out, 27753);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33977);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38052);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38053);
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
		['к', s @ ..] => {
			super::push(out, 19516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27754);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 33978);
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									super::push(out, 38054);
									s.is_empty()
								}
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
			super::push(out, 19517);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27756);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38055);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 38056);
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
				['о', s @ ..] => super::t28::n44434(s, out),
				['п', s @ ..] => {
					super::push(out, 27758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33983);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33984);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38072);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
							super::push(out, 33985);
							match s {
								[] => true,
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
			super::push(out, 19518);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 27759);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t28::n27361(s, out),
		['х', s @ ..] => {
			super::push(out, 19520);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t28::n60154(s, out),
						['а', 'н', '\u{1}', s @ ..] => super::t28::n60155(s, out),
						['з', 'а', 'р', '\u{1}', s @ ..] => super::t28::n60156(s, out),
						['и', 'р', 'п', '\u{1}', s @ ..] => super::t28::n60157(s, out),
						['о', s @ ..] => super::t28::n60158(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t28::n60159(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t28::n44441(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птиться.
pub(crate) fn n13647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11047);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19521);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33995);
							s.is_empty()
						}
						['а', 'з', s @ ..] => {
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33996);
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
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 33997);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38082);
									match s {
										[] => true,
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

/// Words ending -ртиться.
pub(crate) fn n13648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11048);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19522);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27765);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33998);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 38083);
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
				['ч', s @ ..] => {
					super::push(out, 27766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33999);
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 34000);
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
							super::push(out, 34001);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38084);
									match s {
										[] => true,
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
			super::push(out, 19523);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34002);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34003);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38085);
									match s {
										[] => true,
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
						['е', s @ ..] => {
							super::push(out, 34004);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 34005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ститься.
pub(crate) fn n13649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11049);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19524);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 27768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34007);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34008);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38086);
									match s {
										[] => true,
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
							super::push(out, 34009);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38087);
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
							super::push(out, 34010);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38088);
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
							super::push(out, 34011);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38089);
									match s {
										[] => true,
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
							super::push(out, 34012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38090);
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 27770);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27771);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34013);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 38091);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 40618);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 42131);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t28::n87951(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 40619);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 42132);
													s.is_empty()
												}
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
							super::push(out, 34014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['п', 'о', 'р', 'п', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34015);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 34016);
							match s {
								[] => true,
								['с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 34017);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38092);
									match s {
										[] => true,
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
					super::push(out, 27773);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t28::n27367(s, out),
		['и', s @ ..] => {
			super::push(out, 19526);
			match s {
				[] => true,
				['ч', s @ ..] => super::t28::n44458(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19527);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27780);
					match s {
						[] => true,
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
			super::push(out, 19528);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27782);
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27783);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => super::t28::n27372(s, out),
		['р', s @ ..] => super::t28::n27373(s, out),
		['у', s @ ..] => super::t28::n27374(s, out),
		['ы', s @ ..] => super::t28::n27375(s, out),
		['ь', s @ ..] => super::t28::n27376(s, out),
		_ => false,
	}
}

/// Words ending -утиться.
pub(crate) fn n13650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11050);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27798);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27799);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 34065);
							match s {
								[] => true,
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
			super::push(out, 19536);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27800);
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
					super::push(out, 27801);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34066);
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
					super::push(out, 27802);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19537);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27803);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27804);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 34068);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 34069);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38123);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42140);
													s.is_empty()
												}
												['а', 'з', '\u{1}', s @ ..] => {
													s.is_empty()
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
						['н', s @ ..] => {
							super::push(out, 34070);
							match s {
								[] => true,
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
					super::push(out, 27805);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34071);
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
					super::push(out, 27806);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 34072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 34073);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38124);
									match s {
										[] => true,
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
					super::push(out, 27807);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 34074);
							match s {
								[] => true,
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
					super::push(out, 27808);
					match s {
						[] => true,
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
			super::push(out, 19538);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27809);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 34075);
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
		['р', s @ ..] => {
			super::push(out, 19539);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34076);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34077);
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
						['н', s @ ..] => {
							super::push(out, 34078);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38125);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 40637);
											match s {
												[] => true,
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
							super::push(out, 34079);
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
							super::push(out, 34080);
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
							super::push(out, 34081);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 38126);
									match s {
										[] => true,
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
		['ч', s @ ..] => {
			super::push(out, 19540);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 19541);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t28::n44493(s, out),
				['а', s @ ..] => super::t28::n44494(s, out),
				['о', 'д', '\u{1}', s @ ..] => super::t28::n44495(s, out),
				['с', 'а', 'р', '\u{1}', s @ ..] => super::t28::n44496(s, out),
				['т', s @ ..] => super::t28::n44497(s, out),
				['ы', s @ ..] => super::t28::n44498(s, out),
				_ => false,
			}
		}
		['щ', s @ ..] => super::t28::n27384(s, out),
		_ => false,
	}
}

/// Words ending -чтиться.
pub(crate) fn n13651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19543);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19544);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27817);
					match s {
						[] => true,
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

/// Words ending -ытиться.
pub(crate) fn n13652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11052);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 19545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27818);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 27819);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 34083);
							match s {
								[] => true,
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
			super::push(out, 19546);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27820);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 34084);
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
		['р', s @ ..] => {
			super::push(out, 19547);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27821);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 34085);
							match s {
								[] => true,
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
			super::push(out, 19548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27822);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27823);
					match s {
						[] => true,
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
				['е', s @ ..] => {
					super::push(out, 27824);
					match s {
						[] => true,
						['р', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этиться.
pub(crate) fn n13653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11053);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 19549);
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

/// Words ending -ютиться.
pub(crate) fn n13654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11054);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19550);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19551);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27825);
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
		['ъ', s @ ..] => {
			super::push(out, 19552);
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

/// Words ending -ятиться.
pub(crate) fn n13655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11055);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19553);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34086);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 34087);
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
			super::push(out, 19554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27827);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27828);
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
					super::push(out, 27829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27830);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 34088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38127);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 38128);
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
									super::push(out, 38129);
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
									super::push(out, 38130);
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
								['ы', s @ ..] => {
									super::push(out, 38131);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40638);
											match s {
												[] => true,
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
					super::push(out, 27831);
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
					super::push(out, 27832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27833);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27834);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 34089);
							match s {
								[] => true,
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

