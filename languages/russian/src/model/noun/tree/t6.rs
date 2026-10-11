use flexion::Path;

/// Words ending -ро.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3087);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14121);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 3088);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14122);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35980);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 64006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14123);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35981);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64007);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90699);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 111082);
													match s {
														[] => true,
														['и', s @ ..] => super::t22::n125611(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14124);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35982);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64008);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90700);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 111083);
													match s {
														[] => true,
														['а', s @ ..] => super::t22::n125612(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14125);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35983);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64009);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90701);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 111084);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14126);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35984);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 64010);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 90702);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111085);
													match s {
														[] => true,
														['п', s @ ..] => super::t22::n125613(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14127);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35985);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64011);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90703);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111086);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3089);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35986);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 64012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14129);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35987);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64013);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90704);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 64014);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90705);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 64015);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90706);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 111087);
													match s {
														[] => true,
														['п', s @ ..] => super::t22::n125614(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14130);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14131);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35990);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 64017);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90707);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 111088);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3090);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14132);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35992);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 35993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t22::n3101(s, out),
		['д', s @ ..] => super::t22::n3102(s, out),
		['е', s @ ..] => super::t22::n3103(s, out),
		['и', s @ ..] => super::t22::n3104(s, out),
		['й', s @ ..] => {
			super::push(out, 3095);
			match s {
				[] => true,
				['е', s @ ..] => super::t22::n14232(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t22::n3106(s, out),
		['н', s @ ..] => super::t22::n3107(s, out),
		['о', s @ ..] => super::t22::n3108(s, out),
		['р', s @ ..] => super::t22::n3109(s, out),
		['т', s @ ..] => super::t22::n3110(s, out),
		['у', s @ ..] => super::t22::n3111(s, out),
		['ф', s @ ..] => super::t22::n3112(s, out),
		['ю', s @ ..] => super::t22::n3113(s, out),
		_ => false,
	}
}

/// Words ending -со.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3104);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14183);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 14184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36059);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 64070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14186);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 36061);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14187);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 14188);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 36062);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64072);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90743);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111102);
													match s {
														[] => true,
														['к', s @ ..] => super::t22::n125619(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3105);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14189);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36063);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64073);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 90744);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 111103);
													match s {
														[] => true,
														['о', s @ ..] => super::t22::n125620(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3106);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3107);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14192);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3108);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14193);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 36065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3109);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14194);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 36066);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 64074);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90745);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 111104);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3110);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14195);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 14196);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36068);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64075);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90746);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 111105);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3111);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14198);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36070);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64076);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3112);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14199);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36071);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 36072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 36073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3113);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36074);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 36075);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64078);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90748);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 36076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14201);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36077);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 64079);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90749);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 111106);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3114);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t22::n3125(s, out),
		_ => false,
	}
}

/// Words ending -то.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3116);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 3117);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14204);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36078);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64080);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90750);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 64081);
									s.is_empty()
								}
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
			super::push(out, 3118);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14205);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 14206);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 14207);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14208);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36079);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 36080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14209);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36081);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 36082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 36083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14210);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14211);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36084);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14212);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14213);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 36085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14214);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 36086);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64086);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90751);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 111107);
													match s {
														[] => true,
														['у', s @ ..] => super::t22::n125621(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3119);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3120);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14217);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14219);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 36090);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64087);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90752);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 36091);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 64088);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90753);
											s.is_empty()
										}
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
					super::push(out, 14220);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36092);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64089);
									s.is_empty()
								}
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
			super::push(out, 3121);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14221);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 14222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14223);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14224);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36094);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14225);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 36095);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64091);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90754);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 111108);
													match s {
														[] => true,
														['и', s @ ..] => super::t22::n125622(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t22::n36437(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t22::n14300(s, out),
				['р', s @ ..] => super::t22::n14301(s, out),
				['с', s @ ..] => super::t22::n14302(s, out),
				['т', s @ ..] => {
					super::push(out, 14229);
					s.is_empty()
				}
				['ф', s @ ..] => super::t22::n14304(s, out),
				['х', s @ ..] => super::t22::n14305(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t22::n3132(s, out),
		['н', s @ ..] => super::t22::n3133(s, out),
		['о', s @ ..] => super::t22::n3134(s, out),
		['р', s @ ..] => super::t22::n3135(s, out),
		['с', s @ ..] => super::t22::n3136(s, out),
		['т', s @ ..] => super::t22::n3137(s, out),
		['у', s @ ..] => super::t22::n3138(s, out),
		['х', s @ ..] => super::t22::n3139(s, out),
		['ч', s @ ..] => super::t22::n3140(s, out),
		['ы', s @ ..] => super::t22::n3141(s, out),
		['ь', s @ ..] => super::t22::n3142(s, out),
		['э', s @ ..] => super::t22::n3143(s, out),
		_ => false,
	}
}

/// Words ending -уо.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фо.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3135);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14267);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36151);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 64137);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90787);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111127);
													match s {
														[] => true,
														['а', s @ ..] => super::t22::n125632(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14268);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 36152);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64138);
									s.is_empty()
								}
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
			super::push(out, 3136);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14269);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36153);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64139);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90788);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3137);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14270);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 36154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3138);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14271);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3139);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14272);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36156);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 64140);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 90789);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111128);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хо.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3140);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36157);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14274);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 36158);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14275);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14276);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36160);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 64143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36161);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64144);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90790);
											s.is_empty()
										}
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
					super::push(out, 14278);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36162);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64145);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 90791);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111129);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3141);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14281);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14282);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36165);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64147);
									s.is_empty()
								}
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
			super::push(out, 3143);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3144);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14285);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 14286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14287);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 36166);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 36167);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64148);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 90792);
											s.is_empty()
										}
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
					super::push(out, 14288);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 36168);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64149);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90793);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14289);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64150);
									s.is_empty()
								}
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
			super::push(out, 3148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 3149);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14291);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3150);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14292);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цо.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3151);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14293);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 36171);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 64151);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 90794);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 111130);
													match s {
														[] => true,
														['э', s @ ..] => super::t22::n125633(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3152);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14294);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 36172);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64152);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90795);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 36173);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64153);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 90796);
											s.is_empty()
										}
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
					super::push(out, 14295);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14296);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36175);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 64154);
									s.is_empty()
								}
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
			super::push(out, 3153);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14297);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36176);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 36177);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14298);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 36178);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64157);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90797);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 111131);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 64158);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90798);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 111132);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14299);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36179);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64159);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 36180);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64160);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90800);
											s.is_empty()
										}
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
					super::push(out, 14300);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36181);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64161);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90801);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 111133);
													match s {
														[] => true,
														['с', s @ ..] => super::t22::n125634(s, out),
														_ => false,
													}
												}
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
							super::push(out, 36182);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14301);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 36183);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 64163);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90802);
											s.is_empty()
										}
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
					super::push(out, 14302);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 36184);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64164);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90803);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 36185);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64165);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 90804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 36186);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64166);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90805);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 36187);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 64167);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90806);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3154);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 14303);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3155);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t22::n3166(s, out),
		['м', s @ ..] => super::t22::n3167(s, out),
		['н', s @ ..] => super::t22::n3168(s, out),
		['р', s @ ..] => super::t22::n3169(s, out),
		['с', s @ ..] => super::t22::n3170(s, out),
		['т', s @ ..] => super::t22::n3171(s, out),
		['ц', s @ ..] => super::t22::n3172(s, out),
		['ь', s @ ..] => super::t22::n3173(s, out),
		_ => false,
	}
}

/// Words ending -чо.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3164);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14320);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14321);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14322);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36212);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64183);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90816);
											s.is_empty()
										}
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
					super::push(out, 14323);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 36213);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64184);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90817);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3165);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14324);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36214);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64185);
									s.is_empty()
								}
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
			super::push(out, 3166);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14325);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14326);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14327);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 36217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64186);
									s.is_empty()
								}
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
			super::push(out, 3168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14328);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 36218);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14329);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 36219);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64188);
									s.is_empty()
								}
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
			super::push(out, 3169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14330);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 14331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3170);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36220);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шо.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3171);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3172);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14334);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36222);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64190);
									s.is_empty()
								}
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
			super::push(out, 3173);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14335);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36223);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 64191);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 90818);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 111136);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьо.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3174);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14336);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36224);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64192);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90819);
											s.is_empty()
										}
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
					super::push(out, 14337);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36225);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64193);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90820);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3175);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14338);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36226);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64194);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 90821);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 111137);
													s.is_empty()
												}
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
							super::push(out, 36227);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 64195);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 90822);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 111138);
													match s {
														[] => true,
														['э', s @ ..] => super::t22::n125635(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яо.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 3176);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 14339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3177);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14340);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 36228);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 64196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еп.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3190);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14362);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14363);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 36248);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 64209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14364);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36249);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 14365);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3191);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14366);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36251);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64211);
									s.is_empty()
								}
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
			super::push(out, 3192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3193);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14368);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 36252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14369);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 14370);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36254);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 64213);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 90827);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 111139);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 64214);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 90828);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 111140);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3194);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14371);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 14372);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36255);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 64215);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90829);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 36256);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64216);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90830);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3195);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14373);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 36257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 14374);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 36258);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 64218);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90831);
											s.is_empty()
										}
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
					super::push(out, 14375);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 36259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36260);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 64219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90832);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 64220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 14377);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3196);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14380);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14381);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 36262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14382);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36263);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 64221);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 90833);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111141);
													match s {
														[] => true,
														['и', s @ ..] => super::t22::n125636(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14383);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3198);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14385);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 3199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ип.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3200);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3201);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 14387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 3202);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14388);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36267);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64222);
									s.is_empty()
								}
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
			super::push(out, 3203);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3204);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14389);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36268);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64223);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90834);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 111142);
													match s {
														[] => true,
														['у', s @ ..] => super::t22::n125637(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14390);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 36269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14391);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14392);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 36270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 14393);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14394);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36271);
							match s {
								[] => true,
								['в', 'и', s @ ..] => {
									super::push(out, 64226);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90835);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 111143);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t22::n125638(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3205);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3206);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3207);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 14396);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36272);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 36273);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 64227);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90836);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111144);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14397);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36274);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64228);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 64229);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 90837);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111145);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14398);
					match s {
						[] => true,
						['-', 'ф', 'р', 'е', 'с', '\u{1}', s @ ..] => {
							super::push(out, 36275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3208);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14399);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14400);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36276);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 36277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64230);
									s.is_empty()
								}
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
			super::push(out, 3209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14401);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 14402);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 36279);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 64232);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90838);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111146);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3210);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14404);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14405);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64233);
									s.is_empty()
								}
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
			super::push(out, 3211);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14406);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36281);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64234);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 90839);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3212);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 3213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14407);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14408);
					match s {
						[] => true,
						['о', s @ ..] => super::t22::n36623(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t22::n14483(s, out),
				['о', s @ ..] => super::t22::n14484(s, out),
				_ => false,
			}
		}
		['щ', s @ ..] => super::t22::n3224(s, out),
		_ => false,
	}
}

/// Words ending -лп.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3217);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3218);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 14413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3219);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14414);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36287);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мп.
pub(crate) fn n324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3220);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14415);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36288);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64240);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 90842);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111149);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3221);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3222);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14417);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3223);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оп.
pub(crate) fn n325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 324);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3224);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 3225);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 3226);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14420);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 36291);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 64241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14421);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 36292);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64242);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90843);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3227);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 14422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3228);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 14423);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 36294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3229);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14424);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36295);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 64243);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 90844);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 64244);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90845);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 111150);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14425);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36296);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 64245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14426);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36297);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 64246);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90846);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111151);
													s.is_empty()
												}
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
							super::push(out, 36298);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 64247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14427);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36299);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 64248);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90847);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111152);
													match s {
														[] => true,
														['х', s @ ..] => super::t22::n125639(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['э', s @ ..] => {
											super::push(out, 90848);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111153);
													s.is_empty()
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
											super::push(out, 90849);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 111154);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 90850);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 111155);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t22::n125640(s, out),
														_ => false,
													}
												}
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
							super::push(out, 36300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3230);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14428);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 36301);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 36302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14429);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36303);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36304);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 36305);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64250);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 64251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36306);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64252);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90851);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 111156);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14431);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t22::n36649(s, out),
						['х', s @ ..] => {
							super::push(out, 36309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t22::n14506(s, out),
				['ф', s @ ..] => super::t22::n14507(s, out),
				['х', s @ ..] => super::t22::n14508(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t22::n3241(s, out),
		['п', s @ ..] => super::t22::n3242(s, out),
		['р', s @ ..] => super::t22::n3243(s, out),
		['с', s @ ..] => super::t22::n3244(s, out),
		['т', s @ ..] => super::t22::n3245(s, out),
		['х', s @ ..] => super::t22::n3246(s, out),
		['ш', s @ ..] => super::t22::n3247(s, out),
		_ => false,
	}
}

/// Words ending -пп.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 325);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3238);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3239);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14455);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36334);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 64274);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90874);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111174);
													match s {
														[] => true,
														['к', s @ ..] => super::t22::n125649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3240);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14456);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36335);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 64275);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90875);
											s.is_empty()
										}
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
					super::push(out, 14457);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 36336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 3241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14458);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рп.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3242);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14459);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36337);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 36338);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 64276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14460);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 14461);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 36340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3243);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14462);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 14464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36342);
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
				['л', s @ ..] => {
					super::push(out, 14465);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36343);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64277);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90876);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 111175);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36344);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 64278);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90877);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уп.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3244);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3245);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3246);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14469);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 36345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14470);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36346);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 64280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14471);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36347);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64281);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 90878);
											s.is_empty()
										}
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
					super::push(out, 14472);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36348);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 64282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14473);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 14474);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 36349);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 64283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14475);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 36350);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 64284);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90879);
											s.is_empty()
										}
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
					super::push(out, 14476);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 14477);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3247);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14478);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36353);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 64285);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90880);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111176);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14479);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14480);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 36354);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14481);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 36356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 14482);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64288);
									s.is_empty()
								}
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
			super::push(out, 3248);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14483);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3249);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3250);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14486);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 14487);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36359);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 36360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14488);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 36361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3251);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 14489);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 36362);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36363);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 64291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14490);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36364);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 64292);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90881);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3252);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14491);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 36365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t22::n14566(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 3253);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 14493);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 3254);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 14495);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36373);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64302);
									s.is_empty()
								}
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
			super::push(out, 3255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ып.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3256);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14496);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 36374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14497);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 36375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14498);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36376);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64303);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90888);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 36377);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 64304);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90889);
											s.is_empty()
										}
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
					super::push(out, 14499);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36378);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14500);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 36379);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 64306);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кэп.
pub(crate) fn n3269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3258);
	s.is_empty()
}

/// Words ending -рэп.
pub(crate) fn n3270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3259);
	s.is_empty()
}

/// Words ending -юп.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	s.is_empty()
}

/// Words ending -яп.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3260);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14501);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 14502);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36380);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 64307);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90891);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3261);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14503);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36381);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64308);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90892);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 111181);
													match s {
														[] => true,
														['г', s @ ..] => super::t22::n125651(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ар.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3264);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14505);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 14506);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14507);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 36382);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 64309);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90893);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 36383);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64310);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90894);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 36384);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64311);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 90895);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3265);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 14508);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14509);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 36385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 36386);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64313);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90896);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111182);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 64314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 36387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14510);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 14511);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 36388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14512);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36389);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 64316);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90897);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 111183);
													match s {
														[] => true,
														['к', s @ ..] => super::t22::n125652(s, out),
														_ => false,
													}
												}
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
							super::push(out, 36390);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 64317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14513);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14514);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 36391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14515);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 14516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36392);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64318);
									s.is_empty()
								}
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
			super::push(out, 3266);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14517);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 36393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14518);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36394);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 36395);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t22::n14593(s, out),
				['и', s @ ..] => super::t22::n14594(s, out),
				['к', s @ ..] => super::t22::n14595(s, out),
				['н', s @ ..] => {
					super::push(out, 14522);
					match s {
						[] => true,
						['и', s @ ..] => super::t22::n36743(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t22::n14597(s, out),
				['р', s @ ..] => super::t22::n14598(s, out),
				['т', s @ ..] => super::t22::n14599(s, out),
				['у', s @ ..] => super::t22::n14600(s, out),
				['ь', s @ ..] => super::t22::n14601(s, out),
				['ю', s @ ..] => super::t22::n14602(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t22::n3278(s, out),
		['д', s @ ..] => super::t22::n3279(s, out),
		['е', s @ ..] => super::t22::n3280(s, out),
		['ж', s @ ..] => super::t22::n3281(s, out),
		['з', s @ ..] => super::t22::n3282(s, out),
		['и', s @ ..] => super::t22::n3283(s, out),
		['к', s @ ..] => super::t22::n3284(s, out),
		['л', s @ ..] => super::t22::n3285(s, out),
		['м', s @ ..] => super::t22::n3286(s, out),
		['н', s @ ..] => super::t22::n3287(s, out),
		['о', s @ ..] => super::t22::n3288(s, out),
		['п', s @ ..] => super::t22::n3289(s, out),
		['р', s @ ..] => super::t22::n3290(s, out),
		['с', s @ ..] => super::t22::n3291(s, out),
		['т', s @ ..] => super::t22::n3292(s, out),
		['у', s @ ..] => super::t22::n3293(s, out),
		['ф', s @ ..] => super::t22::n3294(s, out),
		['х', s @ ..] => super::t22::n3295(s, out),
		['ц', s @ ..] => super::t22::n3296(s, out),
		['ч', s @ ..] => super::t22::n3297(s, out),
		['ш', s @ ..] => super::t22::n3298(s, out),
		['щ', s @ ..] => super::t22::n3299(s, out),
		['ю', s @ ..] => super::t22::n3300(s, out),
		_ => false,
	}
}

/// The word мелозавр.
pub(crate) fn n112401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111223);
	s.is_empty()
}

/// Words ending -гр.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3303);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3304);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3305);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 3306);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14698);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 36646);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14699);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -др.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3308);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14700);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14701);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3309);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 14703);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 36650);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64516);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 91003);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3310);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3311);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3312);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14706);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 36651);
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64517);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 91004);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 64518);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 91005);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111224);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t22::n125672(s, out),
														_ => false,
													}
												}
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
							super::push(out, 36652);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 64519);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 91006);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111225);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'ф', s @ ..] => {
							super::push(out, 36653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'к', s @ ..] => {
							super::push(out, 36654);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 64521);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 91007);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 111226);
													s.is_empty()
												}
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
							super::push(out, 36655);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14707);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36656);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 64523);
									s.is_empty()
								}
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
			super::push(out, 3313);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3314);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 14709);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36657);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 64524);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 91008);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 111227);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3315);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14710);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 36658);
							match s {
								[] => true,
								['е', 'д', 'о', 'д', '-', s @ ..] => {
									super::push(out, 64525);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 91009);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111228);
													match s {
														[] => true,
														['о', s @ ..] => super::t22::n125673(s, out),
														_ => false,
													}
												}
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
							super::push(out, 36659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14711);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 36660);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 36661);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 64527);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 91010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 36662);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 64528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ер.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3316);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14713);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 14714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 14715);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 14716);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 36664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14717);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 36665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 14718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36666);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 36667);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 64530);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 91011);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 111229);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3318);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14719);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 36668);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 64531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 36669);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64532);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 91012);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 36670);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64533);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 91013);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 36671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64534);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 36672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 14720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36673);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64535);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 91014);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 36674);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 64536);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 91015);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 36675);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 64537);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 91016);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 64538);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 91017);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 64539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 36676);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 64540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14721);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36678);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 36679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 64541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 36680);
							match s {
								[] => true,
								['а', s @ ..] => super::t22::n65206(s, out),
								_ => false,
							}
						}
						['ш', s @ ..] => super::t22::n37026(s, out),
						_ => false,
					}
				}
				['й', s @ ..] => super::t22::n14800(s, out),
				['л', s @ ..] => super::t22::n14801(s, out),
				['м', s @ ..] => super::t22::n14802(s, out),
				['о', s @ ..] => super::t22::n14803(s, out),
				['р', s @ ..] => super::t22::n14804(s, out),
				['с', s @ ..] => super::t22::n14805(s, out),
				['у', s @ ..] => super::t22::n14806(s, out),
				['ь', s @ ..] => super::t22::n14807(s, out),
				['ю', s @ ..] => super::t22::n14808(s, out),
				['я', s @ ..] => super::t22::n14809(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t22::n3330(s, out),
		['г', s @ ..] => super::t22::n3331(s, out),
		['д', s @ ..] => super::t22::n3332(s, out),
		['е', s @ ..] => super::t22::n3333(s, out),
		['ж', s @ ..] => super::t22::n3334(s, out),
		['з', s @ ..] => super::t22::n3335(s, out),
		['и', s @ ..] => super::t22::n3336(s, out),
		['й', s @ ..] => super::t22::n3337(s, out),
		['к', s @ ..] => super::t22::n3338(s, out),
		['л', s @ ..] => super::t22::n3339(s, out),
		['м', s @ ..] => super::t22::n3340(s, out),
		['н', s @ ..] => super::t22::n3341(s, out),
		['п', s @ ..] => super::t22::n3342(s, out),
		['р', s @ ..] => super::t22::n3343(s, out),
		['с', s @ ..] => super::t22::n3344(s, out),
		['т', s @ ..] => super::t23::n3345(s, out),
		['у', s @ ..] => super::t23::n3346(s, out),
		['ф', s @ ..] => super::t23::n3347(s, out),
		['х', s @ ..] => super::t23::n3348(s, out),
		['ц', s @ ..] => super::t23::n3349(s, out),
		['ч', s @ ..] => super::t23::n3350(s, out),
		['ш', s @ ..] => super::t23::n3351(s, out),
		['щ', s @ ..] => super::t23::n3352(s, out),
		['ь', s @ ..] => super::t23::n3353(s, out),
		['ю', s @ ..] => super::t23::n3354(s, out),
		_ => false,
	}
}

/// Words ending -ир.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 340);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3344);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3345);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 14977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 14978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14979);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 14980);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 14981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14982);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 37386);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 65538);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 91812);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3346);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 14983);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 37387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 14984);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 37388);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 65539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14985);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14986);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37390);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 65540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 14987);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37391);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 65541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 37392);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 65542);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14988);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37393);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 65543);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 91813);
											s.is_empty()
										}
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
					super::push(out, 14989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14990);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37395);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 65544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 65545);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 65546);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 91814);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111749);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14991);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 14992);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 37396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', '\u{1}', s @ ..] => {
					super::push(out, 14993);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 14994);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37397);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 65547);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 91815);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 111750);
													match s {
														[] => true,
														['н', s @ ..] => super::t23::n125959(s, out),
														_ => false,
													}
												}
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
							super::push(out, 37398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14996);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37399);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 65548);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 91816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 37400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 14997);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37401);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 65549);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 14998);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37402);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 65550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 37403);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 65551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14999);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 37404);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 65552);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 91817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t23::n3360(s, out),
		['ж', s @ ..] => super::t23::n3361(s, out),
		['з', s @ ..] => super::t23::n3362(s, out),
		['к', s @ ..] => super::t23::n3363(s, out),
		['л', s @ ..] => super::t23::n3364(s, out),
		['м', s @ ..] => super::t23::n3365(s, out),
		['н', s @ ..] => super::t23::n3366(s, out),
		['о', s @ ..] => {
			super::push(out, 3356);
			s.is_empty()
		}
		['п', s @ ..] => super::t23::n3368(s, out),
		['р', s @ ..] => {
			super::push(out, 3358);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15050);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t23::n3370(s, out),
		['т', s @ ..] => super::t23::n3371(s, out),
		['у', s @ ..] => {
			super::push(out, 3361);
			match s {
				[] => true,
				['ж', s @ ..] => super::t23::n15147(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t23::n3373(s, out),
		['х', s @ ..] => super::t23::n3374(s, out),
		['ц', s @ ..] => super::t23::n3375(s, out),
		['ч', s @ ..] => super::t23::n3376(s, out),
		['ш', s @ ..] => super::t23::n3377(s, out),
		_ => false,
	}
}

/// Words ending -йр.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3367);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15089);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 37516);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 65637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 15090);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3368);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3369);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 3370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15093);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 15094);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 37518);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 37519);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 65638);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 91871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кр.
pub(crate) fn n343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3371);
			s.is_empty()
		}
		['е', 'в', 'с', '\u{1}', s @ ..] => {
			super::push(out, 3372);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3373);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 15095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3374);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3376);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нр.
pub(crate) fn n345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3378);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 15100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3379);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15101);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 37520);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 65639);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 91872);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3380);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15102);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 37521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 65640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 37522);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 37523);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 37524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 65641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15103);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15104);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15106);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15107);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37527);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 65642);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 91873);
											s.is_empty()
										}
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
					super::push(out, 15108);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 37528);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 65643);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 91874);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 111774);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 65644);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 91875);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 111775);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15109);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37529);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 65645);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 91876);
											s.is_empty()
										}
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
					super::push(out, 15110);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 37530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15111);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15112);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 65646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 15113);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 37532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 65647);
									s.is_empty()
								}
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
			super::push(out, 3381);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15114);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15115);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 37533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37534);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 37535);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 65648);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 91877);
											s.is_empty()
										}
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
					super::push(out, 15117);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 37536);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 65649);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 91878);
											s.is_empty()
										}
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
					super::push(out, 15118);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 37537);
							match s {
								[] => true,
								['з', s @ ..] => super::t23::n66326(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t23::n15202(s, out),
				['т', s @ ..] => super::t23::n15203(s, out),
				['я', s @ ..] => super::t23::n15204(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t23::n3393(s, out),
		['д', s @ ..] => super::t23::n3394(s, out),
		['е', s @ ..] => super::t23::n3395(s, out),
		['ж', s @ ..] => super::t23::n3396(s, out),
		['з', s @ ..] => super::t23::n3397(s, out),
		['и', s @ ..] => super::t23::n3398(s, out),
		['й', s @ ..] => super::t23::n3399(s, out),
		['к', s @ ..] => super::t23::n3400(s, out),
		['л', s @ ..] => super::t23::n3401(s, out),
		['м', s @ ..] => super::t23::n3402(s, out),
		['н', s @ ..] => super::t23::n3403(s, out),
		['п', s @ ..] => super::t23::n3404(s, out),
		['р', s @ ..] => super::t23::n3405(s, out),
		['с', s @ ..] => super::t23::n3406(s, out),
		['т', s @ ..] => super::t23::n3407(s, out),
		['у', s @ ..] => super::t23::n3408(s, out),
		['ф', s @ ..] => super::t23::n3409(s, out),
		['х', s @ ..] => super::t23::n3410(s, out),
		['ц', s @ ..] => super::t23::n3411(s, out),
		['ш', s @ ..] => super::t23::n3412(s, out),
		['ь', s @ ..] => super::t23::n3413(s, out),
		_ => false,
	}
}

/// Words ending -пр.
pub(crate) fn n347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3403);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15269);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37830);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66028);
									s.is_empty()
								}
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
			super::push(out, 3404);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3405);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рр.
pub(crate) fn n348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 347);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3406);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3407);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3408);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15275);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 37832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ср.
pub(crate) fn n349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 348);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3409);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15277);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 37833);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66029);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92274);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 112097);
													match s {
														[] => true,
														['м', s @ ..] => super::t23::n126246(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тр.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3410);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15278);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 37834);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66030);
									match s {
										[] => true,
										['н', 'и', s @ ..] => {
											super::push(out, 92275);
											match s {
												[] => true,
												['к', '-', s @ ..] => {
													super::push(out, 112098);
													match s {
														[] => true,
														['б', s @ ..] => super::t23::n126247(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15279);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37835);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66031);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 92276);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 112099);
													match s {
														[] => true,
														['ч', s @ ..] => super::t23::n126248(s, out),
														_ => false,
													}
												}
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
							super::push(out, 37836);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 37837);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66033);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92277);
											s.is_empty()
										}
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
					super::push(out, 15280);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15281);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 37839);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66034);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92278);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112100);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15282);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15283);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 37840);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66035);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92279);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112101);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15284);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37841);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t23::n3422(s, out),
		['и', s @ ..] => {
			super::push(out, 3412);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15290);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37857);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15291);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 15292);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3413);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15294);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3414);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3415);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15296);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 37859);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66080);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92346);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 66081);
									s.is_empty()
								}
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
			super::push(out, 3416);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15297);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 37860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t23::n3428(s, out),
		['у', s @ ..] => {
			super::push(out, 3418);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15309);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37884);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66101);
									s.is_empty()
								}
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
			super::push(out, 3419);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15310);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 37885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 3420);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3421);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15312);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 37886);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66102);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92361);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ур.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3422);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15313);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 15314);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15315);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15316);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 37887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15317);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 37888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66103);
									s.is_empty()
								}
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
			super::push(out, 3423);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15318);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15319);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 15320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15321);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37890);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66104);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92362);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112172);
													s.is_empty()
												}
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
							super::push(out, 37891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15322);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37892);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66105);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92363);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 112173);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 66106);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92364);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 112174);
													match s {
														[] => true,
														['о', s @ ..] => super::t23::n126294(s, out),
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
									super::push(out, 66107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 37893);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 37894);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66109);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 92365);
											s.is_empty()
										}
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
					super::push(out, 15323);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 15324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15325);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15326);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37895);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'а', 'б', '\u{1}', s @ ..] => {
							super::push(out, 37896);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 37897);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66111);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 92366);
											s.is_empty()
										}
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
					super::push(out, 15327);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 15328);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 37899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15329);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 37900);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15330);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37901);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 37902);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 15331);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 37903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 15332);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37904);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66115);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 92367);
											s.is_empty()
										}
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
					super::push(out, 15333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3425);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15334);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37905);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 66116);
									match s {
										[] => true,
										['р', s @ ..] => super::t23::n93364(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t23::n38267(s, out),
						['р', s @ ..] => super::t23::n38268(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t23::n15418(s, out),
				['н', s @ ..] => super::t23::n15419(s, out),
				['о', s @ ..] => super::t23::n15420(s, out),
				['у', s @ ..] => super::t23::n15421(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t23::n3437(s, out),
		['з', s @ ..] => super::t23::n3438(s, out),
		['к', s @ ..] => super::t23::n3439(s, out),
		['л', s @ ..] => super::t23::n3440(s, out),
		['м', s @ ..] => super::t23::n3441(s, out),
		['н', s @ ..] => super::t23::n3442(s, out),
		['о', s @ ..] => super::t23::n3443(s, out),
		['п', s @ ..] => super::t23::n3444(s, out),
		['с', s @ ..] => super::t23::n3445(s, out),
		['т', s @ ..] => super::t23::n3446(s, out),
		['ф', s @ ..] => super::t23::n3447(s, out),
		['х', s @ ..] => super::t23::n3448(s, out),
		['ч', s @ ..] => super::t23::n3449(s, out),
		['ш', s @ ..] => super::t23::n3450(s, out),
		['щ', s @ ..] => super::t23::n3451(s, out),
		['я', s @ ..] => super::t23::n3452(s, out),
		_ => false,
	}
}

/// Words ending -фр.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3442);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3443);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 15402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3444);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3445);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хр.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3446);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3447);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15406);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыр.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3448);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3450);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15408);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 37981);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66167);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92385);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3451);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37982);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 37983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37984);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 66168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3452);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15411);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 37985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3453);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15412);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 15413);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 37987);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 66169);
									s.is_empty()
								}
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
			super::push(out, 3454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15414);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15415);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37988);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66170);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 92386);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3455);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15416);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 37989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', 'к', '\u{1}', s @ ..] => {
			super::push(out, 3456);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 3457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15417);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 37990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эр.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3458);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15418);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37991);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66171);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92387);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15419);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37992);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66172);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 92388);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3460);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 15420);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 37993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3461);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15421);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 37994);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 66173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 15422);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37995);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 37996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3462);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 3463);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 3464);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15423);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 37997);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66174);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92389);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112184);
													match s {
														[] => true,
														['р', s @ ..] => super::t23::n126300(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15424);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 37998);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66175);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92390);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 112185);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 3465);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 15426);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 37999);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юр.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3466);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3467);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3468);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15429);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3469);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 3470);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15430);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66177);
									s.is_empty()
								}
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
			super::push(out, 3471);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 15431);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15432);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38003);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 66178);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92391);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112186);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t23::n126301(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яр.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3472);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 3473);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15433);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38004);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15434);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 38005);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66180);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92392);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112187);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15435);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38006);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 38007);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66182);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 66183);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92393);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 112188);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15436);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38008);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 66184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15437);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38009);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 66185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 38010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15438);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38011);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 66186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15439);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38012);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 66187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 38013);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38014);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15440);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15441);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38016);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 66190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15442);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38017);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 66191);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 92394);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112189);
													match s {
														[] => true,
														['ь', s @ ..] => super::t23::n126302(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15443);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38018);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 38019);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66192);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92395);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 66193);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 38020);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 38021);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 38022);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66195);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 92397);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112190);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'н', s @ ..] => {
							super::push(out, 38023);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66196);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92398);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112191);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3475);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 15444);
					match s {
						[] => true,
						['е', s @ ..] => super::t23::n38385(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t23::n3487(s, out),
		['ь', s @ ..] => super::t23::n3488(s, out),
		_ => false,
	}
}

/// Words ending -ас.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3478);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3479);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15452);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15454);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 15455);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38032);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66203);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92403);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 38033);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66204);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92404);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112194);
													match s {
														[] => true,
														['н', s @ ..] => super::t23::n126303(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15456);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 38034);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 38035);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66206);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 66207);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 92405);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 38036);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66208);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92406);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 38037);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 38038);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 38039);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 15457);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38040);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15458);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38041);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 66211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15459);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38042);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66212);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92407);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 38043);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66213);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 92408);
											s.is_empty()
										}
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
					super::push(out, 15460);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38044);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15461);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38045);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15462);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38046);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 38047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15463);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38048);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66216);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 92409);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38049);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66217);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92410);
											s.is_empty()
										}
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
					super::push(out, 15464);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38050);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66218);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92411);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 38051);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 66219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92412);
											s.is_empty()
										}
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
					super::push(out, 15465);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 38052);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66220);
									match s {
										[] => true,
										['н', s @ ..] => super::t23::n93409(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t23::n38414(s, out),
						['р', s @ ..] => super::t23::n38415(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t23::n3492(s, out),
		['г', s @ ..] => super::t23::n3493(s, out),
		['д', s @ ..] => super::t23::n3494(s, out),
		['е', s @ ..] => super::t23::n3495(s, out),
		['ж', s @ ..] => super::t23::n3496(s, out),
		['з', s @ ..] => super::t23::n3497(s, out),
		['и', s @ ..] => super::t23::n3498(s, out),
		['й', s @ ..] => super::t23::n3499(s, out),
		['к', s @ ..] => super::t23::n3500(s, out),
		['л', s @ ..] => super::t23::n3501(s, out),
		['м', s @ ..] => super::t23::n3502(s, out),
		['н', s @ ..] => super::t23::n3503(s, out),
		['о', s @ ..] => super::t23::n3504(s, out),
		['п', s @ ..] => super::t23::n3505(s, out),
		['р', s @ ..] => super::t23::n3506(s, out),
		['с', s @ ..] => super::t23::n3507(s, out),
		['т', s @ ..] => super::t23::n3508(s, out),
		['у', s @ ..] => super::t23::n3509(s, out),
		['ф', s @ ..] => super::t23::n3510(s, out),
		['х', s @ ..] => super::t23::n3511(s, out),
		['ч', s @ ..] => super::t23::n3512(s, out),
		_ => false,
	}
}

/// Words ending -вид-хедс.
pub(crate) fn n113424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112222);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 125022);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ес.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15581);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15582);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 38214);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 66347);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 92477);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66348);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 66349);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92478);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 112223);
													match s {
														[] => true,
														['а', s @ ..] => super::t23::n126321(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38216);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15584);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38217);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66351);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92479);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 112224);
													s.is_empty()
												}
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
							super::push(out, 38218);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66352);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 92480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 38219);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 66353);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92481);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3515);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15585);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 15586);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38220);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 38221);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66355);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 92482);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 66356);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92483);
											s.is_empty()
										}
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
					super::push(out, 15587);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15588);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15589);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 38224);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 38225);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66358);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92484);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 112225);
													match s {
														[] => true,
														['с', s @ ..] => super::t23::n126322(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38226);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66359);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 92485);
											s.is_empty()
										}
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
					super::push(out, 15590);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15591);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38229);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 38230);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66360);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92486);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112226);
													s.is_empty()
												}
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
							super::push(out, 38231);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 38232);
							match s {
								[] => true,
								['г', s @ ..] => super::t23::n67056(s, out),
								_ => false,
							}
						}
						['л', s @ ..] => super::t23::n38596(s, out),
						['н', s @ ..] => super::t23::n38597(s, out),
						['р', s @ ..] => super::t23::n38598(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t23::n15678(s, out),
				['т', s @ ..] => super::t23::n15679(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t23::n3527(s, out),
		['д', s @ ..] => super::t23::n3528(s, out),
		['е', s @ ..] => super::t23::n3529(s, out),
		['з', s @ ..] => super::t23::n3530(s, out),
		['и', s @ ..] => super::t23::n3531(s, out),
		['к', s @ ..] => super::t23::n3532(s, out),
		['л', s @ ..] => super::t23::n3533(s, out),
		['м', s @ ..] => super::t23::n3534(s, out),
		['н', s @ ..] => super::t23::n3535(s, out),
		['п', s @ ..] => super::t23::n3536(s, out),
		['р', s @ ..] => super::t23::n3537(s, out),
		['с', s @ ..] => super::t23::n3538(s, out),
		['т', s @ ..] => super::t24::n3539(s, out),
		['у', s @ ..] => super::t24::n3540(s, out),
		['ф', s @ ..] => super::t24::n3541(s, out),
		['ц', s @ ..] => super::t24::n3542(s, out),
		['ч', s @ ..] => super::t24::n3543(s, out),
		['ш', s @ ..] => super::t24::n3544(s, out),
		['ь', s @ ..] => super::t24::n3545(s, out),
		_ => false,
	}
}

/// Words ending -ис.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3535);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 15661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3536);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15662);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 38323);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 66448);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92541);
											s.is_empty()
										}
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
					super::push(out, 15663);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 38324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'и', '\u{1}', s @ ..] => {
					super::push(out, 15664);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 15665);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 38325);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66449);
									s.is_empty()
								}
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
			super::push(out, 3537);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15666);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 38327);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 66451);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 92542);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 112261);
													match s {
														[] => true,
														['м', s @ ..] => super::t24::n126336(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15667);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15668);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 38329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15669);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38330);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 92543);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 112262);
													match s {
														[] => true,
														['и', s @ ..] => super::t24::n126337(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 92544);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3538);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 15670);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38331);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 66453);
									s.is_empty()
								}
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
			super::push(out, 3539);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15671);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 38332);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 38333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 38334);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66456);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92545);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112263);
													match s {
														[] => true,
														['г', s @ ..] => super::t24::n126338(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 15672);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15673);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38337);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 66457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15674);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 38338);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 15675);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38339);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 66459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 15676);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38340);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66460);
									s.is_empty()
								}
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
			super::push(out, 3540);
			match s {
				[] => true,
				['е', s @ ..] => super::t24::n15762(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t24::n3552(s, out),
		['к', s @ ..] => super::t24::n3553(s, out),
		['л', s @ ..] => super::t24::n3554(s, out),
		['м', s @ ..] => super::t24::n3555(s, out),
		['н', s @ ..] => super::t24::n3556(s, out),
		['о', s @ ..] => {
			super::push(out, 3546);
			s.is_empty()
		}
		['п', s @ ..] => super::t24::n3558(s, out),
		['р', s @ ..] => super::t24::n3559(s, out),
		['с', s @ ..] => super::t24::n3560(s, out),
		['т', s @ ..] => super::t24::n3561(s, out),
		['у', s @ ..] => {
			super::push(out, 3551);
			match s {
				[] => true,
				['л', s @ ..] => super::t24::n15832(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t24::n3563(s, out),
		['х', s @ ..] => super::t24::n3564(s, out),
		['ц', s @ ..] => super::t24::n3565(s, out),
		['ш', s @ ..] => super::t24::n3566(s, out),
		['ю', s @ ..] => super::t24::n3567(s, out),
		_ => false,
	}
}

/// Words ending -ом-прайс.
pub(crate) fn n113511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112308);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 125073);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кс.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3560);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15755);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38459);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 15756);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 15757);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 15758);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15759);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 66566);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92611);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 66567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 38461);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 66568);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92612);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 38462);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66569);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92613);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 38463);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66570);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 92614);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 112309);
													match s {
														[] => true,
														['р', s @ ..] => super::t24::n126372(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38464);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 38465);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66571);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92615);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 38466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 38467);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 66573);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 92616);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112310);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15761);
					match s {
						[] => true,
						['а', 'п', 'о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 66574);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92617);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 66575);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92618);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112311);
													match s {
														[] => true,
														['к', s @ ..] => super::t24::n126373(s, out),
														_ => false,
													}
												}
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
								['р', s @ ..] => {
									super::push(out, 66576);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92619);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112312);
													match s {
														[] => true,
														['к', s @ ..] => super::t24::n126374(s, out),
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
									super::push(out, 66577);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92620);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 112313);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t24::n15848(s, out),
				['р', s @ ..] => super::t24::n15849(s, out),
				['с', s @ ..] => super::t24::n15850(s, out),
				['т', s @ ..] => {
					super::push(out, 15765);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 38474);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t24::n15852(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t24::n3572(s, out),
		['и', s @ ..] => super::t24::n3573(s, out),
		['н', s @ ..] => super::t24::n3574(s, out),
		['о', s @ ..] => super::t24::n3575(s, out),
		['р', s @ ..] => super::t24::n3576(s, out),
		['у', s @ ..] => super::t24::n3577(s, out),
		['ю', s @ ..] => super::t24::n3578(s, out),
		['я', s @ ..] => super::t24::n3579(s, out),
		_ => false,
	}
}

/// Words ending -пелс.
pub(crate) fn n15892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15805);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38548);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 66669);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 92685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телс.
pub(crate) fn n15893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15806);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 38549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -илс.
pub(crate) fn n3582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3571);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15807);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38550);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66670);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92686);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 112352);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125097);
													match s {
														[] => true,
														['д', s @ ..] => super::t24::n134134(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38551);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92687);
									s.is_empty()
								}
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
			super::push(out, 15808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллс.
pub(crate) fn n3583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3572);
	s.is_empty()
}

/// Words ending -улс.
pub(crate) fn n3584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3573);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 15809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элс.
pub(crate) fn n3585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3574);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 15810);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 38553);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66672);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92688);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112353);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125098);
													match s {
														[] => true,
														['с', s @ ..] => super::t24::n134135(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мс.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3575);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15811);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38554);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3576);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 15813);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 38555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3577);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 15814);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 15815);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 38556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66674);
									s.is_empty()
								}
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
			super::push(out, 3578);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15816);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 38557);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 38558);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66676);
									s.is_empty()
								}
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
			super::push(out, 3579);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 15817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3580);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 15818);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 15819);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38560);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66677);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 92689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нс.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15820);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38561);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66678);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92690);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 112354);
													match s {
														[] => true,
														['а', s @ ..] => super::t24::n126397(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15821);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 15822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 15823);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38563);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 38564);
							match s {
								[] => true,
								['о', 'ф', s @ ..] => {
									super::push(out, 66679);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 92691);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112355);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t24::n126398(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15824);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 15825);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 15826);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38565);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66680);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92692);
											s.is_empty()
										}
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
					super::push(out, 15827);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15828);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66681);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 66682);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92693);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112356);
													match s {
														[] => true,
														['п', s @ ..] => super::t24::n126399(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38567);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66683);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 92694);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 112357);
													match s {
														[] => true,
														['е', s @ ..] => super::t24::n126400(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15829);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15830);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38569);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 66684);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92695);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38570);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 38571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66685);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 66686);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92696);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 112358);
													match s {
														[] => true,
														['-', s @ ..] => super::t24::n126401(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 15831);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15832);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38572);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 66687);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92697);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112359);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 15834);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 15835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t24::n3593(s, out),
		['и', s @ ..] => {
			super::push(out, 3583);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 15850);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38594);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66700);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 92710);
											s.is_empty()
										}
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
					super::push(out, 15851);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15852);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 38596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t24::n67397(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t24::n3595(s, out),
		['р', s @ ..] => super::t24::n3596(s, out),
		['у', s @ ..] => super::t24::n3597(s, out),
		['э', s @ ..] => super::t24::n3598(s, out),
		['я', s @ ..] => super::t24::n3599(s, out),
		_ => false,
	}
}

/// Words ending -ос.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3589);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3590);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 15858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15859);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15860);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38601);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66703);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 92712);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112365);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38602);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38603);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92713);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3591);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 15862);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 38604);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66705);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 92714);
											s.is_empty()
										}
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
					super::push(out, 15863);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38605);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15864);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38606);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66707);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 92715);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112366);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15865);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38607);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 15866);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38608);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66709);
									s.is_empty()
								}
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
			super::push(out, 3592);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15867);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 38609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15868);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 38610);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66710);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92716);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 112367);
													match s {
														[] => true,
														['о', s @ ..] => super::t24::n126404(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3593);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15869);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 15870);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 15871);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15872);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3594);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15873);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38615);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66711);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92717);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 112368);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => super::t24::n38988(s, out),
						['н', s @ ..] => super::t24::n38989(s, out),
						['п', s @ ..] => super::t24::n38990(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t24::n15961(s, out),
				['и', s @ ..] => super::t24::n15962(s, out),
				['й', s @ ..] => super::t24::n15963(s, out),
				['л', s @ ..] => super::t24::n15964(s, out),
				['н', s @ ..] => super::t24::n15965(s, out),
				['о', s @ ..] => super::t24::n15966(s, out),
				['у', s @ ..] => super::t24::n15967(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t24::n3606(s, out),
		['к', s @ ..] => super::t24::n3607(s, out),
		['л', s @ ..] => super::t24::n3608(s, out),
		['м', s @ ..] => super::t24::n3609(s, out),
		['н', s @ ..] => super::t24::n3610(s, out),
		['п', s @ ..] => super::t24::n3611(s, out),
		['р', s @ ..] => super::t24::n3612(s, out),
		['с', s @ ..] => super::t24::n3613(s, out),
		['т', s @ ..] => super::t24::n3614(s, out),
		['ф', s @ ..] => super::t24::n3615(s, out),
		['х', s @ ..] => super::t24::n3616(s, out),
		['ч', s @ ..] => super::t24::n3617(s, out),
		['ь', s @ ..] => super::t24::n3618(s, out),
		_ => false,
	}
}

/// Words ending -пс.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3608);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 15979);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15980);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38797);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15981);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38798);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 38799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 15982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3609);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 15983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38800);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 38801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 15984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38802);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 38803);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66891);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92829);
											s.is_empty()
										}
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
					super::push(out, 15985);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38804);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 66892);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92830);
											s.is_empty()
										}
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
					super::push(out, 15986);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 15987);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 15988);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38806);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66893);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 66894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 38807);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66895);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92831);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 112421);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3610);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 15989);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 15990);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38808);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 66896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 38809);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 66897);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92832);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112422);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 15991);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 38810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 15992);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 15993);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 38811);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66898);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92833);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 112423);
													match s {
														[] => true,
														['-', s @ ..] => super::t24::n126431(s, out),
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
									super::push(out, 66899);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 92834);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 112424);
													s.is_empty()
												}
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
							super::push(out, 38812);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 66900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92835);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3611);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 15994);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 38813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 15995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38814);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 38815);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 15996);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 15997);
					s.is_empty()
				}
				['т', s @ ..] => super::t24::n16085(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3612);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 15999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t24::n3624(s, out),
		_ => false,
	}
}

/// Words ending -рс.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3614);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 16004);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38822);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66905);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92838);
											s.is_empty()
										}
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
					super::push(out, 16005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16006);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 16008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16009);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 16010);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38825);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 66906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16011);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 38826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66907);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 66908);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92839);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 112426);
													match s {
														[] => true,
														['о', s @ ..] => super::t24::n126432(s, out),
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
									super::push(out, 66909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 38827);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 38828);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66911);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 92840);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 112427);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16012);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38829);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 66912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16013);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38830);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 38831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16014);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 38832);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66913);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92841);
											s.is_empty()
										}
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
					super::push(out, 16015);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16016);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38833);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 66914);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 92842);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112428);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38834);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 38835);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 38836);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 66915);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 92843);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 112429);
													match s {
														[] => true,
														['и', s @ ..] => super::t24::n126433(s, out),
														_ => false,
													}
												}
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
							super::push(out, 38837);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66916);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 92844);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 38838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16018);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16019);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 16020);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 38839);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 66917);
									s.is_empty()
								}
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
			super::push(out, 3616);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3617);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16023);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16024);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16025);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16026);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16027);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 16028);
					match s {
						[] => true,
						['т', s @ ..] => super::t24::n39213(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t24::n3629(s, out),
		['э', s @ ..] => super::t24::n3630(s, out),
		_ => false,
	}
}

/// Words ending -сс.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3620);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16034);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 38851);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66930);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92851);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 112436);
													s.is_empty()
												}
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
							super::push(out, 38852);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92852);
											s.is_empty()
										}
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
					super::push(out, 16035);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16036);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16037);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 38854);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 38855);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 66932);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 92853);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 112437);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t24::n126437(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3621);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16038);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16039);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3622);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16041);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16042);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38857);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 38858);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66934);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 92854);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 112438);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 66935);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92855);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 112439);
													s.is_empty()
												}
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
							super::push(out, 38859);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66936);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 92856);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 112440);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16044);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 16045);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16046);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16047);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38860);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66937);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 92857);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3623);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16048);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16049);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16050);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38861);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66938);
									s.is_empty()
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
						['\u{1}', s @ ..] => {
							super::push(out, 38862);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 38863);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 38864);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 66939);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92858);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 112441);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t24::n39237(s, out),
						['м', s @ ..] => {
							super::push(out, 38866);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 66942);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 92861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 38867);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 66943);
									s.is_empty()
								}
								['л', s @ ..] => super::t24::n67640(s, out),
								['н', s @ ..] => super::t24::n67641(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t24::n3635(s, out),
		_ => false,
	}
}

/// Words ending -тс.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3625);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16055);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38871);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 66948);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92864);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 112445);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3626);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16057);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 38873);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66949);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92865);
											s.is_empty()
										}
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
					super::push(out, 16058);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 38874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3627);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16059);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 38875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3628);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16060);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 38876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3629);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16062);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 38877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3630);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16063);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38878);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 66950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ус.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3631);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3632);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38879);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 38880);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 66951);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 92866);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112446);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16065);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 38881);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 66952);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92867);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 112447);
													match s {
														[] => true,
														['н', s @ ..] => super::t24::n126440(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16066);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 38882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 38883);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 66954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 16067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 38884);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 38885);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 38886);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 38887);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 38888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 16068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3633);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16069);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 38889);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 66955);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92868);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112448);
													s.is_empty()
												}
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
							super::push(out, 38890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 66956);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 66957);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92869);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112449);
													s.is_empty()
												}
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
							super::push(out, 38891);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 66958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16070);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38892);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 66959);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92870);
											s.is_empty()
										}
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
					super::push(out, 16071);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 38893);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 66960);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92871);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112450);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16072);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 38894);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 66961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16073);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 38895);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 66962);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92872);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112451);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16074);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 38896);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 66963);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 66964);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92873);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t24::n39269(s, out),
						['т', s @ ..] => super::t24::n39270(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t24::n16163(s, out),
				['у', s @ ..] => super::t24::n16164(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t24::n3645(s, out),
		['г', s @ ..] => super::t24::n3646(s, out),
		['д', s @ ..] => super::t24::n3647(s, out),
		['е', s @ ..] => super::t24::n3648(s, out),
		['ж', s @ ..] => super::t24::n3649(s, out),
		['з', s @ ..] => super::t24::n3650(s, out),
		['и', s @ ..] => super::t24::n3651(s, out),
		['к', s @ ..] => super::t24::n3652(s, out),
		['л', s @ ..] => super::t24::n3653(s, out),
		['м', s @ ..] => super::t24::n3654(s, out),
		['н', s @ ..] => super::t24::n3655(s, out),
		['о', s @ ..] => super::t24::n3656(s, out),
		['п', s @ ..] => super::t24::n3657(s, out),
		['р', s @ ..] => super::t24::n3658(s, out),
		['с', s @ ..] => super::t24::n3659(s, out),
		['т', s @ ..] => super::t24::n3660(s, out),
		['ф', s @ ..] => super::t24::n3661(s, out),
		['х', s @ ..] => super::t24::n3662(s, out),
		_ => false,
	}
}

/// Words ending -хс.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 3652);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16180);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39071);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 67112);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92960);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыс.
pub(crate) fn n377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 376);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3653);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16181);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16182);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39072);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67113);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92961);
											s.is_empty()
										}
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
					super::push(out, 16183);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 3654);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 16184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 3655);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39075);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 67114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 16186);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39076);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 67115);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

