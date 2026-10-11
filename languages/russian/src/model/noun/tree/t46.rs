use flexion::Path;

/// Words ending -ченье.
pub(crate) fn n27587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54974);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 83273);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 105891);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121260);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130667);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136154);
													match s {
														[] => true,
														['ф', s @ ..] => super::t50::n140800(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 83274);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 105892);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121261);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130668);
											s.is_empty()
										}
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
							super::push(out, 105893);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 105894);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121262);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130669);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136155);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 83275);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 83276);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105896);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121263);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130670);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136156);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 83277);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 105897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 54975);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 54976);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83278);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105898);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121264);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130671);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136157);
													match s {
														[] => true,
														['у', s @ ..] => super::t50::n140801(s, out),
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
									super::push(out, 121265);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130672);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 136158);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140802(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
						['а', s @ ..] => {
							super::push(out, 105899);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121266);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130673);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 105900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 83279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'с', s @ ..] => {
					super::push(out, 83280);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105902);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 121267);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130674);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136159);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 83281);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 83282);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 83283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'л', s @ ..] => {
			match s {
				[] => true,
				['е', 'в', 'з', s @ ..] => {
					super::push(out, 83284);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105903);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121268);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130675);
											s.is_empty()
										}
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
					super::push(out, 83285);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105904);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 121269);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130676);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136160);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54977);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 83286);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t50::n55547(s, out),
		['у', s @ ..] => {
			super::push(out, 54979);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 83290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 83291);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 83292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 83293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t50::n55549(s, out),
		['ю', s @ ..] => super::t50::n55550(s, out),
		['я', s @ ..] => super::t50::n55551(s, out),
		_ => false,
	}
}

/// Words ending -шенье.
pub(crate) fn n27588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54982);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83298);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 83299);
					s.is_empty()
				}
				['р', 'к', s @ ..] => {
					super::push(out, 83300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105919);
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
					super::push(out, 83301);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105920);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121279);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'о', s @ ..] => {
					super::push(out, 83302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 54983);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 54984);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 54985);
			s.is_empty()
		}
		['у', 'р', '\u{1}', s @ ..] => {
			super::push(out, 54986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щенье.
pub(crate) fn n27589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27401);
	match s {
		[] => true,
		['а', 'р', 'а', s @ ..] => {
			super::push(out, 54987);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 83304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'в', s @ ..] => {
			super::push(out, 54988);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83305);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 105924);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121280);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130683);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136164);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140805(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-бюстье.
pub(crate) fn n137580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136169);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139368);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -житье-бытье.
pub(crate) fn n137581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136170);
	s.is_empty()
}

/// Words ending -пол-америки.
pub(crate) fn n137606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136195);
	s.is_empty()
}

/// Words ending -реопагитики.
pub(crate) fn n137607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136196);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -внутренности.
pub(crate) fn n140828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139388);
	s.is_empty()
}

/// Words ending -аниславский.
pub(crate) fn n137638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136227);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139395);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -асильевский.
pub(crate) fn n137639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136228);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -березовский.
pub(crate) fn n137640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136229);
	s.is_empty()
}

/// Words ending -артемовский.
pub(crate) fn n137641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136230);
	s.is_empty()
}

/// Words ending -дровский.
pub(crate) fn n107658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106525);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 121582);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130823);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 136231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тровский.
pub(crate) fn n107659(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 121583);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 130824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 121584);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 130825);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 136232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товский.
pub(crate) fn n85284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84391);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 106527);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 121586);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дский.
pub(crate) fn n29575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29379);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56606);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 84392);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106528);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121587);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130826);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 56607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84393);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106529);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еский.
pub(crate) fn n29576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29380);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 56608);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84394);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106530);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 121589);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130827);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136233);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n140837(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 130828);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136234);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жский.
pub(crate) fn n29577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29381);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 56609);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 84395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56610);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 56611);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84397);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зский.
pub(crate) fn n29578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29382);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56612);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 84398);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 106532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56613);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 84399);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 106533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 56614);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 84400);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 106534);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121590);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130829);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йский.
pub(crate) fn n29579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29383);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56615);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84401);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106535);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 121591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 84402);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 84403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56616);
			match s {
				[] => true,
				['д', 'у', 'с', '\u{1}', s @ ..] => {
					super::push(out, 84404);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 84405);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106536);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121592);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130830);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136235);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 84406);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 106537);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121593);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130831);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136236);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 56617);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84407);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106538);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121594);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 130832);
											s.is_empty()
										}
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
						['а', s @ ..] => {
							super::push(out, 106539);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 106540);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 106541);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121597);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 130833);
											s.is_empty()
										}
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
					super::push(out, 84408);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 106542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 84409);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 106543);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121598);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130834);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136237);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 84410);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 106544);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121599);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130835);
											s.is_empty()
										}
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
					super::push(out, 84411);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 106545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 56618);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84412);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 106546);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121600);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130836);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 56619);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 84413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кский.
pub(crate) fn n29580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29384);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 56620);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 84414);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 106547);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121601);
									s.is_empty()
								}
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
			super::push(out, 56621);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84415);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 106548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 84416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мский.
pub(crate) fn n29581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29385);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 56622);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 84417);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 84418);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 106549);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121602);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130837);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нский.
pub(crate) fn n29582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29386);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56623);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84419);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106550);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 84420);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 106551);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 106552);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 121605);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130838);
											s.is_empty()
										}
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
					super::push(out, 84421);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 106553);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 84422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 84423);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106555);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121607);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130839);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136238);
													s.is_empty()
												}
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
							super::push(out, 106556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 84424);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106557);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 106558);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121609);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 130840);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136239);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140838(s, out),
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
									super::push(out, 121610);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130841);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 136240);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140839(s, out),
														['ь', s @ ..] => super::t50::n140840(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 84425);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106559);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121611);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130842);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136241);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 56624);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84426);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 106560);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121612);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130843);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 136242);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 84427);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106561);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 84428);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106562);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121614);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130844);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 121615);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 130845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 106563);
							match s {
								[] => true,
								['л', s @ ..] => super::t50::n122890(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n57200(s, out),
		['й', s @ ..] => super::t50::n57201(s, out),
		['о', s @ ..] => super::t50::n57202(s, out),
		['у', s @ ..] => super::t50::n57203(s, out),
		['ы', s @ ..] => super::t50::n57204(s, out),
		['я', s @ ..] => super::t50::n57205(s, out),
		_ => false,
	}
}

/// Words ending -пский.
pub(crate) fn n29583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29387);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 56631);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 84451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рский.
pub(crate) fn n29584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56632);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 84452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 84453);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106583);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121633);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130856);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136247);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140843(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 84454);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 56633);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 84455);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 84456);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106585);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121634);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 130857);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 136248);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n140844(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 56634);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 56635);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 84457);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 106586);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 121635);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 130858);
											s.is_empty()
										}
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
					super::push(out, 84458);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106587);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121636);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130859);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 56636);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84459);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106588);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121637);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130860);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сский.
pub(crate) fn n29585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29389);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 56637);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84460);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 56638);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 84461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 106590);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 106591);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121638);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 106592);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121639);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130862);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136249);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', '\u{1}', s @ ..] => {
							super::push(out, 106593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тский.
pub(crate) fn n29586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29390);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56639);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84462);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 84463);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106595);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121640);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 130863);
											s.is_empty()
										}
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
					super::push(out, 84464);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 84465);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 106596);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 106597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 56640);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106598);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121641);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 130864);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136250);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n140845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 84467);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 84468);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106599);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 84469);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 106600);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 121643);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130865);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136251);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 56641);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84470);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 106601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 56642);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 84471);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 84472);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 106602);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 121644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 106603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 84473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 56643);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84474);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 84475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 56644);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 84476);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106604);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уский.
pub(crate) fn n29587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29391);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 56645);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 84477);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106605);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хский.
pub(crate) fn n29588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29392);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 56646);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чский.
pub(crate) fn n29589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29393);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 56647);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шский.
pub(crate) fn n29590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29394);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 56648);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 84480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 56649);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84481);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 106606);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121647);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьский.
pub(crate) fn n29591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29395);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 56650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84482);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 106607);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121648);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130867);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 130868);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136252);
													s.is_empty()
												}
												['ф', s @ ..] => {
													super::push(out, 136253);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140846(s, out),
														_ => false,
													}
												}
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
							super::push(out, 106608);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 121649);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 130869);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136254);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140847(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 84483);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 84484);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 106609);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121650);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130870);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136255);
													s.is_empty()
												}
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
							super::push(out, 106610);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121651);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'о', s @ ..] => {
							super::push(out, 106611);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121652);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130871);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136256);
													s.is_empty()
												}
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
							super::push(out, 106612);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121653);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 130872);
											s.is_empty()
										}
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
						['г', s @ ..] => {
							super::push(out, 106613);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 106614);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 106615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 84485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 56651);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84486);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 106616);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 84487);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 106617);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 121655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елковенький.
pub(crate) fn n137669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136258);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 139409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етиламмоний.
pub(crate) fn n137671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136260);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 139410);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141239);
					match s {
						[] => true,
						['и', 'н', 'о', 'б', 'р', 'а', 'к', 'и', 'с', 'к', 'о', 'т', 'н', 'е', 'м', 'л', 'и', 'т', 'е', 'м', 'и', 'д', 'н', 'е', 'л', 'и', 'т', 'е', 'м', 'а', 'к', 'е', 'д', '\u{1}', s @ ..] => {
							super::push(out, 142291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --депозитарий.
pub(crate) fn n140851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139411);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141240);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142292);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142890);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 143215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арламентарий.
pub(crate) fn n140852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139412);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141241);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мплементарий.
pub(crate) fn n140853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139413);
	s.is_empty()
}

/// Words ending -струментарий.
pub(crate) fn n140854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139414);
	s.is_empty()
}

/// Words ending -ртерий.
pub(crate) fn n57392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56811);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 84631);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 106722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стерий.
pub(crate) fn n57393(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 106723);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 106724);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121711);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 130902);
									s.is_empty()
								}
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
			super::push(out, 84632);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 106725);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церий.
pub(crate) fn n29722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29519);
	s.is_empty()
}

/// Words ending -черий.
pub(crate) fn n29723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29520);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 56812);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 84633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 56813);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 84634);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 106726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 56814);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84635);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 106727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 56815);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84636);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 56816);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84637);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 106729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клавикулярий.
pub(crate) fn n137678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136267);
	s.is_empty()
}

/// Words ending -ательствующий.
pub(crate) fn n142688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141242);
	s.is_empty()
}

/// Words ending -ссированный.
pub(crate) fn n137697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136286);
	s.is_empty()
}

/// Words ending -ицированный.
pub(crate) fn n137698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136287);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 139425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -голоштанный.
pub(crate) fn n137699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136288);
	s.is_empty()
}

/// Words ending -вугривенный.
pub(crate) fn n137700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136289);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 139426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -любленный.
pub(crate) fn n123064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121789);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 130948);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 130949);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 136290);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вленный.
pub(crate) fn n85758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84859);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 106868);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 121790);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 130950);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гленный.
pub(crate) fn n85759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84860);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 106869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еленный.
pub(crate) fn n85760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84861);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 106870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оленный.
pub(crate) fn n85761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84862);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 106871);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 121791);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 130951);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 136292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пленный.
pub(crate) fn n85762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84863);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 106872);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 121792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оенный.
pub(crate) fn n57707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57124);
	s.is_empty()
}

/// Words ending -ренный.
pub(crate) fn n57708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57125);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 84864);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 106873);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенный.
pub(crate) fn n57709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57126);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 84865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 106874);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 121794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченный.
pub(crate) fn n57710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57127);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 84866);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 106875);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121795);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 130952);
									s.is_empty()
								}
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
			super::push(out, 84867);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 106876);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121796);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 130953);
									match s {
										[] => true,
										['л', 'о', 'п', 'у', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139428);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 139429);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n142690(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 106877);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121797);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 130954);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 136293);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 84868);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 106878);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 121798);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130955);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 136294);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенный.
pub(crate) fn n57711(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 84869);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 84870);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 106879);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121799);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 130956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенный.
pub(crate) fn n57712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57128);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 84871);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 106880);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 121800);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 130957);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136295);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139430);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 84872);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 106881);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 121801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инный.
pub(crate) fn n30045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29838);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 57129);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 84873);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 106882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онный.
pub(crate) fn n30046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29839);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 57130);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 57131);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 84874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынный.
pub(crate) fn n30047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29840);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 57132);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 84875);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106883);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121802);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 130958);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 136296);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янный.
pub(crate) fn n30048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29841);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 57133);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84876);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -благодарный.
pub(crate) fn n137709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136297);
	s.is_empty()
}

/// Words ending -оложаберный.
pub(crate) fn n137710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136298);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 139431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ысокогорный.
pub(crate) fn n137711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136299);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -полугласный.
pub(crate) fn n137712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136300);
	s.is_empty()
}

/// Words ending -бросовестный.
pub(crate) fn n137713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136301);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139433);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word больничный.
pub(crate) fn n137714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136302);
	s.is_empty()
}

/// Words ending --хрустальный.
pub(crate) fn n137715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136303);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 139434);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -естягольный.
pub(crate) fn n137716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136304);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 139435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -досиновичек.
pub(crate) fn n137726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136314);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139439);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word любовничек.
pub(crate) fn n137727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136315);
	s.is_empty()
}

/// Words ending -ковничек.
pub(crate) fn n108148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107011);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 121871);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 130997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 136316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 121872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новничек.
pub(crate) fn n108149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107012);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 121873);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 130998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 136317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дничек.
pub(crate) fn n58145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57561);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 85120);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 85121);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 107013);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 121874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 130999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 107014);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 121875);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еничек.
pub(crate) fn n58146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57562);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 85122);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 107015);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожничек.
pub(crate) fn n58147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57563);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 85123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107016);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 121877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йничек.
pub(crate) fn n58149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57565);
	match s {
		[] => true,
		['е', 'б', s @ ..] => {
			super::push(out, 85124);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107017);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121878);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131003);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136318);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85125);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107018);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 121879);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131004);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136319);
											s.is_empty()
										}
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
					super::push(out, 107019);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121880);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131005);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136320);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нничек.
pub(crate) fn n58151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57567);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85126);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107020);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 121881);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 131006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 121882);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131007);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136321);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 121883);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136322);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85127);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107021);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 121884);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 131009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 121885);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131010);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136323);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 121886);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131011);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136324);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139440);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n142695(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 107022);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 107023);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85128);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 85129);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 85130);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107024);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121888);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131012);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пничек.
pub(crate) fn n58152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57568);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 85131);
			match s {
				[] => true,
				['т', 'с', 'е', s @ ..] => {
					super::push(out, 107025);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 121889);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рничек.
pub(crate) fn n58153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57569);
	s.is_empty()
}

/// Words ending -сничек.
pub(crate) fn n58154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57570);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 85132);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 85133);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107026);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 121890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тничек.
pub(crate) fn n58155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85134);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 107027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 121891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', '\u{1}', s @ ..] => {
					super::push(out, 107028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107029);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121892);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131015);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 136327);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139441);
													match s {
														[] => true,
														['з', s @ ..] => super::t50::n142696(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 107030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'щ', s @ ..] => {
			super::push(out, 85135);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 107031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 85136);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 85137);
			match s {
				[] => true,
				['б', 'а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 107032);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 107033);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 121893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131016);
									s.is_empty()
								}
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
			super::push(out, 85138);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 107034);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 121894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 107035);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121895);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131018);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136328);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85139);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 85140);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107036);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 121896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 107037);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121897);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131020);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136329);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чничек.
pub(crate) fn n58156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57572);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 85141);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 107038);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 121898);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131021);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136330);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139442);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n142697(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шничек.
pub(crate) fn n58157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57573);
	s.is_empty()
}

/// Words ending -щничек.
pub(crate) fn n58158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57574);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85142);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107039);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 121899);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьничек.
pub(crate) fn n58159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57575);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 85143);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 121900);
							match s {
								[] => true,
								['у', 'х', s @ ..] => {
									super::push(out, 131023);
									match s {
										[] => true,
										['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 136331);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 121901);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 131024);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136332);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яничек.
pub(crate) fn n58160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57576);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 85144);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107040);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121902);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131025);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полушубочек.
pub(crate) fn n137746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136334);
	s.is_empty()
}

/// Words ending -недоуздочек.
pub(crate) fn n137747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136335);
	s.is_empty()
}

/// Words ending -одбородочек.
pub(crate) fn n137748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136336);
	s.is_empty()
}

/// Words ending -самородочек.
pub(crate) fn n137749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136337);
	s.is_empty()
}

/// Words ending -ссудочек.
pub(crate) fn n86064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85158);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 107056);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 121914);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 131032);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орядочек.
pub(crate) fn n58171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57585);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 85159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жочек.
pub(crate) fn n30549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30338);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 57586);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 57587);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85160);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 107057);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зочек.
pub(crate) fn n30550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57588);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 57589);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85162);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107058);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121916);
									s.is_empty()
								}
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
			super::push(out, 57590);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85163);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107059);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лочек.
pub(crate) fn n30551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30340);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57591);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85164);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 85165);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107060);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 85166);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 85167);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85168);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107063);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 107064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57592);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85169);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 85170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 57593);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85171);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57594);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 85172);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 85173);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107067);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121919);
									s.is_empty()
								}
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
			super::push(out, 57595);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85174);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85175);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 85176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 57596);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85177);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107070);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочек.
pub(crate) fn n30552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30341);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57597);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 85178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57598);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 85179);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107071);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121921);
									s.is_empty()
								}
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
				['к', s @ ..] => {
					super::push(out, 85180);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 85181);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107072);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121922);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ночек.
pub(crate) fn n30553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30342);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57599);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85182);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107073);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 85183);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107074);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 85184);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 85185);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107075);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 121925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 85186);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107076);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 121926);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131035);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136339);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n140883(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57600);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85187);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107077);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121927);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131036);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 131037);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 136340);
													s.is_empty()
												}
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
							super::push(out, 107078);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121928);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131038);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 136341);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 85189);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107080);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121929);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131039);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 107081);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 107082);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121931);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131040);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 107083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 107084);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121933);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 107085);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 121934);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 131042);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 107086);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 121935);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131043);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136342);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140884(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85190);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107087);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 107088);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 107089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 85191);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107091);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 107092);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t50::n86099(s, out),
				['ч', s @ ..] => super::t50::n86100(s, out),
				['щ', s @ ..] => super::t50::n86101(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n58188(s, out),
		['о', s @ ..] => super::t50::n58189(s, out),
		['у', s @ ..] => super::t50::n58190(s, out),
		['ы', s @ ..] => super::t50::n58191(s, out),
		['ю', s @ ..] => super::t50::n58192(s, out),
		_ => false,
	}
}

/// Words ending -почек.
pub(crate) fn n30554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30343);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57606);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 57607);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57608);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 85208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 57609);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85209);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 85210);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рочек.
pub(crate) fn n30555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30344);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57610);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 85211);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 85212);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107109);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121948);
									s.is_empty()
								}
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
			super::push(out, 57611);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 57612);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85214);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107111);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121949);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131048);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 57613);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 85215);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107112);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 107113);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121951);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131049);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 107114);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 121952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 85216);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107115);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85217);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 85218);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 57614);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85219);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107117);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121954);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131050);
											s.is_empty()
										}
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
					super::push(out, 85220);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 85221);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 107119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 57615);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сочек.
pub(crate) fn n30556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30345);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57616);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57617);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85224);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121955);
									s.is_empty()
								}
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
			super::push(out, 57618);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85225);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 85226);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107122);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121956);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131051);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136346);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точек.
pub(crate) fn n30557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30346);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57620);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 85227);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 85228);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 85229);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107125);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121958);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131052);
											s.is_empty()
										}
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
					super::push(out, 85230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107126);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121959);
									s.is_empty()
								}
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
				['в', s @ ..] => {
					super::push(out, 85231);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 107127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 85232);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 107128);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121960);
									s.is_empty()
								}
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
				['в', s @ ..] => {
					super::push(out, 85233);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 85234);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121961);
									s.is_empty()
								}
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
			super::push(out, 57621);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85235);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 85236);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107130);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121962);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 131053);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136347);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140887(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57622);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85237);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107131);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121963);
									s.is_empty()
								}
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
			super::push(out, 57623);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85238);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 107132);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 121964);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131054);
											s.is_empty()
										}
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
					super::push(out, 85239);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 85240);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 85241);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121965);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131055);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136348);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140888(s, out),
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
									super::push(out, 121966);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131056);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136349);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85242);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107133);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121967);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131057);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136350);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57624);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 85243);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107134);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 121968);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131058);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136351);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n140889(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85245);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 85246);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107135);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121969);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131059);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цочек.
pub(crate) fn n30558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30347);
	s.is_empty()
}

/// Words ending -чочек.
pub(crate) fn n30559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30348);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57625);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 57626);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шочек.
pub(crate) fn n30560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30349);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57627);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 57628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -разовик.
pub(crate) fn n86250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85344);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 107194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 121989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езовик.
pub(crate) fn n58327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57736);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85345);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107195);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 121990);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низовик.
pub(crate) fn n86252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85346);
	s.is_empty()
}

/// Words ending -узовик.
pub(crate) fn n58329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57738);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85347);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 107196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзовик.
pub(crate) fn n58330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57739);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 85348);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 107197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язовик.
pub(crate) fn n58331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57740);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85349);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 107198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковик.
pub(crate) fn n30636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30421);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57741);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85350);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 107199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 57742);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85351);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 107200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 107201);
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
				['б', s @ ..] => {
					super::push(out, 85352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 85353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 57743);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85354);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107203);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121992);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 131063);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 136353);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140891(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57744);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 85355);
					match s {
						[] => true,
						['з', 'х', s @ ..] => {
							super::push(out, 107204);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121993);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131064);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136354);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловик.
pub(crate) fn n30637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30422);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57745);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85356);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 85357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57746);
			match s {
				[] => true,
				['с', 'а', s @ ..] => {
					super::push(out, 85358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57747);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 85359);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 85360);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107205);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121994);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131065);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136355);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140892(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 57748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 57749);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 57750);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 85364);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107206);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121995);
									match s {
										[] => true,
										['р', 'п', '-', s @ ..] => {
											super::push(out, 131066);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136356);
													match s {
														[] => true,
														['и', s @ ..] => super::t50::n140893(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57751);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мовик.
pub(crate) fn n30638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30423);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 57752);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 57753);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новик.
pub(crate) fn n30639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30424);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57754);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 57755);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 85367);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 85368);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107208);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121996);
									s.is_empty()
								}
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
			super::push(out, 57756);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 85369);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85370);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107210);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 57757);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85371);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 57758);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85372);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107212);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 107213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 57759);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85373);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повик.
pub(crate) fn n30640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30425);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57760);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107215);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 107216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 57761);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 57762);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровик.
pub(crate) fn n30641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30426);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 57763);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 57764);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 85377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 57765);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85378);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107218);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 121997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 85379);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107219);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85380);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107220);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121999);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 131067);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 136357);
													match s {
														[] => true,
														['д', s @ ..] => super::t50::n140894(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57766);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 57767);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85382);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 85383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 57768);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 85384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 57769);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85385);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 107222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57770);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85386);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 85387);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 85388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 57771);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 57772);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85390);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 107224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совик.
pub(crate) fn n30642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30427);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 57773);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 57774);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 57775);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 57776);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85391);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 57777);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85392);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107226);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122000);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131068);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136358);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140895(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57778);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85393);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 85394);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 85395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 57779);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 57780);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85396);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 57781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85397);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 57782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товик.
pub(crate) fn n30643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30428);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57783);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85398);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 107230);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 122001);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 131069);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 136359);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57784);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 57785);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85399);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122002);
									s.is_empty()
								}
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
			super::push(out, 57786);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85400);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 107232);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122003);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131070);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 136360);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85401);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107233);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 122004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 85402);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107234);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 107235);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122006);
									s.is_empty()
								}
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
			super::push(out, 57787);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 57788);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 57789);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 57790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85404);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107236);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 85405);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107237);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131071);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 131072);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136361);
													match s {
														[] => true,
														['у', s @ ..] => super::t50::n140896(s, out),
														_ => false,
													}
												}
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
							super::push(out, 107238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 85406);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 107239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 85407);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107240);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107241);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122010);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 107242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 57791);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85409);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107243);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122011);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 131073);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136362);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140897(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57792);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 57793);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 57794);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85410);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107244);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122012);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131074);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 136363);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 57795);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85411);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховик.
pub(crate) fn n30644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30429);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57796);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85412);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 85413);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107245);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122013);
									s.is_empty()
								}
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
			super::push(out, 57797);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 85414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 57798);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85415);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 85416);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107246);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122014);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131075);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 57799);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85417);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 85418);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 85419);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рншельдсвик.
pub(crate) fn n137776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136364);
	match s {
		[] => true,
		['э', '\u{1}', s @ ..] => {
			super::push(out, 139458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рогоглавник.
pub(crate) fn n137785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136373);
	s.is_empty()
}

/// Words ending -р-наставник.
pub(crate) fn n137786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136374);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139466);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 141266);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142300);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 142894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -невник.
pub(crate) fn n58571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57978);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 85534);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 107308);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 122052);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 107309);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122053);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 122054);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 107310);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 122055);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131096);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чевник.
pub(crate) fn n58572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57979);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85535);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107311);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 85536);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85537);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 85538);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 107315);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 107316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 85539);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107317);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 122058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пшевник.
pub(crate) fn n86448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85540);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 107318);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 122059);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивник.
pub(crate) fn n30826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30607);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 57981);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85541);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 57982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85543);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 85544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 57983);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 85545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 57984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85546);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 85547);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвник.
pub(crate) fn n30827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30608);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 57985);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85548);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овник.
pub(crate) fn n30828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30609);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 57986);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85549);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 85550);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107325);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 85551);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 57987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85552);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107327);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 107328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122062);
									s.is_empty()
								}
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
			super::push(out, 57988);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85553);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 85554);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 85555);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 107331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 57989);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85556);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 85557);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107332);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131097);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 57990);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85558);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 107334);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 85559);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107335);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122066);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 122067);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131098);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136376);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140907(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85560);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107336);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122068);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131099);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107337);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 122069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 107338);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85561);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107339);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 85562);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107340);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122072);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131100);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 57991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85563);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122073);
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
						['г', s @ ..] => {
							super::push(out, 107342);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122074);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 122075);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131102);
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
								['а', s @ ..] => {
									super::push(out, 122076);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 122077);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131103);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 122078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 107343);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122079);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131104);
											s.is_empty()
										}
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
					super::push(out, 85564);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 85565);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107344);
							match s {
								[] => true,
								['л', s @ ..] => super::t50::n123355(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => super::t50::n108485(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t50::n58585(s, out),
		['н', s @ ..] => super::t50::n58586(s, out),
		['п', s @ ..] => super::t50::n58587(s, out),
		['р', s @ ..] => super::t50::n58588(s, out),
		['с', s @ ..] => super::t50::n58589(s, out),
		['т', s @ ..] => super::t50::n58590(s, out),
		['х', s @ ..] => super::t50::n58591(s, out),
		['ч', s @ ..] => super::t50::n58592(s, out),
		_ => false,
	}
}

/// Words ending -рвник.
pub(crate) fn n30829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30610);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 58000);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 85590);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывник.
pub(crate) fn n30830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30611);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 58001);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85591);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107367);
							match s {
								[] => true,
								['п', 'о', 'д', s @ ..] => {
									super::push(out, 122093);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131107);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58002);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85592);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107368);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122094);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131108);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 136378);
													match s {
														[] => true,
														['ж', s @ ..] => super::t50::n140908(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107369);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 85594);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явник.
pub(crate) fn n30831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30612);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58003);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85595);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --прикладник.
pub(crate) fn n137791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136379);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 139469);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 141268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -виноградник.
pub(crate) fn n137792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136380);
	s.is_empty()
}

/// Words ending -едник.
pub(crate) fn n30834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30615);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58013);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85613);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 85614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85615);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107389);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85616);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107390);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122105);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 131112);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 136381);
													match s {
														[] => true,
														['д', s @ ..] => super::t50::n140910(s, out),
														['й', s @ ..] => super::t50::n140911(s, out),
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
									super::push(out, 122106);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131113);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136382);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 122107);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 131114);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85617);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 85618);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107391);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122108);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 131115);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136383);
													match s {
														[] => true,
														['в', s @ ..] => super::t50::n140912(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85619);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107392);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85620);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107393);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122110);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131116);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 107394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122111);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131117);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58016);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 58017);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85621);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58018);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85622);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107396);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122112);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131118);
											s.is_empty()
										}
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
					super::push(out, 85623);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 85624);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 85625);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107397);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122113);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131119);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136384);
													s.is_empty()
												}
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
							super::push(out, 107398);
							s.is_empty()
						}
						['ч', s @ ..] => super::t50::n108539(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t50::n86537(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t50::n58612(s, out),
		_ => false,
	}
}

/// The word бороздник.
pub(crate) fn n132479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131122);
	s.is_empty()
}

/// Words ending -идник.
pub(crate) fn n30836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30617);
	s.is_empty()
}

/// Words ending -лдник.
pub(crate) fn n30837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30618);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 58023);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85633);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндник.
pub(crate) fn n30838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30619);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58024);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 85634);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одник.
pub(crate) fn n30839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30620);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 58025);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85635);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107407);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 107408);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122119);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 131123);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 136387);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140915(s, out),
														_ => false,
													}
												}
												['ц', s @ ..] => {
													super::push(out, 136388);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140916(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85636);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107409);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122120);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131124);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136389);
													match s {
														[] => true,
														['м', s @ ..] => super::t50::n140917(s, out),
														_ => false,
													}
												}
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
							super::push(out, 107410);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122121);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131125);
											s.is_empty()
										}
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
					super::push(out, 85637);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107411);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122122);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 131126);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 136390);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140918(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t50::n108552(s, out),
						['х', s @ ..] => {
							super::push(out, 107413);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122125);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131134);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 136397);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t50::n58619(s, out),
		['з', s @ ..] => {
			super::push(out, 58027);
			match s {
				[] => true,
				['и', s @ ..] => super::t50::n86554(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t50::n58621(s, out),
		['л', s @ ..] => super::t50::n58622(s, out),
		['м', s @ ..] => super::t50::n58623(s, out),
		['р', s @ ..] => super::t50::n58624(s, out),
		['х', s @ ..] => super::t50::n58625(s, out),
		_ => false,
	}
}

/// Words ending -рдник.
pub(crate) fn n30840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30621);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58033);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85659);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 58034);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 85660);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58035);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107437);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 107438);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122144);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 131146);
											match s {
												[] => true,
												['щ', s @ ..] => {
													super::push(out, 136405);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удник.
pub(crate) fn n30841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30622);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58038);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58039);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107439);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 107440);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 107441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85665);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 85666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107442);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 107443);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 85667);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107444);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122146);
									s.is_empty()
								}
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
			super::push(out, 58041);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 85668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 58042);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдник.
pub(crate) fn n30842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30623);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 58043);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85670);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдник.
pub(crate) fn n30843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30624);
	match s {
		[] => true,
		['л', 'б', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85671);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107446);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85672);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107447);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122148);
									s.is_empty()
								}
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
			super::push(out, 58044);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 85673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядник.
pub(crate) fn n30844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30625);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58045);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 58046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85674);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107448);
							s.is_empty()
						}
						['н', 'з', s @ ..] => {
							super::push(out, 107449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 85675);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'а', 'р', 'о', s @ ..] => {
					super::push(out, 85676);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 85677);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 85678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word бессеребреник.
pub(crate) fn n137818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136406);
	s.is_empty()
}

/// Words ending -ессребреник.
pub(crate) fn n137819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136407);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 139484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -з-монтажник.
pub(crate) fn n137820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139485);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141280);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142310);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 142903);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 143224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ногоэтажник.
pub(crate) fn n137821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136409);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 139486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -смежник.
pub(crate) fn n86625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85713);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 107474);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 122157);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 131154);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 136410);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 139487);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 141281);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n143763(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 122158);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 131155);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136411);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 139488);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 141282);
													match s {
														[] => true,
														['ч', s @ ..] => super::t50::n143764(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нежник.
pub(crate) fn n58668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58074);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85714);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 107475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 85715);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 107476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 85716);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 107477);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 107478);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122160);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131156);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -режник.
pub(crate) fn n58669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58075);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 85717);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 107479);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122161);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131157);
									s.is_empty()
								}
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
			super::push(out, 85718);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 107480);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тежник.
pub(crate) fn n58670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58076);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 85719);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 85720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ижник.
pub(crate) fn n30855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30636);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 58077);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'к', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85722);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107481);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122162);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131158);
											s.is_empty()
										}
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
					super::push(out, 85723);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лжник.
pub(crate) fn n30856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30637);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 58078);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107482);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 107483);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122163);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131159);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136413);
													s.is_empty()
												}
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
							super::push(out, 107484);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожник.
pub(crate) fn n30857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30638);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58079);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 58080);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85725);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 107485);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122165);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 131160);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136414);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140929(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58081);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 58082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85726);
					s.is_empty()
				}
				['о', 'в', s @ ..] => {
					super::push(out, 85727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107486);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122166);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131161);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58083);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85728);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107487);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122167);
									s.is_empty()
								}
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
			super::push(out, 58084);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58085);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85730);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85731);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107489);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122169);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 122170);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131162);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136415);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n140930(s, out),
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
									super::push(out, 122171);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131163);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136416);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 122172);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131164);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 122173);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131165);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 136417);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140931(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 131166);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 131167);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136418);
													match s {
														[] => true,
														['а', s @ ..] => super::t50::n140932(s, out),
														_ => false,
													}
												}
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
							super::push(out, 107490);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122174);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131168);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136419);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85732);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржник.
pub(crate) fn n30858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30639);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 58086);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107492);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122175);
									s.is_empty()
								}
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
			super::push(out, 58087);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85734);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107493);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужник.
pub(crate) fn n30859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30640);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85735);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58089);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107495);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 107496);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122177);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131169);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136420);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85737);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107497);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 107498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58091);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85739);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 58092);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 85740);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107500);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122179);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжник.
pub(crate) fn n30860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30641);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58093);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85741);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 107501);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122180);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131171);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58094);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85742);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 85743);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 85744);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58095);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 58096);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85745);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 107504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58097);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 85746);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 85747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжник.
pub(crate) fn n30862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30643);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58098);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85748);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107505);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122181);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131172);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58099);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 58100);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 58101);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85749);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122182);
									s.is_empty()
								}
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
			super::push(out, 58102);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 85750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -безобразник.
pub(crate) fn n137833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136421);
	s.is_empty()
}

/// Words ending -толозник.
pub(crate) fn n108666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107525);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 122191);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 131176);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 136422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -розник.
pub(crate) fn n58713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58118);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85773);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рзник.
pub(crate) fn n30867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30648);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 58120);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 85774);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107527);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узник.
pub(crate) fn n30868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30649);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58121);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 85775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 58122);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85776);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 85777);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107529);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122194);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131177);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58123);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 58124);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85779);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107530);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122195);
									s.is_empty()
								}
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
			super::push(out, 58125);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58126);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58127);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85782);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58128);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85783);
					match s {
						[] => true,
						['к', 'у', 'к', '\u{1}', s @ ..] => {
							super::push(out, 107532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58129);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 58130);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 58131);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85786);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзник.
pub(crate) fn n30870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30651);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 58132);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85787);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 107534);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122197);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131178);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136423);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140933(s, out),
														_ => false,
													}
												}
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
							super::push(out, 107535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язник.
pub(crate) fn n30871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30652);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 58133);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85788);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107536);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -баскобайник.
pub(crate) fn n137836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136424);
	s.is_empty()
}

/// Words ending -попрошайник.
pub(crate) fn n137837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136425);
	s.is_empty()
}

/// Words ending -затейник.
pub(crate) fn n108697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107556);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 122203);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 131181);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 136426);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 139494);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141287);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 142316);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -статейник.
pub(crate) fn n123480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122204);
	s.is_empty()
}

/// Words ending -отейник.
pub(crate) fn n86728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85815);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 107558);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фейник.
pub(crate) fn n58749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58149);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 85816);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107559);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чейник.
pub(crate) fn n58750(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 85817);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 85818);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шейник.
pub(crate) fn n58751(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 85819);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 85820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ийник.
pub(crate) fn n30881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30662);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 58150);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 58151);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 58152);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85821);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107561);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122205);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ойник.
pub(crate) fn n30882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30663);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58153);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 85822);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107562);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85823);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 85824);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58154);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 85825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107564);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 107565);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122207);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 131183);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136427);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140935(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85826);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 85827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 85828);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 85829);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58156);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85830);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58157);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85831);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107569);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122208);
									s.is_empty()
								}
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
			super::push(out, 58158);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85832);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58159);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 85833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58160);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58161);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85835);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107573);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122209);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 107574);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122210);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 131185);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136428);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140936(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уйник.
pub(crate) fn n30883(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 58162);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 85836);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58163);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 85837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 58164);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85838);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word восприемник.
pub(crate) fn n137841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136429);
	s.is_empty()
}

/// Words ending -арорежимник.
pub(crate) fn n137842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136430);
	s.is_empty()
}

/// Words ending -оромник.
pub(crate) fn n86773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85859);
	s.is_empty()
}

/// Words ending -томник.
pub(crate) fn n58786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58182);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 107594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85861);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 85862);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107595);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122223);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131190);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136431);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 139497);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n142735(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85863);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 122224);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 122225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умник.
pub(crate) fn n30891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 58183);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 58184);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85864);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107596);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122226);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131191);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136432);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n140938(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58185);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85865);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 107597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 85866);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оздержанник.
pub(crate) fn n137845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136433);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-посланник.
pub(crate) fn n137846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136434);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139500);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 141291);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142319);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 142907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранник.
pub(crate) fn n58799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58195);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 85880);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 107610);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 85881);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 107611);
					s.is_empty()
				}
				['х', 'е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 122236);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131195);
									s.is_empty()
								}
								['и', 'т', 'а', 'ц', 'д', 'а', 'в', 'д', '\u{1}', s @ ..] => {
									super::push(out, 131196);
									s.is_empty()
								}
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
			super::push(out, 85882);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 107612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122237);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 122238);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131197);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 136435);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 139501);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n142738(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107613);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122239);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танник.
pub(crate) fn n58801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58197);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 85885);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 85886);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 107615);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122240);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131199);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 136436);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85887);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107616);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 85888);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107617);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122242);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131200);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136437);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 85889);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 107618);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122243);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131201);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136438);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фанник.
pub(crate) fn n58802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58198);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 85890);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107619);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122244);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енник.
pub(crate) fn n30893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30673);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58199);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 85891);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107620);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122245);
									s.is_empty()
								}
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
			super::push(out, 58200);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 85892);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 107621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 85893);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107622);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122246);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 131203);
											s.is_empty()
										}
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
					super::push(out, 85894);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 107623);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122247);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131204);
											s.is_empty()
										}
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
					super::push(out, 85895);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107624);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 122248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 107625);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122249);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 131205);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 107626);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122250);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 122251);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 122252);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131207);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 122253);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 131208);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136439);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140942(s, out),
														_ => false,
													}
												}
												['е', s @ ..] => {
													super::push(out, 136440);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140943(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 131209);
											match s {
												[] => true,
												['д', s @ ..] => super::t50::n137853(s, out),
												['е', s @ ..] => {
													super::push(out, 136442);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140945(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['щ', s @ ..] => {
											super::push(out, 131210);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136443);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 122254);
									match s {
										[] => true,
										['л', '\u{1}', s @ ..] => {
											super::push(out, 131211);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 122255);
									s.is_empty()
								}
								['м', s @ ..] => super::t50::n123533(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t50::n58805(s, out),
		['ж', s @ ..] => super::t50::n58806(s, out),
		['з', s @ ..] => super::t50::n58807(s, out),
		['л', s @ ..] => super::t50::n58808(s, out),
		['м', s @ ..] => super::t50::n58809(s, out),
		['н', s @ ..] => super::t50::n58810(s, out),
		['о', s @ ..] => super::t50::n58811(s, out),
		['п', s @ ..] => super::t50::n58812(s, out),
		['р', s @ ..] => super::t50::n58813(s, out),
		['с', s @ ..] => super::t50::n58814(s, out),
		['т', s @ ..] => super::t50::n58815(s, out),
		['ц', s @ ..] => super::t50::n58816(s, out),
		['ч', s @ ..] => super::t50::n58817(s, out),
		['ш', s @ ..] => super::t50::n58818(s, out),
		['щ', s @ ..] => super::t50::n58819(s, out),
		_ => false,
	}
}

/// Words ending -инник.
pub(crate) fn n30894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30674);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58216);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85951);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 85952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58217);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58218);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85954);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107689);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122306);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131242);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136459);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 85955);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 58219);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85956);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 85957);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107691);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122307);
									s.is_empty()
								}
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
			super::push(out, 58220);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 85958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58221);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 85959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58222);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85960);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107692);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 107693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 85961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 85962);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107695);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 85963);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 107696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85964);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 85965);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107698);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131243);
											s.is_empty()
										}
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
					super::push(out, 85966);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58223);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 85967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58224);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85968);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107700);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122310);
									s.is_empty()
								}
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
			super::push(out, 58225);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 85969);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122311);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 122312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 107701);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 122313);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 131244);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58226);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58227);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 85971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58228);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85972);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 107702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 85973);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85974);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107704);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122314);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131245);
											s.is_empty()
										}
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
					super::push(out, 85975);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 85976);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107705);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122315);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131246);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58229);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 85977);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107706);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 122316);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131247);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 107707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85978);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107708);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 85979);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107709);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122318);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131248);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58230);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 107711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t50::n86895(s, out),
				['у', s @ ..] => {
					super::push(out, 85980);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => super::t50::n58835(s, out),
		_ => false,
	}
}

/// Words ending -онник.
pub(crate) fn n30895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30675);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 58232);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 85984);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107719);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 85985);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85986);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 85987);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 85988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 58234);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 85989);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 58235);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 85990);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107722);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122327);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131252);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 136461);
													match s {
														[] => true,
														['д', s @ ..] => super::t50::n140951(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 85991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107723);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 107724);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 122328);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131253);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136462);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 85992);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107725);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122329);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 107726);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122330);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131255);
											s.is_empty()
										}
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
					super::push(out, 85993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107727);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122331);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 107728);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122332);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 107729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 85994);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107730);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 85995);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 85996);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 107732);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122335);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131257);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58237);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 85997);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 107733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 85998);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 107734);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 85999);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107735);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86000);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107736);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122339);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 122340);
									s.is_empty()
								}
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
			super::push(out, 58238);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86001);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86002);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t50::n58844(s, out),
		['с', s @ ..] => {
			super::push(out, 58241);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 58242);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86008);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107742);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122348);
									s.is_empty()
								}
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
			super::push(out, 58243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86009);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => super::t50::n58848(s, out),
		_ => false,
	}
}

/// Words ending -унник.
pub(crate) fn n30896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30676);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58245);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 58246);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86011);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58247);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 86012);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 58248);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86013);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынник.
pub(crate) fn n30897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30677);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 58249);
			match s {
				[] => true,
				['л', 'а', 'и', s @ ..] => {
					super::push(out, 86014);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107748);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 122350);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131263);
											s.is_empty()
										}
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
					super::push(out, 86015);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107749);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янник.
pub(crate) fn n30898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30678);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 58250);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86016);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107750);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 107751);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122353);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131264);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136467);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58251);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 86017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 58252);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86018);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word джин-тоник.
pub(crate) fn n137880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136468);
	s.is_empty()
}

/// Words ending -диотоник.
pub(crate) fn n108910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107764);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 122361);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 131270);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 136469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтоник.
pub(crate) fn n58884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58278);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 86037);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107765);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фоник.
pub(crate) fn n30908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30687);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 58279);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 58280);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58281);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86039);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтитоварник.
pub(crate) fn n137882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136470);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дногектарник.
pub(crate) fn n137883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136471);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ахарник.
pub(crate) fn n86985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86066);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 107791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 122379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ухарник.
pub(crate) fn n86986(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 107792);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 107793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 122380);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерник.
pub(crate) fn n30918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30696);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 58311);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 58312);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86067);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 107794);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 86068);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107795);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122382);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131274);
											s.is_empty()
										}
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
					super::push(out, 86069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 58313);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86070);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58314);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 86071);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107797);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122383);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131275);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 107798);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122384);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131276);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136472);
													match s {
														[] => true,
														['ы', s @ ..] => super::t50::n140955(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58315);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86072);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58316);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86073);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107800);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 107801);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 122385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 86074);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107802);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122386);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 131277);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58317);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86075);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107803);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122387);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131278);
											s.is_empty()
										}
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
					super::push(out, 86076);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107804);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122388);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131279);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136473);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58318);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86077);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86078);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86079);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58319);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 58320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86080);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107808);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 107809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86081);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 58321);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 86082);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107811);
							match s {
								[] => true,
								['м', 'т', 'а', '\u{1}', s @ ..] => {
									super::push(out, 122390);
									s.is_empty()
								}
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
			super::push(out, 58322);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 86083);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 58323);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86084);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 86085);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107814);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122392);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131280);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 58324);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86086);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирник.
pub(crate) fn n30919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30697);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 58325);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 86087);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 58326);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 58327);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58328);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 58329);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 86090);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107817);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орник.
pub(crate) fn n30920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30698);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58330);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86091);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107818);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122394);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131281);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136474);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140956(s, out),
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
									super::push(out, 122395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 107819);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'а', s @ ..] => {
					super::push(out, 86092);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86093);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107821);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122398);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131282);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 107822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86094);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58331);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					super::push(out, 86095);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86096);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122399);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131283);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136475);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 122400);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131284);
											s.is_empty()
										}
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
					super::push(out, 86097);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107825);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 86098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 58332);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 86099);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107827);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122402);
									s.is_empty()
								}
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
			super::push(out, 58333);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86100);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86101);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 58334);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86102);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107830);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107831);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 107832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58335);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86104);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 86105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107833);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122404);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131285);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['з', s @ ..] => {
					super::push(out, 86106);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86107);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107835);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 107836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58336);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 58337);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86108);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58338);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 86109);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107838);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 86110);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122406);
									s.is_empty()
								}
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
			super::push(out, 58339);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86111);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107840);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 86112);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107841);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 107842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86113);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122409);
									s.is_empty()
								}
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
			super::push(out, 58340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урник.
pub(crate) fn n30921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30699);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58341);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86114);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 58342);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 58343);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86116);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58344);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					super::push(out, 86117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58345);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86118);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58346);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86119);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 86120);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107847);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 122410);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131286);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58347);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 86121);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырник.
pub(crate) fn n30922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30700);
	match s {
		[] => true,
		['д', 'л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86122);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107849);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 58348);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 86124);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58350);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 86126);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107852);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122412);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131287);
											s.is_empty()
										}
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
					super::push(out, 86127);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярник.
pub(crate) fn n30923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30701);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58351);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86128);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58352);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 86129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осник.
pub(crate) fn n30927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30705);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58369);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 86151);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107872);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122421);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131290);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86152);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 86153);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86154);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107874);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122422);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131291);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 107875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 86155);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58371);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86156);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107877);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122423);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 131292);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58372);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86157);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 86158);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 107879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58373);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86159);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107880);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122424);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131293);
											s.is_empty()
										}
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
					super::push(out, 86160);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107881);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 122425);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131294);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136476);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58374);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 86161);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усник.
pub(crate) fn n30930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30708);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58378);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86164);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86165);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107884);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122426);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131295);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86166);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107885);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86167);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122428);
									s.is_empty()
								}
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
			super::push(out, 58380);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86168);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 107887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86169);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107888);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122429);
									s.is_empty()
								}
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
			super::push(out, 58381);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 86170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58382);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 86171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58383);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86172);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107890);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122430);
									s.is_empty()
								}
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
			super::push(out, 58384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86173);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86174);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122431);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131296);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136477);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140957(s, out),
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
									super::push(out, 122432);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131297);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136478);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 86175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58385);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 86176);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пульсник.
pub(crate) fn n109042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107893);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 122433);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 131298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эсник.
pub(crate) fn n30933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30711);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 58388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юсник.
pub(crate) fn n30934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30712);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58389);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86178);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122434);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 122435);
									match s {
										[] => true,
										['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 131299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясник.
pub(crate) fn n30935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30713);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58390);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86179);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58391);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58392);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86181);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107896);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122436);
									s.is_empty()
								}
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
			super::push(out, 58393);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86182);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 86183);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107897);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сокомнатник.
pub(crate) fn n137891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136479);
	s.is_empty()
}

/// Words ending -бархатник.
pub(crate) fn n123721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122444);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 131301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цатник.
pub(crate) fn n59014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58403);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 86206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чатник.
pub(crate) fn n59015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58404);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 86207);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 107916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 86208);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 107917);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 122445);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131302);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 136480);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139518);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86209);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 107918);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'о', s @ ..] => {
			super::push(out, 86210);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107919);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 122447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86211);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 107920);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122448);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 131303);
									s.is_empty()
								}
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
			super::push(out, 86212);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 107921);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122449);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131304);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шатник.
pub(crate) fn n59016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58405);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 86213);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 107922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86214);
			match s {
				[] => true,
				['г', 'я', s @ ..] => {
					super::push(out, 107923);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 122450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 86215);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 107924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 107925);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 107926);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122452);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щатник.
pub(crate) fn n59017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58406);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 86216);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 107927);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

