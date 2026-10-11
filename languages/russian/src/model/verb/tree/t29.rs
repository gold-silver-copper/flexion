use flexion::Path;

/// Words ending -ллировать.
pub(crate) fn n30638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20932);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29366);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35285);
					match s {
						[] => true,
						['с', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 42435);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 42436);
											match s {
												[] => true,
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
			super::push(out, 29367);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35286);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 41224);
									match s {
										[] => true,
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
							super::push(out, 39068);
							match s {
								[] => true,
								['е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 35287);
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
		['и', s @ ..] => {
			super::push(out, 29368);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35288);
					match s {
						[] => true,
						['б', 'и', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35289);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39069);
							match s {
								[] => true,
								['и', 'ц', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39070);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41225);
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
				['ц', s @ ..] => {
					super::push(out, 35290);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39071);
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
		['о', s @ ..] => {
			super::push(out, 29369);
			match s {
				[] => true,
				['к', s @ ..] => {
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
				['м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олировать.
pub(crate) fn n30639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20933);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29370);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35291);
					match s {
						[] => true,
						['а', 'р', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 29371);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39072);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41226);
									match s {
										[] => true,
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
							super::push(out, 39073);
							match s {
								[] => true,
								['к', 'у', 'в', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29372);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35293);
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
		['к', s @ ..] => {
			super::push(out, 29373);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 35294);
					match s {
						[] => true,
						['т', 'о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41227);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42437);
											match s {
												[] => true,
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
					super::push(out, 35295);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39074);
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
		['л', s @ ..] => {
			super::push(out, 29374);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29375);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35296);
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
		['п', s @ ..] => {
			super::push(out, 29376);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35297);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35298);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39075);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 39076);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39077);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 41228);
									match s {
										[] => true,
										['с', 'к', 'э', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 43155);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n89458(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35299);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39078);
							match s {
								[] => true,
								['т', 'н', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42438);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43156);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89459(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35300);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39079);
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
				['т', s @ ..] => {
					super::push(out, 35301);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39080);
							match s {
								[] => true,
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
					super::push(out, 35302);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 39081);
							match s {
								[] => true,
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
			super::push(out, 29377);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35303);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39082);
							match s {
								[] => true,
								['н', 'а', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35304);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39083);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42439);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42440);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43157);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89460(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39084);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41229);
									s.is_empty()
								}
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
			super::push(out, 29378);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плировать.
pub(crate) fn n30640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20934);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 29379);
			match s {
				[] => true,
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39085);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41230);
									match s {
										[] => true,
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

/// Words ending -слировать.
pub(crate) fn n30641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20935);
	match s {
		[] => true,
		['н', 'а', 'р', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 35305);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39086);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 41231);
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
						['р', s @ ..] => {
							super::push(out, 39087);
							match s {
								[] => true,
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
					super::push(out, 35306);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39088);
							match s {
								[] => true,
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

/// Words ending -улировать.
pub(crate) fn n30642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20936);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29380);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35307);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 35308);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39090);
							match s {
								[] => true,
								['у', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29381);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35309);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 35310);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41232);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41233);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 42441);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 42442);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 43158);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89461(s, out),
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
								['т', s @ ..] => {
									super::push(out, 41234);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 41235);
									match s {
										[] => true,
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
					super::push(out, 35311);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39092);
							match s {
								[] => true,
								['и', 'р', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29382);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35312);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41236);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41237);
									match s {
										[] => true,
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
		['к', s @ ..] => {
			super::push(out, 29383);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35313);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 35314);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39094);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41238);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 42443);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41239);
									match s {
										[] => true,
										['е', 'ж', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 43159);
													match s {
														[] => true,
														['з', s @ ..] => super::t31::n89462(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35315);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39095);
							match s {
								[] => true,
								['л', 'ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 39096);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41240);
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
				['р', s @ ..] => {
					super::push(out, 35317);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39097);
							match s {
								[] => true,
								['ц', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42444);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43160);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89463(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => super::t31::n63728(s, out),
				['я', s @ ..] => {
					super::push(out, 35319);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 39099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t31::n48392(s, out),
		['н', s @ ..] => super::t31::n48393(s, out),
		['п', s @ ..] => super::t31::n48394(s, out),
		['р', s @ ..] => super::t31::n48395(s, out),
		['с', s @ ..] => super::t31::n48396(s, out),
		['т', s @ ..] => super::t31::n48397(s, out),
		['ф', s @ ..] => super::t31::n48398(s, out),
		_ => false,
	}
}

/// Words ending -флировать.
pub(crate) fn n30643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20937);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29391);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35334);
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
		['н', s @ ..] => {
			super::push(out, 29392);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35335);
					match s {
						[] => true,
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
			super::push(out, 29393);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35336);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39106);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42452);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42453);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43163);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89466(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 42454);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43164);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89467(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35337);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39107);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41250);
									match s {
										[] => true,
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
							super::push(out, 39108);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41251);
									match s {
										[] => true,
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

/// The word задекламировать.
pub(crate) fn n88379(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лодекламировать.
pub(crate) fn n88380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43165);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 43530);
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

/// Words ending -родекламировать.
pub(crate) fn n88381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43166);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word разрекламировать.
pub(crate) fn n88382(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word прорекламировать.
pub(crate) fn n88383(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отрекламировать.
pub(crate) fn n88384(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word прокламировать.
pub(crate) fn n88385(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word спанорамировать.
pub(crate) fn n88386(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word кремировать.
pub(crate) fn n75297(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -премировать.
pub(crate) fn n63756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35343);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -змировать.
pub(crate) fn n30648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29401);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39117);
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
				['п', s @ ..] => {
					super::push(out, 35345);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39118);
							match s {
								[] => true,
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
					super::push(out, 35346);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39119);
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
		_ => false,
	}
}

/// Words ending -имировать.
pub(crate) fn n30649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20943);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29402);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35347);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 35348);
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
		['м', s @ ..] => {
			super::push(out, 29403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29404);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35350);
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
		['р', s @ ..] => {
			super::push(out, 29405);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39120);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39121);
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
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 35352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 39122);
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
		['с', s @ ..] => {
			super::push(out, 29406);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35353);
					match s {
						[] => true,
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['т', s @ ..] => {
			super::push(out, 29407);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35354);
					match s {
						[] => true,
						['г', 'е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ммировать.
pub(crate) fn n30650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20944);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29408);
			match s {
				[] => true,
				['р', 'г', 'о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39123);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39124);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29409);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39125);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41253);
									match s {
										[] => true,
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
					super::push(out, 35356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39126);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41254);
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

/// Words ending -омировать.
pub(crate) fn n30651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20945);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29410);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35357);
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
		['л', s @ ..] => {
			super::push(out, 29411);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35358);
					match s {
						[] => true,
						['и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 35359);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39127);
							match s {
								[] => true,
								['ю', 'л', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39128);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41255);
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35360);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39129);
							match s {
								[] => true,
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
			super::push(out, 29412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35361);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41256);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 41257);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41258);
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
				['о', s @ ..] => {
					super::push(out, 35362);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39130);
							match s {
								[] => true,
								['х', 'а', 'р', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмировать.
pub(crate) fn n30652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20946);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29413);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39131);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39132);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41259);
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
				['ф', s @ ..] => {
					super::push(out, 35364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39133);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39134);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41260);
									match s {
										[] => true,
										['р', 'е', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 39135);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41261);
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
											super::push(out, 42465);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43167);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89470(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42466);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43168);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89471(s, out),
														['е', 'н', '\u{1}', s @ ..] => super::t31::n89472(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39136);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41262);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 42467);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 43169);
													match s {
														[] => true,
														['д', '\u{1}', s @ ..] => super::t31::n89473(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42468);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43170);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89474(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39137);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41263);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 41264);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 42469);
											match s {
												[] => true,
												['л', 'х', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 39138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41265);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42470);
											match s {
												[] => true,
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
									super::push(out, 41266);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 42471);
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
		_ => false,
	}
}

/// Words ending -тмировать.
pub(crate) fn n30653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20947);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29414);
			match s {
				[] => true,
				['р', 'о', 'г', 'л', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умировать.
pub(crate) fn n30654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20948);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29415);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 29416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29417);
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

/// Words ending -фмировать.
pub(crate) fn n30655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20949);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29418);
			match s {
				[] => true,
				['р', 'а', 'г', 'о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39139);
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

/// Words ending -ьмировать.
pub(crate) fn n30656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20950);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29419);
			match s {
				[] => true,
				['и', 'ф', 'о', 'р', 'к', 'и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмировать.
pub(crate) fn n30657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20951);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 29420);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 39140);
							match s {
								[] => true,
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
			super::push(out, 29421);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35366);
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
		['ф', s @ ..] => {
			super::push(out, 29422);
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

/// Words ending -компанировать.
pub(crate) fn n82419(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42472);
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
					super::push(out, 43171);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 43172);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43531);
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
					super::push(out, 43173);
					match s {
						[] => true,
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

/// Words ending -ранировать.
pub(crate) fn n48438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29428);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35378);
			match s {
				[] => true,
				['э', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41270);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41271);
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

/// Words ending -санировать.
pub(crate) fn n48439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29429);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -танировать.
pub(crate) fn n48440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29430);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35379);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 39154);
					match s {
						[] => true,
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
			super::push(out, 35380);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41272);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41273);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42473);
									match s {
										[] => true,
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
							super::push(out, 41274);
							match s {
								[] => true,
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

/// Words ending -фанировать.
pub(crate) fn n48441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29431);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 35381);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39155);
					match s {
						[] => true,
						['л', 'е', 'ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гнировать.
pub(crate) fn n30659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20953);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29432);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35382);
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
		['е', s @ ..] => {
			super::push(out, 29433);
			match s {
				[] => true,
				['р', 'п', 'м', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35383);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39156);
							match s {
								[] => true,
								['т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35384);
					match s {
						[] => true,
						['а', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енировать.
pub(crate) fn n30660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20954);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29434);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35385);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39157);
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
					super::push(out, 35386);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39158);
							match s {
								[] => true,
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
		['г', s @ ..] => {
			super::push(out, 29435);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35387);
					match s {
						[] => true,
						['л', 'а', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 29436);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35388);
					match s {
						[] => true,
						['и', 'л', 'о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 29437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29438);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35389);
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
		['р', s @ ..] => {
			super::push(out, 29439);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35390);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39159);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 41275);
									match s {
										[] => true,
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
					super::push(out, 35391);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39160);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39161);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 39162);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41276);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 41277);
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
							super::push(out, 39163);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39164);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39165);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 39166);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 41278);
									match s {
										[] => true,
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
					super::push(out, 35393);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39167);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41279);
									match s {
										[] => true,
										['и', 'ш', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35394);
					match s {
						[] => true,
						['к', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 29441);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35395);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41280);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42474);
											match s {
												[] => true,
												['и', 'м', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89477(s, out),
														['о', s @ ..] => super::t31::n89478(s, out),
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
									super::push(out, 41281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инировать.
pub(crate) fn n30661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20955);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29442);
			match s {
				[] => true,
				['м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39168);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41282);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42475);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42476);
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
						['с', s @ ..] => {
							super::push(out, 39169);
							match s {
								[] => true,
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
		['в', s @ ..] => {
			super::push(out, 29443);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35396);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39170);
							match s {
								[] => true,
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
			super::push(out, 29444);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35397);
					match s {
						[] => true,
						['в', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 29445);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35398);
					match s {
						[] => true,
						['р', 'е', 'т', 'е', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35399);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 41283);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 42477);
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
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42478);
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
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 29446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35400);
					match s {
						[] => true,
						['м', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 35401);
					match s {
						[] => true,
						['а', 'м', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 29447);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35402);
					match s {
						[] => true,
						['г', 'а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29448);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39171);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 39172);
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
						['л', s @ ..] => {
							super::push(out, 39173);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39174);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 41284);
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
				['е', s @ ..] => {
					super::push(out, 35405);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 35406);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39175);
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
					super::push(out, 35407);
					match s {
						[] => true,
						['л', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41285);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 42479);
											s.is_empty()
										}
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
					super::push(out, 35408);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41286);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41287);
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
						['н', s @ ..] => {
							super::push(out, 39177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41288);
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
				['р', s @ ..] => super::t31::n63837(s, out),
				['у', s @ ..] => super::t31::n63838(s, out),
				['ь', s @ ..] => super::t31::n63839(s, out),
				['ю', s @ ..] => super::t31::n63840(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t31::n48461(s, out),
		['т', s @ ..] => super::t31::n48462(s, out),
		['у', s @ ..] => super::t31::n48463(s, out),
		['ф', s @ ..] => super::t31::n48464(s, out),
		['ц', s @ ..] => super::t31::n48465(s, out),
		['ш', s @ ..] => super::t31::n48466(s, out),
		_ => false,
	}
}

/// Words ending -йнировать.
pub(crate) fn n30662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29456);
			match s {
				[] => true,
				['б', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнировать.
pub(crate) fn n30663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20957);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29457);
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

/// Words ending -ннировать.
pub(crate) fn n30664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20958);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29458);
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

/// Words ending -онировать.
pub(crate) fn n30665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20959);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39186);
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
		['г', s @ ..] => {
			super::push(out, 29460);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35425);
					match s {
						[] => true,
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
			super::push(out, 29461);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 29462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35426);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 35427);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41299);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42482);
											match s {
												[] => true,
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
									super::push(out, 41300);
									match s {
										[] => true,
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
					super::push(out, 35428);
					match s {
						[] => true,
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
			super::push(out, 29463);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35429);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39188);
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
				['ц', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39189);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41301);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 41302);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 42483);
											match s {
												[] => true,
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
							super::push(out, 39190);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41303);
									match s {
										[] => true,
										['н', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 41304);
									match s {
										[] => true,
										['о', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43176);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n89480(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39191);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41305);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42484);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 43177);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89481(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => super::t31::n82486(s, out),
								['н', s @ ..] => super::t31::n82487(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39192);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41308);
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
						['ю', s @ ..] => super::t31::n75430(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t31::n48475(s, out),
		['к', s @ ..] => super::t31::n48476(s, out),
		['л', s @ ..] => super::t31::n48477(s, out),
		['м', s @ ..] => super::t31::n48478(s, out),
		['п', s @ ..] => super::t31::n48479(s, out),
		['р', s @ ..] => super::t31::n48480(s, out),
		['с', s @ ..] => super::t31::n48481(s, out),
		['т', s @ ..] => super::t31::n48482(s, out),
		['ф', s @ ..] => super::t31::n48483(s, out),
		['ш', s @ ..] => super::t31::n48484(s, out),
		_ => false,
	}
}

/// Words ending -рнировать.
pub(crate) fn n30666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20960);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29474);
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29475);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35456);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 39224);
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
					super::push(out, 35457);
					match s {
						[] => true,
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ь', 'л', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29476);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35459);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39225);
							match s {
								[] => true,
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
			super::push(out, 29477);
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

/// Words ending -унировать.
pub(crate) fn n30667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20961);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 29478);
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

/// Words ending -копировать.
pub(crate) fn n48505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29492);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 35469);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39232);
					match s {
						[] => true,
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
			super::push(out, 35470);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39233);
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
		['н', s @ ..] => {
			super::push(out, 35471);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39234);
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
		['о', s @ ..] => {
			super::push(out, 35472);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39235);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39236);
					match s {
						[] => true,
						['е', 'с', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42496);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42497);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 42498);
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
					super::push(out, 39237);
					match s {
						[] => true,
						['о', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 35473);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39238);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39239);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41326);
							match s {
								[] => true,
								['и', 'т', 'к', 'а', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 41327);
							match s {
								[] => true,
								['т', 'к', 'е', 'п', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 41328);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42499);
									match s {
										[] => true,
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
			super::push(out, 35474);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39240);
					match s {
						[] => true,
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

/// Words ending -ппировать.
pub(crate) fn n30677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20971);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35479);
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
		['у', s @ ..] => {
			super::push(out, 29498);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35480);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41329);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41330);
									match s {
										[] => true,
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

/// Words ending -рпировать.
pub(crate) fn n30678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20972);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29499);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35481);
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
		['у', s @ ..] => {
			super::push(out, 29500);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35482);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отпировать.
pub(crate) fn n48514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29501);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -упировать.
pub(crate) fn n30680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20974);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'о', s @ ..] => {
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

/// Words ending -ьпировать.
pub(crate) fn n30681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20975);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29503);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35483);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41331);
									match s {
										[] => true,
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
							super::push(out, 39242);
							match s {
								[] => true,
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
					super::push(out, 35484);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39243);
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
		_ => false,
	}
}

/// Words ending -юпировать.
pub(crate) fn n30682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20976);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29504);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отпрепарировать.
pub(crate) fn n88429(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отреставрировать.
pub(crate) fn n88430(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -еврировать.
pub(crate) fn n48525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29511);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 35495);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41339);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41340);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42502);
									match s {
										[] => true,
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
							super::push(out, 41341);
							match s {
								[] => true,
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

/// Words ending -грировать.
pub(crate) fn n30686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20980);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29512);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35496);
					match s {
						[] => true,
						['н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41342);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42503);
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
			super::push(out, 29513);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41343);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42504);
											match s {
												[] => true,
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
						['э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41344);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42505);
											match s {
												[] => true,
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

/// Words ending -дрировать.
pub(crate) fn n30687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20981);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29514);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39254);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39255);
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
						['ы', s @ ..] => {
							super::push(out, 39256);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 41345);
									match s {
										[] => true,
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
			super::push(out, 29515);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39257);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39258);
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
		['н', s @ ..] => {
			super::push(out, 29516);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39259);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 39260);
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

/// Words ending -ерировать.
pub(crate) fn n30688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20982);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29517);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35498);
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
		['д', s @ ..] => {
			super::push(out, 29518);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35499);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 39261);
							match s {
								[] => true,
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
					super::push(out, 35500);
					match s {
						[] => true,
						['с', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35501);
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
		['ж', s @ ..] => {
			super::push(out, 29519);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35502);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39262);
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29520);
			match s {
				[] => true,
				['е', 'с', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29521);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35503);
					match s {
						[] => true,
						['л', 'г', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29522);
			match s {
				[] => true,
				['е', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39263);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 41346);
									match s {
										[] => true,
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
							super::push(out, 39264);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 41347);
									match s {
										[] => true,
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
							super::push(out, 39265);
							match s {
								[] => true,
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
			super::push(out, 29523);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35504);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39266);
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
				['о', s @ ..] => {
					super::push(out, 35505);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39267);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42506);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42507);
											match s {
												[] => true,
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
									super::push(out, 41348);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42508);
											match s {
												[] => true,
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
					super::push(out, 35506);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39268);
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
		['т', s @ ..] => {
			super::push(out, 29524);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 41349);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 42509);
											match s {
												[] => true,
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
									super::push(out, 41350);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 42510);
											match s {
												[] => true,
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
									super::push(out, 41351);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 42511);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29525);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35507);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41352);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41353);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 41354);
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
				['и', s @ ..] => {
					super::push(out, 35508);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39270);
							match s {
								[] => true,
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 39271);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41355);
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
				['н', s @ ..] => super::t31::n63971(s, out),
				['р', s @ ..] => super::t31::n63972(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t31::n48540(s, out),
		_ => false,
	}
}

/// Words ending -ирировать.
pub(crate) fn n30689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20983);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29527);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 29528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35512);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 35513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39274);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41357);
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
						['и', s @ ..] => {
							super::push(out, 39275);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 39276);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42512);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42513);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42514);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икрировать.
pub(crate) fn n48544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29529);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 35514);
			match s {
				[] => true,
				['и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орировать.
pub(crate) fn n30691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20985);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29530);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'л', 'л', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 29531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35515);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 29532);
			match s {
				[] => true,
				['а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29533);
			match s {
				[] => true,
				['л', 'е', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29534);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39277);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41358);
									match s {
										[] => true,
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
			super::push(out, 29535);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 35517);
					match s {
						[] => true,
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39278);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39279);
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
			super::push(out, 29536);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35518);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39280);
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
				['р', s @ ..] => {
					super::push(out, 35519);
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
			super::push(out, 29537);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35520);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41359);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42515);
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
		['п', s @ ..] => {
			super::push(out, 29538);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 29539);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35521);
					match s {
						[] => true,
						['н', 'о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35522);
					match s {
						[] => true,
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 35523);
					match s {
						[] => true,
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
			super::push(out, 29540);
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

/// Words ending -еррировать.
pub(crate) fn n48556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29541);
	s.is_empty()
}

/// Words ending -трировать.
pub(crate) fn n30693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20987);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29542);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39281);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41360);
									match s {
										[] => true,
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 39282);
							match s {
								[] => true,
								['н', 'о', 'р', 'х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42516);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42517);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42518);
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
		['и', s @ ..] => {
			super::push(out, 29543);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29544);
			match s {
				[] => true,
				['е', 'ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 39283);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42519);
											match s {
												[] => true,
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
							super::push(out, 39284);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 41361);
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
						['с', s @ ..] => {
							super::push(out, 39285);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39286);
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
					super::push(out, 35525);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39287);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 39288);
							match s {
								[] => true,
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
					super::push(out, 35526);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 39289);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41362);
									match s {
										[] => true,
										['е', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 39290);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41363);
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
				['и', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 39291);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42520);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42521);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43185);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 42522);
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
						['н', s @ ..] => {
							super::push(out, 39292);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41364);
									match s {
										[] => true,
										['м', 'д', 'а', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43186);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43187);
													match s {
														[] => true,
														['р', 'е', 'п', '\u{1}', s @ ..] => super::t31::n89490(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35527);
					match s {
						[] => true,
						['о', 'м', 'е', 'д', s @ ..] => super::t31::n75592(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t31::n64003(s, out),
				['ю', s @ ..] => super::t31::n64004(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t31::n48561(s, out),
		['ь', s @ ..] => super::t31::n48562(s, out),
		_ => false,
	}
}

/// Words ending -урировать.
pub(crate) fn n30694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20988);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35531);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41368);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 41369);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43190);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t31::n89492(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 43191);
													match s {
														[] => true,
														['т', s @ ..] => super::t31::n89493(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 43192);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89494(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35532);
					match s {
						[] => true,
						['а', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 29547);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 29548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35533);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 35534);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39294);
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
		['т', s @ ..] => {
			super::push(out, 29549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35535);
					match s {
						[] => true,
						['н', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 35536);
					match s {
						[] => true,
						['а', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'р', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41370);
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
				['с', 'к', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрировать.
pub(crate) fn n30695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20989);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 29550);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 35537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35538);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 35539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39296);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 41371);
									match s {
										[] => true,
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

/// Words ending -эрировать.
pub(crate) fn n30696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20990);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 35540);
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

/// Words ending -юрировать.
pub(crate) fn n30697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20991);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29553);
			match s {
				[] => true,
				['р', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 29554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 29555);
			match s {
				[] => true,
				['с', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 29556);
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

/// The word доиндексировать.
pub(crate) fn n88458(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word проиндексировать.
pub(crate) fn n88459(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word мультиплексировать.
pub(crate) fn n88460(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word комплексировать.
pub(crate) fn n88461(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word рефлексировать.
pub(crate) fn n88462(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -орефлексировать.
pub(crate) fn n88463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43193);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 43540);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емиксировать.
pub(crate) fn n75617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39305);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фиксировать.
pub(crate) fn n64034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35549);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39306);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 39307);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 39308);
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

/// Words ending -оксировать.
pub(crate) fn n48582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29565);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35550);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 35551);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39309);
					match s {
						[] => true,
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

/// Words ending -уксировать.
pub(crate) fn n48583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29566);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39310);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 39311);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41376);
							match s {
								[] => true,
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

/// Words ending -нсировать.
pub(crate) fn n30702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20996);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29567);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35553);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41377);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42530);
											match s {
												[] => true,
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
					super::push(out, 35554);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39312);
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
				['е', s @ ..] => {
					super::push(out, 35555);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39313);
							match s {
								[] => true,
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
					super::push(out, 35556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39314);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39315);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42531);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43194);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89496(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 42532);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43195);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89497(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 42533);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43196);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t31::n89498(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 42534);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43197);
													match s {
														[] => true,
														['п', s @ ..] => super::t31::n89499(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42535);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 43198);
													match s {
														[] => true,
														['д', s @ ..] => super::t31::n89500(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 42536);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43199);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35557);
					match s {
						[] => true,
						['и', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41378);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41379);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 42537);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43200);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43201);
													match s {
														[] => true,
														['н', s @ ..] => super::t31::n89502(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => super::t31::n86492(s, out),
										['с', s @ ..] => super::t31::n86493(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => super::t31::n64043(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t31::n48585(s, out),
		['о', s @ ..] => super::t31::n48586(s, out),
		_ => false,
	}
}

/// Words ending -псировать.
pub(crate) fn n30703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20997);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29570);
			match s {
				[] => true,
				['л', 'л', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39321);
							match s {
								[] => true,
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

/// Words ending -рсировать.
pub(crate) fn n30704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20998);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29571);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39322);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41383);
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
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 39323);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29572);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 35562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 29573);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35563);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39324);
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
				['к', s @ ..] => {
					super::push(out, 35564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39325);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41384);
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

/// Words ending -ссировать.
pub(crate) fn n30705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20999);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29574);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 39326);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39327);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41385);
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
				['м', s @ ..] => {
					super::push(out, 35566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39328);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41386);
									match s {
										[] => true,
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
					super::push(out, 35567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35568);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 39329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41387);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 42542);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39330);
							match s {
								[] => true,
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
			super::push(out, 29575);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39331);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39332);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41388);
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
				['р', s @ ..] => {
					super::push(out, 35570);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 39333);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41389);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 41390);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41391);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41392);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 42543);
											match s {
												[] => true,
												['а', 'р', 'т', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 39334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41393);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41394);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 41395);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41396);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 41397);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 41398);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42544);
											match s {
												[] => true,
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
							super::push(out, 39335);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 41399);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 42545);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 42546);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41400);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 42547);
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
				['ц', s @ ..] => {
					super::push(out, 35571);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39336);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41401);
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
		['и', s @ ..] => {
			super::push(out, 29576);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 35572);
					match s {
						[] => true,
						['е', s @ ..] => super::t31::n75665(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t31::n64061(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t31::n48594(s, out),
		['у', s @ ..] => super::t31::n48595(s, out),
		_ => false,
	}
}

/// Words ending -усировать.
pub(crate) fn n30706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21000);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29579);
			match s {
				[] => true,
				['о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39341);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39342);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41402);
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
							super::push(out, 39343);
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
		['п', s @ ..] => {
			super::push(out, 29580);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35580);
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

/// Words ending -ьсировать.
pub(crate) fn n30707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21001);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35581);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41403);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41404);
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
				['у', s @ ..] => {
					super::push(out, 35582);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39344);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 41405);
									match s {
										[] => true,
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
							super::push(out, 39345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41406);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42549);
											match s {
												[] => true,
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

/// Words ending -юсировать.
pub(crate) fn n30708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21002);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29582);
			match s {
				[] => true,
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39346);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39347);
							match s {
								[] => true,
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

/// The word сагрегатировать.
pub(crate) fn n88483(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -натировать.
pub(crate) fn n48606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29589);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35595);
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

/// Words ending -оатировать.
pub(crate) fn n48607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29590);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 35596);
			match s {
				[] => true,
				['п', 'с', 'к', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -патировать.
pub(crate) fn n48608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29591);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35597);
			match s {
				[] => true,
				['л', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 35598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратировать.
pub(crate) fn n48609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29592);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 35599);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39358);
					match s {
						[] => true,
						['г', s @ ..] => {
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
		['и', s @ ..] => {
			super::push(out, 35600);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
		_ => false,
	}
}

/// Words ending -татировать.
pub(crate) fn n48610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29593);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 35601);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39360);
					match s {
						[] => true,
						['м', 'р', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уатировать.
pub(crate) fn n48611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29594);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 35602);
			match s {
				[] => true,
				['п', 'с', 'к', 'э', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41410);
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

/// Words ending -фатировать.
pub(crate) fn n48612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29595);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 35603);
			match s {
				[] => true,
				['о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шатировать.
pub(crate) fn n48613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29596);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етировать.
pub(crate) fn n30711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21005);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29597);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35604);
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
		['в', s @ ..] => {
			super::push(out, 29598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35605);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39361);
							match s {
								[] => true,
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
			super::push(out, 29599);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35606);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35607);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39362);
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
		['ж', s @ ..] => {
			super::push(out, 29600);
			match s {
				[] => true,
				['д', 'ю', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39363);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39364);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41411);
									match s {
										[] => true,
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
		['к', s @ ..] => {
			super::push(out, 29601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35608);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41412);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41413);
									match s {
										[] => true,
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
							super::push(out, 39366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41414);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41415);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41416);
									match s {
										[] => true,
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
					super::push(out, 35609);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39368);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 41417);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42551);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42552);
											match s {
												[] => true,
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
							super::push(out, 39369);
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
				['н', s @ ..] => {
					super::push(out, 35610);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39370);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41418);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42553);
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
				['р', s @ ..] => {
					super::push(out, 35611);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39371);
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
				['у', s @ ..] => {
					super::push(out, 35612);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 35613);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39373);
							match s {
								[] => true,
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
			super::push(out, 29602);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35614);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 35615);
					match s {
						[] => true,
						['а', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 35616);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39374);
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
					super::push(out, 35617);
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
		['м', s @ ..] => {
			super::push(out, 29603);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35618);
					match s {
						[] => true,
						['р', 'п', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41419);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41420);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 41421);
									match s {
										[] => true,
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
			super::push(out, 29604);
			match s {
				[] => true,
				['е', s @ ..] => super::t31::n64115(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t31::n48622(s, out),
		['ф', s @ ..] => {
			super::push(out, 29606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t31::n64120(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t31::n48624(s, out),
		['ш', s @ ..] => super::t31::n48625(s, out),
		_ => false,
	}
}

/// Words ending -итировать.
pub(crate) fn n30712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21006);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29609);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35626);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39382);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39383);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 41434);
									match s {
										[] => true,
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
					super::push(out, 35627);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39384);
							match s {
								[] => true,
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
					super::push(out, 35628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35629);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39385);
							match s {
								[] => true,
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
			super::push(out, 29610);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39386);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 39387);
							match s {
								[] => true,
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
			super::push(out, 29611);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35631);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41435);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42558);
											match s {
												[] => true,
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
							super::push(out, 39389);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41436);
									match s {
										[] => true,
										['к', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29612);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35632);
					match s {
						[] => true,
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
			super::push(out, 29613);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35633);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 35634);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 39391);
							match s {
								[] => true,
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
			super::push(out, 29614);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35635);
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
				['и', s @ ..] => {
					super::push(out, 35636);
					match s {
						[] => true,
						['б', 'а', 'е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 39392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29615);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35637);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39393);
							match s {
								[] => true,
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 39394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41437);
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
						['о', s @ ..] => {
							super::push(out, 39395);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41438);
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
				['ы', s @ ..] => {
					super::push(out, 35638);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 35639);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39397);
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
				['э', s @ ..] => {
					super::push(out, 35640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29616);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35641);
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
		['п', s @ ..] => {
			super::push(out, 29617);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35642);
					match s {
						[] => true,
						['ц', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29618);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35643);
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
		['ф', s @ ..] => {
			super::push(out, 29619);
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
			super::push(out, 29620);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35644);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39398);
							match s {
								[] => true,
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
					super::push(out, 35645);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39399);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t31::n64147(s, out),
				['с', s @ ..] => {
					super::push(out, 35647);
					match s {
						[] => true,
						['к', s @ ..] => super::t31::n75771(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтировать.
pub(crate) fn n30713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21007);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29621);
			match s {
				[] => true,
				['ф', 'р', 'о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктировать.
pub(crate) fn n30714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21008);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35648);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39403);
							match s {
								[] => true,
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
					super::push(out, 35649);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41441);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41442);
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
								['е', s @ ..] => {
									super::push(out, 41443);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42559);
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
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 41444);
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
					super::push(out, 35650);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39404);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 39405);
							match s {
								[] => true,
								['ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 35651);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39406);
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
				['с', s @ ..] => {
					super::push(out, 35652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39407);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 39408);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42560);
											match s {
												[] => true,
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
			super::push(out, 29622);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 35654);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39409);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41445);
									match s {
										[] => true,
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
							super::push(out, 39410);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41446);
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
				['л', s @ ..] => {
					super::push(out, 35655);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39411);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 41447);
									match s {
										[] => true,
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
							super::push(out, 39412);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 41448);
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
						['ф', s @ ..] => {
							super::push(out, 39413);
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
					super::push(out, 35656);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41449);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41450);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => super::t31::n82801(s, out),
								['с', s @ ..] => super::t31::n82802(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t31::n64160(s, out),
				['р', s @ ..] => super::t31::n64161(s, out),
				['с', s @ ..] => super::t31::n64162(s, out),
				['т', s @ ..] => super::t31::n64163(s, out),
				['ф', s @ ..] => super::t31::n64164(s, out),
				['ъ', s @ ..] => super::t31::n64165(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t31::n48641(s, out),
		['у', s @ ..] => super::t31::n48642(s, out),
		_ => false,
	}
}

/// Words ending -нтировать.
pub(crate) fn n30715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21009);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29625);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35668);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 35669);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 41463);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42568);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41464);
									match s {
										[] => true,
										['к', 'э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', 'а', 'р', 'т', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35670);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 35671);
					match s {
						[] => true,
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35672);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39420);
							match s {
								[] => true,
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
			super::push(out, 29626);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35673);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39421);
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
				['г', s @ ..] => {
					super::push(out, 35674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39422);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 39423);
							match s {
								[] => true,
								['и', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 35675);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39424);
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
				['и', s @ ..] => {
					super::push(out, 35676);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39425);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42569);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43206);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t31::n64180(s, out),
				['т', s @ ..] => {
					super::push(out, 35678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39433);
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
				['ц', s @ ..] => {
					super::push(out, 35679);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41475);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42576);
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
		['о', s @ ..] => super::t31::n48645(s, out),
		['у', s @ ..] => {
			super::push(out, 29628);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 35684);
					match s {
						[] => true,
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

/// Words ending -отировать.
pub(crate) fn n30716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21010);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29629);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35685);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35686);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39446);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 29630);
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
		['г', s @ ..] => {
			super::push(out, 29631);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 35687);
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
			super::push(out, 29632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 29633);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35688);
					match s {
						[] => true,
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
			super::push(out, 29634);
			match s {
				[] => true,
				['б', 'м', 'и', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ж', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 29635);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 29636);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['й', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39447);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41479);
									match s {
										[] => true,
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
					super::push(out, 35689);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39448);
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
		['л', s @ ..] => {
			super::push(out, 29637);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35690);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 41480);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 42580);
											match s {
												[] => true,
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
					super::push(out, 35691);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41481);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 41482);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41483);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 41484);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42581);
											match s {
												[] => true,
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
					super::push(out, 35692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 29638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35693);
					match s {
						[] => true,
						['к', 'с', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29639);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 35694);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41485);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42582);
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
		['п', s @ ..] => {
			super::push(out, 29640);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35695);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41486);
									match s {
										[] => true,
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
			super::push(out, 29641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35696);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39449);
							match s {
								[] => true,
								['к', 'о', 'ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птировать.
pub(crate) fn n30717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21011);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29642);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35697);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41487);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42583);
											match s {
												[] => true,
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
									super::push(out, 41488);
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
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35698);
					match s {
						[] => true,
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
			super::push(out, 29643);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35699);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39450);
							match s {
								[] => true,
								['т', 'н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 35700);
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
			super::push(out, 29644);
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

/// Words ending -ртировать.
pub(crate) fn n30718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21012);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29645);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35701);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39451);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41489);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 42584);
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
				['к', s @ ..] => {
					super::push(out, 35702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39452);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39453);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41490);
									match s {
										[] => true,
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
			super::push(out, 29646);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35703);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41491);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41492);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43215);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n89513(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 43216);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t31::n89514(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 43217);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89515(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 43218);
													match s {
														[] => true,
														['о', '\u{1}', s @ ..] => super::t31::n89516(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35704);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39454);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 42585);
											match s {
												[] => true,
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
				['ц', s @ ..] => {
					super::push(out, 35705);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29647);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35706);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39456);
							match s {
								[] => true,
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
					super::push(out, 35707);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39457);
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
				['п', s @ ..] => {
					super::push(out, 35708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39458);
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
									super::push(out, 41493);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 41494);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 42586);
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
						['м', 'и', s @ ..] => super::t31::n75859(s, out),
						['с', s @ ..] => super::t31::n75860(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t31::n64224(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стировать.
pub(crate) fn n30719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21013);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29648);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35710);
					match s {
						[] => true,
						['л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41497);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41498);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42589);
											match s {
												[] => true,
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
									super::push(out, 41499);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42590);
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
				['м', s @ ..] => {
					super::push(out, 35711);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39462);
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
				['р', s @ ..] => {
					super::push(out, 35712);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 29649);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35713);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39464);
							match s {
								[] => true,
								['р', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 39465);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42591);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42592);
											match s {
												[] => true,
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
						['н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 41500);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 41501);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42593);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43220);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89518(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 42594);
											match s {
												[] => true,
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
				['ж', s @ ..] => {
					super::push(out, 35714);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39466);
							match s {
								[] => true,
								['й', 'а', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35715);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39467);
							match s {
								[] => true,
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
					super::push(out, 35716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39468);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39469);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41502);
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
							super::push(out, 39470);
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
				['ф', s @ ..] => {
					super::push(out, 35717);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39471);
							match s {
								[] => true,
								['н', 'а', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29650);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35718);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39472);
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
				['с', s @ ..] => {
					super::push(out, 35719);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41503);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41504);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42595);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 42596);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43221);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89519(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35720);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39473);
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
		['о', s @ ..] => super::t31::n48669(s, out),
		['у', s @ ..] => super::t31::n48670(s, out),
		['ю', s @ ..] => super::t31::n48671(s, out),
		_ => false,
	}
}

/// Words ending -ттировать.
pub(crate) fn n30720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21014);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29654);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35730);
					match s {
						[] => true,
						['н', 'а', 'и', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29655);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35731);
					match s {
						[] => true,
						['р', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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

/// Words ending -утировать.
pub(crate) fn n30721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21015);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29656);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35732);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 41508);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 29657);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35733);
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
		['г', s @ ..] => {
			super::push(out, 29658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 29659);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35734);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39477);
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
					super::push(out, 35735);
					match s {
						[] => true,
						['и', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41509);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41510);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42598);
											match s {
												[] => true,
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
		['м', s @ ..] => {
			super::push(out, 29660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 35736);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39478);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42599);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 42600);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29661);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35737);
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
		['п', s @ ..] => {
			super::push(out, 29662);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35738);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35739);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29663);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтировать.
pub(crate) fn n30722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21016);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39481);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41511);
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
						['с', s @ ..] => {
							super::push(out, 39482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 39483);
							match s {
								[] => true,
								['с', 'а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42601);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43223);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89521(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35740);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39484);
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
				['у', s @ ..] => {
					super::push(out, 35741);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39485);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 41512);
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
									super::push(out, 41513);
									match s {
										[] => true,
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
							super::push(out, 39486);
							match s {
								[] => true,
								['н', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42602);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43224);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t31::n89522(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютировать.
pub(crate) fn n30723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21017);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29664);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35742);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41514);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42603);
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
		['л', s @ ..] => {
			super::push(out, 29665);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 39487);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39488);
							match s {
								[] => true,
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
					super::push(out, 35743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29666);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35744);
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
		['ш', s @ ..] => {
			super::push(out, 29667);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35745);
					match s {
						[] => true,
						['р', 'а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41515);
									match s {
										[] => true,
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

/// The word деконструировать.
pub(crate) fn n88550(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word реконструировать.
pub(crate) fn n88551(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ереконструировать.
pub(crate) fn n88552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43225);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -треконструировать.
pub(crate) fn n88553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43226);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -татуировать.
pub(crate) fn n64275(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39495);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 39496);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41519);
					match s {
						[] => true,
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
			super::push(out, 39497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41520);
					match s {
						[] => true,
						['э', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 39498);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41521);
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
			super::push(out, 39499);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41522);
					match s {
						[] => true,
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

/// The word проституировать.
pub(crate) fn n82951(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ктуировать.
pub(crate) fn n48697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29676);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 35751);
			match s {
				[] => true,
				['л', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 35752);
			match s {
				[] => true,
				['л', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтуировать.
pub(crate) fn n48698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29677);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35753);
			match s {
				[] => true,
				['ц', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35754);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39500);
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

/// The word телеграфировать.
pub(crate) fn n88554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43227);
	s.is_empty()
}

/// The word перетелеграфировать.
pub(crate) fn n88555(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word протелеграфировать.
pub(crate) fn n88556(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ографировать.
pub(crate) fn n75938(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41524);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42605);
					match s {
						[] => true,
						['б', 'и', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 41525);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42606);
					match s {
						[] => true,
						['с', 'к', 'е', 'л', '\u{1}', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 42607);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43228);
							match s {
								[] => true,
								['т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42608);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43550);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 43551);
									match s {
										[] => true,
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
			super::push(out, 41526);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42609);
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42610);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 43229);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43737);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['о', s @ ..] => {
					super::push(out, 42611);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 43230);
							match s {
								[] => true,
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
			super::push(out, 41527);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 42612);
					match s {
						[] => true,
						['к', 'е', 'п', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41528);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42613);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 43231);
							match s {
								[] => true,
								['о', 'р', 'х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 42614);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43552);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43553);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 43554);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 43555);
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
				['к', s @ ..] => {
					super::push(out, 42615);
					match s {
						[] => true,
						['е', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 43556);
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
				['о', s @ ..] => {
					super::push(out, 42616);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43557);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 43558);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43738);
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
									super::push(out, 43559);
									match s {
										[] => true,
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
					super::push(out, 42617);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43560);
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

/// Words ending -тифировать.
pub(crate) fn n48700(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35756);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 39503);
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
		['с', s @ ..] => {
			super::push(out, 35757);
			match s {
				[] => true,
				['и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -офировать.
pub(crate) fn n30734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21028);
	match s {
		[] => true,
		['р', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35759);
					match s {
						[] => true,
						['е', 'п', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35760);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39504);
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
		_ => false,
	}
}

/// Words ending -пфировать.
pub(crate) fn n30735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21029);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 29678);
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

/// Words ending -рфировать.
pub(crate) fn n30736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21030);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29679);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39505);
							match s {
								[] => true,
								['н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 39506);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 41529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьфировать.
pub(crate) fn n30737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21031);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29680);
			match s {
				[] => true,
				['у', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ицировать.
pub(crate) fn n30739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21033);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29685);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35764);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39509);
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
					super::push(out, 35765);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41531);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 41532);
									match s {
										[] => true,
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
			super::push(out, 29686);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35766);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41533);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 41534);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42618);
											match s {
												[] => true,
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
		['к', s @ ..] => {
			super::push(out, 29687);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 29688);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39510);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 39511);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 41535);
									match s {
										[] => true,
										['ь', 'л', 'у', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 39512);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 41536);
									match s {
										[] => true,
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
							super::push(out, 39513);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41537);
									match s {
										[] => true,
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
							super::push(out, 39514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 29689);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35767);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39515);
							match s {
								[] => true,
								['м', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 29690);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35768);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 39516);
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
				['т', s @ ..] => {
					super::push(out, 35769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39517);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42619);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42620);
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
		['ф', s @ ..] => super::t31::n48715(s, out),
		_ => false,
	}
}

/// Words ending -дуцировать.
pub(crate) fn n48723(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35785);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['о', 'р', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41563);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42635);
									match s {
										[] => true,
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
				['т', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушировать.
pub(crate) fn n48739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29711);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35798);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 35799);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41568);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41569);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 42636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 41570);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 41571);
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

/// Words ending -эшировать.
pub(crate) fn n30755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21049);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35800);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39557);
							match s {
								[] => true,
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
			super::push(out, 29713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -доэкзаменовывать.
pub(crate) fn n88608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43245);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 43569);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word проэкзаменовывать.
pub(crate) fn n88609(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -формировывать.
pub(crate) fn n64722(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
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

/// Words ending -нировывать.
pub(crate) fn n49206(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -пировывать.
pub(crate) fn n49207(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -сировывать.
pub(crate) fn n49208(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -тировывать.
pub(crate) fn n49209(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
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

/// Words ending -воровывать.
pub(crate) fn n31107(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тровывать.
pub(crate) fn n31108(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', 'т', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', 'е', 'ц', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'е', 'к', 'р', 'о', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', 'л', 'и', 'ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уровывать.
pub(crate) fn n31109(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
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
		['ш', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ифровывать.
pub(crate) fn n31110(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['ц', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -арестовывать.
pub(crate) fn n64801(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аттестовывать.
pub(crate) fn n64802(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -хтовывать.
pub(crate) fn n31128(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'р', 'ф', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word размачтовывать.
pub(crate) fn n31129(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word полюбопытничать.
pub(crate) fn n88614(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -олеваться.
pub(crate) fn n40412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25190);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 31713);
			match s {
				[] => true,
				['о', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -плеваться.
pub(crate) fn n40413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25191);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31714);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 31715);
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
		['о', s @ ..] => {
			super::push(out, 31716);
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
			super::push(out, 31717);
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
			super::push(out, 31718);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36572);
					match s {
						[] => true,
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

/// Words ending -тлеваться.
pub(crate) fn n40414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25192);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31719);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36573);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 39812);
							match s {
								[] => true,
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
					super::push(out, 36574);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
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
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обдергиваться.
pub(crate) fn n67200(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вдергиваться.
pub(crate) fn n67201(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поддергиваться.
pub(crate) fn n67202(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word передергиваться.
pub(crate) fn n67203(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -здергиваться.
pub(crate) fn n67204(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word придергиваться.
pub(crate) fn n67205(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -одергиваться.
pub(crate) fn n67206(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word сдергиваться.
pub(crate) fn n67207(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отдергиваться.
pub(crate) fn n67208(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -выдергиваться.
pub(crate) fn n67209(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -угиваться.
pub(crate) fn n40510(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
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
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
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
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
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
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгиваться.
pub(crate) fn n40511(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', 'ш', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -тягиваться.
pub(crate) fn n40512(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'д', s @ ..] => {
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
		['с', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -лаживаться.
pub(crate) fn n54831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31748);
	match s {
		[] => true,
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', 'а', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'е', 'п', '\u{1}', s @ ..] => {
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
			match s {
				[] => true,
				['в', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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
		_ => false,
	}
}

/// The word припомаживаться.
pub(crate) fn n77171(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тормаживаться.
pub(crate) fn n67284(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -наживаться.
pub(crate) fn n54833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раживаться.
pub(crate) fn n54834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31750);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['о', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['л', 'б', 'о', '\u{1}', s @ ..] => {
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'у', 'б', 'з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'с', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'к', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -саживаться.
pub(crate) fn n54835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31751);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36602);
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хаживаться.
pub(crate) fn n54836(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'б', 'о', '\u{1}', s @ ..] => {
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бживаться.
pub(crate) fn n40521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25233);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вживаться.
pub(crate) fn n40522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25234);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еживаться.
pub(crate) fn n40523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25235);
	match s {
		[] => true,
		['д', 'а', 'н', s @ ..] => {
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
		['л', s @ ..] => {
			super::push(out, 31753);
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
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'з', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 31754);
			match s {
				[] => true,
				['в', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 36603);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39835);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41648);
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
						['в', s @ ..] => {
							super::push(out, 39836);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41649);
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
						['п', s @ ..] => {
							super::push(out, 39837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 41650);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 42648);
											match s {
												[] => true,
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
									super::push(out, 41651);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 42649);
											match s {
												[] => true,
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
									super::push(out, 41652);
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
								['ы', s @ ..] => {
									super::push(out, 41653);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42650);
											match s {
												[] => true,
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
					super::push(out, 36604);
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
					super::push(out, 36605);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
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
								_ => false,
							}
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
		['с', 'о', 'с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 31755);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 31756);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 39838);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41654);
									match s {
										[] => true,
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

/// Words ending -важживаться.
pub(crate) fn n40524(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зживаться.
pub(crate) fn n40525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25236);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31757);
			match s {
				[] => true,
				['м', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 36606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31758);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36607);
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
				['ъ', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36608);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иживаться.
pub(crate) fn n40526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25237);
	match s {
		[] => true,
		['в', 'д', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 31760);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31761);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word одалживаться.
pub(crate) fn n40527(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оживаться.
pub(crate) fn n40528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25238);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'т', 'с', s @ ..] => {
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
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'о', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 31762);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'у', 'к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'п', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', 'а', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'е', 'р', 'т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', 'л', 'г', 'у', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'л', 'с', 'и', 'к', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 36611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'ы', 'д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рживаться.
pub(crate) fn n40529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25239);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', 'т', s @ ..] => {
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', s @ ..] => {
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
								_ => false,
							}
						}
						['у', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -сживаться.
pub(crate) fn n40530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25240);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тживаться.
pub(crate) fn n40531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25241);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31763);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

