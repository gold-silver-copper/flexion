use flexion::Path;

/// Words ending -пи.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1984);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9695);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 9696);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28668);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 56029);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84005);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 106301);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1985);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9697);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 9698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1986);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9699);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9700);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28671);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56030);
									s.is_empty()
								}
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
			super::push(out, 1987);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9701);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28672);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56031);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 84006);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 106302);
													match s {
														[] => true,
														['м', s @ ..] => super::t16::n122751(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9702);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28673);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 56032);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 84007);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 106303);
													match s {
														[] => true,
														['э', s @ ..] => super::t16::n122752(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1988);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28675);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56033);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 84008);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1989);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9705);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1990);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9706);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9707);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 28678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1992);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9709);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28679);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9710);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 28680);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 28681);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 28682);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56035);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 84009);
											s.is_empty()
										}
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
					super::push(out, 9711);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28683);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 56036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9712);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28684);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28685);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 56037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 9713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1993);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9714);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28686);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 56038);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 84010);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 106304);
													match s {
														[] => true,
														['н', s @ ..] => super::t16::n122753(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ри.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9716);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28688);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 56039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9717);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28689);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56040);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 84011);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106305);
													s.is_empty()
												}
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
							super::push(out, 28690);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 56041);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84012);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106306);
													match s {
														[] => true,
														['т', s @ ..] => super::t16::n122754(s, out),
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
									super::push(out, 56042);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84013);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 106307);
													s.is_empty()
												}
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
							super::push(out, 28691);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56043);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84014);
											s.is_empty()
										}
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
					super::push(out, 9718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28692);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 56044);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84015);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 106308);
													match s {
														[] => true,
														['е', s @ ..] => super::t16::n122755(s, out),
														_ => false,
													}
												}
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
							super::push(out, 28693);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56045);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 84016);
											s.is_empty()
										}
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
					super::push(out, 9719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28694);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 28695);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 56046);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84017);
											s.is_empty()
										}
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
					super::push(out, 9720);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28696);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9721);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28697);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56048);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84018);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 56049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28698);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 56050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28699);
							match s {
								[] => true,
								['б', s @ ..] => super::t16::n56624(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t16::n28894(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9722);
					s.is_empty()
				}
				['м', s @ ..] => super::t16::n9750(s, out),
				['н', s @ ..] => super::t16::n9751(s, out),
				['п', s @ ..] => super::t16::n9752(s, out),
				['р', s @ ..] => super::t16::n9753(s, out),
				['с', s @ ..] => super::t16::n9754(s, out),
				['т', s @ ..] => super::t16::n9755(s, out),
				['ф', s @ ..] => super::t16::n9756(s, out),
				['ш', s @ ..] => super::t16::n9757(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t16::n2001(s, out),
		['г', s @ ..] => super::t16::n2002(s, out),
		['д', s @ ..] => super::t16::n2003(s, out),
		['е', s @ ..] => super::t16::n2004(s, out),
		['з', s @ ..] => super::t16::n2005(s, out),
		['и', s @ ..] => super::t16::n2006(s, out),
		['м', s @ ..] => super::t16::n2007(s, out),
		['н', s @ ..] => super::t16::n2008(s, out),
		['о', s @ ..] => super::t16::n2009(s, out),
		['п', s @ ..] => super::t16::n2010(s, out),
		['р', s @ ..] => super::t16::n2011(s, out),
		['т', s @ ..] => super::t16::n2012(s, out),
		['у', s @ ..] => super::t16::n2013(s, out),
		['ф', s @ ..] => super::t16::n2014(s, out),
		['х', s @ ..] => super::t16::n2015(s, out),
		['ы', s @ ..] => super::t16::n2016(s, out),
		['э', s @ ..] => super::t16::n2017(s, out),
		['ю', s @ ..] => super::t16::n2018(s, out),
		['я', s @ ..] => super::t16::n2019(s, out),
		_ => false,
	}
}

/// Words ending -си.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9803);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28806);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 56140);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 84093);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106352);
													match s {
														[] => true,
														['у', s @ ..] => super::t16::n122770(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2015);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 9805);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 9806);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28809);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 56141);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84094);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106353);
													match s {
														[] => true,
														['в', s @ ..] => super::t16::n122771(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9807);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28810);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56142);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84095);
											s.is_empty()
										}
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
					super::push(out, 9808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2016);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9809);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28811);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56143);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 84096);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106354);
													s.is_empty()
												}
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
							super::push(out, 28812);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56144);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 84097);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106355);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2017);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9810);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28813);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 56145);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84098);
											s.is_empty()
										}
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
					super::push(out, 9811);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28814);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 56146);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84099);
											s.is_empty()
										}
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
					super::push(out, 9812);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28815);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56147);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84100);
											s.is_empty()
										}
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
					super::push(out, 9813);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28816);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56148);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84101);
											s.is_empty()
										}
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
					super::push(out, 9814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2018);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9815);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28818);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28819);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56149);
									s.is_empty()
								}
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
			super::push(out, 2019);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9816);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28821);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 56151);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84102);
											match s {
												[] => true,
												['е', s @ ..] => super::t16::n107489(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 56152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t16::n9844(s, out),
				['о', s @ ..] => super::t16::n9845(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t16::n2026(s, out),
		['м', s @ ..] => super::t16::n2027(s, out),
		['н', s @ ..] => super::t16::n2028(s, out),
		['о', s @ ..] => super::t16::n2029(s, out),
		['п', s @ ..] => super::t16::n2030(s, out),
		['р', s @ ..] => super::t16::n2031(s, out),
		['с', s @ ..] => super::t16::n2032(s, out),
		['т', s @ ..] => super::t16::n2033(s, out),
		['у', s @ ..] => super::t16::n2034(s, out),
		['ь', s @ ..] => super::t16::n2035(s, out),
		['ю', s @ ..] => super::t16::n2036(s, out),
		['я', s @ ..] => super::t16::n2037(s, out),
		_ => false,
	}
}

/// Words ending -ти.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2032);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9841);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 28850);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56170);
									s.is_empty()
								}
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
			super::push(out, 2033);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28851);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28852);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56171);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84109);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 106360);
													s.is_empty()
												}
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
							super::push(out, 28853);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56172);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84110);
											s.is_empty()
										}
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
					super::push(out, 9843);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28854);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 56173);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 84111);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106361);
													s.is_empty()
												}
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
							super::push(out, 28855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28856);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 56174);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84112);
											s.is_empty()
										}
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
					super::push(out, 9845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28857);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28858);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 56175);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84113);
											s.is_empty()
										}
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
					super::push(out, 9846);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28859);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 56176);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 84114);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 106362);
													match s {
														[] => true,
														['и', s @ ..] => super::t16::n122775(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9847);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9848);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28861);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28862);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 56177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28863);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 56178);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84115);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 106363);
													match s {
														[] => true,
														['а', s @ ..] => super::t16::n122776(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9849);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28864);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 56179);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 84116);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 106364);
													match s {
														[] => true,
														['г', s @ ..] => super::t16::n122777(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9851);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28866);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 56180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 9852);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28867);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56181);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 84117);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106365);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2034);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9853);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 28868);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 56182);
									s.is_empty()
								}
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
			super::push(out, 2035);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9854);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28869);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 56183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t16::n2042(s, out),
		['й', s @ ..] => {
			super::push(out, 2037);
			match s {
				[] => true,
				['а', s @ ..] => super::t16::n9892(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t16::n2044(s, out),
		['л', s @ ..] => super::t16::n2045(s, out),
		['м', s @ ..] => super::t16::n2046(s, out),
		['н', s @ ..] => super::t16::n2047(s, out),
		['о', s @ ..] => super::t16::n2048(s, out),
		['р', s @ ..] => super::t16::n2049(s, out),
		['с', s @ ..] => super::t16::n2050(s, out),
		['т', s @ ..] => super::t16::n2051(s, out),
		['у', s @ ..] => super::t16::n2052(s, out),
		['ф', s @ ..] => super::t16::n2053(s, out),
		['х', s @ ..] => super::t16::n2054(s, out),
		['ь', s @ ..] => super::t16::n2055(s, out),
		['я', s @ ..] => super::t16::n2056(s, out),
		_ => false,
	}
}

/// Words ending -уи.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9901);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фи.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2053);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 9903);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 28944);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56237);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 84157);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106395);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2054);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9904);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 9905);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9906);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28946);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56238);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 84158);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 106396);
													match s {
														[] => true,
														['в', s @ ..] => super::t16::n122795(s, out),
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
									super::push(out, 56239);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 84159);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106397);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2056);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 9907);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9908);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28947);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 56240);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84160);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хи.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9909);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28948);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 56241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 9910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9911);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28950);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56242);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 28951);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9912);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9913);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28953);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56244);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9914);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28954);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56245);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 84161);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2061);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9915);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28955);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 56246);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 84162);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106398);
													match s {
														[] => true,
														['б', s @ ..] => super::t16::n122796(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9916);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28956);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56247);
									s.is_empty()
								}
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
			super::push(out, 2062);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9917);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28957);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56248);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84163);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106399);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2063);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 9918);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9920);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28960);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 56249);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9921);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 28962);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56250);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 84165);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 106400);
													match s {
														[] => true,
														['л', s @ ..] => super::t16::n122797(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28963);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 56251);
									s.is_empty()
								}
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
			super::push(out, 2065);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 9923);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28964);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56252);
									s.is_empty()
								}
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
			super::push(out, 2066);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9924);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28965);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 56253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ци.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9925);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28966);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56254);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84166);
											match s {
												[] => true,
												['у', '\u{1}', s @ ..] => {
													super::push(out, 106401);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2068);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 9926);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 9927);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2070);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9928);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28969);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 56255);
									s.is_empty()
								}
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
			super::push(out, 2071);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9929);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28970);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28971);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56256);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 84167);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106402);
													match s {
														[] => true,
														['п', s @ ..] => super::t16::n122798(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чи.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2072);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 9930);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28972);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 56257);
									s.is_empty()
								}
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
			super::push(out, 2073);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28973);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28974);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 56258);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 84168);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106403);
													match s {
														[] => true,
														['м', s @ ..] => super::t16::n122799(s, out),
														_ => false,
													}
												}
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
							super::push(out, 28975);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56259);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 84169);
											s.is_empty()
										}
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
					super::push(out, 9932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9933);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28977);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 56260);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 84170);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106404);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9934);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28978);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9935);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28979);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 56262);
									s.is_empty()
								}
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
			super::push(out, 2074);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9936);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28980);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56263);
									s.is_empty()
								}
								['ц', 'н', s @ ..] => {
									super::push(out, 56264);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84171);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 106405);
													s.is_empty()
												}
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
							super::push(out, 28981);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56265);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84172);
											s.is_empty()
										}
										['ю', s @ ..] => {
											super::push(out, 84173);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 56266);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84174);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106406);
													match s {
														[] => true,
														['к', s @ ..] => super::t16::n122800(s, out),
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
									super::push(out, 56267);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84175);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106407);
													match s {
														[] => true,
														['а', s @ ..] => super::t16::n122801(s, out),
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
									super::push(out, 56268);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84176);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106408);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 56269);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 84177);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 106409);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 56270);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 84178);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 106410);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9937);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 9938);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28982);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56271);
									match s {
										[] => true,
										['в', s @ ..] => super::t16::n85071(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t16::n2081(s, out),
		['н', s @ ..] => super::t16::n2082(s, out),
		['о', s @ ..] => super::t16::n2083(s, out),
		['р', s @ ..] => super::t16::n2084(s, out),
		['с', s @ ..] => super::t16::n2085(s, out),
		['т', s @ ..] => super::t16::n2086(s, out),
		['у', s @ ..] => super::t16::n2087(s, out),
		['ч', s @ ..] => {
			super::push(out, 2082);
			match s {
				[] => true,
				['э', s @ ..] => super::t16::n9978(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t16::n2089(s, out),
		_ => false,
	}
}

/// Words ending -ши.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2084);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 9953);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28998);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 9954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2085);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9955);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 9956);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29000);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56285);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84186);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 106416);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9957);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29001);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84187);
											s.is_empty()
										}
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
					super::push(out, 9958);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29002);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 29003);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56288);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 84188);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9959);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 29004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2087);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2088);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9961);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29005);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56289);
									s.is_empty()
								}
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
			super::push(out, 2089);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9962);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9963);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 29007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9964);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 29008);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56290);
									s.is_empty()
								}
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
			super::push(out, 2090);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9965);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29009);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84189);
											s.is_empty()
										}
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
					super::push(out, 9966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2091);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9967);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2092);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9968);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29011);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56292);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 84190);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щи.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2093);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2094);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 9969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 29012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2095);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9970);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 29013);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 9971);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 29014);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 56294);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 84191);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 106417);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9972);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29015);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 56295);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84192);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 106418);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9973);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29016);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 29017);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56297);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9974);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29018);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 56298);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 84193);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 106419);
													match s {
														[] => true,
														['-', s @ ..] => super::t16::n122805(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9975);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 29019);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 56299);
									s.is_empty()
								}
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
			super::push(out, 2096);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9976);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29020);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56300);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 84194);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 106420);
													match s {
														[] => true,
														['с', s @ ..] => super::t16::n122806(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 9977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9978);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 29021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьи.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2098);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9979);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ай.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2099);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9980);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 29023);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 56301);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84195);
											s.is_empty()
										}
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
					super::push(out, 9981);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56302);
									s.is_empty()
								}
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
			super::push(out, 2100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29025);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 29026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 29027);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 9983);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29028);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29029);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 29030);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 56305);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84196);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106421);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 9984);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29031);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 56306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 9985);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29032);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 56307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9986);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29033);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 56308);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84197);
											s.is_empty()
										}
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
					super::push(out, 9987);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9988);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29035);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9989);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 9990);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 9991);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2101);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9992);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 29039);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29040);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56310);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84198);
											s.is_empty()
										}
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
					super::push(out, 9993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29041);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56311);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84199);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 106422);
													s.is_empty()
												}
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
							super::push(out, 29042);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56312);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 84200);
											s.is_empty()
										}
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
					super::push(out, 9994);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29043);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56313);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84201);
											s.is_empty()
										}
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
					super::push(out, 9995);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2102);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9996);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9997);
					match s {
						[] => true,
						['л', s @ ..] => super::t16::n29239(s, out),
						['с', s @ ..] => super::t16::n29240(s, out),
						['т', s @ ..] => super::t16::n29241(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t16::n10025(s, out),
				['л', s @ ..] => super::t16::n10026(s, out),
				['о', s @ ..] => super::t16::n10027(s, out),
				['у', s @ ..] => super::t16::n10028(s, out),
				['ф', s @ ..] => super::t16::n10029(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t16::n2109(s, out),
		['ж', s @ ..] => super::t16::n2110(s, out),
		['з', s @ ..] => super::t16::n2111(s, out),
		['к', s @ ..] => super::t16::n2112(s, out),
		['л', s @ ..] => super::t16::n2113(s, out),
		['м', s @ ..] => super::t16::n2114(s, out),
		['н', s @ ..] => super::t16::n2115(s, out),
		['п', s @ ..] => super::t16::n2116(s, out),
		['р', s @ ..] => super::t16::n2117(s, out),
		['с', s @ ..] => super::t16::n2118(s, out),
		['т', s @ ..] => super::t16::n2119(s, out),
		['ф', s @ ..] => super::t16::n2120(s, out),
		['х', s @ ..] => super::t16::n2121(s, out),
		['ц', s @ ..] => super::t16::n2122(s, out),
		['ч', s @ ..] => super::t16::n2123(s, out),
		['ш', s @ ..] => super::t16::n2124(s, out),
		_ => false,
	}
}

/// Words ending -ей.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2119);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 10072);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29148);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 56398);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 84249);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 106453);
													match s {
														[] => true,
														['х', s @ ..] => super::t16::n122820(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 10073);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29149);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56399);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84250);
											s.is_empty()
										}
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
					super::push(out, 10074);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29150);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 56400);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 84251);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 56401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29151);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 56402);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 84252);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106454);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29152);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84253);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 56404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10076);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 29153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10077);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29154);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 56405);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 84254);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 106455);
													match s {
														[] => true,
														['ч', s @ ..] => super::t16::n122821(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 10078);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10079);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29156);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 56406);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 84255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 29157);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56407);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 84256);
											s.is_empty()
										}
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
					super::push(out, 10080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29158);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 56408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 10081);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10082);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10083);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10084);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29161);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 56409);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 84257);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106456);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10085);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 10086);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10087);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t16::n10115(s, out),
				['р', s @ ..] => {
					super::push(out, 10089);
					match s {
						[] => true,
						['а', s @ ..] => super::t16::n29362(s, out),
						['е', s @ ..] => super::t16::n29363(s, out),
						['о', s @ ..] => super::t16::n29364(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t16::n10117(s, out),
				['т', s @ ..] => super::t16::n10118(s, out),
				['ш', s @ ..] => super::t16::n10119(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t16::n2127(s, out),
		['д', s @ ..] => super::t16::n2128(s, out),
		['ж', s @ ..] => super::t16::n2129(s, out),
		['з', s @ ..] => super::t16::n2130(s, out),
		['к', s @ ..] => super::t16::n2131(s, out),
		['л', s @ ..] => super::t16::n2132(s, out),
		['м', s @ ..] => super::t16::n2133(s, out),
		['н', s @ ..] => super::t16::n2134(s, out),
		['п', s @ ..] => super::t16::n2135(s, out),
		['р', s @ ..] => super::t16::n2136(s, out),
		['с', s @ ..] => super::t16::n2137(s, out),
		['т', s @ ..] => super::t16::n2138(s, out),
		['ф', s @ ..] => super::t16::n2139(s, out),
		['х', s @ ..] => super::t16::n2140(s, out),
		['ц', s @ ..] => super::t16::n2141(s, out),
		['ч', s @ ..] => super::t16::n2142(s, out),
		['ш', s @ ..] => super::t16::n2143(s, out),
		['щ', s @ ..] => super::t16::n2144(s, out),
		_ => false,
	}
}

/// Words ending -ий.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2139);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10191);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10192);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29309);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10193);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29310);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 56526);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 84327);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 56527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10194);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29311);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 56528);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 84328);
											s.is_empty()
										}
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
					super::push(out, 10195);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29312);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56529);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84329);
											s.is_empty()
										}
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
					super::push(out, 10196);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29313);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56530);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 56531);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84330);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 56532);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84331);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 106489);
													s.is_empty()
												}
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
							super::push(out, 29314);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56533);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 84332);
											s.is_empty()
										}
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
					super::push(out, 10197);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29315);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 56534);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 56535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 29316);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 56536);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 84333);
											s.is_empty()
										}
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
					super::push(out, 10198);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29317);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 56537);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 84334);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106490);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29318);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 56538);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84335);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 106491);
													s.is_empty()
												}
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
							super::push(out, 29319);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56539);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 84336);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106492);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 56540);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84337);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 106493);
													s.is_empty()
												}
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
							super::push(out, 29320);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 56541);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84338);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 106494);
													s.is_empty()
												}
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
							super::push(out, 29321);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 56542);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 84339);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => super::t16::n57117(s, out),
								_ => false,
							}
						}
						['э', s @ ..] => super::t16::n29516(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t16::n10228(s, out),
				['т', s @ ..] => super::t16::n10229(s, out),
				['у', s @ ..] => super::t16::n10230(s, out),
				['ш', s @ ..] => super::t16::n10231(s, out),
				['ь', s @ ..] => super::t16::n10232(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t16::n2146(s, out),
		['г', s @ ..] => super::t16::n2147(s, out),
		['д', s @ ..] => super::t16::n2148(s, out),
		['ж', s @ ..] => super::t16::n2149(s, out),
		['з', s @ ..] => super::t16::n2150(s, out),
		['к', s @ ..] => super::t16::n2151(s, out),
		['л', s @ ..] => super::t16::n2152(s, out),
		['м', s @ ..] => super::t16::n2153(s, out),
		['н', s @ ..] => super::t16::n2154(s, out),
		['п', s @ ..] => super::t16::n2155(s, out),
		['р', s @ ..] => super::t16::n2156(s, out),
		['с', s @ ..] => super::t16::n2157(s, out),
		['т', s @ ..] => super::t16::n2158(s, out),
		['ф', s @ ..] => super::t16::n2159(s, out),
		['х', s @ ..] => super::t16::n2160(s, out),
		['ц', s @ ..] => super::t16::n2161(s, out),
		['ч', s @ ..] => super::t17::n2162(s, out),
		['ш', s @ ..] => super::t17::n2163(s, out),
		['щ', s @ ..] => super::t17::n2164(s, out),
		_ => false,
	}
}

/// Words ending -ой.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 10345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10346);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 10347);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29619);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56922);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 84710);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 106779);
													s.is_empty()
												}
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
							super::push(out, 29620);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 56923);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 84711);
											s.is_empty()
										}
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
					super::push(out, 10348);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10349);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29622);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 56924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10350);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29623);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 56925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10351);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29624);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 56926);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 84712);
											s.is_empty()
										}
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
					super::push(out, 10352);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29625);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 56927);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 29626);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 56928);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84713);
											s.is_empty()
										}
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
					super::push(out, 10353);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 56929);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29628);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 56930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10354);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29629);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 56931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84714);
											s.is_empty()
										}
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
					super::push(out, 10355);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29630);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 56932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29631);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 56933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10356);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29632);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 56934);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84715);
											s.is_empty()
										}
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
					super::push(out, 10357);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29633);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 56935);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 29634);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 29635);
							s.is_empty()
						}
						['к', s @ ..] => super::t17::n29843(s, out),
						['л', s @ ..] => super::t17::n29844(s, out),
						['н', s @ ..] => super::t17::n29845(s, out),
						['р', s @ ..] => super::t17::n29846(s, out),
						['т', s @ ..] => super::t17::n29847(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t17::n10388(s, out),
				['т', s @ ..] => super::t17::n10389(s, out),
				['у', s @ ..] => super::t17::n10390(s, out),
				['ы', s @ ..] => super::t17::n10391(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t17::n2167(s, out),
		['г', s @ ..] => super::t17::n2168(s, out),
		['д', s @ ..] => super::t17::n2169(s, out),
		['ж', s @ ..] => super::t17::n2170(s, out),
		['з', s @ ..] => super::t17::n2171(s, out),
		['к', s @ ..] => super::t17::n2172(s, out),
		['л', s @ ..] => super::t17::n2173(s, out),
		['м', s @ ..] => super::t17::n2174(s, out),
		['н', s @ ..] => super::t17::n2175(s, out),
		['п', s @ ..] => super::t17::n2176(s, out),
		['р', s @ ..] => super::t17::n2177(s, out),
		['с', 'о', 'к', '\u{1}', s @ ..] => super::t17::n2178(s, out),
		['т', s @ ..] => super::t17::n2179(s, out),
		['у', s @ ..] => super::t17::n2180(s, out),
		['х', s @ ..] => super::t17::n2181(s, out),
		['ч', s @ ..] => super::t17::n2182(s, out),
		['ш', s @ ..] => super::t17::n2183(s, out),
		['я', s @ ..] => super::t17::n2184(s, out),
		_ => false,
	}
}

/// Words ending -радиобуй.
pub(crate) fn n107955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106819);
	s.is_empty()
}

/// Words ending -ндивидуй.
pub(crate) fn n107956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106820);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 121763);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мозгодуй.
pub(crate) fn n107957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106821);
	s.is_empty()
}

/// Words ending -тодуй.
pub(crate) fn n29962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29755);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 57040);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 84791);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жуй.
pub(crate) fn n2189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2183);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10443);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57041);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 84792);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106823);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10444);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29757);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57042);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84793);
									s.is_empty()
								}
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
			super::push(out, 10445);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29758);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57043);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84794);
									s.is_empty()
								}
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
			super::push(out, 10446);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29759);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -куй.
pub(crate) fn n2191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2185);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10448);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 29760);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10449);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29761);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 29762);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -луй.
pub(crate) fn n2192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2186);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10450);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10451);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 29764);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57046);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84795);
									s.is_empty()
								}
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
			super::push(out, 10452);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 29765);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руй.
pub(crate) fn n2193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2187);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10453);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10454);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29766);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57048);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84796);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106824);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суй.
pub(crate) fn n2194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2188);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10455);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10456);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29767);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57049);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -туй.
pub(crate) fn n2195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2189);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10457);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29769);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хуй.
pub(crate) fn n2196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2190);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10460);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10461);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29770);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29771);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57051);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84798);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106825);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10462);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29772);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57052);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84799);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106826);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 121764);
													match s {
														[] => true,
														['у', s @ ..] => super::t17::n132286(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10463);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29773);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57053);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84800);
									s.is_empty()
								}
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
			super::push(out, 10464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуй.
pub(crate) fn n2197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2191);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10465);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 29775);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57054);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуй.
pub(crate) fn n2198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2192);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10466);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29776);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 57055);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 29777);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57056);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84803);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 106827);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10467);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 29778);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 57057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ый.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2193);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 10468);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29779);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 57058);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 84804);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 106828);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2194);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10469);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 29780);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10470);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 29782);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 57059);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84805);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106829);
													match s {
														[] => true,
														['о', s @ ..] => super::t17::n123040(s, out),
														_ => false,
													}
												}
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
							super::push(out, 29783);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 57060);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 57061);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 84806);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106830);
													match s {
														[] => true,
														['х', s @ ..] => super::t17::n123041(s, out),
														_ => false,
													}
												}
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
							super::push(out, 29784);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 57062);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 84807);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106831);
													match s {
														[] => true,
														['о', s @ ..] => super::t17::n123042(s, out),
														_ => false,
													}
												}
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
							super::push(out, 29785);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 57063);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 84808);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 106832);
													match s {
														[] => true,
														['н', s @ ..] => super::t17::n123043(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 10471);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29786);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 57064);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 84809);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 106833);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => super::t17::n29994(s, out),
						['ш', s @ ..] => super::t17::n29995(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t17::n10502(s, out),
				['т', s @ ..] => super::t17::n10503(s, out),
				['я', s @ ..] => super::t17::n10504(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t17::n2201(s, out),
		['з', s @ ..] => {
			super::push(out, 2196);
			match s {
				[] => true,
				['а', s @ ..] => super::t17::n10507(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t17::n2203(s, out),
		['м', s @ ..] => super::t17::n2204(s, out),
		['н', s @ ..] => super::t17::n2205(s, out),
		['п', s @ ..] => super::t17::n2206(s, out),
		['р', s @ ..] => super::t17::n2207(s, out),
		['с', s @ ..] => super::t17::n2208(s, out),
		['т', s @ ..] => super::t17::n2209(s, out),
		['х', s @ ..] => super::t17::n2210(s, out),
		['ц', s @ ..] => super::t17::n2211(s, out),
		_ => false,
	}
}

/// Words ending -эй.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2206);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 10513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29880);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 57186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 10514);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 29881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2207);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10515);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 10516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2208);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 10517);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 29882);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 57187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10518);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 29883);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 57188);
									s.is_empty()
								}
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
			super::push(out, 2209);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 10519);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юй.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2210);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10520);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10521);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29886);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 57189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яй.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2212);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10523);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 29887);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 57190);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84921);
											s.is_empty()
										}
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
					super::push(out, 10524);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 10525);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29889);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 57191);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 84922);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 106926);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10526);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2213);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 10527);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29891);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 57192);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84923);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 106927);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10528);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 29892);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 57193);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 84924);
											s.is_empty()
										}
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
					super::push(out, 10529);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2214);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10530);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29894);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 57194);
									s.is_empty()
								}
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
			super::push(out, 2215);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 10531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ак.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2217);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 10533);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 57195);
									s.is_empty()
								}
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
			super::push(out, 2219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10534);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 10535);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29897);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 57196);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 84925);
											s.is_empty()
										}
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
					super::push(out, 10536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29898);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 29899);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 29900);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 57197);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84926);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 29901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 57198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 10538);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29903);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 57199);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 57200);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 84927);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 106928);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10539);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29904);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 57201);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84928);
											s.is_empty()
										}
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
					super::push(out, 10540);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29905);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 57202);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84929);
											s.is_empty()
										}
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
					super::push(out, 10541);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29906);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 57203);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 84930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 29907);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 29908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10542);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 29909);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 57204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 29910);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 57205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84931);
											s.is_empty()
										}
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
					super::push(out, 10543);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 29911);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 29912);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 57206);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 84932);
											s.is_empty()
										}
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
					super::push(out, 10544);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2220);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10545);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29914);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 57207);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 84933);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 84934);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 57208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10546);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29915);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 29916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10547);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29918);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29919);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 57209);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 84935);
											match s {
												[] => true,
												['и', s @ ..] => super::t17::n108066(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t17::n10579(s, out),
				['о', s @ ..] => super::t17::n10580(s, out),
				['р', s @ ..] => super::t17::n10581(s, out),
				['у', s @ ..] => super::t17::n10582(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t17::n2227(s, out),
		['д', s @ ..] => super::t17::n2228(s, out),
		['ж', s @ ..] => super::t17::n2229(s, out),
		['з', s @ ..] => super::t17::n2230(s, out),
		['и', s @ ..] => super::t17::n2231(s, out),
		['к', s @ ..] => super::t17::n2232(s, out),
		['л', s @ ..] => super::t17::n2233(s, out),
		['м', s @ ..] => super::t17::n2234(s, out),
		['н', s @ ..] => super::t17::n2235(s, out),
		['о', s @ ..] => super::t17::n2236(s, out),
		['п', s @ ..] => super::t17::n2237(s, out),
		['р', s @ ..] => super::t17::n2238(s, out),
		['с', s @ ..] => super::t17::n2239(s, out),
		['т', s @ ..] => super::t17::n2240(s, out),
		['у', s @ ..] => super::t17::n2241(s, out),
		['ф', s @ ..] => super::t17::n2242(s, out),
		['х', s @ ..] => super::t17::n2243(s, out),
		['ч', s @ ..] => super::t17::n2244(s, out),
		['ш', s @ ..] => super::t17::n2245(s, out),
		['щ', s @ ..] => super::t17::n2246(s, out),
		_ => false,
	}
}

/// Words ending -ек.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10709);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10710);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30153);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 57405);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 85032);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 57406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 10712);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10713);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10714);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30156);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 57407);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 85033);
											s.is_empty()
										}
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
					super::push(out, 10715);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30157);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 57408);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 85034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 57409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 10716);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30159);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 57410);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 85035);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106966);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10717);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30160);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 57411);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 85036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 30161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10718);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 10719);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30162);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 57412);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 85037);
											s.is_empty()
										}
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
					super::push(out, 10720);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 30163);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 57413);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 85038);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106967);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10721);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 10722);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10723);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 57414);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 85039);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106968);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 57415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10724);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30164);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 30165);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 57416);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 85040);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 106969);
													match s {
														[] => true,
														['с', s @ ..] => super::t17::n123125(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 10725);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 30166);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 57417);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 85041);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 106970);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10726);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30167);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									super::push(out, 57418);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 85042);
											s.is_empty()
										}
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
					super::push(out, 10727);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30168);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 57419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t17::n2250(s, out),
		['д', s @ ..] => super::t17::n2251(s, out),
		['е', s @ ..] => super::t17::n2252(s, out),
		['ж', s @ ..] => super::t17::n2253(s, out),
		['з', s @ ..] => super::t17::n2254(s, out),
		['и', s @ ..] => {
			super::push(out, 2249);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2250);
			match s {
				[] => true,
				['ш', s @ ..] => super::t17::n10780(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t17::n2257(s, out),
		['м', s @ ..] => super::t17::n2258(s, out),
		['н', s @ ..] => super::t17::n2259(s, out),
		['о', s @ ..] => super::t17::n2260(s, out),
		['п', s @ ..] => super::t17::n2261(s, out),
		['р', s @ ..] => super::t17::n2262(s, out),
		['с', s @ ..] => super::t17::n2263(s, out),
		['т', s @ ..] => super::t17::n2264(s, out),
		['у', s @ ..] => super::t17::n2265(s, out),
		['х', s @ ..] => super::t17::n2266(s, out),
		['ц', s @ ..] => super::t17::n2267(s, out),
		['ч', s @ ..] => super::t17::n2268(s, out),
		['ш', s @ ..] => super::t17::n2269(s, out),
		_ => false,
	}
}

/// Words ending -ик.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 10836);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 57663);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 85275);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 57664);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 85276);
											s.is_empty()
										}
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
					super::push(out, 10837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 10838);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['а', 'р', 'к', s @ ..] => {
					super::push(out, 10839);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', 'й', '\u{1}', s @ ..] => {
					super::push(out, 10840);
					s.is_empty()
				}
				['е', '\u{1}', s @ ..] => {
					super::push(out, 10841);
					s.is_empty()
				}
				['и', 'ч', s @ ..] => {
					super::push(out, 10842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 10843);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30386);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 57665);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 10844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30387);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 57666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 10845);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30388);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 57667);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 85277);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 107150);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t17::n123252(s, out),
														_ => false,
													}
												}
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
							super::push(out, 30389);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 57668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10846);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 30390);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 57669);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 57670);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 85278);
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
									super::push(out, 57671);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 57672);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 85279);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 30391);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 57673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 30392);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 57674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10847);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10848);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 30393);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 57675);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 85280);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 107151);
													match s {
														[] => true,
														['к', s @ ..] => super::t17::n123253(s, out),
														_ => false,
													}
												}
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
							super::push(out, 30394);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 30395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 10849);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30396);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 57676);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 85281);
											s.is_empty()
										}
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
					super::push(out, 10850);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 30397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t17::n2272(s, out),
		['г', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10859);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10860);
					match s {
						[] => true,
						['л', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 57803);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 85423);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 57804);
									match s {
										[] => true,
										['р', s @ ..] => super::t17::n86331(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t17::n30649(s, out),
						['т', s @ ..] => super::t17::n30650(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t17::n10893(s, out),
				['р', s @ ..] => super::t17::n10894(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t17::n2274(s, out),
		['ж', s @ ..] => super::t17::n2275(s, out),
		['з', s @ ..] => super::t17::n2276(s, out),
		['к', s @ ..] => super::t17::n2277(s, out),
		['л', s @ ..] => super::t17::n2278(s, out),
		['м', s @ ..] => super::t17::n2279(s, out),
		['н', s @ ..] => super::t17::n2280(s, out),
		['о', s @ ..] => super::t17::n2281(s, out),
		['п', s @ ..] => super::t17::n2282(s, out),
		['р', s @ ..] => super::t18::n2283(s, out),
		['с', s @ ..] => super::t18::n2284(s, out),
		['т', s @ ..] => super::t18::n2285(s, out),
		['у', s @ ..] => super::t18::n2286(s, out),
		['ф', s @ ..] => super::t18::n2287(s, out),
		['ц', s @ ..] => super::t18::n2288(s, out),
		['ч', s @ ..] => super::t18::n2289(s, out),
		['ш', s @ ..] => super::t18::n2290(s, out),
		['щ', s @ ..] => super::t18::n2291(s, out),
		['ю', s @ ..] => super::t18::n2292(s, out),
		['я', s @ ..] => super::t18::n2293(s, out),
		_ => false,
	}
}

/// Words ending -аунтбайк.
pub(crate) fn n110148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108985);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 123215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 131720);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катафалк.
pub(crate) fn n110149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108986);
	s.is_empty()
}

/// Words ending -перещелк.
pub(crate) fn n110150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108987);
	s.is_empty()
}

/// Words ending -олк.
pub(crate) fn n2300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2294);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11085);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31148);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 31149);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59278);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 87308);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 108988);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123216);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11086);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11087);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11088);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31152);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 31153);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 59280);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 87309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 31154);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59281);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87310);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 108989);
											s.is_empty()
										}
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
					super::push(out, 31155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59282);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31156);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59283);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87312);
									s.is_empty()
								}
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
			super::push(out, 11089);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31157);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31158);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59284);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 87313);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 108990);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123217);
													s.is_empty()
												}
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
							super::push(out, 59285);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 87314);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 108991);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11090);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 31159);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31160);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31161);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31162);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59286);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 87315);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108992);
											s.is_empty()
										}
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
					super::push(out, 31163);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59287);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87316);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108993);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нк.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11091);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 11092);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31164);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 31165);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 31166);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 59288);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 87317);
											s.is_empty()
										}
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
					super::push(out, 11094);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 31167);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 59289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11095);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31168);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 59290);
									s.is_empty()
								}
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
			super::push(out, 2296);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11096);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 31169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 59291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11097);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11098);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 31170);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 59292);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 87318);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 108994);
													match s {
														[] => true,
														['о', s @ ..] => super::t18::n124501(s, out),
														_ => false,
													}
												}
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
							super::push(out, 31171);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 59293);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 87319);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 108995);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n124502(s, out),
														_ => false,
													}
												}
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
							super::push(out, 31172);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 59294);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 87320);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 108996);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2297);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11099);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11100);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 31173);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 59295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11101);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11102);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 11103);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 59296);
									s.is_empty()
								}
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
			super::push(out, 2298);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11104);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31176);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 59297);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 87321);
											s.is_empty()
										}
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
					super::push(out, 11105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2299);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11106);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31177);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 59298);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 87322);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 108997);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2300);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11107);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 31178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 59299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ок.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11108);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11109);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31179);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 59300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 11110);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31180);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 59301);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 87323);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 108998);
													match s {
														[] => true,
														['с', s @ ..] => super::t18::n124503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11111);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31181);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 59302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 31182);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 59303);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 59304);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 87324);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 108999);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 59305);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 87325);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 109000);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n124504(s, out),
														_ => false,
													}
												}
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
							super::push(out, 31183);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 59306);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 87326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11112);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31185);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 59307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11113);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31186);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 59308);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 87327);
											s.is_empty()
										}
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
					super::push(out, 11114);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31187);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 59309);
									match s {
										[] => true,
										['ж', 'з', s @ ..] => {
											super::push(out, 87328);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109001);
													match s {
														[] => true,
														['р', s @ ..] => super::t18::n124505(s, out),
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
									super::push(out, 59310);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 87329);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109002);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 59311);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 87330);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109003);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t18::n31442(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t18::n11154(s, out),
				['у', s @ ..] => super::t18::n11155(s, out),
				['ь', s @ ..] => super::t18::n11156(s, out),
				['я', s @ ..] => super::t18::n11157(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t18::n2308(s, out),
		['г', s @ ..] => super::t18::n2309(s, out),
		['д', s @ ..] => super::t18::n2310(s, out),
		['ж', s @ ..] => super::t18::n2311(s, out),
		['з', s @ ..] => super::t18::n2312(s, out),
		['и', s @ ..] => super::t18::n2313(s, out),
		['к', s @ ..] => super::t18::n2314(s, out),
		['л', s @ ..] => super::t18::n2315(s, out),
		['м', s @ ..] => super::t18::n2316(s, out),
		['н', s @ ..] => super::t18::n2317(s, out),
		['п', s @ ..] => super::t18::n2318(s, out),
		['р', s @ ..] => super::t18::n2319(s, out),
		['с', s @ ..] => super::t18::n2320(s, out),
		['т', s @ ..] => super::t18::n2321(s, out),
		['у', s @ ..] => super::t18::n2322(s, out),
		['ф', s @ ..] => super::t18::n2323(s, out),
		['х', s @ ..] => super::t18::n2324(s, out),
		['ч', s @ ..] => super::t18::n2325(s, out),
		['ш', s @ ..] => super::t18::n2326(s, out),
		['щ', s @ ..] => super::t18::n2327(s, out),
		_ => false,
	}
}

/// Words ending -рк.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2322);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11278);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31698);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 31699);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 60040);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 87929);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 109341);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n124649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11279);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31701);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31702);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60041);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 87930);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109342);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11282);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 31704);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31705);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 31706);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60042);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 87931);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109343);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11283);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60043);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31708);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 60044);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 87932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 31709);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 11284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 11285);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 31711);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60046);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 87933);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2323);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11286);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31712);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60047);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 87934);
											s.is_empty()
										}
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
					super::push(out, 11287);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31713);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60048);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 87935);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 31714);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60049);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 87936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 31715);
							match s {
								[] => true,
								['е', s @ ..] => super::t18::n60687(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t18::n31983(s, out),
						['т', s @ ..] => super::t18::n31984(s, out),
						['х', s @ ..] => super::t18::n31985(s, out),
						['ь', s @ ..] => super::t18::n31986(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11288);
					match s {
						[] => true,
						['у', s @ ..] => super::t18::n31987(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t18::n11328(s, out),
				['л', s @ ..] => super::t18::n11329(s, out),
				['с', s @ ..] => super::t18::n11330(s, out),
				['ч', s @ ..] => super::t18::n11331(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t18::n2330(s, out),
		['к', s @ ..] => super::t18::n2331(s, out),
		['о', s @ ..] => super::t18::n2332(s, out),
		['у', s @ ..] => super::t18::n2333(s, out),
		['ы', s @ ..] => super::t18::n2334(s, out),
		['ю', s @ ..] => super::t18::n2335(s, out),
		_ => false,
	}
}

/// Words ending -ск.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2330);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11302);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31734);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60068);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 87951);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 31735);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 60069);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 87952);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109350);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11303);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31736);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60070);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 87953);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 109351);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31737);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60071);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 87954);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 109352);
													s.is_empty()
												}
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
							super::push(out, 31738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11304);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31739);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60072);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 87955);
											s.is_empty()
										}
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
					super::push(out, 11305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11306);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11307);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31740);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60073);
									s.is_empty()
								}
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
			super::push(out, 2331);
			match s {
				[] => true,
				['е', s @ ..] => super::t18::n11349(s, out),
				['о', s @ ..] => super::t18::n11350(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2332);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11310);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t18::n2340(s, out),
		['ж', s @ ..] => {
			super::push(out, 2334);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11318);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31770);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60117);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11319);
					match s {
						[] => true,
						['л', s @ ..] => super::t18::n32040(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2335);
			s.is_empty()
		}
		['и', s @ ..] => super::t18::n2343(s, out),
		['й', s @ ..] => super::t18::n2344(s, out),
		['к', s @ ..] => {
			super::push(out, 2338);
			s.is_empty()
		}
		['м', s @ ..] => super::t18::n2346(s, out),
		['н', s @ ..] => super::t18::n2347(s, out),
		['о', s @ ..] => super::t18::n2348(s, out),
		['р', s @ ..] => super::t18::n2349(s, out),
		['с', s @ ..] => super::t18::n2350(s, out),
		['т', s @ ..] => super::t18::n2351(s, out),
		['у', s @ ..] => super::t18::n2352(s, out),
		['ы', s @ ..] => super::t18::n2353(s, out),
		['ь', s @ ..] => super::t18::n2354(s, out),
		['э', s @ ..] => super::t18::n2355(s, out),
		['ю', s @ ..] => super::t18::n2356(s, out),
		['я', s @ ..] => super::t18::n2357(s, out),
		_ => false,
	}
}

/// Words ending -ук.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2351);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2352);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11358);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11359);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11360);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11361);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31845);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60198);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88054);
											s.is_empty()
										}
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
					super::push(out, 11362);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 31846);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60199);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 60200);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 88055);
											s.is_empty()
										}
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
					super::push(out, 11363);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11364);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 31848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60201);
									s.is_empty()
								}
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
			super::push(out, 2353);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11365);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31849);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 31850);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60203);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 88056);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31851);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60204);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88057);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109420);
													s.is_empty()
												}
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
							super::push(out, 31852);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88058);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2354);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2355);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11367);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31853);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11368);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31854);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60207);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88059);
											s.is_empty()
										}
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
					super::push(out, 11369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31855);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 60208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11370);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31856);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60209);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88060);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 109421);
													s.is_empty()
												}
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
							super::push(out, 31857);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 60210);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88061);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 60211);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 88062);
											match s {
												[] => true,
												['б', '\u{1}', s @ ..] => {
													super::push(out, 109422);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 60212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11371);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31858);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 60213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11372);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31859);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 60214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11373);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31860);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 31861);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 60215);
									s.is_empty()
								}
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
			super::push(out, 2356);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11375);
					s.is_empty()
				}
				['д', s @ ..] => super::t18::n11417(s, out),
				['у', s @ ..] => super::t18::n11418(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t18::n2365(s, out),
		['к', s @ ..] => super::t18::n2366(s, out),
		['л', s @ ..] => super::t18::n2367(s, out),
		['м', s @ ..] => super::t18::n2368(s, out),
		['н', s @ ..] => super::t18::n2369(s, out),
		['п', s @ ..] => super::t18::n2370(s, out),
		['р', s @ ..] => super::t18::n2371(s, out),
		['с', s @ ..] => super::t18::n2372(s, out),
		['т', s @ ..] => super::t18::n2373(s, out),
		['у', s @ ..] => super::t18::n2374(s, out),
		['ф', s @ ..] => {
			super::push(out, 2368);
			s.is_empty()
		}
		['х', s @ ..] => super::t18::n2376(s, out),
		['ц', s @ ..] => super::t18::n2377(s, out),
		['ч', s @ ..] => super::t18::n2378(s, out),
		['ш', s @ ..] => super::t18::n2379(s, out),
		['щ', s @ ..] => super::t18::n2380(s, out),
		_ => false,
	}
}

/// Words ending -цк.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2374);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2375);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 11431);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31939);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 60285);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88101);
											s.is_empty()
										}
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
					super::push(out, 11432);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11433);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11434);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31940);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 60286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88102);
											s.is_empty()
										}
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
					super::push(out, 11435);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2376);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31942);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60287);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88103);
											s.is_empty()
										}
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
					super::push(out, 11437);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31943);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11438);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2377);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11439);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31945);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11440);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 31946);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60290);
									s.is_empty()
								}
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
			super::push(out, 2378);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ык.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2379);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11442);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 31947);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60291);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88104);
											s.is_empty()
										}
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
					super::push(out, 11443);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31948);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60292);
									s.is_empty()
								}
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
			super::push(out, 2380);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11444);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11445);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31950);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 31951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11446);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 31952);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60294);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88105);
											s.is_empty()
										}
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
					super::push(out, 11447);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31953);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 60295);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88106);
											s.is_empty()
										}
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
					super::push(out, 11448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31954);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60296);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 88107);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 109442);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 60297);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88108);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 31955);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 60298);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88109);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 109443);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11449);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31956);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 11450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2382);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11451);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 11452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 11453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31959);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 31960);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31961);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60300);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 88110);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 60301);
									match s {
										[] => true,
										['е', 'м', '\u{1}', s @ ..] => {
											super::push(out, 88111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2383);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11454);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31963);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 31964);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60302);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88112);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109444);
													s.is_empty()
												}
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
							super::push(out, 31965);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60303);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 88113);
											s.is_empty()
										}
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
					super::push(out, 11455);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11456);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31966);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60304);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88114);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31967);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 31968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 11457);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11458);
					match s {
						[] => true,
						['з', s @ ..] => super::t18::n32240(s, out),
						['й', s @ ..] => super::t18::n32241(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t18::n2391(s, out),
		['н', s @ ..] => {
			super::push(out, 2385);
			match s {
				[] => true,
				['х', s @ ..] => super::t18::n11504(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t18::n2393(s, out),
		['т', s @ ..] => super::t18::n2394(s, out),
		['ц', s @ ..] => super::t18::n2395(s, out),
		['ч', s @ ..] => super::t18::n2396(s, out),
		_ => false,
	}
}

/// Words ending -ьк.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2390);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11477);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 31982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11478);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 31983);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60317);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 88122);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109447);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11479);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31984);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 31985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эк.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2391);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11480);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31986);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60318);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 88123);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 109448);
													match s {
														[] => true,
														['а', s @ ..] => super::t18::n124692(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11481);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2392);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11483);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2393);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 2394);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11484);
					s.is_empty()
				}
				['т', 'и', 'л', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 11485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2395);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31988);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60319);
									s.is_empty()
								}
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
			super::push(out, 2396);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11487);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2397);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11489);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 31989);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60320);
									s.is_empty()
								}
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
			super::push(out, 2398);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2399);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11491);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юк.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2400);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2401);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11493);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11494);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31990);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 31991);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60321);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11495);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31992);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60322);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 31993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11496);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31994);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 31995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11497);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31996);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88125);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31997);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 60324);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 60325);
									s.is_empty()
								}
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
			super::push(out, 2403);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11498);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2404);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11499);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11500);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 31999);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60326);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11501);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32001);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60328);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88127);
											s.is_empty()
										}
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
					super::push(out, 11502);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32002);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11503);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32003);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11504);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32004);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60331);
									s.is_empty()
								}
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
			super::push(out, 2405);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11505);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11506);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32006);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11507);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32008);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60332);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88128);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 109449);
													match s {
														[] => true,
														['д', s @ ..] => super::t18::n124693(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2406);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11508);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32009);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60333);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88129);
											s.is_empty()
										}
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
					super::push(out, 11509);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32010);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60334);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88130);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2407);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11510);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 11511);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32012);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32013);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t18::n11554(s, out),
				['у', s @ ..] => {
					super::push(out, 11514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t18::n11556(s, out),
				['ю', s @ ..] => super::t18::n11557(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t18::n2415(s, out),
		['т', s @ ..] => super::t18::n2416(s, out),
		['ь', s @ ..] => super::t18::n2417(s, out),
		_ => false,
	}
}

/// Words ending -як.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2411);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2412);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11525);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11526);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2413);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11527);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32027);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 60340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11528);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11529);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32029);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32030);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 88133);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109451);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 88134);
											s.is_empty()
										}
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
					super::push(out, 11531);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 32031);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60341);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88135);
											s.is_empty()
										}
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
					super::push(out, 11532);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32032);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32033);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88136);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 109452);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n124694(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 88137);
											s.is_empty()
										}
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
					super::push(out, 11533);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32034);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60343);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 60344);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88138);
											s.is_empty()
										}
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
					super::push(out, 11534);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32035);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60345);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88139);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32036);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60346);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88140);
											s.is_empty()
										}
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
					super::push(out, 11535);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 32037);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60347);
									s.is_empty()
								}
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
			super::push(out, 2415);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11536);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11537);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 32039);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2416);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32040);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32041);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11539);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32042);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11540);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32043);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60351);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88141);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109453);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2417);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11541);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32044);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60352);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88142);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109454);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t18::n2425(s, out),
		['м', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['о', s @ ..] => super::t18::n11601(s, out),
				['р', s @ ..] => super::t18::n11602(s, out),
				['х', s @ ..] => super::t18::n11603(s, out),
				['ш', s @ ..] => {
					super::push(out, 11562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t18::n32351(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t18::n2427(s, out),
		['о', s @ ..] => super::t18::n2428(s, out),
		['р', s @ ..] => super::t18::n2429(s, out),
		['с', s @ ..] => super::t18::n2430(s, out),
		['т', s @ ..] => super::t18::n2431(s, out),
		['у', s @ ..] => super::t18::n2432(s, out),
		['ф', s @ ..] => super::t18::n2433(s, out),
		['ь', s @ ..] => super::t19::n2434(s, out),
		_ => false,
	}
}

/// Words ending -ал.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2428);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					super::push(out, 11606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2429);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11607);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 11608);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32153);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60447);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88189);
											s.is_empty()
										}
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
					super::push(out, 11609);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32154);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11610);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60449);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 60450);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88190);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109468);
													s.is_empty()
												}
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
							super::push(out, 32156);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 60451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11611);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32157);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 60452);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 88191);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 109469);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11612);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32158);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60453);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88192);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109470);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11613);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 32159);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60454);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88193);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109471);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11614);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11615);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 11616);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11617);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32162);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11618);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11619);
					match s {
						[] => true,
						['а', s @ ..] => super::t19::n32438(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t19::n11662(s, out),
				['й', s @ ..] => super::t19::n11663(s, out),
				['к', s @ ..] => super::t19::n11664(s, out),
				['н', s @ ..] => super::t19::n11665(s, out),
				['о', s @ ..] => super::t19::n11666(s, out),
				['р', s @ ..] => super::t19::n11667(s, out),
				['с', s @ ..] => super::t19::n11668(s, out),
				['т', s @ ..] => {
					super::push(out, 11627);
					match s {
						[] => true,
						['о', s @ ..] => super::t19::n32455(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t19::n11670(s, out),
				['х', s @ ..] => super::t19::n11671(s, out),
				['ы', s @ ..] => super::t19::n11672(s, out),
				['ю', s @ ..] => super::t19::n11673(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t19::n2438(s, out),
		['д', s @ ..] => super::t19::n2439(s, out),
		['е', s @ ..] => super::t19::n2440(s, out),
		['ж', s @ ..] => super::t19::n2441(s, out),
		['з', s @ ..] => super::t19::n2442(s, out),
		['и', s @ ..] => super::t19::n2443(s, out),
		['к', s @ ..] => super::t19::n2444(s, out),
		['л', 'а', s @ ..] => super::t19::n2445(s, out),
		['м', s @ ..] => super::t19::n2446(s, out),
		['н', s @ ..] => super::t19::n2447(s, out),
		['п', s @ ..] => super::t19::n2448(s, out),
		['р', s @ ..] => super::t19::n2449(s, out),
		['с', s @ ..] => super::t19::n2450(s, out),
		['т', s @ ..] => super::t19::n2451(s, out),
		['у', s @ ..] => super::t19::n2452(s, out),
		['ф', s @ ..] => super::t19::n2453(s, out),
		['х', s @ ..] => super::t19::n2454(s, out),
		['ц', s @ ..] => super::t19::n2455(s, out),
		['ч', s @ ..] => super::t19::n2456(s, out),
		['ш', s @ ..] => super::t19::n2457(s, out),
		_ => false,
	}
}

/// Words ending -ел.
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11724);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32312);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88286);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32313);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60597);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88287);
											s.is_empty()
										}
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
					super::push(out, 11725);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32314);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11726);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11728);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 32317);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60600);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88288);
											s.is_empty()
										}
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
					super::push(out, 11729);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32318);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60601);
									s.is_empty()
								}
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
			super::push(out, 2458);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11730);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11731);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32320);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88289);
											s.is_empty()
										}
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
					super::push(out, 11732);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32321);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32322);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11734);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32323);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60606);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 88290);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', '\u{1}', s @ ..] => {
							super::push(out, 32324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11735);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32325);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60607);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88291);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109521);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11736);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 11737);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11738);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32327);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 60608);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88292);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 109522);
													match s {
														[] => true,
														['ы', s @ ..] => super::t19::n124724(s, out),
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
									super::push(out, 60609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t19::n2467(s, out),
		['ж', s @ ..] => {
			super::push(out, 2461);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11748);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 32343);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60623);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 88301);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2462);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11749);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11750);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32345);
							match s {
								[] => true,
								['е', s @ ..] => super::t19::n61267(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t19::n11797(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t19::n2470(s, out),
		['к', s @ ..] => super::t19::n2471(s, out),
		['л', s @ ..] => super::t19::n2472(s, out),
		['м', s @ ..] => super::t19::n2473(s, out),
		['н', s @ ..] => super::t19::n2474(s, out),
		['п', s @ ..] => super::t19::n2475(s, out),
		['р', s @ ..] => super::t19::n2476(s, out),
		['с', s @ ..] => super::t19::n2477(s, out),
		['т', s @ ..] => super::t19::n2478(s, out),
		['ф', s @ ..] => super::t19::n2479(s, out),
		['ц', s @ ..] => super::t19::n2480(s, out),
		['ч', s @ ..] => super::t19::n2481(s, out),
		_ => false,
	}
}

/// Words ending -ил.
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11790);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11791);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11793);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11794);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11795);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11796);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32399);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60667);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88324);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 109540);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2479);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11797);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2480);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11799);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32400);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60668);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 88325);
											s.is_empty()
										}
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
					super::push(out, 11800);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60669);
									s.is_empty()
								}
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
			super::push(out, 2481);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11801);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11802);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32402);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60670);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88326);
											s.is_empty()
										}
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
							super::push(out, 32403);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60671);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 88327);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 109541);
													s.is_empty()
												}
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
							super::push(out, 32404);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60672);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88328);
											s.is_empty()
										}
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
					super::push(out, 11803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2482);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11804);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32405);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60673);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88329);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109542);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11805);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11806);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32407);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 60674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 32408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11807);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2484);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11809);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11810);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2485);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2486);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11812);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 32412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11813);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32413);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60675);
									s.is_empty()
								}
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
			super::push(out, 2487);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32414);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 32415);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60677);
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
						['а', 'г', 'и', 'р', 'б', '\u{1}', s @ ..] => {
							super::push(out, 32416);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32417);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60678);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11814);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32418);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60679);
									match s {
										[] => true,
										['б', s @ ..] => super::t19::n89281(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t19::n11864(s, out),
				['т', s @ ..] => super::t19::n11865(s, out),
				['у', s @ ..] => super::t19::n11866(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t19::n2495(s, out),
		['о', s @ ..] => super::t19::n2496(s, out),
		['п', s @ ..] => super::t19::n2497(s, out),
		['р', s @ ..] => super::t19::n2498(s, out),
		['с', s @ ..] => super::t19::n2499(s, out),
		['т', s @ ..] => super::t19::n2500(s, out),
		['у', s @ ..] => super::t19::n2501(s, out),
		['ф', s @ ..] => super::t19::n2502(s, out),
		['х', s @ ..] => super::t19::n2503(s, out),
		['ц', s @ ..] => super::t19::n2504(s, out),
		['ч', s @ ..] => super::t19::n2505(s, out),
		['ш', s @ ..] => super::t19::n2506(s, out),
		_ => false,
	}
}

/// The word бобтейл.
pub(crate) fn n110724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109560);
	s.is_empty()
}

/// Words ending -юзикл.
pub(crate) fn n32759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32472);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 60726);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 88363);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 109561);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 123460);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131834);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136750);
													match s {
														[] => true,
														['ф', s @ ..] => super::t19::n141126(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерикл.
pub(crate) fn n32760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32473);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 60727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 88364);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тикл.
pub(crate) fn n11919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11866);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 32474);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 60728);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 88365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 109562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оцикл.
pub(crate) fn n32762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32475);
	s.is_empty()
}

/// Words ending -йкл.
pub(crate) fn n2511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2502);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11868);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32477);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окл.
pub(crate) fn n2512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2503);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11869);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32478);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60730);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88366);
									s.is_empty()
								}
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
			super::push(out, 11870);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32479);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11871);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32480);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60732);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экл.
pub(crate) fn n2513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2504);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 11872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32481);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лл.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2505);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11873);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 11874);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11875);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32483);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60733);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88368);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109563);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11876);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32484);
							match s {
								[] => true,
								['м', '-', s @ ..] => {
									super::push(out, 60734);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 88369);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 109564);
													match s {
														[] => true,
														['е', s @ ..] => super::t19::n124744(s, out),
														_ => false,
													}
												}
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
							super::push(out, 32485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 11877);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32486);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60735);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88370);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109565);
													match s {
														[] => true,
														['а', s @ ..] => super::t19::n124745(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11878);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32487);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60736);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88371);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109566);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2506);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11879);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11880);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32488);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60737);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88372);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 109567);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11881);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32489);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60738);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88373);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 109568);
													match s {
														[] => true,
														['т', s @ ..] => super::t19::n124746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11882);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32490);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60739);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88374);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 109569);
													match s {
														[] => true,
														['а', s @ ..] => super::t19::n124747(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t19::n2516(s, out),
		['о', s @ ..] => {
			super::push(out, 2508);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11895);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32508);
							match s {
								[] => true,
								['н', '-', 'к', 'о', s @ ..] => {
									super::push(out, 60754);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 88385);
											s.is_empty()
										}
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
					super::push(out, 11896);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32509);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60755);
									match s {
										[] => true,
										['о', s @ ..] => super::t19::n89337(s, out),
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
					super::push(out, 11897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t19::n2518(s, out),
		['э', s @ ..] => super::t19::n2519(s, out),
		_ => false,
	}
}

/// Words ending -ол.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2511);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11900);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32513);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 60759);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 88389);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109579);
													s.is_empty()
												}
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
							super::push(out, 32514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60760);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 88390);
											s.is_empty()
										}
										['ш', s @ ..] => {
											super::push(out, 88391);
											s.is_empty()
										}
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
					super::push(out, 11901);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32515);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60761);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88392);
											s.is_empty()
										}
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
					super::push(out, 11902);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32516);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60762);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 88393);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 88394);
											s.is_empty()
										}
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
					super::push(out, 11903);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 11904);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32517);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32518);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 60763);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88395);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 109580);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11905);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11906);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32519);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60764);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88396);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109581);
													s.is_empty()
												}
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
							super::push(out, 32520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11908);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32521);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32522);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60766);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88397);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 109582);
													match s {
														[] => true,
														['д', s @ ..] => super::t19::n124753(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11909);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32523);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 60767);
									match s {
										[] => true,
										['и', 'п', '\u{1}', s @ ..] => {
											super::push(out, 88398);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 32524);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60768);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 88399);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 109583);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n124754(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11910);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11911);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 32525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11912);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32526);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60769);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88400);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109584);
													match s {
														[] => true,
														['б', s @ ..] => super::t19::n124755(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', 'р', s @ ..] => {
							super::push(out, 32527);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60770);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88401);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 32528);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 11913);
					s.is_empty()
				}
				['ц', s @ ..] => super::t19::n11967(s, out),
				['ч', s @ ..] => {
					super::push(out, 11915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t19::n2521(s, out),
		['г', s @ ..] => super::t19::n2522(s, out),
		['д', s @ ..] => super::t19::n2523(s, out),
		['е', s @ ..] => super::t19::n2524(s, out),
		['з', s @ ..] => super::t19::n2525(s, out),
		['и', s @ ..] => {
			super::push(out, 2517);
			s.is_empty()
		}
		['й', s @ ..] => super::t19::n2527(s, out),
		['к', s @ ..] => super::t19::n2528(s, out),
		['л', s @ ..] => super::t19::n2529(s, out),
		['м', s @ ..] => super::t19::n2530(s, out),
		['н', s @ ..] => super::t19::n2531(s, out),
		['п', s @ ..] => super::t19::n2532(s, out),
		['р', s @ ..] => super::t19::n2533(s, out),
		['с', s @ ..] => super::t19::n2534(s, out),
		['т', s @ ..] => super::t19::n2535(s, out),
		['у', s @ ..] => super::t19::n2536(s, out),
		['ф', s @ ..] => super::t19::n2537(s, out),
		['х', s @ ..] => super::t19::n2538(s, out),
		_ => false,
	}
}

/// Words ending -пл.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2530);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2531);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11991);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рл.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2532);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2533);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11993);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32627);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60846);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88443);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 109605);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n124766(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11994);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11995);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2534);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 11997);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 2535);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2536);
			s.is_empty()
		}
		_ => false,
	}
}

