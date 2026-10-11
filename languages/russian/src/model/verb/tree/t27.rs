use flexion::Path;

/// Words ending -вываться.
pub(crate) fn n25883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18189);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25412);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32033);
					match s {
						[] => true,
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
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
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
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => {
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['я', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t30::n78987(s, out),
								['з', s @ ..] => super::t30::n78988(s, out),
								['ы', 'в', '\u{1}', s @ ..] => super::t30::n78989(s, out),
								_ => false,
							}
						}
						['б', s @ ..] => super::t30::n68920(s, out),
						['е', 'п', 'м', 'е', 'т', 'ш', s @ ..] => super::t30::n68921(s, out),
						['к', s @ ..] => super::t30::n68922(s, out),
						['п', s @ ..] => super::t30::n68923(s, out),
						_ => false,
					}
				}
				['н', 'и', 'л', 'з', 'а', 'р', '\u{1}', s @ ..] => super::t30::n55599(s, out),
				['о', 'в', s @ ..] => super::t30::n55600(s, out),
				['ч', 'р', 'о', 'к', s @ ..] => super::t30::n55601(s, out),
				['ш', 'у', 'т', s @ ..] => super::t30::n55602(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t30::n40809(s, out),
		_ => false,
	}
}

/// Words ending -дываться.
pub(crate) fn n25884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18190);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25414);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
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
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
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
								['т', 'о', s @ ..] => {
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
								['у', s @ ..] => {
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
				['р', 'к', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'а', 'р', 'п', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25415);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t30::n69030(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => super::t31::n69032(s, out),
						['о', s @ ..] => super::t31::n69033(s, out),
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => super::t31::n69035(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t31::n40813(s, out),
		['и', s @ ..] => super::t31::n40814(s, out),
		['я', s @ ..] => super::t31::n40815(s, out),
		_ => false,
	}
}

/// Words ending -зываться.
pub(crate) fn n25885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18191);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25418);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32037);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
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
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
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
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				['м', s @ ..] => {
					super::push(out, 32038);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'и', 'м', '\u{1}', s @ ..] => {
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
				['н', s @ ..] => {
					super::push(out, 32039);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['б', s @ ..] => {
			super::push(out, 25419);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32040);
					match s {
						[] => true,
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
			super::push(out, 25420);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25421);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32041);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25422);
			match s {
				[] => true,
				['р', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', 'е', 'а', 'н', '\u{1}', s @ ..] => {
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => super::t31::n69084(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t31::n40821(s, out),
		['л', 'а', 'п', s @ ..] => super::t31::n40822(s, out),
		['о', s @ ..] => super::t31::n40823(s, out),
		['р', 'е', s @ ..] => super::t31::n40824(s, out),
		['с', s @ ..] => super::t31::n40825(s, out),
		['т', s @ ..] => super::t31::n40826(s, out),
		['ы', s @ ..] => super::t31::n40827(s, out),
		['ь', 'л', 'а', 'к', 'с', 'о', s @ ..] => super::t31::n40828(s, out),
		['я', s @ ..] => super::t31::n40829(s, out),
		_ => false,
	}
}

/// Words ending -лываться.
pub(crate) fn n25886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18192);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25429);
			match s {
				[] => true,
				['ж', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
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
						['в', s @ ..] => {
							super::push(out, 37000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40301);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 42055);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						['и', s @ ..] => {
							super::push(out, 37001);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42056);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42990);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89263(s, out),
														_ => false,
													}
												}
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32050);
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
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
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['д', 'е', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
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
		['е', s @ ..] => {
			super::push(out, 25430);
			match s {
				[] => true,
				['д', s @ ..] => super::t31::n55666(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 25431);
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32051);
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

/// Words ending -мываться.
pub(crate) fn n25887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18193);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25432);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32052);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 32053);
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
				['н', s @ ..] => {
					super::push(out, 32054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 25433);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32055);
					match s {
						[] => true,
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
			super::push(out, 25434);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25435);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32056);
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
			match s {
				[] => true,
				['л', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32057);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37004);
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
			super::push(out, 25436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32058);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37005);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40302);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25437);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32060);
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
			super::push(out, 25438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 32061);
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
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25440);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32062);
					match s {
						[] => true,
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
			super::push(out, 25441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32063);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 32064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 37006);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 42057);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 42991);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89264(s, out),
														_ => false,
													}
												}
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
		['ы', s @ ..] => super::t31::n40846(s, out),
		_ => false,
	}
}

/// Words ending -нываться.
pub(crate) fn n25888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18194);
	match s {
		[] => true,
		['а', 'к', 'о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'л', 'е', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'а', 'к', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пываться.
pub(crate) fn n25889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18195);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32066);
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 37007);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42058);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 42992);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89265(s, out),
														_ => false,
													}
												}
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'ц', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
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
										['\u{1}', s @ ..] => {
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
										['и', '\u{1}', s @ ..] => {
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
						['к', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', 'а', s @ ..] => {
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
				['ц', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
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
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => super::t31::n79343(s, out),
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => super::t31::n69201(s, out),
						_ => false,
					}
				}
				['р', 'т', s @ ..] => super::t31::n55699(s, out),
				_ => false,
			}
		}
		['и', 'щ', s @ ..] => super::t31::n40852(s, out),
		['о', s @ ..] => super::t31::n40853(s, out),
		['р', s @ ..] => super::t31::n40854(s, out),
		['у', s @ ..] => super::t31::n40855(s, out),
		['я', s @ ..] => super::t31::n40856(s, out),
		_ => false,
	}
}

/// Words ending -рываться.
pub(crate) fn n25890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18196);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25444);
			match s {
				[] => true,
				['б', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', 'х', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 32067);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 32068);
					match s {
						[] => true,
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
						['з', 'и', '\u{1}', s @ ..] => {
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
				['н', s @ ..] => {
					super::push(out, 32069);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 32070);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 25445);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 25446);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 25447);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32072);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								['р', 'п', '\u{1}', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 32073);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 25448);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25449);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32074);
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
			super::push(out, 25450);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32075);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 32076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37009);
							match s {
								[] => true,
								['м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 40304);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25451);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32077);
					match s {
						[] => true,
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
		['к', s @ ..] => super::t31::n40865(s, out),
		['о', s @ ..] => {
			super::push(out, 25453);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 25454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32081);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37010);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25455);
			match s {
				[] => true,
				['о', s @ ..] => super::t31::n55744(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 25456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t31::n55745(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t31::n40870(s, out),
		_ => false,
	}
}

/// Words ending -сываться.
pub(crate) fn n25891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18197);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
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
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
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
						['ы', s @ ..] => {
							super::push(out, 37012);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42062);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42993);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89266(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25458);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => super::t31::n79441(s, out),
								['н', '\u{1}', s @ ..] => super::t31::n79442(s, out),
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => super::t31::n69326(s, out),
						['д', 'о', 'п', '\u{1}', s @ ..] => super::t31::n69327(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t31::n69328(s, out),
						['о', s @ ..] => super::t31::n69329(s, out),
						['с', s @ ..] => super::t31::n69330(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t31::n69331(s, out),
						['у', '\u{1}', s @ ..] => super::t31::n69332(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t31::n69333(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'п', s @ ..] => super::t31::n40873(s, out),
		['у', 'к', s @ ..] => super::t31::n40874(s, out),
		['я', s @ ..] => super::t31::n40875(s, out),
		_ => false,
	}
}

/// Words ending -тываться.
pub(crate) fn n25892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18198);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', 'а', 'р', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
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
								['р', 'п', '\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
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
						['х', s @ ..] => {
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
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
								['ы', 'в', '\u{1}', s @ ..] => {
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
						['\u{1}', s @ ..] => {
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
								['\u{1}', s @ ..] => super::t31::n79478(s, out),
								['д', '\u{1}', s @ ..] => super::t31::n79479(s, out),
								['п', '\u{1}', s @ ..] => super::t31::n79480(s, out),
								['р', 'п', '\u{1}', s @ ..] => super::t31::n79481(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t31::n69371(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t31::n69372(s, out),
						['у', '\u{1}', s @ ..] => super::t31::n69373(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t31::n69374(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t31::n55777(s, out),
				['м', s @ ..] => super::t31::n55778(s, out),
				['п', 'о', 'л', 'х', s @ ..] => super::t31::n55779(s, out),
				['х', 'о', 'х', s @ ..] => super::t31::n55780(s, out),
				['ч', 'е', 'п', s @ ..] => super::t31::n55781(s, out),
				['ш', s @ ..] => super::t31::n55782(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t31::n40877(s, out),
		['и', s @ ..] => super::t31::n40878(s, out),
		['л', 'а', 'б', s @ ..] => super::t31::n40879(s, out),
		['о', 'б', 'а', 'р', s @ ..] => super::t31::n40880(s, out),
		['п', s @ ..] => super::t31::n40881(s, out),
		['р', 'е', s @ ..] => super::t31::n40882(s, out),
		['с', s @ ..] => super::t31::n40883(s, out),
		['у', s @ ..] => super::t31::n40884(s, out),
		['ы', 'п', s @ ..] => super::t31::n40885(s, out),
		['я', 'р', 'п', s @ ..] => super::t31::n40886(s, out),
		_ => false,
	}
}

/// The word повосторгаться.
pub(crate) fn n87887(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -понаслаждаться.
pub(crate) fn n87888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42994);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мождаться.
pub(crate) fn n41037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25531);
	match s {
		[] => true,
		['о', 'р', 'г', 'а', s @ ..] => {
			super::push(out, 32137);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37037);
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

/// Words ending -рождаться.
pub(crate) fn n41038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 32138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37038);
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

/// Words ending -рждаться.
pub(crate) fn n25946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18249);
	match s {
		[] => true,
		['е', 'в', 'т', s @ ..] => {
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
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

/// Words ending -уждаться.
pub(crate) fn n25947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18250);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 25533);
			match s {
				[] => true,
				['з', 'о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
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
		['л', 'б', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 25534);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32139);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 37039);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 25535);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 25536);
			match s {
				[] => true,
				['б', 'о', s @ ..] => {
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
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37040);
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
		['х', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 25537);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32141);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37041);
							match s {
								[] => true,
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word поразвлекаться.
pub(crate) fn n87889(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word иссекаться.
pub(crate) fn n56319(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отсекаться.
pub(crate) fn n41327(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word усекаться.
pub(crate) fn n41328(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word высекаться.
pub(crate) fn n41329(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -текаться.
pub(crate) fn n26066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18352);
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
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
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word подторкаться.
pub(crate) fn n69871(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оторкаться.
pub(crate) fn n56394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32274);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уркаться.
pub(crate) fn n26094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18379);
	match s {
		[] => true,
		['т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32275);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32276);
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

/// Words ending -ыркаться.
pub(crate) fn n26095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18380);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 25712);
			match s {
				[] => true,
				['у', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37102);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37103);
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
							super::push(out, 37104);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40332);
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
							super::push(out, 37105);
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
			super::push(out, 25713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32277);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 37106);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 37107);
							match s {
								[] => true,
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
					super::push(out, 32278);
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
		['ф', s @ ..] => {
			super::push(out, 25715);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
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

/// Words ending -реискаться.
pub(crate) fn n56422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32285);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иискаться.
pub(crate) fn n41386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25722);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 32286);
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

/// Words ending -нискаться.
pub(crate) fn n41387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25723);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 32287);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оискаться.
pub(crate) fn n41388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25724);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 32288);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 32289);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тискаться.
pub(crate) fn n41389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32290);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 32291);
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
			super::push(out, 32292);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
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
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40334);
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
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 32293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 32294);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыискаться.
pub(crate) fn n41390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25726);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оскаться.
pub(crate) fn n26099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18384);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 25727);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37117);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37118);
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
							super::push(out, 37119);
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
							super::push(out, 37120);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40335);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 37121);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ускаться.
pub(crate) fn n26100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18385);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 25728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
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
						['а', 'р', s @ ..] => {
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
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', s @ ..] => {
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

/// Words ending -ыскаться.
pub(crate) fn n26101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18386);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 25729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32296);
					match s {
						[] => true,
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
			super::push(out, 25730);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32297);
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
			super::push(out, 25731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32298);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 32299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32300);
					match s {
						[] => true,
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
			super::push(out, 25732);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37122);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37123);
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
							super::push(out, 37124);
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
							super::push(out, 37125);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40337);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 25733);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 25734);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32302);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -втыкаться.
pub(crate) fn n41450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25773);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дтыкаться.
pub(crate) fn n41451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25774);
	match s {
		[] => true,
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етыкаться.
pub(crate) fn n41452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25775);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37133);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40339);
							match s {
								[] => true,
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
					super::push(out, 37134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итыкаться.
pub(crate) fn n41453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25776);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 32333);
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

/// Words ending -отыкаться.
pub(crate) fn n41454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25777);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37135);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 37136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40340);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 32334);
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

/// Words ending -стыкаться.
pub(crate) fn n41455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25778);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32335);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 32336);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 32337);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттыкаться.
pub(crate) fn n41456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25779);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утыкаться.
pub(crate) fn n41457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25780);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытыкаться.
pub(crate) fn n41458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25781);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -навспоминаться.
pub(crate) fn n87891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42995);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жираться.
pub(crate) fn n26265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18526);
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пираться.
pub(crate) fn n26268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18529);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
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
		['у', s @ ..] => {
			super::push(out, 25942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32479);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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

/// Words ending -сираться.
pub(crate) fn n26269(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
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

/// Words ending -тираться.
pub(crate) fn n26270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18530);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25943);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
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
			super::push(out, 25944);
			match s {
				[] => true,
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
			super::push(out, 25945);
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
			super::push(out, 25946);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 37196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 37197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 37198);
							match s {
								[] => true,
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
					super::push(out, 32480);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 32481);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 37200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 37201);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
							super::push(out, 37202);
							match s {
								[] => true,
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
					super::push(out, 32483);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 32484);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 37203);
							match s {
								[] => true,
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
					super::push(out, 32485);
					match s {
						[] => true,
						['л', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 32486);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 25947);
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

/// Words ending -ометаться.
pub(crate) fn n41998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26026);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 32533);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 32534);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 32535);
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

/// The word сметаться.
pub(crate) fn n57097(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тметаться.
pub(crate) fn n42000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26028);
	match s {
		[] => true,
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уметаться.
pub(crate) fn n42001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26029);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыметаться.
pub(crate) fn n42002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26030);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нетаться.
pub(crate) fn n26318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18577);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 26031);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -петаться.
pub(crate) fn n26319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18578);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26032);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32537);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40356);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретаться.
pub(crate) fn n26320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18579);
	match s {
		[] => true,
		['б', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -четаться.
pub(crate) fn n26321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18580);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26033);
			match s {
				[] => true,
				['с', 'о', 'к', 'а', 'р', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмотаться.
pub(crate) fn n42045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26057);
	match s {
		[] => true,
		['о', 'б', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37235);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 37236);
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
					super::push(out, 37237);
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

/// Words ending -смотаться.
pub(crate) fn n42046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26058);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word умотаться.
pub(crate) fn n42048(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ымотаться.
pub(crate) fn n42049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26059);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -потаться.
pub(crate) fn n26331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18587);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26060);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32552);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40360);
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
									super::push(out, 40361);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 40362);
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
								['т', s @ ..] => {
									super::push(out, 40363);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 40364);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42070);
											s.is_empty()
										}
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
					super::push(out, 32553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротаться.
pub(crate) fn n26332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18588);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26061);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37239);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 37240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фотаться.
pub(crate) fn n26333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18589);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26062);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26063);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотаться.
pub(crate) fn n26334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18590);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26064);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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

/// Words ending -истаться.
pub(crate) fn n26341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18597);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26074);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37257);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40368);
									match s {
										[] => true,
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
							super::push(out, 37258);
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
							super::push(out, 37259);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40369);
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
							super::push(out, 37260);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40370);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26075);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32566);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32567);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32568);
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
			super::push(out, 26076);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32569);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37261);
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
				['х', s @ ..] => {
					super::push(out, 32570);
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

/// Words ending -востаться.
pub(crate) fn n42070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26078);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 32571);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -достаться.
pub(crate) fn n42071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26079);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ростаться.
pub(crate) fn n42072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26080);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 32572);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 37263);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40372);
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
					super::push(out, 37264);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40373);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рстаться.
pub(crate) fn n26343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18599);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26081);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37265);
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
						['в', s @ ..] => {
							super::push(out, 37266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40374);
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
						['о', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сстаться.
pub(crate) fn n26344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18600);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26082);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстаться.
pub(crate) fn n26345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18601);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 26083);
			match s {
				[] => true,
				['о', 'л', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['т', s @ ..] => {
					super::push(out, 32574);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 37267);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ястаться.
pub(crate) fn n26346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18602);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 26084);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 32575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word забултыхаться.
pub(crate) fn n87892(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word наябедничаться.
pub(crate) fn n87893(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word изъябедничаться.
pub(crate) fn n87894(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word нанежничаться.
pub(crate) fn n70521(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word разнежничаться.
pub(crate) fn n70522(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -яжничаться.
pub(crate) fn n57380(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', 'о', 'р', 'б', 'з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'у', 'с', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зничаться.
pub(crate) fn n42256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26194);
	match s {
		[] => true,
		['а', 'р', 'б', 'о', 'з', 'е', 'б', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'б', 'ю', 'л', s @ ..] => {
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
		['и', 'р', 'п', 'а', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 32635);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 37293);
					match s {
						[] => true,
						['л', 'к', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йничаться.
pub(crate) fn n42257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26195);
	match s {
		[] => true,
		['я', 'з', 'о', 'х', s @ ..] => {
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
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мничаться.
pub(crate) fn n42258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26196);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 32636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37294);
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

/// Words ending -нничаться.
pub(crate) fn n42259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26197);
	match s {
		[] => true,
		['а', 'р', 'т', 'с', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', 'о', 'р', 'к', 'т', 'о', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37295);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40378);
							match s {
								[] => true,
								['е', 'в', 'з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', 'о', 'м', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рничаться.
pub(crate) fn n42260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26198);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32637);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 32638);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37296);
					match s {
						[] => true,
						['о', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 32639);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37297);
					match s {
						[] => true,
						['о', 'л', 'з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 32640);
			match s {
				[] => true,
				['ь', 'л', 'и', 'м', 'а', 'ф', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сничаться.
pub(crate) fn n42261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26199);
	match s {
		[] => true,
		['е', 'в', 'о', 'п', 'с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 32641);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37298);
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
		_ => false,
	}
}

/// Words ending -тничаться.
pub(crate) fn n42262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26200);
	match s {
		[] => true,
		['а', 'р', 'в', 'з', 'а', 'р', 'з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42074);
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
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 42075);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 37299);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 32642);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37300);
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

/// Words ending -ьничаться.
pub(crate) fn n42263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26201);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['д', 'н', 'а', 'к', 'с', 'с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', 'г', 'и', 'р', 'о', 'о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'н', 'е', 'м', 'и', 'т', 'н', 'е', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'д', 'з', 'е', 'б', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'в', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ричаться.
pub(crate) fn n26418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18666);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 26202);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32643);
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
				['с', s @ ..] => {
					super::push(out, 32644);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 32645);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37301);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word повосхищаться.
pub(crate) fn n87897(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ырубиться.
pub(crate) fn n42726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26423);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 32841);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упротивиться.
pub(crate) fn n70954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37407);
	match s {
		[] => true,
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птивиться.
pub(crate) fn n42816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26480);
	match s {
		[] => true,
		['о', 'р', 'т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стивиться.
pub(crate) fn n42817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26481);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32894);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37408);
					match s {
						[] => true,
						['и', 'м', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бновиться.
pub(crate) fn n42833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26496);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'о', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -одновиться.
pub(crate) fn n58079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32917);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оновиться.
pub(crate) fn n42835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26498);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 32918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хновиться.
pub(crate) fn n42836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26499);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32919);
			match s {
				[] => true,
				['д', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыновиться.
pub(crate) fn n42837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26500);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 32920);
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

/// Words ending -повиться.
pub(crate) fn n26719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18890);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровиться.
pub(crate) fn n26720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18891);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 26502);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32921);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32922);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37417);
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
		['о', s @ ..] => {
			super::push(out, 26503);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32923);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 40409);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 42996);
													match s {
														[] => true,
														['з', s @ ..] => super::t31::n89269(s, out),
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
									super::push(out, 40410);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42079);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 42080);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32924);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37418);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 37419);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26504);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совиться.
pub(crate) fn n26721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18892);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26505);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товиться.
pub(crate) fn n26722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18893);
	match s {
		[] => true,
		['й', 'а', 'н', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26506);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37420);
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
						['д', 'о', 'п', s @ ..] => {
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 26507);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32926);
					match s {
						[] => true,
						['в', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40411);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40412);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовиться.
pub(crate) fn n26723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18894);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 26508);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32927);
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

/// The word обезызвествиться.
pub(crate) fn n87902(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -обороздиться.
pub(crate) fn n71101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37448);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роводиться.
pub(crate) fn n58235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32993);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 37455);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 40423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42082);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42083);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42997);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40424);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 40425);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42084);
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
					super::push(out, 40426);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42085);
							match s {
								[] => true,
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
					super::push(out, 40427);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42086);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овыводиться.
pub(crate) fn n71122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37457);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 40428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -годиться.
pub(crate) fn n26771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18942);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26594);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 26595);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26596);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32994);
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
			super::push(out, 26597);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32995);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37458);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40429);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 40430);
									match s {
										[] => true,
										['к', 'о', 'м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 37459);
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
			super::push(out, 26598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лодиться.
pub(crate) fn n26772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18943);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26599);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37460);
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
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37461);
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
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37462);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37463);
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
						['ы', s @ ..] => {
							super::push(out, 37464);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40431);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26600);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32997);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 32998);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32999);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37465);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 37466);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 40432);
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
					super::push(out, 33000);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -родиться.
pub(crate) fn n26773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18944);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26601);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26602);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33002);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 37468);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40433);
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
		['б', s @ ..] => {
			super::push(out, 26603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33003);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33004);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33005);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37469);
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
					super::push(out, 33006);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 33007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 33008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 33009);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 26604);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33010);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 37471);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 42087);
											match s {
												[] => true,
												['е', 'б', 'о', '\u{1}', s @ ..] => {
													super::push(out, 42998);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 33011);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37472);
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
			super::push(out, 26605);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33012);
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
		['о', s @ ..] => {
			super::push(out, 26606);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37473);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37474);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40434);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 40435);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 42088);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42999);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 43000);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89271(s, out),
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
									super::push(out, 40436);
									match s {
										[] => true,
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
							super::push(out, 37475);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40437);
									match s {
										[] => true,
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
							super::push(out, 37476);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40438);
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
							super::push(out, 37477);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40439);
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
							super::push(out, 37478);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 37479);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 40440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 37480);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40441);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 33013);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37481);
							match s {
								[] => true,
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
					super::push(out, 33014);
					match s {
						[] => true,
						['с', s @ ..] => super::t31::n71167(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t31::n58265(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t31::n42984(s, out),
		['у', s @ ..] => super::t31::n42985(s, out),
		['ы', s @ ..] => super::t31::n42986(s, out),
		_ => false,
	}
}

/// Words ending -ходиться.
pub(crate) fn n26774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18945);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26610);
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
		['б', s @ ..] => {
			super::push(out, 26611);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 26612);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33018);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 37483);
							match s {
								[] => true,
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
					super::push(out, 33019);
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
			super::push(out, 26613);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33021);
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
			super::push(out, 26614);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33022);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40442);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42089);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
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
				['о', 'в', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 26615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26616);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33023);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word смилосердиться.
pub(crate) fn n87911(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word умилосердиться.
pub(crate) fn n87912(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -етрудиться.
pub(crate) fn n58315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33050);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отрудиться.
pub(crate) fn n58316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33051);
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

/// The word утрудиться.
pub(crate) fn n58318(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -судиться.
pub(crate) fn n26786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18957);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26641);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26642);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 26643);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26644);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33052);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 26646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33053);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40445);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40446);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 42090);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26647);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26648);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тудиться.
pub(crate) fn n26787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18958);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 26649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33055);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33056);
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
					super::push(out, 33057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37499);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40447);
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
				['о', s @ ..] => {
					super::push(out, 33058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 33059);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37500);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -худиться.
pub(crate) fn n26788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18959);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26650);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26651);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33060);
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

/// Words ending -чудиться.
pub(crate) fn n26789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18960);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26652);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 26653);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33061);
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
			super::push(out, 26654);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33062);
					match s {
						[] => true,
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
			super::push(out, 26655);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33063);
					match s {
						[] => true,
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
			super::push(out, 26656);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыудиться.
pub(crate) fn n26790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18961);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 26657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word принарядиться.
pub(crate) fn n87913(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word попринарядиться.
pub(crate) fn n87914(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word рукоположиться.
pub(crate) fn n87915(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -множиться.
pub(crate) fn n43132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26722);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33093);
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 33094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -рожиться.
pub(crate) fn n26834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19004);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26724);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33095);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37532);
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
						['т', s @ ..] => {
							super::push(out, 37533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40454);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 40455);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 33096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37534);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 33097);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40456);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40457);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 26725);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26726);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33099);
					match s {
						[] => true,
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
		_ => false,
	}
}

/// Words ending -тожиться.
pub(crate) fn n26835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19005);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 26727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 26729);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33102);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'м', s @ ..] => {
											match s {
												[] => true,
												['а', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['и', 'а', 'з', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
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

/// Words ending -зоружиться.
pub(crate) fn n58466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37548);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 37549);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40458);
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

/// Words ending -ооружиться.
pub(crate) fn n58467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33116);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -пружиться.
pub(crate) fn n43148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26736);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33117);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 33118);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 33119);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тружиться.
pub(crate) fn n43149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26737);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 33120);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сужиться.
pub(crate) fn n26842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19011);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26738);
			match s {
				[] => true,
				['д', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тужиться.
pub(crate) fn n26843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19012);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26739);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26740);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 37550);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 37551);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 37552);
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
		['о', s @ ..] => {
			super::push(out, 26741);
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
			super::push(out, 26742);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33121);
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

/// Words ending -розиться.
pub(crate) fn n26871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19039);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 26788);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33163);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 33164);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33165);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 26789);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37574);
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
						['б', 'о', s @ ..] => {
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
							super::push(out, 37575);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40463);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word дозагрузиться.
pub(crate) fn n87921(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word самозагрузиться.
pub(crate) fn n87922(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -онтузиться.
pub(crate) fn n58576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33181);
	match s {
		[] => true,
		['к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отузиться.
pub(crate) fn n43233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26803);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 33182);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утузиться.
pub(crate) fn n43234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26804);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 33183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фузиться.
pub(crate) fn n26882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19050);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 26805);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37584);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
								['п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -цузиться.
pub(crate) fn n26883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19051);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 26806);
			match s {
				[] => true,
				['а', 'р', 'ф', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word разбахвалиться.
pub(crate) fn n87923(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -охвалиться.
pub(crate) fn n58599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33187);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 37592);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 37593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -схвалиться.
pub(crate) fn n58600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33188);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37594);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37595);
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

/// Words ending -ыхвалиться.
pub(crate) fn n58601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33189);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 37596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывалиться.
pub(crate) fn n43254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26820);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33190);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 37597);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -галиться.
pub(crate) fn n26888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19056);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26821);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -далиться.
pub(crate) fn n26889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19057);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 26822);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33191);
					match s {
						[] => true,
						['к', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40471);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40472);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 40473);
									match s {
										[] => true,
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40474);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40475);
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
		['т', s @ ..] => {
			super::push(out, 26823);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жалиться.
pub(crate) fn n26890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19058);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26825);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 26826);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33193);
					match s {
						[] => true,
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
			super::push(out, 26827);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -залиться.
pub(crate) fn n26891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19059);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26830);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26831);
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

/// Words ending -иалиться.
pub(crate) fn n26892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19060);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 26832);
			match s {
				[] => true,
				['н', 'и', 'в', 'о', 'р', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -калиться.
pub(crate) fn n26893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19061);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26833);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26834);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33194);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
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
			super::push(out, 26835);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33195);
					match s {
						[] => true,
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
			super::push(out, 26836);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26837);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26838);
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
		['р', s @ ..] => {
			super::push(out, 26839);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33196);
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
		['с', s @ ..] => {
			super::push(out, 26840);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33197);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33198);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37598);
							match s {
								[] => true,
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
					super::push(out, 33199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 26841);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33200);
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
		['ы', s @ ..] => {
			super::push(out, 26842);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33202);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 37599);
							match s {
								[] => true,
								['м', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -малиться.
pub(crate) fn n26894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19062);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 26843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 26844);
			match s {
				[] => true,
				['а', 'р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37600);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -налиться.
pub(crate) fn n26895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19063);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -палиться.
pub(crate) fn n26896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19064);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26846);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26847);
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
		['д', s @ ..] => {
			super::push(out, 26848);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 26849);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26850);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 33204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33206);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26851);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26852);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33207);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ралиться.
pub(crate) fn n26897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19065);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 26853);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33208);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 33209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37601);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -салиться.
pub(crate) fn n26898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19066);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26854);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26855);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33210);
					match s {
						[] => true,
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
			super::push(out, 26856);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26857);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 26858);
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
		['с', s @ ..] => {
			super::push(out, 26859);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 26860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 33211);
					match s {
						[] => true,
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
			super::push(out, 26861);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чалиться.
pub(crate) fn n26899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19067);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26862);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 26863);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37602);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 37603);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40476);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42097);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 37604);
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 26864);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37605);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 37606);
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
		['с', s @ ..] => {
			super::push(out, 26865);
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
		['ы', s @ ..] => {
			super::push(out, 26866);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шалиться.
pub(crate) fn n26900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19068);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26867);
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
		['о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26868);
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

/// The word доопределиться.
pub(crate) fn n87925(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word самоопределиться.
pub(crate) fn n87926(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тделиться.
pub(crate) fn n43336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26896);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 33237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уделиться.
pub(crate) fn n43337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26897);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 33238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыделиться.
pub(crate) fn n43338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26898);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -желиться.
pub(crate) fn n26913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19081);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 26899);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
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

/// Words ending -мелиться.
pub(crate) fn n26914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19082);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26900);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26901);
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
			super::push(out, 26902);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26903);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33240);
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
		['з', s @ ..] => {
			super::push(out, 26904);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26905);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 26906);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33242);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 26907);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пелиться.
pub(crate) fn n26915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19083);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26908);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37623);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -релиться.
pub(crate) fn n26916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19084);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 26909);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26910);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 33246);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37625);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 37626);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40481);
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
							super::push(out, 37627);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 37628);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 37629);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -селиться.
pub(crate) fn n26917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19085);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26911);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26912);
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
		['в', s @ ..] => {
			super::push(out, 26913);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26914);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 26915);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37630);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 37631);
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', '\u{1}', s @ ..] => {
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
								['с', s @ ..] => {
									super::push(out, 40483);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42098);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33248);
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
			super::push(out, 26916);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33249);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37632);
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
				['р', s @ ..] => {
					super::push(out, 33250);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37633);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 26917);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33251);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							super::push(out, 37634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 26919);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 26920);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телиться.
pub(crate) fn n26918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19086);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26921);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 26922);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33253);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40484);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40485);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42099);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 42100);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43003);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89273(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 26923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33254);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33255);
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
			super::push(out, 26924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33256);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33257);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 37635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40486);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33258);
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
					super::push(out, 33259);
					match s {
						[] => true,
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
					super::push(out, 33260);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -целиться.
pub(crate) fn n26919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19087);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26925);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26926);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33261);
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
		['и', s @ ..] => {
			super::push(out, 26927);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33262);
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
			super::push(out, 26928);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33263);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шелиться.
pub(crate) fn n26920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19088);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26929);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33264);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щелиться.
pub(crate) fn n26921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19089);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26930);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33265);
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

/// Words ending -смолиться.
pub(crate) fn n43435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26984);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33293);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 33294);
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
			super::push(out, 33295);
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
		['с', s @ ..] => {
			super::push(out, 33296);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37646);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40487);
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
			super::push(out, 33297);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37647);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word политься.
pub(crate) fn n43439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26988);
	s.is_empty()
}

/// Words ending -ролиться.
pub(crate) fn n26947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19115);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 26989);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33300);
					match s {
						[] => true,
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
			super::push(out, 26990);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -солиться.
pub(crate) fn n26948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19116);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26991);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 26992);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 26993);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33304);
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
			super::push(out, 26994);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33305);
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
			super::push(out, 26995);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33308);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 37650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 26996);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33309);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37651);
							match s {
								[] => true,
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
					super::push(out, 33310);
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
		['у', s @ ..] => {
			super::push(out, 26997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33311);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 33312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37652);
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
				['с', s @ ..] => {
					super::push(out, 33313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37653);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 26998);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -толиться.
pub(crate) fn n26949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19117);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 26999);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33315);
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
		['у', s @ ..] => {
			super::push(out, 27000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фолиться.
pub(crate) fn n26950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19118);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27001);
			match s {
				[] => true,
				['н', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37654);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37655);
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

/// Words ending -холиться.
pub(crate) fn n26951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19119);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27002);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 27003);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word засуслиться.
pub(crate) fn n71684(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word насуслиться.
pub(crate) fn n71685(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word обсуслиться.
pub(crate) fn n58881(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыслиться.
pub(crate) fn n26963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19130);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33348);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33349);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 33350);
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
					super::push(out, 33351);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33352);
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
					super::push(out, 33353);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 37671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 40488);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42101);
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
						['с', s @ ..] => {
							super::push(out, 37672);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40489);
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
				['у', s @ ..] => {
					super::push(out, 33354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 33355);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ромиться.
pub(crate) fn n27012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19177);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 27088);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27089);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33403);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40495);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 33404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -томиться.
pub(crate) fn n27013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19178);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27090);
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
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
		['у', s @ ..] => {
			super::push(out, 27091);
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

/// Words ending -зелениться.
pub(crate) fn n59128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33468);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37738);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 37739);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -злениться.
pub(crate) fn n43673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33469);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 37740);
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

/// Words ending -рилениться.
pub(crate) fn n59131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33470);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -клениться.
pub(crate) fn n43675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27172);
	match s {
		[] => true,
		['е', 'т', 'с', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олениться.
pub(crate) fn n43676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27173);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			super::push(out, 33471);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 33472);
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

/// Words ending -плениться.
pub(crate) fn n43677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27174);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -члениться.
pub(crate) fn n43678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27175);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33473);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 33474);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 33475);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 33476);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 33477);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37741);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мениться.
pub(crate) fn n27054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19218);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27176);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33478);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 33479);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37742);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 27177);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 27178);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 27179);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27180);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33480);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37743);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40515);
									match s {
										[] => true,
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
							super::push(out, 37744);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40516);
									match s {
										[] => true,
										['с', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 37745);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 40517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 37746);
							match s {
								[] => true,
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
					super::push(out, 33481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37747);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 37748);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 37749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 37750);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 37751);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40518);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27181);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37752);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 40519);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 42104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 40520);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27182);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27183);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 27184);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пениться.
pub(crate) fn n27055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19219);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27185);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27186);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 33485);
					match s {
						[] => true,
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
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37753);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40521);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40522);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ъ', s @ ..] => {
									super::push(out, 40523);
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43004);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n89274(s, out),
														_ => false,
													}
												}
												['в', s @ ..] => {
													super::push(out, 43005);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n89275(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 33487);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40524);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40525);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27188);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33488);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рениться.
pub(crate) fn n27056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19220);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 27189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33489);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33490);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33491);
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
			super::push(out, 27190);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37755);
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 37756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сениться.
pub(crate) fn n27057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19221);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тениться.
pub(crate) fn n27058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19222);
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цениться.
pub(crate) fn n27059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19223);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27193);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 27194);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 33493);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37757);
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
				['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 27196);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33494);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33495);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37758);
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
		['у', s @ ..] => {
			super::push(out, 27197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чениться.
pub(crate) fn n27060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19224);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27198);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
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

/// Words ending -щениться.
pub(crate) fn n27061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19225);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27199);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 27200);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27201);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word уполовиниться.
pub(crate) fn n87934(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word воссоединиться.
pub(crate) fn n87935(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -удлиниться.
pub(crate) fn n59198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33511);
	s.is_empty()
}

/// Words ending -клиниться.
pub(crate) fn n43728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33512);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37781);
					match s {
						[] => true,
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
			super::push(out, 33513);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 33514);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37782);
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
			super::push(out, 33515);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37783);
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
			super::push(out, 33516);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -синиться.
pub(crate) fn n27074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19238);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27222);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27223);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27224);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27225);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33517);
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
			super::push(out, 27226);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27227);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиниться.
pub(crate) fn n27075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19239);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27228);
			match s {
				[] => true,
				['щ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37785);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27229);
			match s {
				[] => true,
				['а', 'р', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37787);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 37788);
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
		['о', s @ ..] => {
			super::push(out, 27230);
			match s {
				[] => true,
				['к', 'с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27231);
			match s {
				[] => true,
				['а', 'п', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -финиться.
pub(crate) fn n27076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19240);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27232);
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

/// Words ending -чиниться.
pub(crate) fn n27077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19241);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27233);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27234);
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
			super::push(out, 27235);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33519);
					match s {
						[] => true,
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
			super::push(out, 27236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 27237);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33520);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 27238);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33521);
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 33522);
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33523);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 33524);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40533);
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
								['с', s @ ..] => {
									super::push(out, 40534);
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
		['ы', s @ ..] => {
			super::push(out, 27241);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щиниться.
pub(crate) fn n27078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19242);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 27242);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37789);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 37790);
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

/// The word растрезвониться.
pub(crate) fn n87936(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word наодеколониться.
pub(crate) fn n87937(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -полониться.
pub(crate) fn n59274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33553);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37799);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 37800);
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

/// Words ending -осолониться.
pub(crate) fn n72011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37801);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -слониться.
pub(crate) fn n43766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27257);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 33555);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37802);
					match s {
						[] => true,
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
			super::push(out, 33556);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 33557);
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

/// Words ending -мониться.
pub(crate) fn n27091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19255);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27258);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33558);
					match s {
						[] => true,
						['е', 'ц', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40537);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40538);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 40539);
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
			super::push(out, 27259);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 33559);
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
		['о', s @ ..] => {
			super::push(out, 27260);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 33560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37803);
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 37804);
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

/// Words ending -пониться.
pub(crate) fn n27092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19256);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 27261);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37805);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37806);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 37807);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40540);
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
							super::push(out, 37808);
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

/// Words ending -рониться.
pub(crate) fn n27093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19257);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27262);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 33562);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 37809);
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
		['б', s @ ..] => {
			super::push(out, 27263);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33563);
					match s {
						[] => true,
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
			super::push(out, 27264);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37810);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37811);
							match s {
								[] => true,
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
					super::push(out, 33565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33566);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37812);
							match s {
								[] => true,
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
					super::push(out, 33567);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40541);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40542);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 40543);
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
				['х', s @ ..] => {
					super::push(out, 33568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37814);
							s.is_empty()
						}
						['а', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40544);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 40545);
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
						['е', s @ ..] => {
							super::push(out, 37815);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 37816);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40546);
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
							super::push(out, 37817);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 37818);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 37819);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27265);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27266);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33569);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37820);
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
		['у', s @ ..] => {
			super::push(out, 27267);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 27268);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сониться.
pub(crate) fn n27094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19258);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27269);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37821);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 37822);
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

/// Words ending -тониться.
pub(crate) fn n27095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27270);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 33571);
					match s {
						[] => true,
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
			super::push(out, 27271);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фониться.
pub(crate) fn n27096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19260);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рисовокупиться.
pub(crate) fn n87938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43006);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тупиться.
pub(crate) fn n27190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19351);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27388);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 33654);
					match s {
						[] => true,
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
			super::push(out, 27389);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33655);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 33656);
					match s {
						[] => true,
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
			super::push(out, 27391);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 33657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37855);
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word высеребриться.
pub(crate) fn n87939(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word зазубриться.
pub(crate) fn n72235(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word назубриться.
pub(crate) fn n72236(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ззубриться.
pub(crate) fn n59604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33708);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ызубриться.
pub(crate) fn n59605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33709);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 37885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбриться.
pub(crate) fn n27215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19376);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27459);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word соразмериться.
pub(crate) fn n59663(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -измериться.
pub(crate) fn n59664(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -имериться.
pub(crate) fn n44089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27494);
	match s {
		[] => true,
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омериться.
pub(crate) fn n44090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27495);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 33743);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37894);
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

/// The word смериться.
pub(crate) fn n44091(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отмериться.
pub(crate) fn n44092(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -умериться.
pub(crate) fn n44093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27496);
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

/// Words ending -ьмериться.
pub(crate) fn n44094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27497);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 33744);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37895);
					match s {
						[] => true,
						['в', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нериться.
pub(crate) fn n27232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27498);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33745);
					match s {
						[] => true,
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
			super::push(out, 27499);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33746);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27500);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33747);
					match s {
						[] => true,
						['а', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -периться.
pub(crate) fn n27233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19394);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27501);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 27502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 27503);
			match s {
				[] => true,
				['ш', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['щ', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27504);
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

/// Words ending -сериться.
pub(crate) fn n27234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19395);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27505);
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -териться.
pub(crate) fn n27235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19396);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27506);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37896);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37897);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 37898);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 37899);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 37900);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 37901);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 40569);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27507);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37902);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37903);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 37904);
							match s {
								[] => true,
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
					super::push(out, 33748);
					match s {
						[] => true,
						['ш', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33749);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 37905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27508);
			match s {
				[] => true,
				['в', 'е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хериться.
pub(crate) fn n27236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19397);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27509);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27510);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33750);
					match s {
						[] => true,
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
			super::push(out, 27511);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 33751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37906);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37907);
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
		['ш', s @ ..] => {
			super::push(out, 27512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33752);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33753);
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

/// Words ending -чериться.
pub(crate) fn n27237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19398);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27513);
			match s {
				[] => true,
				['д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щериться.
pub(crate) fn n27238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19399);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27514);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьериться.
pub(crate) fn n27239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19400);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 27516);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 33754);
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

/// The word недоговориться.
pub(crate) fn n87940(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word передоговориться.
pub(crate) fn n87941(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -твориться.
pub(crate) fn n44142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27538);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33767);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 33768);
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
			super::push(out, 33769);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'и', 'л', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33770);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33771);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', 'и', 'ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40574);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 37918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40575);
							match s {
								[] => true,
								['л', 'б', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42113);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42114);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'о', 'л', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37919);
					match s {
						[] => true,
						['и', 'м', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'у', 'д', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 33772);
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 33773);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -задориться.
pub(crate) fn n59723(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 37921);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40576);
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

/// Words ending -зориться.
pub(crate) fn n27252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19413);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27541);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27542);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33774);
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
			super::push(out, 27543);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37922);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йориться.
pub(crate) fn n27253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19414);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27544);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33776);
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

/// Words ending -кориться.
pub(crate) fn n27254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19415);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27545);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 27546);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33777);
					match s {
						[] => true,
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
			super::push(out, 27547);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 33778);
					match s {
						[] => true,
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
			super::push(out, 27548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 27549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33779);
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

/// Words ending -мориться.
pub(crate) fn n27255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19416);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27550);
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
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 27551);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word понориться.
pub(crate) fn n44164(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -пориться.
pub(crate) fn n27257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19418);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 27552);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 33781);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40577);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 42115);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43007);
													s.is_empty()
												}
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
							super::push(out, 37924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33782);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33783);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 37925);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 37926);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
					super::push(out, 33784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
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
		['у', s @ ..] => {
			super::push(out, 27554);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 33785);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37927);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 37928);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 37929);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 37930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27555);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33786);
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

/// Words ending -сориться.
pub(crate) fn n27258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19419);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27556);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27557);
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
			super::push(out, 27558);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33787);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33788);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 33789);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 33790);
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
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 27559);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 33791);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ториться.
pub(crate) fn n27259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19420);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27560);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27561);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 33793);
					match s {
						[] => true,
						['а', 'т', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 27562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33794);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 33795);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 27563);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 33796);
					match s {
						[] => true,
						['о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27564);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 33797);
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
			super::push(out, 27565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33798);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 33799);
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
		['ф', s @ ..] => {
			super::push(out, 27566);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 27567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 33800);
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

/// Words ending -фориться.
pub(crate) fn n27260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19421);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 27568);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33801);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40578);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 40579);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хориться.
pub(crate) fn n27261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19422);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27569);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 33802);
					match s {
						[] => true,
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
			super::push(out, 27570);
			match s {
				[] => true,
				['р', 'о', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37933);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37934);
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

/// Words ending -шориться.
pub(crate) fn n27262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19423);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27571);
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

/// Words ending -жмуриться.
pub(crate) fn n44217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27598);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33829);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 33830);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хмуриться.
pub(crate) fn n44218(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 33831);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 33832);
			match s {
				[] => true,
				['н', s @ ..] => {
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
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 33833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37948);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 37949);
					match s {
						[] => true,
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

/// Words ending -нуриться.
pub(crate) fn n27274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19435);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 27599);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 33834);
					match s {
						[] => true,
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
			super::push(out, 27600);
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

/// Words ending -пуриться.
pub(crate) fn n27275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19436);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27601);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37950);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 37951);
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

/// Words ending -туриться.
pub(crate) fn n27276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19437);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27602);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 27603);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'а', 'к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40583);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 40584);
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
									super::push(out, 40585);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42116);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 42117);
											match s {
												[] => true,
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
									super::push(out, 40586);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42118);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 27604);
			match s {
				[] => true,
				['а', 'х', 'с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27605);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 33835);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 37952);
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
		['ь', s @ ..] => {
			super::push(out, 27606);
			match s {
				[] => true,
				['л', 'у', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуриться.
pub(crate) fn n27277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19438);
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шуриться.
pub(crate) fn n27278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19439);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 27607);
			match s {
				[] => true,
				['м', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щуриться.
pub(crate) fn n27279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19440);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27608);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 27609);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27610);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
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

/// Words ending -пообноситься.
pub(crate) fn n80748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40591);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вноситься.
pub(crate) fn n44318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27662);
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

/// Words ending -дноситься.
pub(crate) fn n44319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27663);
	match s {
		[] => true,
		['е', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', s @ ..] => {
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

/// Words ending -реноситься.
pub(crate) fn n44320(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -зноситься.
pub(crate) fn n44321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27664);
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 33878);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 33879);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 40592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 37980);
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

/// The word приноситься.
pub(crate) fn n44322(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оноситься.
pub(crate) fn n44323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27665);
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 33880);
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

/// Words ending -рноситься.
pub(crate) fn n44324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27666);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 33881);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сноситься.
pub(crate) fn n44325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27667);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тноситься.
pub(crate) fn n44326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27668);
	match s {
		[] => true,
		['о', s @ ..] => {
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

/// Words ending -уноситься.
pub(crate) fn n44327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27669);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыноситься.
pub(crate) fn n44328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27670);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роситься.
pub(crate) fn n27315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19473);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 27671);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 27672);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33883);
					match s {
						[] => true,
						['з', s @ ..] => {
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
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 33884);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 33885);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37982);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40593);
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
		['о', s @ ..] => {
			super::push(out, 27673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33886);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 33887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 33888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37983);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 37985);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 40594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42119);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 42120);
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
					super::push(out, 33889);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37986);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 37987);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 40595);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 33890);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 33891);
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
					super::push(out, 33892);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 37989);
							match s {
								[] => true,
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
					super::push(out, 33893);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 37990);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40597);
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
						['и', '\u{1}', s @ ..] => {
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
				['т', s @ ..] => {
					super::push(out, 33894);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 33895);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соситься.
pub(crate) fn n27316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19474);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 27675);
			match s {
				[] => true,
				['л', 'ы', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37992);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37993);
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

/// The word посетиться.
pub(crate) fn n60094(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

