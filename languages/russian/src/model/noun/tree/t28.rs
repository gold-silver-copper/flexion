use flexion::Path;

/// Words ending -зая.
pub(crate) fn n4798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4787);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19332);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19333);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43878);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19334);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43879);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72264);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97274);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 115678);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127482);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n135892(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кая.
pub(crate) fn n4799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4788);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19335);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 19336);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43880);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72265);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97275);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 115679);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19337);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 43881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19338);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 43882);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72266);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97276);
									s.is_empty()
								}
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
			super::push(out, 19339);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 43883);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72267);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 97277);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115680);
											s.is_empty()
										}
										_ => false,
									}
								}
								['щ', s @ ..] => {
									super::push(out, 97278);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 115681);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72268);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97279);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 115682);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127483);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43884);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72269);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97280);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115683);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 127484);
													s.is_empty()
												}
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
							super::push(out, 72270);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43885);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 72271);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97282);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 115684);
											s.is_empty()
										}
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
					super::push(out, 43886);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72272);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 43887);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72273);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97284);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115685);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127485);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 97285);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 115686);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127486);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n135893(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43888);
					match s {
						[] => true,
						['а', s @ ..] => super::t40::n73028(s, out),
						['и', s @ ..] => {
							super::push(out, 72275);
							match s {
								[] => true,
								['р', s @ ..] => super::t40::n98327(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t40::n44313(s, out),
				['с', s @ ..] => super::t40::n44314(s, out),
				['т', s @ ..] => super::t40::n44315(s, out),
				['ь', s @ ..] => super::t40::n44316(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t40::n19445(s, out),
		['ь', s @ ..] => super::t40::n19446(s, out),
		_ => false,
	}
}

/// Words ending -лая.
pub(crate) fn n4800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4789);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19342);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43897);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 43898);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 72289);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97305);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 115707);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19343);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 43900);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19344);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19345);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 43902);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мая.
pub(crate) fn n4801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4790);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19346);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43903);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72293);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97306);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 115708);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 127502);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43904);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72294);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 97307);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 115709);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127503);
													match s {
														[] => true,
														['г', s @ ..] => super::t40::n135905(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43905);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72295);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97308);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 115710);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127504);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 43907);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72297);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97309);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 115711);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 127505);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19347);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43908);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 72298);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 43909);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72299);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 43910);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72300);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97312);
									s.is_empty()
								}
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
			super::push(out, 19348);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43911);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72301);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97313);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 115712);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19349);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 43912);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72302);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 43913);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72303);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97315);
									s.is_empty()
								}
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
			super::push(out, 19350);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43914);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ная.
pub(crate) fn n4802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19351);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 43915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 43916);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72306);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97316);
									s.is_empty()
								}
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
			super::push(out, 19352);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 43917);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19353);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43918);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72308);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97317);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115713);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 127506);
													match s {
														[] => true,
														['в', s @ ..] => super::t40::n135906(s, out),
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
									super::push(out, 97318);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115714);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 127507);
													match s {
														[] => true,
														['а', s @ ..] => super::t40::n135907(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43919);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72309);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97319);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115715);
											s.is_empty()
										}
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
					super::push(out, 43920);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72310);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97320);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 72311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43921);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72312);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 72313);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97321);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115717);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 72314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19354);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43922);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72315);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97322);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115718);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 72316);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97323);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 115719);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 72317);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97324);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115720);
											s.is_empty()
										}
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
					super::push(out, 43923);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72318);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 97325);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 115721);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 127508);
													match s {
														[] => true,
														['н', s @ ..] => super::t40::n135908(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 43924);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72319);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 97326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 72320);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97327);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 115722);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 72321);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97328);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 115723);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 127509);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 97329);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115724);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 97330);
									match s {
										[] => true,
										['в', s @ ..] => super::t40::n116972(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t40::n44349(s, out),
				['я', s @ ..] => super::t40::n44350(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t40::n19460(s, out),
		['ж', s @ ..] => super::t40::n19461(s, out),
		['з', s @ ..] => super::t40::n19462(s, out),
		['и', s @ ..] => super::t40::n19463(s, out),
		['й', s @ ..] => super::t40::n19464(s, out),
		['к', s @ ..] => super::t40::n19465(s, out),
		['м', s @ ..] => super::t40::n19466(s, out),
		['н', s @ ..] => super::t40::n19467(s, out),
		['п', s @ ..] => super::t40::n19468(s, out),
		['р', s @ ..] => super::t40::n19469(s, out),
		['с', s @ ..] => super::t40::n19470(s, out),
		['т', s @ ..] => super::t40::n19471(s, out),
		['ф', s @ ..] => super::t40::n19472(s, out),
		['ч', s @ ..] => super::t40::n19473(s, out),
		['ш', s @ ..] => super::t40::n19474(s, out),
		['щ', s @ ..] => super::t40::n19475(s, out),
		['ь', s @ ..] => super::t40::n19476(s, out),
		['я', s @ ..] => super::t40::n19477(s, out),
		_ => false,
	}
}

/// Words ending -пая.
pub(crate) fn n4803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4792);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19373);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43983);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 43984);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рая.
pub(crate) fn n4804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4793);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19374);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19375);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 43985);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19376);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19377);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43987);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сая.
pub(crate) fn n4805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4794);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19378);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 43988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 43989);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72440);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97455);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 115842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 72441);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 97456);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 115843);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тая.
pub(crate) fn n4806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4795);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19379);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 43990);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72442);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97457);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 115844);
											s.is_empty()
										}
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
					super::push(out, 43991);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72443);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97458);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 115845);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 127592);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43992);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72444);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97459);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115846);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19380);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43993);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72445);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97460);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115847);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 127593);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 43994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19381);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43995);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72446);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97461);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 115848);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19382);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43996);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 43997);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 72447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 43998);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72448);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97462);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 115849);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 127594);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19383);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43999);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72449);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97463);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115850);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19384);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44000);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72450);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 97464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 72451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44001);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72452);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 97465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44002);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72453);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 97466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 44003);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72454);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 97467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хая.
pub(crate) fn n4808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4797);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19386);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 19387);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44004);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 72455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 44005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чая.
pub(crate) fn n4809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4798);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 19388);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44006);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72456);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97469);
									s.is_empty()
								}
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
			super::push(out, 19389);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44007);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 72457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19390);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44008);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72458);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97470);
									s.is_empty()
								}
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
			super::push(out, 19391);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 44009);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19392);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44010);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72460);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 97471);
									s.is_empty()
								}
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
			super::push(out, 19393);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44011);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шая.
pub(crate) fn n4810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4799);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19394);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44012);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19395);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44013);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 72463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19396);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44014);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 19397);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44015);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72465);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 97472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щая.
pub(crate) fn n4811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4800);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19398);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 44016);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72466);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97473);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 115851);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19399);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19400);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44018);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72467);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97474);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 115852);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 127595);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n135962(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44019);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72468);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 44020);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 72469);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97476);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 115853);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127596);
													match s {
														[] => true,
														['м', s @ ..] => super::t40::n135963(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44021);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 72470);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97477);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 115854);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 127597);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19401);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44022);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72471);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97478);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 115855);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 127598);
													s.is_empty()
												}
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
							super::push(out, 72472);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 97479);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 115856);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 72473);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97480);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 115857);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 72474);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 97481);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 115858);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 127599);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44023);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72475);
							match s {
								[] => true,
								['т', s @ ..] => super::t40::n98526(s, out),
								_ => false,
							}
						}
						['д', s @ ..] => super::t40::n73230(s, out),
						['з', s @ ..] => super::t40::n73231(s, out),
						['р', s @ ..] => super::t40::n73232(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t40::n44448(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t40::n19507(s, out),
		_ => false,
	}
}

/// Words ending -паламедея.
pub(crate) fn n128952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127606);
	s.is_empty()
}

/// Words ending -пингоидея.
pub(crate) fn n128953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127607);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 134565);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хориоидея.
pub(crate) fn n128954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127608);
	s.is_empty()
}

/// Words ending -лея.
pub(crate) fn n4823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4812);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19425);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 44044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 44045);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 44046);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72504);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 97503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 44047);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 72505);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97504);
									s.is_empty()
								}
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
			super::push(out, 19426);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44048);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72506);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97505);
									s.is_empty()
								}
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
			super::push(out, 19427);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44049);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 72507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97506);
									s.is_empty()
								}
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
			super::push(out, 19428);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19429);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 44051);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 44052);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72508);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 97507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19430);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19431);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44055);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 44056);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72510);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97508);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 115870);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 127609);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t40::n135971(s, out),
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
									super::push(out, 97509);
									s.is_empty()
								}
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
			super::push(out, 19432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44057);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 44058);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 44059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19433);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19434);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44061);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72512);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97510);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115871);
											s.is_empty()
										}
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
					super::push(out, 44062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 19435);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мея.
pub(crate) fn n4824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4813);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19436);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 19437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44064);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 44065);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72514);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97511);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 115872);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 127610);
													match s {
														[] => true,
														['т', s @ ..] => super::t40::n135972(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19438);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44066);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72515);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115873);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нея.
pub(crate) fn n4825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4814);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19439);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44067);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 72516);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 44068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19441);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44070);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72517);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97514);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 115874);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 127611);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44071);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 44072);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 72518);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97515);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 115875);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 127612);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опея.
pub(crate) fn n19547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19442);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44073);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 72519);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 97516);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 115876);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 127613);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рея.
pub(crate) fn n4827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4816);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19443);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 19444);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 44076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19445);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44077);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97517);
									s.is_empty()
								}
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
			super::push(out, 19446);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 44079);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 44080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 44081);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72524);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97518);
									match s {
										[] => true,
										['л', 'а', 'г', 'ж', s @ ..] => {
											super::push(out, 115877);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 127614);
													match s {
														[] => true,
														['к', s @ ..] => super::t40::n135973(s, out),
														_ => false,
													}
												}
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
							super::push(out, 72525);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97519);
									s.is_empty()
								}
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
			super::push(out, 19447);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19448);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44084);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72527);
							match s {
								[] => true,
								['м', 'и', s @ ..] => {
									super::push(out, 97520);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 115878);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 127615);
													match s {
														[] => true,
														['п', s @ ..] => super::t40::n135974(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44085);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72528);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97521);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 115879);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 127616);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t40::n135975(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19449);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44086);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 72529);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97522);
									s.is_empty()
								}
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
			super::push(out, 19450);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19451);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 44088);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72530);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97523);
									s.is_empty()
								}
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
			super::push(out, 19452);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44089);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97524);
									s.is_empty()
								}
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
			super::push(out, 19453);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44090);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 72532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сея.
pub(crate) fn n4828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4817);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 19454);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44091);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19455);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44092);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97526);
									s.is_empty()
								}
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
			super::push(out, 19456);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44093);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 72535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тея.
pub(crate) fn n4829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4818);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19457);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 44094);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 44095);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72536);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 97527);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115880);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 97528);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115881);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19458);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19459);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19460);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 44096);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72537);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97529);
									s.is_empty()
								}
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
			super::push(out, 19461);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44097);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 72539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фея.
pub(crate) fn n4830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4819);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19462);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19463);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19464);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44099);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72540);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 97531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хея.
pub(crate) fn n4831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4820);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19465);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44100);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 72541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19466);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72542);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 72543);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97532);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 115882);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127617);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цея.
pub(crate) fn n4832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4821);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19468);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44102);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72544);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 72545);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 97533);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 115883);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127618);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19469);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19470);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44104);
					match s {
						[] => true,
						['ц', '\u{1}', s @ ..] => {
							super::push(out, 72546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чея.
pub(crate) fn n4833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4822);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19471);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44105);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44106);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 72548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97535);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 115884);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19472);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44107);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 72549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 19473);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44108);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-амфибия.
pub(crate) fn n128965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127619);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 134571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -р-амфибия.
pub(crate) fn n128966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127620);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 134572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -т-амфибия.
pub(crate) fn n128967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127621);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 134573);
			match s {
				[] => true,
				['л', 'о', 'т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
					super::push(out, 138540);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь-амфибия.
pub(crate) fn n128968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127622);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 134574);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 138541);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 140770);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142045);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142778);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 143169);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -офобия.
pub(crate) fn n73313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72559);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 97538);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 115887);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 127623);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 134575);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 138542);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140771);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97539);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 115888);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 127624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'я', s @ ..] => {
			super::push(out, 97540);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 115889);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 127625);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'д', s @ ..] => {
			super::push(out, 97541);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 115890);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 97542);
			match s {
				[] => true,
				['е', 'л', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
					super::push(out, 115891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жулябия.
pub(crate) fn n98587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97543);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 115892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вия.
pub(crate) fn n4844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4833);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19489);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19490);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44128);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 44129);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72562);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97544);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 115893);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127627);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19491);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19492);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					super::push(out, 44130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19493);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44131);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72564);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 97545);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115894);
											s.is_empty()
										}
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
					super::push(out, 44132);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72565);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 97546);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115895);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 97547);
									s.is_empty()
								}
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
			super::push(out, 19494);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44133);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72566);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97548);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гия.
pub(crate) fn n4845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4834);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19495);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19496);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44134);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72567);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 97549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115897);
											s.is_empty()
										}
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
					super::push(out, 44135);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 44136);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72568);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 97550);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115898);
											s.is_empty()
										}
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
					super::push(out, 44137);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72569);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97551);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 72570);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97552);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 115899);
											s.is_empty()
										}
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
					super::push(out, 44138);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72571);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 72572);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 72573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19497);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44139);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72574);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 97553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 72575);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97554);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 115900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 72576);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97555);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 115901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 115902);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 127628);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 115903);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 127629);
													match s {
														[] => true,
														['а', s @ ..] => super::t40::n135982(s, out),
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
									super::push(out, 97556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 72577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44140);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 72578);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 97557);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 115904);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 127630);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n135983(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19498);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44142);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 44143);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72580);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97558);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115905);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 127631);
													match s {
														[] => true,
														['т', s @ ..] => super::t40::n135984(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44144);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72581);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97559);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115906);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 127632);
													match s {
														[] => true,
														['к', s @ ..] => super::t40::n135985(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t40::n19604(s, out),
		['н', s @ ..] => super::t40::n19605(s, out),
		['о', s @ ..] => super::t40::n19606(s, out),
		['р', s @ ..] => super::t40::n19607(s, out),
		['ь', s @ ..] => super::t40::n19608(s, out),
		_ => false,
	}
}

/// Words ending -дия.
pub(crate) fn n4846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4835);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19504);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44156);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72597);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 97589);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115951);
											s.is_empty()
										}
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
					super::push(out, 44157);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72598);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97590);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 115952);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'а', '\u{1}', s @ ..] => {
							super::push(out, 72599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44158);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72600);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97591);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 115953);
											s.is_empty()
										}
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
					super::push(out, 44159);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72601);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97592);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 115954);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 127660);
													match s {
														[] => true,
														['г', s @ ..] => super::t40::n136005(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44160);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 72602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 19505);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44161);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72603);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97593);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 115955);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19506);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44162);
					match s {
						[] => true,
						['о', 'н', s @ ..] => {
							super::push(out, 72604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19507);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44163);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 44164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72605);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 72606);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44165);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72607);
							match s {
								[] => true,
								['к', 'о', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 115956);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 127661);
													match s {
														[] => true,
														['э', s @ ..] => super::t40::n136006(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 115957);
											s.is_empty()
										}
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
					super::push(out, 44166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72608);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 72609);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 115958);
											s.is_empty()
										}
										['и', s @ ..] => {
											match s {
												[] => true,
												['л', 'о', 'п', 'о', s @ ..] => {
													match s {
														[] => true,
														['з', s @ ..] => super::t40::n136007(s, out),
														['л', s @ ..] => super::t40::n136008(s, out),
														['т', s @ ..] => super::t40::n136009(s, out),
														_ => false,
													}
												}
												['х', s @ ..] => {
													super::push(out, 127662);
													match s {
														[] => true,
														['л', s @ ..] => super::t40::n136010(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => super::t40::n117208(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => super::t40::n73365(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t40::n44591(s, out),
				['с', s @ ..] => super::t40::n44592(s, out),
				['ц', s @ ..] => super::t40::n44593(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t40::n19613(s, out),
		['о', s @ ..] => super::t40::n19614(s, out),
		['р', s @ ..] => super::t40::n19615(s, out),
		['у', s @ ..] => super::t40::n19616(s, out),
		['ь', s @ ..] => super::t40::n19617(s, out),
		['ю', s @ ..] => super::t40::n19618(s, out),
		_ => false,
	}
}

/// Words ending -жия.
pub(crate) fn n4847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4836);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 19514);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44182);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зия.
pub(crate) fn n4848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4837);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19515);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44183);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 44184);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72631);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 44185);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72632);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44186);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97615);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 115976);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 127673);
													match s {
														[] => true,
														['в', s @ ..] => super::t40::n136015(s, out),
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
									super::push(out, 97616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97617);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 115977);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 127674);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n136016(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 115978);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 72633);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 97618);
									s.is_empty()
								}
								['о', 'р', s @ ..] => {
									super::push(out, 97619);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 115979);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 127675);
													match s {
														[] => true,
														['о', s @ ..] => super::t40::n136017(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44187);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 72634);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97620);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 97621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => super::t40::n98673(s, out),
								['р', s @ ..] => super::t40::n98674(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t40::n44614(s, out),
				['р', s @ ..] => super::t40::n44615(s, out),
				['т', s @ ..] => super::t40::n44616(s, out),
				['у', s @ ..] => super::t40::n44617(s, out),
				['ф', s @ ..] => super::t40::n44618(s, out),
				['х', s @ ..] => super::t40::n44619(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t40::n19621(s, out),
		['е', s @ ..] => super::t41::n19622(s, out),
		['и', s @ ..] => super::t41::n19623(s, out),
		['н', s @ ..] => super::t41::n19624(s, out),
		['о', s @ ..] => super::t41::n19625(s, out),
		['р', s @ ..] => {
			super::push(out, 19521);
			s.is_empty()
		}
		['у', s @ ..] => super::t41::n19627(s, out),
		['э', s @ ..] => super::t41::n19628(s, out),
		['ю', s @ ..] => super::t41::n19629(s, out),
		_ => false,
	}
}

/// Words ending -кия.
pub(crate) fn n4849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19525);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44219);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72681);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97674);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116016);
											s.is_empty()
										}
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
					super::push(out, 44220);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72682);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97675);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 116017);
											s.is_empty()
										}
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
					super::push(out, 44221);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72683);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 72684);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97677);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116018);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 127698);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44222);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44223);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72686);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97678);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 72687);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 44224);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19526);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44225);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 72689);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 97680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 44226);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97681);
									s.is_empty()
								}
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
			super::push(out, 19527);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 44227);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 72691);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44228);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72692);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44229);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72693);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 97684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44230);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 72694);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 44231);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 72695);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97686);
									s.is_empty()
								}
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
			super::push(out, 19528);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44232);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72696);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97687);
									s.is_empty()
								}
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
			super::push(out, 19529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44234);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72697);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 97688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 44235);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 72698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19530);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44236);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72699);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 97689);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116019);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 72700);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 97690);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116020);
											s.is_empty()
										}
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
					super::push(out, 44237);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 97691);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116021);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 127699);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 97692);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 116022);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72701);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97693);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116023);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 127700);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19531);
			match s {
				[] => true,
				['а', s @ ..] => super::t41::n44667(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t41::n19638(s, out),
		['ы', s @ ..] => super::t41::n19639(s, out),
		_ => false,
	}
}

/// Words ending -лия.
pub(crate) fn n4850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4839);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19534);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 19535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44241);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 44242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72704);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 72705);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97696);
									match s {
										[] => true,
										['о', 'н', s @ ..] => {
											super::push(out, 116024);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 127701);
													match s {
														[] => true,
														['л', s @ ..] => super::t41::n136028(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44243);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 44244);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 72706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 44245);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72707);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 97697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72708);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 44246);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 72709);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97699);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116025);
											s.is_empty()
										}
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
					super::push(out, 44247);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72710);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 97700);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116026);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 72711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72713);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97702);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 97703);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116028);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 97704);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 116029);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127702);
													s.is_empty()
												}
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
							super::push(out, 72714);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97705);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 116030);
											s.is_empty()
										}
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
					super::push(out, 44249);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72716);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44250);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 44251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44252);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 44253);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 44254);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 72718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'в', s @ ..] => {
							super::push(out, 72719);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 72720);
							s.is_empty()
						}
						['с', 'а', s @ ..] => {
							super::push(out, 72721);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116031);
											s.is_empty()
										}
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
					super::push(out, 44255);
					match s {
						[] => true,
						['е', s @ ..] => super::t41::n73485(s, out),
						['т', s @ ..] => {
							super::push(out, 72723);
							match s {
								[] => true,
								['с', s @ ..] => super::t41::n98765(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t41::n19642(s, out),
		['г', s @ ..] => super::t41::n19643(s, out),
		['е', s @ ..] => super::t41::n19644(s, out),
		['и', s @ ..] => super::t41::n19645(s, out),
		['й', s @ ..] => super::t41::n19646(s, out),
		['л', s @ ..] => super::t41::n19647(s, out),
		['о', s @ ..] => super::t41::n19648(s, out),
		['п', s @ ..] => super::t41::n19649(s, out),
		['у', s @ ..] => super::t41::n19650(s, out),
		['ю', s @ ..] => {
			super::push(out, 19545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t41::n44731(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мия.
pub(crate) fn n4851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4840);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19546);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44302);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 44303);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 72793);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 72794);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97784);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116089);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127740);
													s.is_empty()
												}
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
							super::push(out, 72795);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 97785);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116090);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 127741);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44304);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72796);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 97786);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116091);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 116092);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19547);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 44305);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72797);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 97787);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116093);
											s.is_empty()
										}
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
					super::push(out, 44306);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72798);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97788);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116094);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 72799);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116095);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127742);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136052(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['э', s @ ..] => {
											super::push(out, 116096);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 97789);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116097);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 72800);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97790);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116098);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 72801);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 97791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 44307);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 72802);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 44308);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 44309);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 44310);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 72803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44311);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44312);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72805);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 72806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44313);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72807);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 97793);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 72808);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 72809);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97794);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116100);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => super::t41::n98849(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t41::n73574(s, out),
						['п', s @ ..] => super::t41::n73575(s, out),
						['т', s @ ..] => {
							super::push(out, 72812);
							s.is_empty()
						}
						['у', s @ ..] => super::t41::n73577(s, out),
						['я', s @ ..] => super::t41::n73578(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t41::n44744(s, out),
				['т', s @ ..] => super::t41::n44745(s, out),
				['ш', s @ ..] => super::t41::n44746(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t41::n19654(s, out),
		['и', s @ ..] => super::t41::n19655(s, out),
		['й', s @ ..] => super::t41::n19656(s, out),
		['м', s @ ..] => super::t41::n19657(s, out),
		['о', s @ ..] => super::t41::n19658(s, out),
		['р', s @ ..] => super::t41::n19659(s, out),
		['с', s @ ..] => super::t41::n19660(s, out),
		['т', s @ ..] => super::t41::n19661(s, out),
		['у', s @ ..] => super::t41::n19662(s, out),
		['ь', s @ ..] => super::t41::n19663(s, out),
		['э', s @ ..] => super::t41::n19664(s, out),
		['ю', s @ ..] => super::t41::n19665(s, out),
		_ => false,
	}
}

/// Words ending -ния.
pub(crate) fn n4852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19560);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44341);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72851);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 97849);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116150);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 127776);
													match s {
														[] => true,
														['е', s @ ..] => super::t41::n136067(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44342);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 44343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72852);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97850);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 72853);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97851);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116152);
											s.is_empty()
										}
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
					super::push(out, 44344);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 72854);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 97852);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116153);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127777);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'г', s @ ..] => {
									super::push(out, 97853);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116154);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 72855);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 97854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44345);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 72857);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97856);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116155);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 127778);
													match s {
														[] => true,
														['с', s @ ..] => super::t41::n136068(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44346);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 97857);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116156);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127779);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 97858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44347);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72858);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 72859);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 72860);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97861);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 97862);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116157);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 97863);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116158);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 72861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 44348);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72862);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97864);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116159);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 72863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 44349);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72864);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97865);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116160);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127780);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'е', s @ ..] => super::t41::n73632(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t41::n19667(s, out),
		['е', s @ ..] => super::t41::n19668(s, out),
		['и', s @ ..] => super::t41::n19669(s, out),
		['л', s @ ..] => super::t41::n19670(s, out),
		['м', s @ ..] => super::t41::n19671(s, out),
		['н', s @ ..] => super::t41::n19672(s, out),
		['о', s @ ..] => super::t41::n19673(s, out),
		['п', s @ ..] => super::t41::n19674(s, out),
		['р', s @ ..] => super::t41::n19675(s, out),
		['у', s @ ..] => {
			super::push(out, 19570);
			s.is_empty()
		}
		['ф', s @ ..] => super::t41::n19677(s, out),
		['х', s @ ..] => super::t41::n19678(s, out),
		['ю', s @ ..] => super::t41::n19679(s, out),
		_ => false,
	}
}

/// Words ending -пия.
pub(crate) fn n4853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19574);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44403);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 44404);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 72967);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 97970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44405);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 72968);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 97971);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116236);
											s.is_empty()
										}
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
					super::push(out, 44406);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 72969);
							match s {
								[] => true,
								['т', 'о', 'т', s @ ..] => {
									super::push(out, 97972);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116237);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 127820);
													s.is_empty()
												}
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
							super::push(out, 72970);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 97973);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116238);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127821);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19575);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44407);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 72971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97974);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116239);
											s.is_empty()
										}
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
					super::push(out, 44408);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19576);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44409);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 97975);
									match s {
										[] => true,
										['о', 'р', s @ ..] => {
											super::push(out, 116240);
											match s {
												[] => true,
												['х', 'а', s @ ..] => {
													super::push(out, 127822);
													match s {
														[] => true,
														['т', s @ ..] => super::t41::n136085(s, out),
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
									super::push(out, 97976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 44410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19577);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44411);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 72973);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 97977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 44412);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 72974);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 97978);
									s.is_empty()
								}
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
			super::push(out, 19578);
			match s {
				[] => true,
				['и', s @ ..] => super::t41::n44844(s, out),
				['к', s @ ..] => super::t41::n44845(s, out),
				['л', s @ ..] => super::t41::n44846(s, out),
				['н', 'а', s @ ..] => super::t41::n44847(s, out),
				['р', s @ ..] => super::t41::n44848(s, out),
				['т', s @ ..] => super::t41::n44849(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t41::n19685(s, out),
		['у', s @ ..] => super::t41::n19686(s, out),
		['э', s @ ..] => super::t41::n19687(s, out),
		['я', s @ ..] => super::t41::n19688(s, out),
		_ => false,
	}
}

/// Words ending -рия.
pub(crate) fn n4854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4843);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19583);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73000);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 73001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 44425);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73002);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 73003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 44426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 44427);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73005);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98013);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 116271);
											s.is_empty()
										}
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
					super::push(out, 44428);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 44429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73006);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 73007);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 73008);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98015);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 116272);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 127838);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44430);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73009);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98016);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116273);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 127839);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136093(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 116274);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 127840);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t41::n136094(s, out),
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
									super::push(out, 98017);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116275);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 116276);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 73010);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98018);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116277);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 127841);
													match s {
														[] => true,
														['а', s @ ..] => super::t41::n136095(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73011);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 73012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44432);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 44433);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73013);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98019);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116278);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 127842);
													match s {
														[] => true,
														['н', s @ ..] => super::t41::n136096(s, out),
														_ => false,
													}
												}
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
							super::push(out, 73014);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98020);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116279);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 127843);
													match s {
														[] => true,
														['у', s @ ..] => super::t41::n136097(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 127844);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44434);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73015);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98021);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116280);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127845);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t41::n19690(s, out),
		['г', s @ ..] => super::t41::n19691(s, out),
		['д', s @ ..] => super::t41::n19692(s, out),
		['е', s @ ..] => super::t41::n19693(s, out),
		['и', s @ ..] => super::t41::n19694(s, out),
		['й', s @ ..] => super::t41::n19695(s, out),
		['о', s @ ..] => super::t41::n19696(s, out),
		['р', s @ ..] => {
			super::push(out, 19591);
			s.is_empty()
		}
		['т', s @ ..] => super::t41::n19698(s, out),
		['у', s @ ..] => super::t41::n19699(s, out),
		['х', s @ ..] => {
			super::push(out, 19594);
			s.is_empty()
		}
		['э', s @ ..] => super::t41::n19701(s, out),
		['ю', s @ ..] => super::t41::n19702(s, out),
		['я', s @ ..] => super::t41::n19703(s, out),
		_ => false,
	}
}

/// Words ending -сия.
pub(crate) fn n4855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4844);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19598);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44500);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73127);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44501);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73128);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98147);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116404);
											s.is_empty()
										}
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
					super::push(out, 44502);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73129);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98148);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 116405);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 127951);
													s.is_empty()
												}
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
							super::push(out, 73130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44503);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73131);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 98149);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116406);
											s.is_empty()
										}
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
					super::push(out, 44504);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73132);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98150);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 116407);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127952);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 98151);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 44505);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73133);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98152);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116408);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 127953);
													s.is_empty()
												}
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
							super::push(out, 73134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19599);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19600);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44507);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44508);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 44509);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 73137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44510);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73138);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98155);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116409);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19601);
			match s {
				[] => true,
				['а', s @ ..] => super::t41::n44942(s, out),
				['е', s @ ..] => super::t41::n44943(s, out),
				['и', s @ ..] => {
					super::push(out, 44513);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 73148);
							match s {
								[] => true,
								['с', s @ ..] => super::t41::n99239(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 44514);
					match s {
						[] => true,
						['а', s @ ..] => super::t41::n73922(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t41::n44946(s, out),
				['у', s @ ..] => super::t41::n44947(s, out),
				['ю', s @ ..] => super::t41::n44948(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t41::n19708(s, out),
		['о', s @ ..] => super::t41::n19709(s, out),
		['п', s @ ..] => super::t41::n19710(s, out),
		['р', s @ ..] => super::t41::n19711(s, out),
		['с', s @ ..] => super::t41::n19712(s, out),
		['т', s @ ..] => super::t41::n19713(s, out),
		['у', s @ ..] => super::t41::n19714(s, out),
		['ь', s @ ..] => super::t41::n19715(s, out),
		_ => false,
	}
}

/// Words ending -тия.
pub(crate) fn n4856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4845);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19610);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44535);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73176);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98196);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116442);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 127977);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 98197);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 116443);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127978);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136184(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 127979);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136185(s, out),
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
									super::push(out, 98198);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116444);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 127980);
													match s {
														[] => true,
														['о', s @ ..] => super::t41::n136186(s, out),
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
									super::push(out, 98199);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 116445);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 127981);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t41::n136187(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 116446);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 127982);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136188(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44536);
					match s {
						[] => true,
						['г', 'о', 'р', s @ ..] => {
							super::push(out, 73177);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98200);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116447);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t41::n44968(s, out),
				['р', s @ ..] => {
					super::push(out, 44538);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73182);
							match s {
								[] => true,
								['о', 'д', 'з', 'и', 'п', '\u{1}', s @ ..] => {
									super::push(out, 98217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 73183);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 98218);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 116469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t41::n19717(s, out),
		['и', s @ ..] => super::t41::n19718(s, out),
		['к', s @ ..] => super::t41::n19719(s, out),
		['н', s @ ..] => super::t41::n19720(s, out),
		['о', s @ ..] => super::t41::n19721(s, out),
		['р', s @ ..] => super::t41::n19722(s, out),
		['с', s @ ..] => super::t41::n19723(s, out),
		['т', s @ ..] => {
			super::push(out, 19618);
			s.is_empty()
		}
		['ф', s @ ..] => super::t41::n19725(s, out),
		_ => false,
	}
}

/// Words ending -фия.
pub(crate) fn n4857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4846);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19620);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 44563);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44564);
					match s {
						[] => true,
						['г', s @ ..] => super::t41::n73996(s, out),
						['о', s @ ..] => {
							super::push(out, 73223);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98254);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 116505);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128033);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136228(s, out),
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
									super::push(out, 98255);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 116506);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128034);
													match s {
														[] => true,
														['е', s @ ..] => super::t41::n136229(s, out),
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
									super::push(out, 98256);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 116507);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128035);
													match s {
														[] => true,
														['о', s @ ..] => super::t41::n136230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19621);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 44565);
					match s {
						[] => true,
						['п', s @ ..] => super::t41::n73998(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t41::n19728(s, out),
		['м', s @ ..] => super::t41::n19729(s, out),
		['о', s @ ..] => super::t41::n19730(s, out),
		['п', s @ ..] => super::t41::n19731(s, out),
		['р', s @ ..] => super::t41::n19732(s, out),
		['у', s @ ..] => super::t41::n19733(s, out),
		['ь', s @ ..] => super::t41::n19734(s, out),
		_ => false,
	}
}

/// Words ending -хия.
pub(crate) fn n4858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4847);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19629);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73237);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98267);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116514);
											s.is_empty()
										}
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
					super::push(out, 44579);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73238);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98268);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116515);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128039);
													s.is_empty()
												}
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
							super::push(out, 73239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44580);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73240);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98269);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 98270);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116516);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19630);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19631);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44581);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73242);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 98271);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 73243);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44582);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73244);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 98273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 44583);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 44584);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19632);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19633);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44585);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73247);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 73248);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98274);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116518);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128040);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44586);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ция.
pub(crate) fn n4859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4848);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 19635);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 44587);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73249);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98275);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116519);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128041);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19636);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44588);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73250);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98276);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116520);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 73251);
							match s {
								[] => true,
								['у', 'т', s @ ..] => {
									super::push(out, 98277);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116521);
											s.is_empty()
										}
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
					super::push(out, 44589);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 44590);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 44591);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73252);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 73253);
							match s {
								[] => true,
								['е', s @ ..] => super::t41::n99347(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t41::n45024(s, out),
				['и', s @ ..] => {
					super::push(out, 44593);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73255);
							match s {
								[] => true,
								['е', 'р', s @ ..] => super::t41::n99354(s, out),
								_ => false,
							}
						}
						['х', s @ ..] => super::t41::n74030(s, out),
						['ц', s @ ..] => super::t41::n74031(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t41::n45026(s, out),
				['м', s @ ..] => super::t41::n45027(s, out),
				['н', s @ ..] => super::t41::n45028(s, out),
				['р', s @ ..] => super::t41::n45029(s, out),
				['с', s @ ..] => super::t41::n45030(s, out),
				['т', s @ ..] => super::t41::n45031(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t41::n19743(s, out),
		['и', s @ ..] => super::t41::n19744(s, out),
		['к', s @ ..] => super::t41::n19745(s, out),
		['н', s @ ..] => super::t41::n19746(s, out),
		['п', s @ ..] => super::t41::n19747(s, out),
		['р', s @ ..] => super::t41::n19748(s, out),
		['ю', s @ ..] => super::t41::n19749(s, out),
		['я', s @ ..] => super::t41::n19750(s, out),
		_ => false,
	}
}

/// Words ending -чия.
pub(crate) fn n4860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4849);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19645);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44614);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73292);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98319);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116562);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 128078);
													match s {
														[] => true,
														['о', s @ ..] => super::t41::n136259(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19646);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44615);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73293);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98320);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116563);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19647);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44616);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19648);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44617);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73295);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98321);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 116564);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128079);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136260(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19649);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44618);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73296);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98322);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116565);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шия.
pub(crate) fn n4861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4850);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 19650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44619);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пустомеля.
pub(crate) fn n129432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128082);
	s.is_empty()
}

/// Words ending -ол-апреля.
pub(crate) fn n129433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128083);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теля.
pub(crate) fn n19796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19690);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44662);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 73330);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98341);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 116575);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128084);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 134858);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n140125(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 44663);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73331);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иля.
pub(crate) fn n4871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4860);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19692);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 19693);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 19694);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73332);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 98343);
									s.is_empty()
								}
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
			super::push(out, 19695);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 19696);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 19697);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44666);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73333);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98344);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116577);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128085);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кля.
pub(crate) fn n4872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19698);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19699);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 44667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19700);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44668);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98345);
									s.is_empty()
								}
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
			super::push(out, 19701);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44669);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 44670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лля.
pub(crate) fn n4873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4862);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19702);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 44671);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73335);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116578);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мля.
pub(crate) fn n4874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4863);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19703);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 44672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 19704);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44673);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оля.
pub(crate) fn n4875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4864);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19705);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 19706);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44674);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19707);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44675);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 44676);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 44677);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73339);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98347);
									match s {
										[] => true,
										['м', 'е', 'с', '\u{1}', s @ ..] => {
											super::push(out, 116579);
											s.is_empty()
										}
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
					super::push(out, 44678);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73340);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98348);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116580);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44679);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 44680);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98349);
									s.is_empty()
								}
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
			super::push(out, 19709);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44681);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 44682);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73342);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 98350);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 116581);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 128086);
													match s {
														[] => true,
														['-', s @ ..] => super::t41::n136264(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19710);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 19711);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44683);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19713);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44685);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73344);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98351);
									s.is_empty()
								}
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
			super::push(out, 19714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пля.
pub(crate) fn n4876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4865);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19715);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44686);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 44687);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 44688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19716);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 44689);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73347);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 44690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19717);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44691);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арля.
pub(crate) fn n19824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19718);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 44692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сля.
pub(crate) fn n4878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4867);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 19719);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тля.
pub(crate) fn n4879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4868);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19720);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44694);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 73350);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98353);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116582);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128087);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19721);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уля.
pub(crate) fn n4880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4869);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44696);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 44697);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73351);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 73352);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98354);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116583);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19723);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44698);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 44699);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73354);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98355);
									s.is_empty()
								}
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
			super::push(out, 19724);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44700);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 44701);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73355);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98356);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 116584);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 73356);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98357);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 116585);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19725);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44702);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73357);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98358);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116586);
											s.is_empty()
										}
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
					super::push(out, 44703);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 44704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 44705);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 73360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 44706);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 73361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98359);
									s.is_empty()
								}
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
			super::push(out, 19726);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44707);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73362);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98360);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116587);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128088);
													match s {
														[] => true,
														['в', s @ ..] => super::t41::n136265(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19727);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44708);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73363);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98361);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116588);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 128089);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44709);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44710);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73365);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98362);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116589);
											s.is_empty()
										}
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
					super::push(out, 44711);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 73367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19729);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44712);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73368);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 73369);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 98364);
									s.is_empty()
								}
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
			super::push(out, 19730);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44713);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73370);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 44714);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 73371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 44715);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 73372);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98366);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 116590);
											s.is_empty()
										}
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
					super::push(out, 44716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 44717);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73374);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98367);
									s.is_empty()
								}
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
			super::push(out, 19731);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44718);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 44719);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73375);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 73376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 44720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t41::n19838(s, out),
		['с', s @ ..] => super::t41::n19839(s, out),
		['т', s @ ..] => super::t41::n19840(s, out),
		['ч', s @ ..] => {
			super::push(out, 19735);
			match s {
				[] => true,
				['о', s @ ..] => super::t41::n45162(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фля.
pub(crate) fn n4881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4870);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19736);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19737);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19738);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44731);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хля.
pub(crate) fn n4882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4871);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19739);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыля.
pub(crate) fn n4883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4872);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 19740);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44733);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эля.
pub(crate) fn n4884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4873);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19741);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юля.
pub(crate) fn n4885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4874);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19742);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 19743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44734);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 44735);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19744);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 44736);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73391);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98372);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116593);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19745);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44737);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19746);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44738);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73393);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98373);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116594);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19747);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73394);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98374);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116595);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19748);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 44740);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73395);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 98375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 44741);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73396);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 73397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яля.
pub(crate) fn n4886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4875);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 19749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тво-время.
pub(crate) fn n129443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128093);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 134861);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 138692);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алеовремя.
pub(crate) fn n129444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128094);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овня.
pub(crate) fn n19882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19776);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 44771);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73421);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 98387);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 73422);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 98388);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 116603);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128095);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 44772);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 73423);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 73424);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 98389);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 98390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 98391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 44773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73425);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 73426);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 98392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 73427);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 98393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 73428);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 116606);
									s.is_empty()
								}
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
			super::push(out, 44774);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73429);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 98395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44775);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 73430);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 98396);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 116607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 73431);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 98397);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 116608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фигня.
pub(crate) fn n45210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44776);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 73432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -угня.
pub(crate) fn n19884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19778);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 44777);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 44778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дня.
pub(crate) fn n4896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4885);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19779);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44779);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73433);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44780);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73434);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 98399);
									s.is_empty()
								}
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
			super::push(out, 19780);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44781);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44782);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19781);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44783);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19782);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 44784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44785);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19783);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44786);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98400);
									s.is_empty()
								}
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
			super::push(out, 19784);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44787);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73441);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98401);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116609);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 128096);
													match s {
														[] => true,
														['ч', s @ ..] => super::t41::n136268(s, out),
														_ => false,
													}
												}
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
							super::push(out, 73442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 44788);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73443);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116610);
											s.is_empty()
										}
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
					super::push(out, 44789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73444);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 73445);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 44790);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19785);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44791);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 44792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 44793);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73449);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98406);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116611);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 128097);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19786);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44794);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 73450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еня.
pub(crate) fn n4897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4886);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19787);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19788);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 19789);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 19790);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44796);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 73452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19791);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 44797);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 73453);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98408);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116612);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128098);
													match s {
														[] => true,
														['т', s @ ..] => super::t41::n136269(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19792);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 19793);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44799);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 73454);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98409);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 116613);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19794);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 19795);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44801);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44802);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73457);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жня.
pub(crate) fn n4898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4887);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19796);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44803);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73458);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98411);
									s.is_empty()
								}
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
			super::push(out, 19797);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19798);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73459);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44806);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19799);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зня.
pub(crate) fn n4899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4888);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19800);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44808);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 73460);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98413);
									s.is_empty()
								}
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
			super::push(out, 19801);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19802);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44810);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73461);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 98414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 44811);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19803);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44812);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73463);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98415);
									s.is_empty()
								}
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
			super::push(out, 19804);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44813);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иня.
pub(crate) fn n4900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4889);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 19805);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44814);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 44815);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 19806);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44816);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73466);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 98416);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116614);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128099);
													match s {
														[] => true,
														['и', s @ ..] => super::t41::n136270(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19807);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44817);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73467);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98417);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116615);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19808);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44818);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73468);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 116616);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 128100);
													match s {
														[] => true,
														['т', s @ ..] => super::t41::n136271(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 116617);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19809);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44819);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19810);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44820);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 73470);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98418);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116618);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 73471);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 73472);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98420);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116619);
											s.is_empty()
										}
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
					super::push(out, 44821);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73473);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98421);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116620);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128101);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йня.
pub(crate) fn n4901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4890);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19811);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19812);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44822);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							super::push(out, 73474);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98422);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116621);
											match s {
												[] => true,
												['с', '\u{1}', s @ ..] => {
													super::push(out, 128102);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44823);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 44824);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 44825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19813);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 44826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кня.
pub(crate) fn n4902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4891);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 19814);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44827);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 73476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мня.
pub(crate) fn n4903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4892);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19815);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 19816);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44828);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73477);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оня.
pub(crate) fn n4904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4893);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 19818);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44830);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19819);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44831);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 73479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19820);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 44832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19821);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 44833);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19822);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19823);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44835);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 73483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19824);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44836);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 19825);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 19826);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44838);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73484);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98424);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 98425);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 116622);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пня.
pub(crate) fn n4905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4894);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19827);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 44839);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73485);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98426);
									s.is_empty()
								}
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
			super::push(out, 19828);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44840);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 44841);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19829);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44842);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 19830);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44843);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73488);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рня.
pub(crate) fn n4906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4895);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19831);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19832);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44845);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73489);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98428);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 116623);
											s.is_empty()
										}
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
					super::push(out, 44846);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73490);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98429);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116624);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 128103);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44847);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73491);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98430);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 116625);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 73492);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 44848);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73493);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 98432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 44849);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 44850);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 73494);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 98433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 44851);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73495);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98434);
									s.is_empty()
								}
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
			super::push(out, 19833);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44852);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73496);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98435);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116626);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128104);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44853);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 44854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19834);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 19835);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 19836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сня.
pub(crate) fn n4907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4896);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19837);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44856);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73497);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 98436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44857);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73498);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 98437);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116627);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128105);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19838);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 44858);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 73499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44859);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73500);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98438);
									s.is_empty()
								}
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
			super::push(out, 19839);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44860);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98439);
									s.is_empty()
								}
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
			super::push(out, 19840);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44861);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73502);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44862);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 73503);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98441);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116628);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19841);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44863);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73504);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98442);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116629);
											match s {
												[] => true,
												['б', '\u{1}', s @ ..] => {
													super::push(out, 128106);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тня.
pub(crate) fn n4908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4897);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19842);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 44865);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73505);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98443);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116630);
											s.is_empty()
										}
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
					super::push(out, 44866);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73506);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98444);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116631);
											s.is_empty()
										}
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
					super::push(out, 44867);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73507);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 44868);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 73508);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98446);
									s.is_empty()
								}
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
			super::push(out, 19843);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44869);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 73509);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 98447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44870);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73510);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 44871);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 73511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19844);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44872);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44873);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73513);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98449);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116632);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19845);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 44874);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 44875);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73514);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 98450);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116633);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128107);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44876);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73515);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 73516);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 73517);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98451);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 116634);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128108);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136272(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'а', s @ ..] => {
							super::push(out, 73518);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98452);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116635);
											s.is_empty()
										}
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
					super::push(out, 44877);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 44878);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73519);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116636);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 73520);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 73521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44879);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73522);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98454);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 116637);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128109);
													match s {
														[] => true,
														['п', s @ ..] => super::t41::n136273(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44881);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73524);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 98455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 73525);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98456);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116638);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128110);
													match s {
														[] => true,
														['л', s @ ..] => super::t41::n136274(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 44882);
					match s {
						[] => true,
						['к', s @ ..] => super::t41::n74303(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t41::n19953(s, out),
		['ы', s @ ..] => {
			super::push(out, 19848);
			match s {
				[] => true,
				['м', s @ ..] => super::t41::n45318(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t41::n19955(s, out),
		['я', s @ ..] => super::t41::n19956(s, out),
		_ => false,
	}
}

/// Words ending -уня.
pub(crate) fn n4909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4898);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 19851);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44888);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73530);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98460);
									s.is_empty()
								}
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
			super::push(out, 19852);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 44889);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 19853);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19854);
			s.is_empty()
		}
		['с', 'и', 'к', '\u{1}', s @ ..] => {
			super::push(out, 19855);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 19856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44891);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 44892);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 19857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44893);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 73533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фня.
pub(crate) fn n4910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4899);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 19858);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хня.
pub(crate) fn n4911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4900);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19859);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19860);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44896);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 73535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19861);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 44897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 44898);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 73537);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98461);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116641);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 128111);
													match s {
														[] => true,
														['р', s @ ..] => super::t41::n136275(s, out),
														_ => false,
													}
												}
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
							super::push(out, 73538);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98462);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 116642);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128112);
													s.is_empty()
												}
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
							super::push(out, 73539);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98463);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116643);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 128113);
													match s {
														[] => true,
														['к', s @ ..] => super::t41::n136276(s, out),
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
									super::push(out, 98464);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 116644);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128114);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44899);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 44900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чня.
pub(crate) fn n4912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19862);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 44901);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73542);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 44902);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73543);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 98467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19863);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 44904);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73544);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98468);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116645);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 128115);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19864);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 44906);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 73546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 19865);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 44907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шня.
pub(crate) fn n4913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4902);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19866);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 44908);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73547);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 98469);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 73548);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98470);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116646);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128116);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44909);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 44910);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73550);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44911);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 44912);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19867);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44913);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 73552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73553);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 73554);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98472);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116647);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128117);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44915);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73555);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 98473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 73556);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98474);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116648);
											s.is_empty()
										}
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
					super::push(out, 44916);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 73557);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98475);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 116649);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128118);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19868);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 44917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 19869);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 44918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 44919);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19870);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 44920);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73560);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98476);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116650);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 128119);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 19871);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 44921);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 44922);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 73561);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73562);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98478);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 116651);
											s.is_empty()
										}
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
					super::push(out, 44924);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 73563);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 98479);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 116652);
											s.is_empty()
										}
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
					super::push(out, 44925);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73564);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 98480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 73565);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 98481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 44926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 19872);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44927);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73567);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 44928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73568);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 73569);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98484);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 116653);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19873);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 44929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t41::n45364(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

