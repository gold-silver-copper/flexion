use flexion::Path;

/// Words ending -очный.
pub(crate) fn n1338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3916);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3917);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9197);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9198);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16146);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 16147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9199);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16148);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22954);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27917);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 22955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16149);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22956);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22957);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 22958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 9200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3918);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9201);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16150);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 16151);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22959);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 27918);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 27919);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 30852);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 22960);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 27920);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16152);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 27921);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 27922);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 16153);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 27923);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 27924);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 27925);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 27926);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 27927);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 30853);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9202);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16154);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22961);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 27928);
											s.is_empty()
										}
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
					super::push(out, 9203);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16155);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 22962);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 16156);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22963);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 27930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', 'у', s @ ..] => {
							super::push(out, 16157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t8::n9653(s, out),
				['ы', s @ ..] => {
					super::push(out, 9205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3919);
			s.is_empty()
		}
		['д', s @ ..] => super::t8::n4065(s, out),
		['з', s @ ..] => {
			super::push(out, 3921);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					super::push(out, 9213);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9214);
					s.is_empty()
				}
				['о', 'в', 'о', s @ ..] => {
					super::push(out, 9215);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9216);
					s.is_empty()
				}
				['я', 'в', 'е', s @ ..] => {
					super::push(out, 9217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t8::n4067(s, out),
		['м', s @ ..] => super::t8::n4068(s, out),
		['н', s @ ..] => super::t8::n4069(s, out),
		['п', s @ ..] => {
			super::push(out, 3925);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9243);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9244);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16242);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t8::n17075(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t8::n9694(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t8::n4071(s, out),
		['с', s @ ..] => super::t8::n4072(s, out),
		['т', s @ ..] => super::t8::n4073(s, out),
		['ш', s @ ..] => super::t8::n4074(s, out),
		_ => false,
	}
}

/// Words ending -учный.
pub(crate) fn n1339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3930);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3931);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9279);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'й', 'е', 'ф', '\u{1}', s @ ..] => {
					super::push(out, 9280);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 9281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3932);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16331);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 16332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3933);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3934);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9283);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 9284);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23102);
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									super::push(out, 23103);
									s.is_empty()
								}
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
			super::push(out, 3935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9285);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9286);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 16333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16334);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3936);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 3937);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9288);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 9289);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9290);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9291);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9292);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9293);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 9294);
					match s {
						[] => true,
						['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 16337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3938);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9295);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9296);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9297);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16339);
							match s {
								[] => true,
								['а', 'г', 'о', 'л', s @ ..] => {
									super::push(out, 23104);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 27994);
											s.is_empty()
										}
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
					super::push(out, 9298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язычный.
pub(crate) fn n9749(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16340);
			match s {
				[] => true,
				['н', 'д', 'а', s @ ..] => {
					super::push(out, 23105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16341);
			s.is_empty()
		}
		['ъ', 'х', 'е', 'р', s @ ..] => {
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 23106);
					s.is_empty()
				}
				['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 23107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючный.
pub(crate) fn n1341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1309);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3943);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3944);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9300);
					s.is_empty()
				}
				['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 9301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3945);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 3946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ячный.
pub(crate) fn n1342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3947);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3948);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9303);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3949);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3950);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3951);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3952);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3953);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3954);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9309);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['е', 'н', s @ ..] => {
									super::push(out, 23108);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 23109);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 23110);
									match s {
										[] => true,
										['е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 30875);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 30876);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9310);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9311);
					match s {
						[] => true,
						['т', 'а', s @ ..] => {
							super::push(out, 16344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3955);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 9313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3957);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -душный.
pub(crate) fn n4138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3993);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9334);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 9335);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 23117);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 23118);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 27995);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 30877);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пазушный.
pub(crate) fn n9791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9337);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лушный.
pub(crate) fn n4140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3995);
	s.is_empty()
}

/// Words ending -мушный.
pub(crate) fn n4141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3996);
	match s {
		[] => true,
		['е', 'р', s @ ..] => {
			super::push(out, 9338);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16360);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нушный.
pub(crate) fn n4142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3997);
	match s {
		[] => true,
		['р', 'е', s @ ..] => {
			super::push(out, 9339);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 16361);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушный.
pub(crate) fn n4143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3998);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9340);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 16362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышный.
pub(crate) fn n1351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1319);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3999);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4000);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16363);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16364);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23119);
									s.is_empty()
								}
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
			super::push(out, 4001);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4002);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9344);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23120);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 23121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эшный.
pub(crate) fn n1352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1320);
	s.is_empty()
}

/// Words ending -юшный.
pub(crate) fn n1353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	s.is_empty()
}

/// Words ending -яшный.
pub(crate) fn n1354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1322);
	s.is_empty()
}

/// Words ending -льный.
pub(crate) fn n1360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1328);
	match s {
		[] => true,
		['а', s @ ..] => super::t8::n4157(s, out),
		['б', s @ ..] => {
			super::push(out, 4013);
			s.is_empty()
		}
		['е', s @ ..] => super::t8::n4159(s, out),
		['и', s @ ..] => {
			super::push(out, 4015);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9393);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16503);
							s.is_empty()
						}
						['о', 'м', s @ ..] => {
							super::push(out, 16504);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 16505);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9394);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23303);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 31004);
													match s {
														[] => true,
														['л', s @ ..] => super::t8::n34057(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 31005);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 23304);
									match s {
										[] => true,
										['п', 'е', s @ ..] => {
											super::push(out, 28173);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 16506);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 16507);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23306);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 28174);
											s.is_empty()
										}
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
					super::push(out, 9395);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 16508);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16509);
							match s {
								[] => true,
								['м', 'а', 'з', s @ ..] => {
									super::push(out, 23308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9396);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16510);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23309);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 23310);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 28175);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'х', s @ ..] => {
							super::push(out, 16511);
							s.is_empty()
						}
						['я', 'р', 'п', 'о', 'к', s @ ..] => {
							super::push(out, 16512);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23311);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28176);
											match s {
												[] => true,
												['ш', '\u{1}', s @ ..] => {
													super::push(out, 31006);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9397);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 9398);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16513);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9399);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16514);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16515);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23313);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9400);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16516);
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 23314);
									s.is_empty()
								}
								['с', s @ ..] => super::t8::n24506(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t8::n17365(s, out),
						['у', s @ ..] => {
							super::push(out, 16518);
							s.is_empty()
						}
						['х', s @ ..] => super::t8::n17367(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t8::n9855(s, out),
				['о', s @ ..] => super::t8::n9856(s, out),
				['п', s @ ..] => super::t8::n9857(s, out),
				['р', s @ ..] => super::t8::n9858(s, out),
				['с', s @ ..] => super::t8::n9859(s, out),
				['т', s @ ..] => super::t8::n9860(s, out),
				['ф', s @ ..] => super::t8::n9861(s, out),
				['ц', s @ ..] => super::t8::n9862(s, out),
				['ч', s @ ..] => super::t8::n9863(s, out),
				['ш', s @ ..] => super::t8::n9864(s, out),
				['щ', s @ ..] => super::t8::n9865(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t8::n4161(s, out),
		['л', s @ ..] => super::t8::n4162(s, out),
		['о', s @ ..] => super::t8::n4163(s, out),
		['у', s @ ..] => super::t8::n4164(s, out),
		['ы', s @ ..] => super::t8::n4165(s, out),
		['э', s @ ..] => super::t8::n4166(s, out),
		['ю', s @ ..] => super::t8::n4167(s, out),
		['я', s @ ..] => super::t8::n4168(s, out),
		_ => false,
	}
}

/// Words ending -а-аминомасляный.
pub(crate) fn n32555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31021);
	match s {
		[] => true,
		['м', 'м', 'а', 'г', '\u{1}', s @ ..] => {
			super::push(out, 32447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -воватый.
pub(crate) fn n9994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9534);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 16648);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23442);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -доватый.
pub(crate) fn n9996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9536);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 16649);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16650);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коватый.
pub(crate) fn n9998(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16651);
			match s {
				[] => true,
				['д', 'у', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28249);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 28250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 23445);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16652);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23446);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16653);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 16654);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23447);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16655);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28254);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31025);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 31026);
									s.is_empty()
								}
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
			super::push(out, 16656);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23448);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16657);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23449);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 16658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ловатый.
pub(crate) fn n9999(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16659);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16660);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 16661);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23451);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'б', s @ ..] => {
			super::push(out, 16662);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23453);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16663);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новатый.
pub(crate) fn n10000(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16665);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23454);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28259);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28260);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28261);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 28262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'л', 'е', 'з', '-', 'о', 'т', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
			super::push(out, 16666);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 16667);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23455);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23456);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16668);
			s.is_empty()
		}
		['т', 'с', s @ ..] => {
			super::push(out, 16669);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23457);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16670);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23458);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 16671);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23460);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 28265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риповатый.
pub(crate) fn n10001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9538);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 16672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роватый.
pub(crate) fn n10002(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16673);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23461);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 28266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16674);
			match s {
				[] => true,
				['и', 'л', s @ ..] => {
					super::push(out, 23462);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'б', '-', 'о', 'т', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
			super::push(out, 16675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соватый.
pub(crate) fn n10003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9539);
	s.is_empty()
}

/// Words ending -товатый.
pub(crate) fn n10004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9540);
	s.is_empty()
}

/// Words ending -ховатый.
pub(crate) fn n10005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9541);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 23463);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'л', s @ ..] => {
			super::push(out, 16676);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 16677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гатый.
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1387);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4104);
			match s {
				[] => true,
				['б', 'р', s @ ..] => {
					super::push(out, 9542);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16678);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -датый.
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1388);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4105);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4106);
			s.is_empty()
		}
		['з', 'е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9543);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4107);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9546);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жатый.
pub(crate) fn n1425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4108);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4109);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4110);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4111);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 4112);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4113);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4114);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9548);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16679);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23466);
									s.is_empty()
								}
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
			super::push(out, 4115);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9549);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4116);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 4117);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9551);
					match s {
						[] => true,
						['е', 'ж', s @ ..] => {
							super::push(out, 16680);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -затый.
pub(crate) fn n1426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1390);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 4118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -катый.
pub(crate) fn n1427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	s.is_empty()
}

/// Words ending -латый.
pub(crate) fn n1428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4119);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4120);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4121);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 4122);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4123);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9554);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матый.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	s.is_empty()
}

/// Words ending -натый.
pub(crate) fn n1430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 9555);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 9556);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -патый.
pub(crate) fn n1431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4124);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4125);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ратый.
pub(crate) fn n1432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4126);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 4127);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4128);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9559);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сатый.
pub(crate) fn n1433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1397);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4129);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4130);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4131);
			match s {
				[] => true,
				['л', 'о', 'п', '-', 'о', 'н', 'д', 'з', 'е', 'в', 'з', '\u{1}', s @ ..] => {
					super::push(out, 9563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4132);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 9564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -татый.
pub(crate) fn n1434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1398);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 4134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хатый.
pub(crate) fn n1435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1399);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4135);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4136);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4137);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цатый.
pub(crate) fn n1436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1400);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4138);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16682);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 16683);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23469);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 23470);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23471);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 28268);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 23472);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 28269);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 23473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9566);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16684);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 23474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чатый.
pub(crate) fn n1437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1401);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4139);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9567);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 9568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4140);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9569);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 9570);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23475);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23476);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 23477);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28270);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 31028);
													match s {
														[] => true,
														['т', '\u{1}', s @ ..] => super::t8::n34063(s, out),
														_ => false,
													}
												}
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
							super::push(out, 16685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4141);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9571);
					s.is_empty()
				}
				['ы', 'р', s @ ..] => {
					super::push(out, 9572);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4142);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9573);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9574);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							super::push(out, 16688);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 23478);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 28271);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4143);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9575);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4144);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9576);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4145);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9577);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16692);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 16693);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9578);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 16694);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 16695);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 23480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 16696);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 23481);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23483);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9580);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9581);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9582);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23485);
									s.is_empty()
								}
								['х', 'а', 'б', 'о', s @ ..] => {
									super::push(out, 23486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9583);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9584);
					match s {
						[] => true,
						['д', '-', 'о', 'н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
							super::push(out, 16698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4147);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16699);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['о', 'к', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 23487);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'у', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 23489);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23490);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23491);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28272);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 23492);
									match s {
										[] => true,
										['е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 31029);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 31030);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['л', s @ ..] => {
							super::push(out, 16700);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 16701);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 23494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => super::t8::n24693(s, out),
								['у', s @ ..] => super::t8::n24694(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t8::n10052(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t8::n4305(s, out),
		['п', s @ ..] => super::t8::n4306(s, out),
		['р', s @ ..] => super::t8::n4307(s, out),
		['с', s @ ..] => super::t8::n4308(s, out),
		['т', s @ ..] => super::t8::n4309(s, out),
		['у', s @ ..] => super::t8::n4310(s, out),
		['ш', s @ ..] => super::t8::n4311(s, out),
		['ь', s @ ..] => super::t8::n4312(s, out),
		_ => false,
	}
}

/// Words ending -щатый.
pub(crate) fn n1438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1402);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4156);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 4157);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9598);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лено-желтый.
pub(crate) fn n32566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31032);
	s.is_empty()
}

/// Words ending -емно-желтый.
pub(crate) fn n32567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31033);
	s.is_empty()
}

/// Words ending -ачно-желтый.
pub(crate) fn n32568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31034);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 32449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ично-желтый.
pub(crate) fn n32569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31035);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 32450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -истый.
pub(crate) fn n1462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1426);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4248);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4249);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9713);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4250);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9715);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9716);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9717);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16781);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 23523);
									s.is_empty()
								}
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
			super::push(out, 4251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9718);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16782);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9719);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9720);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16783);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23525);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28286);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 23526);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28287);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 23527);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 28288);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 23528);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 28289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 16784);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 23529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16785);
							match s {
								[] => true,
								['о', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28290);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 28291);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 16786);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 23530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 9722);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16788);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23531);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 28292);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4252);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9723);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16789);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 16790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9724);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9725);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16791);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9726);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 9727);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16792);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9728);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 16793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9729);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 9730);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4253);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9731);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9732);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4254);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9734);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9735);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16798);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9736);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 16799);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 23537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9737);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 16800);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 16801);
							s.is_empty()
						}
						['р', 'о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23538);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 23539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9738);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 16802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 9739);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 23540);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t8::n4417(s, out),
		['л', s @ ..] => super::t8::n4418(s, out),
		['м', s @ ..] => super::t8::n4419(s, out),
		['н', s @ ..] => super::t8::n4420(s, out),
		['о', s @ ..] => super::t8::n4421(s, out),
		['п', s @ ..] => super::t8::n4422(s, out),
		['р', s @ ..] => super::t8::n4423(s, out),
		['с', s @ ..] => super::t8::n4424(s, out),
		['т', s @ ..] => super::t8::n4425(s, out),
		['у', s @ ..] => {
			super::push(out, 4264);
			s.is_empty()
		}
		['ч', s @ ..] => super::t8::n4427(s, out),
		['ш', s @ ..] => super::t8::n4428(s, out),
		['ь', s @ ..] => {
			super::push(out, 4267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ностый.
pub(crate) fn n4432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4270);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 9814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ростый.
pub(crate) fn n4433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4271);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9815);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рстый.
pub(crate) fn n1465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1429);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4272);
			match s {
				[] => true,
				['ш', 'о', 'к', 'д', s @ ..] => {
					super::push(out, 9816);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 16922);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устый.
pub(crate) fn n1466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1430);
	s.is_empty()
}

/// Words ending -ястый.
pub(crate) fn n1467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1431);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4273);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гнутый.
pub(crate) fn n4449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4287);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9830);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9831);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16933);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 23624);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28331);
									s.is_empty()
								}
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
			super::push(out, 9832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инутый.
pub(crate) fn n4450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4288);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9833);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23625);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28332);
									match s {
										[] => true,
										['п', 'е', s @ ..] => {
											super::push(out, 31047);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 32454);
													s.is_empty()
												}
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
							super::push(out, 23626);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23627);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28333);
									s.is_empty()
								}
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
			super::push(out, 9834);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16934);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 16935);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кнутый.
pub(crate) fn n4451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4289);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9835);
			match s {
				[] => true,
				['х', 'о', s @ ..] => {
					super::push(out, 16936);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 23629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9836);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16937);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 23630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9837);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23631);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23632);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28334);
									s.is_empty()
								}
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
			super::push(out, 9838);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16938);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 23633);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 23634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9839);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 16939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9840);
			match s {
				[] => true,
				['е', 'ч', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23635);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23636);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28336);
									s.is_empty()
								}
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
			super::push(out, 9841);
			match s {
				[] => true,
				['о', 'р', s @ ..] => {
					super::push(out, 16940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онутый.
pub(crate) fn n4452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4290);
	match s {
		[] => true,
		['р', 'т', 'а', s @ ..] => {
			super::push(out, 9843);
			match s {
				[] => true,
				['з', 'е', s @ ..] => {
					super::push(out, 16941);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 23637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнутый.
pub(crate) fn n4454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4292);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9845);
			match s {
				[] => true,
				['в', 'ы', s @ ..] => {
					super::push(out, 16942);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'ы', s @ ..] => {
					super::push(out, 16943);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9846);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 23640);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 28337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 23641);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28338);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28339);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снутый.
pub(crate) fn n4455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4293);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9847);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16944);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28340);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28341);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31049);
											s.is_empty()
										}
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
					super::push(out, 16945);
					match s {
						[] => true,
						['т', 'д', s @ ..] => {
							super::push(out, 23642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9848);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 9849);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 9850);
			match s {
				[] => true,
				['л', 'п', 'и', s @ ..] => {
					super::push(out, 16946);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 23643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнутый.
pub(crate) fn n4456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4294);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9851);
			match s {
				[] => true,
				['м', '\u{1}', s @ ..] => {
					super::push(out, 16947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унутый.
pub(crate) fn n4457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4295);
	match s {
		[] => true,
		['с', 'ы', s @ ..] => {
			super::push(out, 9852);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнутый.
pub(crate) fn n4458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4296);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9853);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9854);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16949);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 23644);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 23645);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9855);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16951);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 23646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9856);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16953);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28343);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28344);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31050);
											s.is_empty()
										}
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
					super::push(out, 16954);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнутый.
pub(crate) fn n4459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4297);
	match s {
		[] => true,
		['л', 'п', 'ы', s @ ..] => {
			super::push(out, 9857);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янутый.
pub(crate) fn n4460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4298);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 9858);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 23648);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 28345);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'е', s @ ..] => {
							super::push(out, 23649);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 28346);
									s.is_empty()
								}
								['ш', 'ы', 'в', '\u{1}', s @ ..] => {
									super::push(out, 28347);
									s.is_empty()
								}
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
			super::push(out, 9859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16956);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16957);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 16958);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16959);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 23650);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28348);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 23651);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16960);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 16961);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шийся.
pub(crate) fn n1509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1472);
	match s {
		[] => true,
		['в', s @ ..] => super::t8::n4523(s, out),
		['г', s @ ..] => {
			super::push(out, 4360);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4361);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 17048);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23919);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 17049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4362);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4363);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4364);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4365);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23920);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 28735);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 23921);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 28736);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4366);
			match s {
				[] => true,
				['е', 'н', 'ы', s @ ..] => {
					super::push(out, 9923);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4367);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 4368);
			match s {
				[] => true,
				['о', 'д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17052);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17053);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щийся.
pub(crate) fn n1510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1473);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4369);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 9925);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17054);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 23923);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 23924);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 23925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17055);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23927);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 23928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 17056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9926);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17057);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17058);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23929);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17059);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 17060);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17061);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23930);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 23931);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17062);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17063);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 23933);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 23934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 17064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17065);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17066);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17067);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17068);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23935);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 23936);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 23937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 9927);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4370);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9928);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9929);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 9930);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 17072);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 9931);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17073);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17074);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 17075);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 23939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 9932);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 9933);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17076);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17077);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9934);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 9935);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 9936);
					match s {
						[] => true,
						['г', 'р', s @ ..] => {
							super::push(out, 17078);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 23941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9937);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 9938);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17079);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17080);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 23942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 17081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9939);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17082);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 17083);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 23943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 9940);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 9941);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 9942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t8::n4535(s, out),
		['я', s @ ..] => super::t8::n4536(s, out),
		_ => false,
	}
}

/// Words ending -илевский.
pub(crate) fn n10536(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17231);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 17232);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 17233);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 17234);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 17235);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллевский.
pub(crate) fn n10538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10053);
	s.is_empty()
}

/// Words ending -млевский.
pub(crate) fn n10539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10054);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олевский.
pub(crate) fn n10540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10055);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 17238);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17239);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улевский.
pub(crate) fn n10541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10056);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17240);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17241);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -невский.
pub(crate) fn n4662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4486);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10057);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17242);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 17243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10058);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оевский.
pub(crate) fn n4663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4487);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 10060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ревский.
pub(crate) fn n4664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4488);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10061);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 10062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уевский.
pub(crate) fn n4665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4489);
	s.is_empty()
}

/// Words ending -цевский.
pub(crate) fn n4666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4490);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 10063);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 10064);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17245);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10065);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чевский.
pub(crate) fn n4667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4491);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10066);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17247);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10067);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10068);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шевский.
pub(crate) fn n4668(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10069);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10070);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 10071);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 10072);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17251);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 24151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17252);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щевский.
pub(crate) fn n4669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4492);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10073);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьевский.
pub(crate) fn n4670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4493);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10075);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17254);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10076);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17255);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эевский.
pub(crate) fn n4671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4494);
	s.is_empty()
}

/// Words ending -ивский.
pub(crate) fn n1642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1595);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овский.
pub(crate) fn n1643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1596);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4496);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10077);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10078);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10079);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 10080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4497);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 10082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4498);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10083);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10084);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10085);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17257);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17258);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17259);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 17260);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10086);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							super::push(out, 17261);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 24156);
									s.is_empty()
								}
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
			super::push(out, 4499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10087);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10088);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17262);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24157);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28891);
											match s {
												[] => true,
												['п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t8::n34270(s, out),
														['а', s @ ..] => super::t8::n34271(s, out),
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
									super::push(out, 24158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10089);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17263);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17264);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24159);
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
						['а', s @ ..] => {
							super::push(out, 17265);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 17266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 10090);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10091);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17267);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 24162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17268);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 24163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10092);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17269);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 24164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17270);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 24165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17271);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24166);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24167);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28892);
											s.is_empty()
										}
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
					super::push(out, 10093);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10094);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							super::push(out, 17272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4500);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4501);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17273);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17274);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 17275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10098);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10099);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17277);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 17278);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 24170);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 24171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17279);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 17280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10100);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17281);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 17282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10101);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 10102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4502);
			s.is_empty()
		}
		['к', s @ ..] => super::t8::n4680(s, out),
		['л', s @ ..] => super::t8::n4681(s, out),
		['м', s @ ..] => {
			super::push(out, 4505);
			match s {
				[] => true,
				['о', s @ ..] => super::t8::n10619(s, out),
				['р', s @ ..] => super::t8::n10620(s, out),
				['я', s @ ..] => super::t8::n10621(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t8::n4683(s, out),
		['п', s @ ..] => super::t8::n4684(s, out),
		['р', s @ ..] => super::t8::n4685(s, out),
		['с', s @ ..] => super::t8::n4686(s, out),
		['т', s @ ..] => super::t8::n4687(s, out),
		['ф', s @ ..] => super::t8::n4688(s, out),
		['х', s @ ..] => super::t8::n4689(s, out),
		['ц', s @ ..] => super::t8::n4690(s, out),
		['ш', s @ ..] => super::t8::n4691(s, out),
		_ => false,
	}
}

/// Words ending -рвский.
pub(crate) fn n1644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1597);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4515);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ический.
pub(crate) fn n4720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4542);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10200);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 10201);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 10202);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17441);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 24284);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 28918);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 31448);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 32655);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28919);
									s.is_empty()
								}
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
			super::push(out, 10203);
			match s {
				[] => true,
				['о', 'и', s @ ..] => {
					super::push(out, 17442);
					match s {
						[] => true,
						['р', 'е', 'п', 'о', 'к', s @ ..] => {
							super::push(out, 24285);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28920);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31449);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 28921);
									s.is_empty()
								}
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
			super::push(out, 10204);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10205);
			match s {
				[] => true,
				['о', 'в', 'м', 'и', 'с', '-', 'о', 'н', 'н', 'е', 'щ', 'б', 'о', 'б', 'о', '\u{1}', s @ ..] => {
					super::push(out, 17443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10206);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 10207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17444);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17445);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 24286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17446);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24287);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28922);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31450);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 32656);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 31451);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28924);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 31452);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 28925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17447);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 24289);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 28926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 24290);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24291);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28927);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 31453);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 28928);
									match s {
										[] => true,
										['о', 'г', s @ ..] => {
											super::push(out, 31454);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 32657);
													s.is_empty()
												}
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
							super::push(out, 24292);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 24293);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 24294);
							match s {
								[] => true,
								['у', 'м', s @ ..] => {
									super::push(out, 28929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 17448);
					s.is_empty()
				}
				['з', s @ ..] => super::t8::n18367(s, out),
				['и', s @ ..] => super::t8::n18368(s, out),
				['й', s @ ..] => super::t8::n18369(s, out),
				['к', s @ ..] => super::t8::n18370(s, out),
				['м', s @ ..] => super::t8::n18371(s, out),
				['н', s @ ..] => super::t8::n18372(s, out),
				['о', s @ ..] => super::t8::n18373(s, out),
				['п', s @ ..] => super::t8::n18374(s, out),
				['р', s @ ..] => super::t8::n18375(s, out),
				['с', s @ ..] => super::t8::n18376(s, out),
				['т', s @ ..] => super::t8::n18377(s, out),
				['у', s @ ..] => super::t8::n18378(s, out),
				['х', s @ ..] => super::t8::n18379(s, out),
				['ч', s @ ..] => super::t8::n18380(s, out),
				['ш', s @ ..] => super::t8::n18381(s, out),
				['щ', s @ ..] => super::t8::n18382(s, out),
				['ь', s @ ..] => super::t8::n18383(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t8::n10701(s, out),
		['р', s @ ..] => super::t8::n10702(s, out),
		['с', s @ ..] => super::t8::n10703(s, out),
		['т', s @ ..] => super::t8::n10704(s, out),
		['ф', s @ ..] => super::t8::n10705(s, out),
		['х', s @ ..] => super::t8::n10706(s, out),
		['щ', s @ ..] => super::t8::n10707(s, out),
		_ => false,
	}
}

/// Words ending -ошеский.
pub(crate) fn n4726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4548);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 10216);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 17482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щеский.
pub(crate) fn n1664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1617);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4549);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10218);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -истанский.
pub(crate) fn n18436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17518);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 24341);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 24342);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аханский.
pub(crate) fn n10765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10267);
	match s {
		[] => true,
		['р', 'т', 'с', 'а', '\u{1}', s @ ..] => {
			super::push(out, 17519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шанский.
pub(crate) fn n4778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4600);
	s.is_empty()
}

/// Words ending -енский.
pub(crate) fn n1693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1643);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4601);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4602);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10268);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10269);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17521);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17522);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 17523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10270);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 10271);
					s.is_empty()
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
						['г', s @ ..] => {
							super::push(out, 17525);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 17526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10272);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10274);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10275);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17527);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10276);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17528);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24345);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4604);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 4605);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10277);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4606);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 10278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4607);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 4608);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4609);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10280);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 10281);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10282);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10283);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10284);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 10285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4611);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10286);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17532);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 17533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10287);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 17534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10288);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10289);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10290);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10291);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 10292);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 10293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4612);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10294);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10295);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10296);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10297);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'о', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10298);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 10299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10300);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10301);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4613);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10303);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10304);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10305);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10306);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 17541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4614);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 4615);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 4616);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 4617);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17542);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 17543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10307);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 4618);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 4619);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10308);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10309);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10310);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инский.
pub(crate) fn n1694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1644);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4621);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17547);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 17548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4622);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 10311);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10312);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10313);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10314);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17549);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 24347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 17550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10315);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10316);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17551);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17552);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 24348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 10317);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 10318);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 10319);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10320);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 10321);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 10322);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10323);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10324);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17554);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 17555);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10325);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10326);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10327);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 10328);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17559);
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
					super::push(out, 10329);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10330);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 10331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 10332);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10333);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10334);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4623);
			match s {
				[] => true,
				['а', 'д', s @ ..] => {
					super::push(out, 10336);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 10337);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'ы', s @ ..] => {
					super::push(out, 10338);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					super::push(out, 10339);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10340);
					s.is_empty()
				}
				['у', 'е', s @ ..] => {
					super::push(out, 10341);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4624);
			s.is_empty()
		}
		['ж', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10342);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10343);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10344);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4625);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17564);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10346);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10347);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10348);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 10349);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4626);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4627);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10351);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10352);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 10353);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 10354);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17570);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17571);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 24349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10355);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10356);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 10357);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24350);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 28973);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 24351);
									s.is_empty()
								}
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
			super::push(out, 4628);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10358);
					match s {
						[] => true,
						['в', s @ ..] => super::t8::n18491(s, out),
						['к', s @ ..] => super::t8::n18492(s, out),
						['т', s @ ..] => super::t8::n18493(s, out),
						['х', s @ ..] => super::t8::n18494(s, out),
						_ => false,
					}
				}
				['б', 'у', s @ ..] => super::t8::n10865(s, out),
				['г', s @ ..] => super::t8::n10866(s, out),
				['з', s @ ..] => super::t8::n10867(s, out),
				['й', s @ ..] => super::t8::n10868(s, out),
				['к', 'у', s @ ..] => super::t8::n10869(s, out),
				['л', s @ ..] => super::t8::n10870(s, out),
				['о', s @ ..] => super::t8::n10871(s, out),
				['р', s @ ..] => super::t8::n10872(s, out),
				['с', s @ ..] => super::t8::n10873(s, out),
				['у', s @ ..] => super::t8::n10874(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t8::n4814(s, out),
		['н', s @ ..] => super::t8::n4815(s, out),
		['о', s @ ..] => super::t8::n4816(s, out),
		['п', s @ ..] => super::t8::n4817(s, out),
		['р', s @ ..] => super::t8::n4818(s, out),
		['с', s @ ..] => super::t8::n4819(s, out),
		['т', s @ ..] => super::t8::n4820(s, out),
		['у', s @ ..] => super::t8::n4821(s, out),
		['ф', s @ ..] => super::t8::n4822(s, out),
		['х', s @ ..] => super::t8::n4823(s, out),
		['ц', s @ ..] => super::t8::n4824(s, out),
		['ч', s @ ..] => super::t8::n4825(s, out),
		['ш', s @ ..] => super::t8::n4826(s, out),
		['щ', s @ ..] => super::t8::n4827(s, out),
		['ь', s @ ..] => super::t8::n4828(s, out),
		_ => false,
	}
}

/// Words ending -йнский.
pub(crate) fn n1695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1645);
	s.is_empty()
}

/// Words ending -ннский.
pub(crate) fn n1696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1646);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4641);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4642);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10416);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онский.
pub(crate) fn n1697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1647);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4643);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10417);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10418);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17617);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17618);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 24369);
									s.is_empty()
								}
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
			super::push(out, 4644);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4645);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4646);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10419);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10420);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10421);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 10422);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 17621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4647);
			s.is_empty()
		}
		['м', 'е', s @ ..] => {
			super::push(out, 4648);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4649);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4650);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10424);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'к', 'и', s @ ..] => {
			super::push(out, 4651);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4652);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10426);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 10427);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10428);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17625);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 17626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 4653);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 4654);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнский.
pub(crate) fn n1698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1648);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4655);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4656);
			match s {
				[] => true,
				['б', 'ь', s @ ..] => {
					super::push(out, 10429);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17627);
							match s {
								[] => true,
								['е', 'м', '\u{1}', s @ ..] => {
									super::push(out, 24371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унский.
pub(crate) fn n1699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1649);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4657);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 4658);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4659);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4660);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4661);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4662);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10432);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 10433);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17628);
							match s {
								[] => true,
								['л', 'р', 'а', 'к', '\u{1}', s @ ..] => {
									super::push(out, 24372);
									s.is_empty()
								}
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
			super::push(out, 4663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынский.
pub(crate) fn n1700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1650);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 4664);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 4665);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10434);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -энский.
pub(crate) fn n1701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1651);
	s.is_empty()
}

/// Words ending -янский.
pub(crate) fn n1702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1652);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4666);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4667);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10435);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4668);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10436);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 17631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4669);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10437);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерский.
pub(crate) fn n1716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4692);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 4693);
			s.is_empty()
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 10468);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10469);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17654);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17656);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 17657);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4694);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10470);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10471);
					s.is_empty()
				}
				['и', 'л', s @ ..] => {
					super::push(out, 10472);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10473);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17660);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10474);
					s.is_empty()
				}
				['р', 'е', s @ ..] => {
					super::push(out, 10475);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 17662);
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
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17663);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 17664);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 17665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10476);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10477);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10478);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10479);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10480);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4695);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10482);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10483);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17666);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 17667);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10484);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10485);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 10486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4696);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10487);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10489);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10490);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', 'ь', 'б', 'а', '\u{1}', s @ ..] => {
					super::push(out, 10491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4697);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4698);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10492);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10493);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17670);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 17671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 10494);
					s.is_empty()
				}
				['н', 'а', s @ ..] => {
					super::push(out, 10495);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17672);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 24380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10496);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10497);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4699);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10498);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10499);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 10500);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10501);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17676);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 17677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10502);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10503);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 10504);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 17679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 10505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4700);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10506);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10507);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10508);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10509);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17680);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17681);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 17682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 10510);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17683);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4701);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					super::push(out, 10511);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					super::push(out, 10512);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17685);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'д', s @ ..] => {
					super::push(out, 10513);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10514);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10515);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17688);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17689);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24381);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 28977);
											s.is_empty()
										}
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
					super::push(out, 10516);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10517);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t8::n4898(s, out),
		['р', s @ ..] => super::t8::n4899(s, out),
		['с', s @ ..] => super::t8::n4900(s, out),
		['т', s @ ..] => super::t8::n4901(s, out),
		['ф', s @ ..] => super::t8::n4902(s, out),
		['х', s @ ..] => super::t8::n4903(s, out),
		['ц', s @ ..] => super::t8::n4904(s, out),
		['ч', s @ ..] => super::t8::n4905(s, out),
		['ш', s @ ..] => super::t8::n4906(s, out),
		['ь', s @ ..] => {
			super::push(out, 4708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омирский.
pub(crate) fn n11076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10556);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 17720);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 17721);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ширский.
pub(crate) fn n4913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4714);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 10558);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17722);
					match s {
						[] => true,
						['о', 'й', '\u{1}', s @ ..] => {
							super::push(out, 24392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10559);
			match s {
				[] => true,
				['м', 'е', 'г', 'ю', s @ ..] => {
					super::push(out, 17723);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 24393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орский.
pub(crate) fn n1719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1669);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4715);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4716);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 4717);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4718);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10562);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10563);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'е', 'в', s @ ..] => {
					super::push(out, 10564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4719);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 4720);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10565);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10566);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10567);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4721);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 10569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4722);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4723);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 10571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4724);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4725);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4726);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4727);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4728);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 10575);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10576);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4729);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17727);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24394);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 24395);
									match s {
										[] => true,
										['е', 'с', s @ ..] => {
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 31475);
													s.is_empty()
												}
												['н', 'о', 'к', '\u{1}', s @ ..] => {
													super::push(out, 31476);
													s.is_empty()
												}
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
							super::push(out, 17728);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 17729);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24396);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 28982);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 28983);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 28984);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'а', s @ ..] => {
							super::push(out, 17730);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17731);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10578);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17733);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24400);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 28985);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 28986);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 31477);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 24401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 17734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10579);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10580);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17735);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17736);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10581);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17737);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24403);
									s.is_empty()
								}
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
			super::push(out, 4730);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 4731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10583);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прский.
pub(crate) fn n1720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1670);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4732);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -истрский.
pub(crate) fn n1722(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4733);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10584);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10585);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 10586);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урский.
pub(crate) fn n1723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1672);
	match s {
		[] => true,
		['г', 'о', 'н', 'е', 'л', 'е', 'з', '\u{1}', s @ ..] => {
			super::push(out, 4734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фрский.
pub(crate) fn n1724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1673);
	s.is_empty()
}

/// Words ending -ырский.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4735);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 4736);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 4737);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4738);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17739);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 17740);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17741);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 17742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10587);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрский.
pub(crate) fn n1726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1674);
	s.is_empty()
}

/// Words ending -юрский.
pub(crate) fn n1727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1675);
	s.is_empty()
}

/// Words ending -ярский.
pub(crate) fn n1728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1676);
	s.is_empty()
}

/// Words ending -рельский.
pub(crate) fn n11163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10640);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 17767);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сельский.
pub(crate) fn n11164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10641);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 17768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тельский.
pub(crate) fn n11165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17769);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24413);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28987);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28988);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28989);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31479);
									s.is_empty()
								}
								['з', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32671);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33240);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 32672);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28990);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 24414);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24415);
					match s {
						[] => true,
						['з', 'и', '-', 'о', 'н', 'т', 'е', 'ч', 'у', '\u{1}', s @ ..] => {
							super::push(out, 28992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 24416);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24417);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24418);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24419);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24420);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 24421);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31480);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 32673);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31482);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 31483);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 32674);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17770);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 17771);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шельский.
pub(crate) fn n11166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10643);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 17772);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ильский.
pub(crate) fn n4985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4781);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17773);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17774);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 24423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10645);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 10646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лльский.
pub(crate) fn n4986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4782);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10647);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10648);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 10649);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольский.
pub(crate) fn n4987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4783);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10650);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10651);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17777);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 24424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17779);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10652);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10653);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10654);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 10655);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17781);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17782);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17783);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 24425);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 24426);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28993);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 31484);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 28994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 24427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17784);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10656);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17786);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'с', s @ ..] => {
			super::push(out, 10657);
			match s {
				[] => true,
				['и', 'р', 'б', '\u{1}', s @ ..] => {
					super::push(out, 17788);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ульский.
pub(crate) fn n4988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4784);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10659);
			s.is_empty()
		}
		['к', 'а', 'р', 'о', s @ ..] => {
			super::push(out, 10660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10661);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыльский.
pub(crate) fn n4989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4785);
	match s {
		[] => true,
		['б', 'о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17792);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17793);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10663);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 17794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эльский.
pub(crate) fn n4990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4786);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 10664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юльский.
pub(crate) fn n4991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4787);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яльский.
pub(crate) fn n4992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4788);
	s.is_empty()
}

/// Words ending -ньский.
pub(crate) fn n1763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рьский.
pub(crate) fn n1764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1711);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енький.
pub(crate) fn n1797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1743);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10689);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17805);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 24431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10690);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 10691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4822);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10692);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17806);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 17807);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 17808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10693);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17809);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 24433);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17810);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 24434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 17811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10694);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 17812);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 17813);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 24435);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 28995);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 24436);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 24437);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 24438);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 28996);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 17814);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24439);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 17815);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 17816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10695);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17817);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24441);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 28997);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 17818);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 17819);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 24442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 17820);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17821);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17822);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24444);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 28998);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10696);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 17823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 10697);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17824);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 24446);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 28999);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 24447);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 17825);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 24448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4823);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4824);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10698);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17827);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 17828);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 24449);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 24450);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 24451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10699);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17829);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17830);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10700);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4825);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10702);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10703);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4826);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17831);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 17832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4827);
			match s {
				[] => true,
				['о', 'с', s @ ..] => {
					super::push(out, 10706);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 17833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4828);
			match s {
				[] => true,
				['а', s @ ..] => super::t8::n11234(s, out),
				['и', s @ ..] => super::t8::n11235(s, out),
				['п', 'у', s @ ..] => super::t8::n11236(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t8::n5035(s, out),
		['н', s @ ..] => super::t8::n5036(s, out),
		['п', s @ ..] => super::t8::n5037(s, out),
		['р', s @ ..] => super::t8::n5038(s, out),
		['с', s @ ..] => super::t8::n5039(s, out),
		['т', s @ ..] => super::t8::n5040(s, out),
		['х', s @ ..] => super::t8::n5041(s, out),
		['ч', s @ ..] => super::t8::n5042(s, out),
		['ш', s @ ..] => super::t8::n5043(s, out),
		['щ', s @ ..] => super::t8::n5044(s, out),
		_ => false,
	}
}

/// Words ending -вавший.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4959);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10828);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10829);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10830);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10831);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 10832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4960);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10833);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10834);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 10835);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24506);
									match s {
										[] => true,
										['с', 'ы', s @ ..] => {
											super::push(out, 29026);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31488);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', 'ы', s @ ..] => {
									super::push(out, 24507);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29027);
											s.is_empty()
										}
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
					super::push(out, 10836);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17932);
							match s {
								[] => true,
								['м', 'ы', s @ ..] => {
									super::push(out, 24508);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29028);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'ы', s @ ..] => {
							super::push(out, 17933);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24509);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17934);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24510);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29029);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 24511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'ы', s @ ..] => {
							super::push(out, 17935);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17936);
							s.is_empty()
						}
						['п', 'ы', s @ ..] => {
							super::push(out, 17937);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10837);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 17938);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 17939);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 17940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10838);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 10839);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10840);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10841);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 10842);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							super::push(out, 17941);
							match s {
								[] => true,
								['т', 'ы', s @ ..] => {
									super::push(out, 24515);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29030);
											s.is_empty()
										}
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
					super::push(out, 10843);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17942);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17943);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17944);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17945);
							match s {
								[] => true,
								['о', 'к', 'ы', s @ ..] => {
									super::push(out, 24516);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29031);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17946);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4961);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10845);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10846);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10847);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t8::n5179(s, out),
		['о', s @ ..] => super::t8::n5180(s, out),
		['р', s @ ..] => {
			super::push(out, 4964);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10877);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10878);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4965);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t8::n5183(s, out),
		_ => false,
	}
}

/// Words ending -игавший.
pub(crate) fn n5187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4969);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10897);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24755);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 29197);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 29198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 24756);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 24757);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 24758);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29199);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 29200);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24759);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10898);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10899);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10900);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лгавший.
pub(crate) fn n5188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4970);
	s.is_empty()
}

/// Words ending -огавший.
pub(crate) fn n5189(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 10902);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10903);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24761);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 24762);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 24763);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 24764);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29204);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31593);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29205);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29206);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31594);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргавший.
pub(crate) fn n5190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4971);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10904);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18162);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18163);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 24765);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 24766);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 24767);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29207);
									s.is_empty()
								}
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
			super::push(out, 10905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -угавший.
pub(crate) fn n5191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4972);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 10906);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['т', 'с', 'ы', s @ ..] => {
					super::push(out, 18164);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 18165);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгавший.
pub(crate) fn n5192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4973);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 10907);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18166);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 18167);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 18168);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 24770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18169);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягавший.
pub(crate) fn n5193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4974);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 10908);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10909);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18170);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -давший.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1893);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4975);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10910);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18171);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10911);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 10912);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18172);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 18173);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18174);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18175);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18176);
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							super::push(out, 18177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18178);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10913);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24774);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24775);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29208);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4976);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4977);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10914);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10915);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10916);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18181);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18182);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18183);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10917);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10918);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10919);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 10920);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4978);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10922);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 18184);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 18185);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18186);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10923);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10924);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10925);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10926);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10927);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18189);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 24777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4980);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 10929);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4981);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10930);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4982);
			match s {
				[] => true,
				['л', 'г', s @ ..] => {
					super::push(out, 10931);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18192);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24778);
									s.is_empty()
								}
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
			super::push(out, 4983);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10932);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 4984);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 4985);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 10934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жавший.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1894);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4986);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4987);
			match s {
				[] => true,
				['б', 'ы', s @ ..] => {
					super::push(out, 10935);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'ы', s @ ..] => {
					super::push(out, 10936);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4988);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4989);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4990);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18195);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18196);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24779);
									s.is_empty()
								}
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
			super::push(out, 4991);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 4992);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10937);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -завший.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1895);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4993);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10939);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18197);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18198);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 24780);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24781);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18199);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10940);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 10941);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18200);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18201);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24783);
									s.is_empty()
								}
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
			super::push(out, 4994);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10942);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10943);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 18202);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18203);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18204);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24785);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29210);
											s.is_empty()
										}
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
					super::push(out, 10944);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 10945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4995);
			match s {
				[] => true,
				['л', 'ы', s @ ..] => {
					super::push(out, 10946);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'ы', s @ ..] => {
					super::push(out, 10947);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4996);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18207);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 18208);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 18209);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 18210);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18211);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18212);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18213);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24787);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 24788);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 24789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24790);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 24791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18214);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4997);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4998);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18216);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 18217);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 18218);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18219);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18220);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 18221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4999);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 5000);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5001);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5002);
			match s {
				[] => true,
				['в', 'ы', s @ ..] => {
					super::push(out, 10949);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кавший.
pub(crate) fn n1961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1896);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10950);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 10951);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18224);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18225);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18226);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 24795);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29211);
											s.is_empty()
										}
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
					super::push(out, 10953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18227);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18228);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18229);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18230);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24797);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24798);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24799);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18231);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 10954);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18232);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10955);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 18233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18234);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5004);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10958);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10959);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10960);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 10961);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18237);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24802);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18238);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18239);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 24803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10963);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18241);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24804);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 24805);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 29213);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 24806);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18242);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 24807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18243);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10965);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10966);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 10967);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18245);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24808);
									s.is_empty()
								}
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
			super::push(out, 5006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10968);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10969);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10970);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18246);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 18247);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									super::push(out, 24809);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 24810);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29214);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24811);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29215);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 24812);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24813);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29216);
											s.is_empty()
										}
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
					super::push(out, 10971);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10972);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10973);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10974);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 10975);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 10976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 5007);
			s.is_empty()
		}
		['л', s @ ..] => super::t8::n5227(s, out),
		['м', s @ ..] => {
			super::push(out, 5009);
			s.is_empty()
		}
		['о', s @ ..] => super::t8::n5229(s, out),
		['р', s @ ..] => super::t8::n5230(s, out),
		['с', s @ ..] => super::t8::n5231(s, out),
		['т', s @ ..] => super::t8::n5232(s, out),
		['у', s @ ..] => super::t8::n5233(s, out),
		['ч', s @ ..] => super::t8::n5234(s, out),
		['ш', s @ ..] => super::t8::n5235(s, out),
		['ы', s @ ..] => super::t8::n5236(s, out),
		['ь', s @ ..] => super::t9::n5237(s, out),
		['ю', s @ ..] => {
			super::push(out, 5019);
			s.is_empty()
		}
		['я', s @ ..] => super::t9::n5239(s, out),
		_ => false,
	}
}

/// Words ending -лавший.
pub(crate) fn n1962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1897);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5021);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5022);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11023);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18316);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18317);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 11024);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5023);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5024);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 5025);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5026);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5027);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11026);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11027);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5028);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18320);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18321);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24850);
									s.is_empty()
								}
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
			super::push(out, 5029);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мавший.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1898);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5030);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5031);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5032);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18322);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18323);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18324);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24851);
									s.is_empty()
								}
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
			super::push(out, 5033);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11028);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18325);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18326);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18327);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24852);
									s.is_empty()
								}
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
			super::push(out, 5034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -навший.
pub(crate) fn n1964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1899);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 5035);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11029);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11030);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5036);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5037);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11031);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11032);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5038);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 11033);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18330);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18331);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18332);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -павший.
pub(crate) fn n1965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1900);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5039);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11035);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11036);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11037);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11038);
					match s {
						[] => true,
						['а', 'ц', 'ы', s @ ..] => {
							super::push(out, 18333);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 11039);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 11040);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18334);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24855);
									s.is_empty()
								}
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
			super::push(out, 5040);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5041);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5042);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11041);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18335);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 24856);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29231);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 18336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11042);
					match s {
						[] => true,
						['т', 'ы', s @ ..] => {
							super::push(out, 18337);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 11043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5043);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11044);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11045);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 18338);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11046);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18339);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 11047);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18340);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24860);
									s.is_empty()
								}
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
			super::push(out, 5044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11048);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 11049);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11050);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18341);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18342);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18343);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11051);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 18344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11052);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11053);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11054);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 18347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5045);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11055);
					s.is_empty()
				}
				['е', 'ч', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18348);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 24863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18349);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18350);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24864);
									s.is_empty()
								}
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
			super::push(out, 5046);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5047);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5048);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11057);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11058);
					match s {
						[] => true,
						['о', 'к', 'ы', s @ ..] => {
							super::push(out, 18352);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11059);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 18353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11060);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 11061);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18354);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18355);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24866);
									s.is_empty()
								}
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
			super::push(out, 5049);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11062);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['д', 'а', s @ ..] => {
							super::push(out, 18356);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 18357);
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 18358);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18359);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29232);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 29233);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 18360);
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							super::push(out, 18361);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18362);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24867);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29234);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5050);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -равший.
pub(crate) fn n1966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5052);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 11064);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5053);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11065);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11066);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5054);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5055);
			match s {
				[] => true,
				['и', 'ы', s @ ..] => {
					super::push(out, 11067);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5056);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11069);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11070);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11071);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5057);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11072);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11073);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5058);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 11074);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 18369);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24868);
									s.is_empty()
								}
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
			super::push(out, 5059);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11075);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -савший.
pub(crate) fn n1967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1902);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5061);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5062);
			match s {
				[] => true,
				['т', 'ы', s @ ..] => {
					super::push(out, 11076);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', 'ы', s @ ..] => {
					super::push(out, 11077);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18372);
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
				['в', s @ ..] => {
					super::push(out, 11078);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11079);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18373);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18374);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18375);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18376);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18377);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24870);
									s.is_empty()
								}
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
			super::push(out, 5063);
			match s {
				[] => true,
				['о', 'р', 'к', 'ы', s @ ..] => {
					super::push(out, 11081);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5064);
			match s {
				[] => true,
				['р', 'б', 'ы', s @ ..] => {
					super::push(out, 11082);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'ы', s @ ..] => {
					super::push(out, 11083);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5065);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 11084);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5066);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11085);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 18382);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11086);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тавший.
pub(crate) fn n1968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1903);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5067);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18384);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24872);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24873);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29235);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 18385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11088);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18386);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11089);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 11090);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 11091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5068);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11092);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11093);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5069);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11094);
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24875);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24876);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18389);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 24877);
									s.is_empty()
								}
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
			super::push(out, 5070);
			match s {
				[] => true,
				['о', 'б', 'ы', s @ ..] => {
					super::push(out, 11095);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5071);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11096);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18391);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29237);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29238);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 29239);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31599);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11097);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11098);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11099);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11100);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18392);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11101);
					match s {
						[] => true,
						['о', 'л', 'х', 'ы', s @ ..] => {
							super::push(out, 18393);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11102);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 11103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5072);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11104);
					s.is_empty()
				}
				['о', 'т', s @ ..] => {
					super::push(out, 11105);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18394);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18395);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18396);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24880);
									s.is_empty()
								}
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
			super::push(out, 5073);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11106);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18397);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 18398);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 24882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 18399);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 18400);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 18401);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18402);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 18403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11107);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11108);
					match s {
						[] => true,
						['л', 'х', 'ы', s @ ..] => {
							super::push(out, 18404);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11109);
					match s {
						[] => true,
						['в', 'с', 'ы', s @ ..] => {
							super::push(out, 18405);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 18406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11110);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18407);
							match s {
								[] => true,
								['п', 'ы', s @ ..] => {
									super::push(out, 24885);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29240);
											s.is_empty()
										}
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
					super::push(out, 11111);
					s.is_empty()
				}
				['у', 'р', s @ ..] => {
					super::push(out, 11112);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 18408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 11113);
					match s {
						[] => true,
						['р', 'х', '\u{1}', s @ ..] => {
							super::push(out, 18409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5075);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11114);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11115);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t9::n11667(s, out),
				['п', s @ ..] => super::t9::n11668(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t9::n5296(s, out),
		['ч', s @ ..] => {
			super::push(out, 5077);
			s.is_empty()
		}
		['ы', s @ ..] => super::t9::n5298(s, out),
		['я', s @ ..] => super::t9::n5299(s, out),
		_ => false,
	}
}

/// Words ending -хавший.
pub(crate) fn n1969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1904);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11122);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11123);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 11124);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 11125);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11126);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18416);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11127);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11128);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18417);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11129);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5080);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11130);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11131);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 11132);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11133);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11134);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11135);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 11136);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11137);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5081);
			match s {
				[] => true,
				['п', 'ы', s @ ..] => {
					super::push(out, 11138);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11139);
					s.is_empty()
				}
				['ч', 'ы', s @ ..] => {
					super::push(out, 11140);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5082);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5083);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5084);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11141);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11142);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 11143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18424);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 18425);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18426);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18427);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18428);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11144);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11145);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11146);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 11147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5085);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5086);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					super::push(out, 11148);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11149);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11151);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5087);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11153);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18431);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цавший.
pub(crate) fn n1970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5088);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11154);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5089);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чавший.
pub(crate) fn n1971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1906);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5091);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 11156);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11157);
					s.is_empty()
				}
				['т', 'ы', s @ ..] => {
					super::push(out, 11158);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5092);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 11159);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5093);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5094);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11161);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11162);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18435);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 18436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 18437);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18438);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18439);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18440);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11163);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18442);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24893);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 29241);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 29242);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31600);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 24894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t9::n19392(s, out),
						['е', s @ ..] => super::t9::n19393(s, out),
						['ж', s @ ..] => super::t9::n19394(s, out),
						['з', s @ ..] => super::t9::n19395(s, out),
						['й', s @ ..] => super::t9::n19396(s, out),
						['м', s @ ..] => super::t9::n19397(s, out),
						['н', s @ ..] => super::t9::n19398(s, out),
						['р', s @ ..] => super::t9::n19399(s, out),
						['с', s @ ..] => super::t9::n19400(s, out),
						['т', s @ ..] => super::t9::n19401(s, out),
						['ч', s @ ..] => super::t9::n19402(s, out),
						['ш', s @ ..] => super::t9::n19403(s, out),
						['ь', s @ ..] => super::t9::n19404(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t9::n11715(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t9::n5317(s, out),
		['м', s @ ..] => super::t9::n5318(s, out),
		['н', s @ ..] => super::t9::n5319(s, out),
		['о', s @ ..] => super::t9::n5320(s, out),
		['п', s @ ..] => super::t9::n5321(s, out),
		['р', s @ ..] => super::t9::n5322(s, out),
		['у', s @ ..] => super::t9::n5323(s, out),
		['ы', s @ ..] => super::t9::n5324(s, out),
		['ь', s @ ..] => super::t9::n5325(s, out),
		['ю', s @ ..] => super::t9::n5326(s, out),
		['я', s @ ..] => super::t9::n5327(s, out),
		_ => false,
	}
}

/// Words ending -шавший.
pub(crate) fn n1972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1907);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5106);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5107);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11180);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11181);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18459);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11182);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11183);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5108);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5109);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5110);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5111);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5112);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11185);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18460);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 18461);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18462);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24927);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 24928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18463);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18464);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11186);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18465);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18466);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24930);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24931);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29262);
											s.is_empty()
										}
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
					super::push(out, 11187);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11188);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11189);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5113);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5114);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11191);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 11192);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11193);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 5115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щавший.
pub(crate) fn n1973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1908);
	s.is_empty()
}

/// Words ending -выздоровевший.
pub(crate) fn n33175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31617);
	s.is_empty()
}

/// Words ending -девший.
pub(crate) fn n1977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1912);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5125);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11221);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11222);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18487);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 24941);
									s.is_empty()
								}
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
			super::push(out, 5126);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5127);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5128);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11223);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11224);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 11225);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11226);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18489);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18490);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24942);
									s.is_empty()
								}
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
			super::push(out, 5129);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5130);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11227);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18491);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24943);
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
						['о', s @ ..] => {
							super::push(out, 18492);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 18493);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24944);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29266);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5131);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5132);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11228);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							super::push(out, 18494);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24945);
									s.is_empty()
								}
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
			super::push(out, 5133);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5134);
			match s {
				[] => true,
				['л', 'г', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18495);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18496);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жевший.
pub(crate) fn n1978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1913);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5135);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11230);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11231);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5136);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5137);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -левший.
pub(crate) fn n1980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1915);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5139);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5140);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5141);
			match s {
				[] => true,
				['м', 'е', s @ ..] => {
					super::push(out, 11233);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18498);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24947);
									s.is_empty()
								}
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
			super::push(out, 5142);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11234);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 11235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5143);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11236);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18500);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11237);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18501);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24949);
									s.is_empty()
								}
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
			super::push(out, 5144);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5145);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 11238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мевший.
pub(crate) fn n1981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1916);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5146);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5147);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5148);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11239);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18502);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18503);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11240);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 11241);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5149);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11243);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -невший.
pub(crate) fn n1982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1917);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5150);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11244);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5151);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11246);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11247);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 11248);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18508);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24952);
									s.is_empty()
								}
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
			super::push(out, 5152);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11249);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 11250);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11251);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18509);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18510);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24953);
									s.is_empty()
								}
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
			super::push(out, 5153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11252);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 18511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5154);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5155);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5156);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 5157);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5158);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11254);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18512);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 11255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -певший.
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1919);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5159);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5160);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11256);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18513);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5161);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5162);
			match s {
				[] => true,
				['е', 'т', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18514);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18515);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18516);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'к', 'ы', s @ ..] => {
					super::push(out, 11258);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5163);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11259);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5164);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ревший.
pub(crate) fn n1985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1920);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5165);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5166);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5167);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5168);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11261);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5169);
			match s {
				[] => true,
				['г', 'ы', s @ ..] => {
					super::push(out, 11262);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5170);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11263);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11264);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5171);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11265);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18522);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11266);
					match s {
						[] => true,
						['м', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24958);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24959);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29267);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -севший.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1921);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5172);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11267);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'в', 'а', s @ ..] => {
			super::push(out, 5173);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			super::push(out, 5174);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5175);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тевший.
pub(crate) fn n1987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1922);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5177);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11270);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11271);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11272);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11273);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 11274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5178);
			match s {
				[] => true,
				['л', 'ы', s @ ..] => {
					super::push(out, 11275);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5179);
			match s {
				[] => true,
				['п', 'ы', s @ ..] => {
					super::push(out, 11276);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5180);
			match s {
				[] => true,
				['е', 'в', 'ы', s @ ..] => {
					super::push(out, 11277);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5181);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11278);
					match s {
						[] => true,
						['в', 'с', 'ы', s @ ..] => {
							super::push(out, 18528);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24960);
									s.is_empty()
								}
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
			super::push(out, 5182);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5183);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11279);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чевший.
pub(crate) fn n1989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1924);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5184);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 11280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5185);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шевший.
pub(crate) fn n1990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1925);
	s.is_empty()
}

/// Words ending -ъевший.
pub(crate) fn n1991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1926);
	s.is_empty()
}

/// Words ending -ыевший.
pub(crate) fn n1992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1927);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5186);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скорбивший.
pub(crate) fn n26230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24968);
	s.is_empty()
}

/// Words ending -сбивший.
pub(crate) fn n5422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5199);
	s.is_empty()
}

/// Words ending -тбивший.
pub(crate) fn n5423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5200);
	s.is_empty()
}

/// Words ending -убивший.
pub(crate) fn n5424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5201);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 11298);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18552);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 24969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11299);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 18553);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11300);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18554);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 24971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 18555);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 18556);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18557);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 24972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11301);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18558);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18559);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 18560);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 24973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбивший.
pub(crate) fn n5425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5202);
	s.is_empty()
}

/// Words ending -юбивший.
pub(crate) fn n5426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5203);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ябивший.
pub(crate) fn n5427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5204);
	s.is_empty()
}

/// Words ending -вивший.
pub(crate) fn n1995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5205);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11303);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11304);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 11305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18562);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18563);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 11306);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11307);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11308);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11309);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 18565);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11311);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18568);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 18569);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24976);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24977);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29271);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18570);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24978);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24979);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24980);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29272);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18571);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24981);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 29273);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31618);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 24982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11312);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11313);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24983);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24984);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29274);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5206);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5207);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5208);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5209);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 11315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5210);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11316);
					match s {
						[] => true,
						['р', 'т', 'ы', s @ ..] => {
							super::push(out, 18572);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 11317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5211);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11318);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 11319);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18573);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11320);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 18574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11321);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11322);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11323);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18576);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 11324);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 18577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5212);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11325);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24988);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 24989);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29275);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5213);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11326);
					s.is_empty()
				}
				['л', s @ ..] => super::t9::n11884(s, out),
				['н', s @ ..] => {
					super::push(out, 11328);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11329);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11330);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18588);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24993);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18589);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18590);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 24994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11331);
					match s {
						[] => true,
						['о', s @ ..] => super::t9::n19546(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t9::n5437(s, out),
		['т', s @ ..] => super::t9::n5438(s, out),
		['у', s @ ..] => super::t9::n5439(s, out),
		['ы', s @ ..] => super::t9::n5440(s, out),
		['я', s @ ..] => super::t9::n5441(s, out),
		_ => false,
	}
}

/// Words ending -дивший.
pub(crate) fn n1996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5219);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11342);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 11343);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11344);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11345);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18597);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 18598);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 24995);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29279);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18599);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24996);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 24997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 18600);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11346);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11347);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18602);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 18603);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 24999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18604);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 25000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11348);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18605);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18606);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 11349);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11350);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 11351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5220);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5221);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11352);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11353);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 18607);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11354);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18608);
							s.is_empty()
						}
						['в', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25003);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 25004);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29280);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18609);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 18610);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 18611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11355);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 11356);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18613);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25006);
									s.is_empty()
								}
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
			super::push(out, 5222);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5223);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11358);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11359);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18615);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25007);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 29281);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18616);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 18617);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18618);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11360);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5224);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5225);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11362);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11363);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 11364);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t9::n5449(s, out),
		['р', s @ ..] => {
			super::push(out, 5227);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18636);
							match s {
								[] => true,
								['т', 'ы', s @ ..] => {
									super::push(out, 25030);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29303);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18637);
							match s {
								[] => true,
								['о', 'л', s @ ..] => {
									super::push(out, 25031);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29304);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n5451(s, out),
		['ы', s @ ..] => {
			super::push(out, 5229);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5230);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5231);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18648);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18649);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еивший.
pub(crate) fn n1997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1931);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5232);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 11380);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -живший.
pub(crate) fn n1998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1932);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5233);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11381);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11382);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18652);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 18653);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18654);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 18655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11383);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11384);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18656);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25041);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18657);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5234);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5235);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5236);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11385);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11386);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11387);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18662);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 25042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18663);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25043);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29309);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 18664);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18665);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11388);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18667);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 11389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5237);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11390);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11391);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11392);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 18670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5238);
			s.is_empty()
		}
		['л', 'о', 'д', 'о', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11393);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11394);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5239);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5240);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11395);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11396);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11397);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18673);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25045);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 29310);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31632);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18674);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11398);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 18675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11399);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', 'т', s @ ..] => {
									super::push(out, 25048);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29311);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 25049);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18676);
							match s {
								[] => true,
								['с', 'у', s @ ..] => {
									super::push(out, 25051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5241);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5242);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5243);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11401);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11402);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25052);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25053);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29312);
											s.is_empty()
										}
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
					super::push(out, 11403);
					match s {
						[] => true,
						['а', s @ ..] => super::t9::n19638(s, out),
						['д', s @ ..] => super::t9::n19639(s, out),
						['к', s @ ..] => super::t9::n19640(s, out),
						['о', s @ ..] => super::t9::n19641(s, out),
						['п', s @ ..] => super::t9::n19642(s, out),
						_ => false,
					}
				}
				['т', 'а', s @ ..] => super::t9::n11967(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t9::n5468(s, out),
		['ю', s @ ..] => super::t9::n5469(s, out),
		['я', s @ ..] => super::t9::n5470(s, out),
		_ => false,
	}
}

