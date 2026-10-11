use flexion::Path;

/// Words ending -ога.
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5214);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20395);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45416);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73909);
									s.is_empty()
								}
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
			super::push(out, 5215);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20396);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20397);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20398);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5217);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20399);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45419);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20400);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45420);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73911);
									s.is_empty()
								}
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
			super::push(out, 5218);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 5219);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5220);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5221);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20404);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5222);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20407);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20408);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45422);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20410);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45424);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20411);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45425);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73914);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 98686);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116761);
													match s {
														[] => true,
														['д', s @ ..] => super::t29::n129535(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20412);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20413);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5224);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20414);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45428);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73915);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98687);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116762);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129536(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5225);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20416);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20417);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45431);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73916);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98688);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 116763);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 73917);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98689);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116764);
													s.is_empty()
												}
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
							super::push(out, 45432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20418);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20419);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45433);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73919);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 98690);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116765);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129537(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45434);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98691);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рга.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5227);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20420);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73921);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 73922);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45436);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45438);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20423);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45439);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73925);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98692);
											s.is_empty()
										}
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
					super::push(out, 20424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5228);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20425);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 45441);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73926);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98693);
											s.is_empty()
										}
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
					super::push(out, 20426);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20427);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20428);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45443);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73927);
									s.is_empty()
								}
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
			super::push(out, 5229);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 20430);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5230);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20432);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20433);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45447);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73928);
									s.is_empty()
								}
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
			super::push(out, 5231);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20434);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45448);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73929);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98694);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 116766);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129538(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20436);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5232);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20437);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сга.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 658);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 5233);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20438);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45450);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уга.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 659);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5234);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20439);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45451);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98695);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5235);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20440);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20441);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45452);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 45453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20442);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45454);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73932);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 98696);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 116767);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129539(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20443);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45455);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73933);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98697);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5236);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5237);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20445);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 20446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20447);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45458);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 45459);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 73935);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45460);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 45461);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 73936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 45462);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 45463);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73937);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 45464);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 45465);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73939);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98699);
											s.is_empty()
										}
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
					super::push(out, 20449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5238);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20451);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45466);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73940);
									s.is_empty()
								}
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
			super::push(out, 5241);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20452);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45467);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 73941);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98700);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45468);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20453);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45469);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73943);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98701);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116768);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129540(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20454);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45470);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73944);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 45471);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73945);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98702);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 45472);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20455);
					match s {
						[] => true,
						['с', s @ ..] => super::t29::n45908(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5242);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t29::n5254(s, out),
		['ф', s @ ..] => super::t29::n5255(s, out),
		['ч', s @ ..] => super::t29::n5256(s, out),
		['ш', s @ ..] => super::t29::n5257(s, out),
		['щ', s @ ..] => super::t29::n5258(s, out),
		_ => false,
	}
}

/// Words ending -ыга.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5248);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20469);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20470);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45485);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 73955);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98710);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 116773);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5249);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20472);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45486);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73956);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98711);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 45487);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73957);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 98712);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 45488);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20473);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 45489);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73959);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98713);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116774);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20474);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45490);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 73960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20475);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5250);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20477);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45492);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73961);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 98714);
											s.is_empty()
										}
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
					super::push(out, 20478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20479);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45493);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 45494);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73962);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20480);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20481);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45495);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73963);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98715);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20482);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20483);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45496);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73964);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98716);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 116775);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129541(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20484);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20485);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45498);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 73965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20486);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 45499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20487);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20488);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 45500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5253);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьга.
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5254);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20491);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5255);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 20492);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 45502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73966);
									s.is_empty()
								}
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
			super::push(out, 5256);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20493);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45503);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73967);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98717);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116776);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45504);
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 45505);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 45506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5257);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20495);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 45508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20496);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45509);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 45510);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73969);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98718);
											s.is_empty()
										}
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
					super::push(out, 20497);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73970);
									s.is_empty()
								}
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
			super::push(out, 5258);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20498);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эга.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 662);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 5259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юга.
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 663);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5260);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5261);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20499);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20500);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45513);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20501);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45514);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73973);
									s.is_empty()
								}
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
			super::push(out, 5262);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20502);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45515);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 45516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20503);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45517);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73974);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 98719);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116777);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20504);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20505);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45519);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 73975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 20506);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45520);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73976);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98720);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5263);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20507);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45521);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 73977);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98721);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 45522);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73978);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 98722);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 116778);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 73979);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98723);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 116779);
													s.is_empty()
												}
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
							super::push(out, 45523);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73980);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98724);
											s.is_empty()
										}
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
					super::push(out, 20508);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45524);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 73981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20509);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73982);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 73983);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98725);
											s.is_empty()
										}
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
					super::push(out, 20510);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20511);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45525);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20512);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45526);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73985);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 98726);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 116780);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20513);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5264);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20514);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45528);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73986);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98727);
											s.is_empty()
										}
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
					super::push(out, 20515);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20516);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45531);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t29::n5277(s, out),
		['ь', s @ ..] => {
			super::push(out, 5266);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яга.
pub(crate) fn n665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 664);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5267);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 5268);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20525);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45543);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73997);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98731);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', 'р', 'е', 'в', 'с', '\u{1}', s @ ..] => {
			super::push(out, 5269);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5270);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20526);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45544);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73998);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98732);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 116782);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20527);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45545);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73999);
									s.is_empty()
								}
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
			super::push(out, 5271);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20528);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20529);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20530);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45547);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 45548);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74000);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98733);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 45549);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 74001);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98734);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116783);
													s.is_empty()
												}
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
							super::push(out, 45550);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74002);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98735);
											s.is_empty()
										}
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
					super::push(out, 20531);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45551);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 74003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20532);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45552);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74004);
									s.is_empty()
								}
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
			super::push(out, 5272);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20533);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20534);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20535);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45555);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 45556);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 74005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 45557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74006);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20536);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20537);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45558);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 74007);
									s.is_empty()
								}
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
			super::push(out, 5274);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20538);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20539);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5275);
			match s {
				[] => true,
				['а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 20540);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20541);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45561);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 74008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'в', '\u{1}', s @ ..] => {
					super::push(out, 20542);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20543);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45562);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74009);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98736);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 116784);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20544);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45563);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 45564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t29::n20652(s, out),
				['т', s @ ..] => super::t29::n20653(s, out),
				['у', s @ ..] => {
					super::push(out, 20547);
					match s {
						[] => true,
						['м', s @ ..] => super::t29::n46007(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t29::n20655(s, out),
				['ь', s @ ..] => super::t29::n20656(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t29::n5288(s, out),
		['с', s @ ..] => super::t29::n5289(s, out),
		['т', s @ ..] => super::t29::n5290(s, out),
		_ => false,
	}
}

/// Words ending -тхеравада.
pub(crate) fn n118050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116791);
	s.is_empty()
}

/// Words ending -шахризада.
pub(crate) fn n118051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116792);
	s.is_empty()
}

/// Words ending -амадриада.
pub(crate) fn n118052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116793);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 128193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -амада.
pub(crate) fn n20691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20583);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 45610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 74044);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нада.
pub(crate) fn n5301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5289);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 20585);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45612);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 74046);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98758);
									s.is_empty()
								}
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
			super::push(out, 20587);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45613);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74047);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 98759);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116794);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128194);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n136312(s, out),
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
									super::push(out, 98760);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116795);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 128195);
													match s {
														[] => true,
														['т', s @ ..] => super::t29::n136313(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пада.
pub(crate) fn n5302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5290);
	s.is_empty()
}

/// Words ending -рада.
pub(crate) fn n5303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5291);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20588);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 20589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 20590);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 20591);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 45615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сада.
pub(crate) fn n5304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5292);
	s.is_empty()
}

/// Words ending -тада.
pub(crate) fn n5305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5293);
	s.is_empty()
}

/// Words ending -бда.
pub(crate) fn n667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 666);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5294);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20592);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45616);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 74049);
									s.is_empty()
								}
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
			super::push(out, 5295);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 20593);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вда.
pub(crate) fn n668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 667);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5296);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20595);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гда.
pub(crate) fn n669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 668);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5297);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20596);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45620);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74050);
									s.is_empty()
								}
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
			super::push(out, 5298);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20597);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45621);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 74051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45622);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дда.
pub(crate) fn n670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 669);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5299);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20599);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45623);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74053);
									s.is_empty()
								}
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
			super::push(out, 5300);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20600);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еда.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 670);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5301);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5302);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20602);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20603);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20604);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5304);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20606);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45627);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74054);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 98761);
											s.is_empty()
										}
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
					super::push(out, 20607);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 45628);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74055);
									s.is_empty()
								}
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
			super::push(out, 5305);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20608);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5306);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5307);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20609);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74056);
									s.is_empty()
								}
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
			super::push(out, 5308);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20610);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45631);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74057);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98762);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5309);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20611);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45632);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20612);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45633);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 45634);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74060);
									s.is_empty()
								}
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
			super::push(out, 5310);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 20613);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45635);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74061);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98763);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116796);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20614);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45636);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74062);
									s.is_empty()
								}
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
			super::push(out, 5311);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45637);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 74063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98764);
											s.is_empty()
										}
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
					super::push(out, 20616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20617);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45639);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 74064);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98765);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116797);
													s.is_empty()
												}
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
							super::push(out, 45640);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 45641);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 45642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20618);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45643);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74066);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98766);
											s.is_empty()
										}
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
					super::push(out, 20619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45644);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 45645);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 45646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5312);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20621);
					match s {
						[] => true,
						['б', s @ ..] => super::t29::n46084(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t29::n20730(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жда.
pub(crate) fn n672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 671);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5313);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20623);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 20624);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20625);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5314);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20626);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45651);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20627);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45652);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 45653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5315);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20628);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зда.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 672);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5316);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20629);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20630);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20631);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5317);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20632);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45657);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74070);
									s.is_empty()
								}
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
			super::push(out, 5318);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20633);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 45658);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 74071);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98770);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116800);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129548(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 116801);
													match s {
														[] => true,
														['р', s @ ..] => super::t29::n129549(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 98771);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 116802);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n129550(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 98772);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116803);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129551(s, out),
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
									super::push(out, 74072);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 98773);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 116804);
													match s {
														[] => true,
														['м', s @ ..] => super::t29::n129552(s, out),
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
									super::push(out, 74073);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98774);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 116805);
													match s {
														[] => true,
														['т', s @ ..] => super::t29::n129553(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 98775);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 74074);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98776);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116806);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129554(s, out),
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
									super::push(out, 74075);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 98777);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 116807);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 98778);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 116808);
													match s {
														[] => true,
														['к', s @ ..] => super::t29::n129556(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 98779);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116809);
													match s {
														[] => true,
														['р', s @ ..] => super::t29::n129557(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => super::t29::n74853(s, out),
								['х', s @ ..] => super::t29::n74854(s, out),
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
			super::push(out, 5319);
			match s {
				[] => true,
				['п', s @ ..] => super::t29::n20742(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t29::n5332(s, out),
		['о', s @ ..] => super::t29::n5333(s, out),
		['у', s @ ..] => super::t29::n5334(s, out),
		['ю', s @ ..] => super::t29::n5335(s, out),
		_ => false,
	}
}

/// Words ending -ида.
pub(crate) fn n674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5324);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20637);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45662);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 74079);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98784);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116813);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20638);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5325);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45663);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20641);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45664);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 74081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5326);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20643);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20644);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45665);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 20645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5327);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20646);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20647);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5328);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20648);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45668);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 74083);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98785);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45669);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74084);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98786);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 45670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20649);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45671);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 74085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 20650);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45672);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74086);
									s.is_empty()
								}
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
			super::push(out, 5329);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20652);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20653);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5331);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20655);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45674);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20656);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45675);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 74088);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 98787);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5332);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5333);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20658);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20659);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20660);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20661);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45677);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74089);
									match s {
										[] => true,
										['и', 'ц', 'к', 'о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 98788);
											s.is_empty()
										}
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
					super::push(out, 20662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5334);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45679);
							s.is_empty()
						}
						['л', s @ ..] => super::t29::n46116(s, out),
						['р', s @ ..] => {
							super::push(out, 45680);
							match s {
								[] => true,
								['и', s @ ..] => super::t29::n74869(s, out),
								_ => false,
							}
						}
						['х', s @ ..] => super::t29::n46118(s, out),
						['ш', s @ ..] => super::t29::n46119(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t29::n20773(s, out),
				['з', s @ ..] => super::t29::n20774(s, out),
				['и', s @ ..] => super::t29::n20775(s, out),
				['о', s @ ..] => super::t29::n20776(s, out),
				['р', s @ ..] => super::t29::n20777(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t29::n5347(s, out),
		['о', s @ ..] => super::t29::n5348(s, out),
		['п', s @ ..] => super::t29::n5349(s, out),
		['р', s @ ..] => super::t29::n5350(s, out),
		['с', s @ ..] => super::t29::n5351(s, out),
		['т', s @ ..] => super::t29::n5352(s, out),
		['у', s @ ..] => super::t29::n5353(s, out),
		['ф', s @ ..] => super::t29::n5354(s, out),
		['х', s @ ..] => super::t29::n5355(s, out),
		['ц', s @ ..] => super::t29::n5356(s, out),
		_ => false,
	}
}

/// Words ending -йда.
pub(crate) fn n675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5345);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20709);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20710);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20711);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5346);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5347);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20713);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20715);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74133);
									s.is_empty()
								}
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
			super::push(out, 5348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лда.
pub(crate) fn n676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5349);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20716);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45742);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 74134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20717);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45743);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20718);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20719);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45744);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 74136);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98812);
											s.is_empty()
										}
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
					super::push(out, 20720);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 20721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5350);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5351);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5352);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20724);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20725);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45746);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 74137);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98813);
											s.is_empty()
										}
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
					super::push(out, 20726);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45747);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 74138);
									s.is_empty()
								}
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
			super::push(out, 5353);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мда.
pub(crate) fn n677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	s.is_empty()
}

/// Words ending -нда.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5354);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45748);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 20729);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45749);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 45750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20730);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45751);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 74140);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98814);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 74141);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98815);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 74142);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98816);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116826);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129565(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45752);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74143);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98817);
											s.is_empty()
										}
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
					super::push(out, 20731);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45753);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 74144);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 74145);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 45754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45755);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 45756);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74146);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98819);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45757);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74147);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 74148);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98820);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116827);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129566(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45758);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74149);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98821);
											s.is_empty()
										}
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
					super::push(out, 20733);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45759);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74150);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 98822);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116828);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20735);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45761);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 45762);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 45763);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 74153);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 45764);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20736);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45765);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74155);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98824);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 45766);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 74156);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 98825);
											match s {
												[] => true,
												['н', s @ ..] => super::t29::n118088(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t29::n5367(s, out),
		['и', s @ ..] => super::t29::n5368(s, out),
		['о', s @ ..] => super::t29::n5369(s, out),
		['у', s @ ..] => super::t29::n5370(s, out),
		['ы', s @ ..] => super::t29::n5371(s, out),
		['я', s @ ..] => {
			super::push(out, 5360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ода.
pub(crate) fn n679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5361);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20753);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45788);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 74170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 45789);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 74171);
									s.is_empty()
								}
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
			super::push(out, 5362);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20754);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20755);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45790);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45791);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20757);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45792);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74174);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98833);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 45793);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74175);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98834);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116832);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 74176);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98835);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116833);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5363);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20758);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20759);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45795);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74177);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98836);
											s.is_empty()
										}
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
					super::push(out, 20760);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45796);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20761);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45797);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74179);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 98837);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 116834);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129570(s, out),
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
									super::push(out, 74180);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98838);
											s.is_empty()
										}
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
					super::push(out, 20762);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20763);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20764);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45799);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74181);
									s.is_empty()
								}
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
			super::push(out, 5364);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5365);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20766);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45800);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 74182);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98839);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5366);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5367);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20768);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45802);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 74183);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 98840);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116835);
													match s {
														[] => true,
														['е', s @ ..] => super::t29::n129571(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74184);
									s.is_empty()
								}
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
			super::push(out, 5368);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20769);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5369);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5370);
			match s {
				[] => true,
				['а', s @ ..] => super::t29::n20883(s, out),
				['и', s @ ..] => super::t29::n20884(s, out),
				['о', s @ ..] => super::t29::n20885(s, out),
				['с', s @ ..] => super::t29::n20886(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t29::n5383(s, out),
		['с', s @ ..] => super::t29::n5384(s, out),
		['т', s @ ..] => super::t29::n5385(s, out),
		_ => false,
	}
}

/// Words ending -рда.
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5374);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20784);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 20785);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20786);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20787);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 20788);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45822);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 74202);
									s.is_empty()
								}
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
			super::push(out, 5375);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20789);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45823);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74203);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 45824);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20790);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45825);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74205);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98851);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 116839);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20791);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45826);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74206);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98852);
											s.is_empty()
										}
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
					super::push(out, 20792);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5376);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20793);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5377);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20794);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 20795);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45829);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 74207);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 98853);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116840);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20796);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20797);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 45830);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 74208);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 98854);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116841);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20798);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45831);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 74209);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98855);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 116842);
													match s {
														[] => true,
														['д', s @ ..] => super::t29::n129573(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20799);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5378);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20800);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 20801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20802);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45834);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74210);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98856);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 116843);
													match s {
														[] => true,
														['у', s @ ..] => super::t29::n129574(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5379);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5380);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20804);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45835);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74211);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98857);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116844);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20805);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45836);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98858);
											s.is_empty()
										}
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
					super::push(out, 20806);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уда.
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 680);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5381);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5382);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5383);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20808);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74213);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 74214);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 98859);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5384);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5386);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5387);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20812);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45840);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74215);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98860);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 116845);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20813);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45841);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20814);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45842);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74217);
									s.is_empty()
								}
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
			super::push(out, 5388);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20815);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45843);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74218);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 98861);
											s.is_empty()
										}
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
					super::push(out, 20816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20817);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20818);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45844);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98862);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20819);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20820);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 45845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45846);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74220);
									s.is_empty()
								}
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
			super::push(out, 5391);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20822);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20823);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45847);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20824);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20825);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45848);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74222);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98863);
											s.is_empty()
										}
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
					super::push(out, 20826);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45849);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 74223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20827);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45850);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74224);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 98864);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20828);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20829);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5393);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20831);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5394);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20833);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45853);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыда.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 681);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5395);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20834);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45854);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74226);
									s.is_empty()
								}
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
			super::push(out, 5396);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьда.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 682);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5397);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20836);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45855);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 74227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20837);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45856);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74228);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98865);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116846);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20838);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74229);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 74230);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 98866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 45857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20839);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 45858);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 74231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20840);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эда.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 683);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5398);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20841);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юда.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 684);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5399);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20842);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20843);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45861);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74232);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98867);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 116847);
													match s {
														[] => true,
														['с', s @ ..] => super::t29::n129575(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20844);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45862);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яда.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 685);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5400);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5401);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20846);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5402);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20847);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5403);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45865);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74234);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 98868);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116848);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20849);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20850);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45867);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 74235);
									s.is_empty()
								}
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
			super::push(out, 5404);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20851);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20852);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20853);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45869);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 74236);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98869);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116849);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129576(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -авка-продажа.
pub(crate) fn n129577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128227);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 134921);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 138718);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 140861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упля-продажа.
pub(crate) fn n129578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128228);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 134922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пол-парижа.
pub(crate) fn n129579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128229);
	s.is_empty()
}

/// The word кидагаза.
pub(crate) fn n118117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116858);
	s.is_empty()
}

/// Words ending -танаклаза.
pub(crate) fn n118118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116859);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 128230);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -имераза.
pub(crate) fn n75037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74260);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 98885);
			match s {
				[] => true,
				['о', 'п', '-', 'м', s @ ..] => {
					super::push(out, 116860);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 128231);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 134924);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138719);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 140862);
													match s {
														[] => true,
														['и', s @ ..] => super::t29::n143548(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фераза.
pub(crate) fn n46347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45908);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 74261);
			match s {
				[] => true,
				['н', 'а', 'р', 'т', 'о', s @ ..] => {
					super::push(out, 98886);
					match s {
						[] => true,
						['н', 'и', 'м', 'а', 'н', s @ ..] => {
							super::push(out, 116861);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128232);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134925);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138720);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n142308(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ураза.
pub(crate) fn n21020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20908);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 45909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -таза.
pub(crate) fn n5471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5459);
	s.is_empty()
}

/// Words ending -уаза.
pub(crate) fn n5472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5460);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 20910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фаза.
pub(crate) fn n5473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5461);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20911);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 45911);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 74262);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 98887);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 116862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бза.
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 707);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5462);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5463);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20913);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дза.
pub(crate) fn n709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 708);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 5464);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20914);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20915);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 45913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еза.
pub(crate) fn n710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 709);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5465);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20916);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45914);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74265);
									s.is_empty()
								}
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
			super::push(out, 5466);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5467);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20917);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45915);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74266);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98888);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5468);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20919);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20920);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20921);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45917);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74267);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 98889);
											s.is_empty()
										}
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
					super::push(out, 20922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20923);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45919);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 45920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20924);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45921);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20925);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5471);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20926);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20927);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45923);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 45924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 45925);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20928);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 20929);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45926);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74271);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 74272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 45927);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74273);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98890);
											s.is_empty()
										}
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
					super::push(out, 20930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5472);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20931);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20932);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20933);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20934);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45928);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 74274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98891);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 98892);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 45929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 5473);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20935);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20936);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иза.
pub(crate) fn n711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 710);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5474);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5475);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20937);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74275);
									s.is_empty()
								}
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
			super::push(out, 5476);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5477);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20938);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45931);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									super::push(out, 74276);
									s.is_empty()
								}
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
			super::push(out, 5478);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20939);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20940);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20941);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45932);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20943);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74278);
									s.is_empty()
								}
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
			super::push(out, 5479);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5480);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20945);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45935);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74279);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98893);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5481);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45936);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 45937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5482);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45938);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 45939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20948);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20949);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5483);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20950);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5484);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 5485);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20953);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20954);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45943);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20955);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45944);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74282);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 98894);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йза.
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 711);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5487);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20956);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45945);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74283);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98895);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116863);
													match s {
														[] => true,
														['м', s @ ..] => super::t29::n129583(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лза.
pub(crate) fn n713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5488);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мза.
pub(crate) fn n714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 713);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5489);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5490);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20960);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45948);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нза.
pub(crate) fn n715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5491);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20961);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 20962);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45949);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74285);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98896);
											s.is_empty()
										}
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
					super::push(out, 20963);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5492);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5493);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5494);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20968);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5495);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20969);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оза.
pub(crate) fn n716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 715);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5496);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20970);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20971);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45956);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74286);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98897);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5498);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20972);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5499);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5500);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20973);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20974);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45958);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74287);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98898);
											s.is_empty()
										}
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
					super::push(out, 20975);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 20976);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45960);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74288);
									s.is_empty()
								}
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
			super::push(out, 5502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20977);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20978);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45961);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20979);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20980);
					s.is_empty()
				}
				['у', 'л', s @ ..] => {
					super::push(out, 20981);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 20982);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5503);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20983);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45963);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20984);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45964);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74290);
									s.is_empty()
								}
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
			super::push(out, 5504);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20985);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45965);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98899);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5505);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5506);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20986);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 20987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45966);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 45967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5507);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20988);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45968);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 45969);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74292);
									s.is_empty()
								}
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
			super::push(out, 5508);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рза.
pub(crate) fn n717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 716);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5509);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5510);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45970);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74293);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 74294);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98900);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45971);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74295);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98901);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116864);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129584(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5511);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45973);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 45974);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74296);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98902);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116865);
													s.is_empty()
												}
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
							super::push(out, 45975);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 74297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 45976);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 45977);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74299);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 45978);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74300);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98903);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116866);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 98904);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 116867);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5512);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45979);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 45980);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 45981);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74301);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98905);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116868);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5513);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20995);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45982);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74302);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98906);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5514);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уза.
pub(crate) fn n718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5515);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20997);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45983);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 74303);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 74304);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 74305);
									s.is_empty()
								}
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
			super::push(out, 5516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20998);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20999);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45984);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74306);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98907);
											s.is_empty()
										}
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
					super::push(out, 21000);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21001);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45985);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 74307);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98908);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116869);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129585(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5517);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21002);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5518);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21003);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21004);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5519);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21005);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21006);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45988);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 74308);
									s.is_empty()
								}
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
			super::push(out, 5520);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5521);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21007);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45989);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74309);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98909);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5522);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 21008);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 45990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5523);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21009);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21010);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 45991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21011);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45992);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74311);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98910);
											s.is_empty()
										}
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
					super::push(out, 21012);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45993);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 74312);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98911);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5524);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5525);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21014);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5526);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21015);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыза.
pub(crate) fn n719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 718);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 5527);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5528);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21016);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45996);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 45997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьза.
pub(crate) fn n720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 719);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5529);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21017);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 21018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юза.
pub(crate) fn n721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 720);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5530);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21019);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45999);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74313);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98912);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116870);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5532);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21020);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 46000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яза.
pub(crate) fn n722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 721);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5533);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21021);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46001);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74314);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98913);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116871);
													match s {
														[] => true,
														['м', s @ ..] => super::t29::n129586(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ападравиа.
pub(crate) fn n118131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116872);
	s.is_empty()
}

/// Words ending -асс-медиа.
pub(crate) fn n118132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116873);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 128237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рич-медиа.
pub(crate) fn n118133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116874);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 128238);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льтимедиа.
pub(crate) fn n118134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116875);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 128239);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 134927);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ипермедиа.
pub(crate) fn n118135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116876);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 128240);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -неумывака.
pub(crate) fn n118136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116877);
	s.is_empty()
}

/// Words ending -неунывака.
pub(crate) fn n118137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116878);
	s.is_empty()
}

/// Words ending -нака.
pub(crate) fn n5557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5545);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21042);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 46022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 21043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 46023);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 74331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лоака.
pub(crate) fn n21156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21044);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 46024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пака.
pub(crate) fn n5559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5547);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21045);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 21046);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 46025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21047);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46026);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 74333);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98924);
									s.is_empty()
								}
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
			super::push(out, 21048);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 46027);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 74334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рака.
pub(crate) fn n5560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5548);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21049);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 46028);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 46029);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 74335);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 98926);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 116879);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 128241);
													match s {
														[] => true,
														['у', s @ ..] => super::t29::n136333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 21050);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сака.
pub(crate) fn n5561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5549);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 21052);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 21053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 46031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 21054);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 21055);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 46032);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 46033);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -така.
pub(crate) fn n5562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5550);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21056);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 46034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 74336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хака.
pub(crate) fn n5563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5551);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 21057);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 46035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бка.
pub(crate) fn n727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 726);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5552);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46036);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74337);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98927);
											s.is_empty()
										}
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
					super::push(out, 21059);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 46037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 21060);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 21061);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 46038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21062);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46039);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 46040);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21063);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 46041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5553);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21064);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46042);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74339);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98928);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 74340);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98929);
											s.is_empty()
										}
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
					super::push(out, 21065);
					match s {
						[] => true,
						['х', 'о', 'в', s @ ..] => {
							super::push(out, 46043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21066);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 46044);
							match s {
								[] => true,
								['з', 'о', 'м', '\u{1}', s @ ..] => {
									super::push(out, 74341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21067);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 46045);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 74342);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 98930);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116880);
													s.is_empty()
												}
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
							super::push(out, 46046);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 74343);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 98931);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 116881);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129592(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21068);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 21069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5554);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21070);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 46047);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74344);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98932);
											s.is_empty()
										}
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
					super::push(out, 21071);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46048);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74345);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98933);
											s.is_empty()
										}
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
					super::push(out, 21072);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46049);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 46050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98934);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5555);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21073);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 46051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21074);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46052);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21075);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 74348);
									s.is_empty()
								}
								['к', 'п', s @ ..] => {
									super::push(out, 74349);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98935);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 116882);
													match s {
														[] => true,
														['х', s @ ..] => super::t29::n129593(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t29::n46493(s, out),
						['т', s @ ..] => super::t29::n46494(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t29::n21189(s, out),
				['ф', s @ ..] => {
					super::push(out, 21077);
					match s {
						[] => true,
						['о', s @ ..] => super::t29::n46497(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t29::n5568(s, out),
		['у', s @ ..] => super::t29::n5569(s, out),
		['ы', s @ ..] => super::t29::n5570(s, out),
		['ю', s @ ..] => super::t29::n5571(s, out),
		_ => false,
	}
}

/// Words ending -вка.
pub(crate) fn n728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 727);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5560);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21093);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 21094);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 46077);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74368);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98950);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116889);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21095);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21096);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46078);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74369);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 98951);
											s.is_empty()
										}
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
					super::push(out, 21097);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 46079);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 46080);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 46081);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74370);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98952);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116890);
													match s {
														[] => true,
														['в', s @ ..] => super::t29::n129598(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 98953);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 74371);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 74372);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 98954);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116891);
													s.is_empty()
												}
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
							super::push(out, 46082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74373);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 74374);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 98955);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 116892);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 98956);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 116893);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46084);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 46085);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 46086);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74376);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98957);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116894);
													s.is_empty()
												}
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
							super::push(out, 46087);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74377);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 98958);
											s.is_empty()
										}
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
					super::push(out, 21100);
					match s {
						[] => true,
						['п', s @ ..] => super::t29::n46528(s, out),
						['т', 'ы', s @ ..] => {
							super::push(out, 46088);
							match s {
								[] => true,
								['в', s @ ..] => super::t29::n75158(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t29::n21214(s, out),
				['т', s @ ..] => super::t29::n21215(s, out),
				['х', s @ ..] => super::t29::n21216(s, out),
				['ш', s @ ..] => super::t29::n21217(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t29::n5573(s, out),
		['и', s @ ..] => super::t29::n5574(s, out),
		['л', s @ ..] => super::t29::n5575(s, out),
		['о', s @ ..] => super::t29::n5576(s, out),
		['у', s @ ..] => super::t29::n5577(s, out),
		['ы', s @ ..] => super::t29::n5578(s, out),
		['ю', s @ ..] => super::t29::n5579(s, out),
		['я', s @ ..] => super::t29::n5580(s, out),
		_ => false,
	}
}

/// Words ending -гка.
pub(crate) fn n729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 728);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5569);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21168);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 46286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дка.
pub(crate) fn n730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 729);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5570);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21169);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 21170);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 21171);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 46287);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 74677);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99227);
											s.is_empty()
										}
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
					super::push(out, 21172);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 46288);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 74678);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 99228);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117071);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21173);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46289);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21174);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 46290);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74680);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99229);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 74681);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 99230);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'и', 'н', 'и', s @ ..] => {
									super::push(out, 74682);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 99231);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99232);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 99233);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 117072);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 99234);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 117073);
													s.is_empty()
												}
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
								['л', 'о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 74683);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 74684);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 99235);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 117074);
													s.is_empty()
												}
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
							super::push(out, 46291);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21175);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46292);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74686);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 99236);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 117075);
													match s {
														[] => true,
														['е', s @ ..] => super::t29::n129708(s, out),
														_ => false,
													}
												}
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
							super::push(out, 46293);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74687);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99237);
											s.is_empty()
										}
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
					super::push(out, 21176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 21177);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46295);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 74688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 21178);
					match s {
						[] => true,
						['о', s @ ..] => super::t29::n46740(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t29::n5583(s, out),
		['з', s @ ..] => {
			super::push(out, 5572);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21188);
					match s {
						[] => true,
						['о', s @ ..] => super::t29::n46757(s, out),
						['ы', s @ ..] => super::t29::n46758(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t29::n5585(s, out),
		['н', s @ ..] => super::t29::n5586(s, out),
		['о', s @ ..] => super::t29::n5587(s, out),
		['р', s @ ..] => super::t29::n5588(s, out),
		['у', s @ ..] => super::t29::n5589(s, out),
		['ь', s @ ..] => super::t29::n5590(s, out),
		['ю', 'л', s @ ..] => super::t29::n5591(s, out),
		['я', s @ ..] => super::t29::n5592(s, out),
		_ => false,
	}
}

/// Words ending -ека.
pub(crate) fn n731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 730);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5580);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21218);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46362);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74763);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99302);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5581);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 5582);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5583);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5584);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21221);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 46363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21222);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46364);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74764);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99303);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5585);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21223);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 46365);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 74765);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99304);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5586);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5587);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21225);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 21226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46366);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74766);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 99305);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 117122);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129730(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46367);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 46368);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 74767);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 99306);
											s.is_empty()
										}
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
					super::push(out, 21228);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 46369);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 46370);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 74768);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 99307);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117123);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5588);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21229);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 46371);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 46372);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 46373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21231);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 46375);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 46376);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 46377);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74771);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 99308);
											s.is_empty()
										}
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
					super::push(out, 21232);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 46378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21233);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46379);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74772);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 99309);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117124);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t29::n21348(s, out),
				['п', s @ ..] => {
					super::push(out, 21235);
					match s {
						[] => true,
						['а', s @ ..] => super::t29::n46830(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5590);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 5591);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жка.
pub(crate) fn n732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 731);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5592);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21238);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21239);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46384);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 74776);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99312);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 46385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21240);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46386);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74777);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 99313);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 117129);
													match s {
														[] => true,
														['т', s @ ..] => super::t29::n129735(s, out),
														_ => false,
													}
												}
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
							super::push(out, 46387);
							match s {
								[] => true,
								['х', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 99314);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 99315);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5593);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 21241);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 46388);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74778);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 99316);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5594);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 21242);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46389);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74779);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 99317);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 117130);
													match s {
														[] => true,
														['б', s @ ..] => super::t29::n129736(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21243);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46390);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 74780);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 99318);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 117131);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129737(s, out),
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
									super::push(out, 74781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21244);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46391);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74782);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 99319);
											s.is_empty()
										}
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
					super::push(out, 21245);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 21246);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 46392);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74783);
									match s {
										[] => true,
										['-', s @ ..] => super::t29::n100396(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 46393);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 46394);
							match s {
								[] => true,
								['в', s @ ..] => super::t29::n75568(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t29::n21361(s, out),
				['н', s @ ..] => super::t29::n21362(s, out),
				['о', s @ ..] => super::t29::n21363(s, out),
				['р', s @ ..] => super::t29::n21364(s, out),
				['т', s @ ..] => super::t29::n21365(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t29::n5608(s, out),
		['о', s @ ..] => super::t29::n5609(s, out),
		['р', s @ ..] => super::t29::n5610(s, out),
		['у', s @ ..] => super::t29::n5611(s, out),
		['ы', s @ ..] => super::t29::n5612(s, out),
		['ю', s @ ..] => super::t29::n5613(s, out),
		['я', s @ ..] => super::t29::n5614(s, out),
		_ => false,
	}
}

/// Words ending -зка.
pub(crate) fn n733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 732);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5602);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 21280);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 46459);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74863);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 99384);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 46460);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 74864);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 99385);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 117173);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n129757(s, out),
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
									super::push(out, 74865);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99386);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 117174);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 74866);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 99387);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 117175);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 99388);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 117176);
													s.is_empty()
												}
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
							super::push(out, 46461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21281);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 46462);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74867);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 74868);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 99389);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 117177);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n129758(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 99390);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 117178);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 99391);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 99392);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 74869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 46463);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 74870);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99393);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 117179);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129759(s, out),
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
									super::push(out, 74871);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 99394);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 117180);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 74872);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 99395);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 46464);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74873);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99396);
											s.is_empty()
										}
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
					super::push(out, 21282);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46465);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 46466);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 46467);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74874);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99397);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t29::n21397(s, out),
				['р', s @ ..] => {
					super::push(out, 21284);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t29::n21399(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t29::n5616(s, out),
		['и', s @ ..] => super::t29::n5617(s, out),
		['о', s @ ..] => super::t29::n5618(s, out),
		['у', s @ ..] => super::t29::n5619(s, out),
		['ы', 'г', 'р', 'ы', 'к', '\u{1}', s @ ..] => super::t29::n5620(s, out),
		['э', s @ ..] => super::t29::n5621(s, out),
		['я', s @ ..] => super::t29::n5622(s, out),
		_ => false,
	}
}

/// Words ending -ика.
pub(crate) fn n734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 733);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5610);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46505);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 46506);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 74908);
									s.is_empty()
								}
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
			super::push(out, 5611);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21305);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46507);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 74909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 21306);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 46508);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21307);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21308);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 46510);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 74911);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 99426);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5612);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21309);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 46511);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 74912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21310);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 46512);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 74913);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 99427);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5614);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21312);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46513);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 74914);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99428);
											s.is_empty()
										}
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
					super::push(out, 21313);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46514);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 74915);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 99429);
											s.is_empty()
										}
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
					super::push(out, 21314);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 21315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5615);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46515);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74916);
									s.is_empty()
								}
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
			super::push(out, 5616);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21317);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 46516);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 74917);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 99430);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117203);
													s.is_empty()
												}
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
							super::push(out, 46517);
							match s {
								[] => true,
								['о', 'и', s @ ..] => {
									super::push(out, 74918);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 99431);
											s.is_empty()
										}
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
					super::push(out, 21318);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 46518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74919);
									s.is_empty()
								}
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
			super::push(out, 5617);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5618);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21319);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21320);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 46519);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 74920);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 99432);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 46520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 74921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t29::n46969(s, out),
						['у', s @ ..] => super::t29::n46970(s, out),
						_ => false,
					}
				}
				['б', s @ ..] => super::t29::n21436(s, out),
				['в', s @ ..] => super::t29::n21437(s, out),
				['е', s @ ..] => super::t29::n21438(s, out),
				['и', s @ ..] => super::t29::n21439(s, out),
				['к', s @ ..] => super::t29::n21440(s, out),
				['о', s @ ..] => super::t29::n21441(s, out),
				['п', s @ ..] => super::t29::n21442(s, out),
				['т', s @ ..] => super::t29::n21443(s, out),
				['у', s @ ..] => super::t29::n21444(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t29::n5632(s, out),
		['н', s @ ..] => super::t29::n5633(s, out),
		['п', s @ ..] => super::t29::n5634(s, out),
		['р', s @ ..] => super::t29::n5635(s, out),
		['с', s @ ..] => super::t29::n5636(s, out),
		['т', s @ ..] => super::t29::n5637(s, out),
		['ф', s @ ..] => super::t29::n5638(s, out),
		['х', s @ ..] => super::t29::n5639(s, out),
		['ч', s @ ..] => super::t29::n5640(s, out),
		['ш', s @ ..] => super::t29::n5641(s, out),
		['щ', s @ ..] => super::t29::n5642(s, out),
		['ь', s @ ..] => super::t29::n5643(s, out),
		_ => false,
	}
}

/// Words ending -йка.
pub(crate) fn n735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 734);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5631);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21376);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46606);
							match s {
								[] => true,
								['н', 'с', s @ ..] => {
									super::push(out, 75021);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 99515);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 117247);
													match s {
														[] => true,
														['к', s @ ..] => super::t29::n129790(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21377);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46607);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 75022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 46608);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 75023);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 99516);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 117248);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 75024);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 99517);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 117249);
													s.is_empty()
												}
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
							super::push(out, 46609);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 75025);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99518);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 117250);
													match s {
														[] => true,
														['п', 'е', 'н', '-', s @ ..] => super::t29::n129791(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'р', 'о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 46610);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 46611);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 46612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 21378);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46613);
							match s {
								[] => true,
								['н', s @ ..] => super::t29::n75814(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t29::n47067(s, out),
						['р', s @ ..] => super::t29::n47068(s, out),
						_ => false,
					}
				}
				['д', s @ ..] => super::t29::n21494(s, out),
				['ж', s @ ..] => super::t29::n21495(s, out),
				['з', s @ ..] => super::t29::n21496(s, out),
				['л', s @ ..] => super::t29::n21497(s, out),
				['м', s @ ..] => super::t29::n21498(s, out),
				['н', s @ ..] => super::t29::n21499(s, out),
				['п', s @ ..] => super::t29::n21500(s, out),
				['р', s @ ..] => super::t29::n21501(s, out),
				['с', s @ ..] => super::t29::n21502(s, out),
				['т', s @ ..] => super::t29::n21503(s, out),
				['ф', s @ ..] => super::t29::n21504(s, out),
				['х', s @ ..] => super::t29::n21505(s, out),
				['ч', s @ ..] => super::t29::n21506(s, out),
				['ш', s @ ..] => super::t29::n21507(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t29::n5645(s, out),
		['и', s @ ..] => super::t29::n5646(s, out),
		['о', s @ ..] => super::t29::n5647(s, out),
		['у', s @ ..] => super::t29::n5648(s, out),
		['ы', s @ ..] => super::t29::n5649(s, out),
		['э', s @ ..] => super::t29::n5650(s, out),
		['ю', s @ ..] => super::t29::n5651(s, out),
		['я', s @ ..] => super::t30::n5652(s, out),
		_ => false,
	}
}

/// Words ending -кка.
pub(crate) fn n736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 735);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5639);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21451);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 46772);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 75187);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 99643);
											s.is_empty()
										}
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
					super::push(out, 21452);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46773);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 75188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21453);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46774);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 75189);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 99644);
											s.is_empty()
										}
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
					super::push(out, 21454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5640);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21455);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 46775);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 75190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21456);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 46776);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 75191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 21457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5641);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 21458);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 46777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 21459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5642);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 21460);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 46779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5643);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лка.
pub(crate) fn n737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 736);
	match s {
		[] => true,
		['а', s @ ..] => super::t30::n5659(s, out),
		['е', s @ ..] => {
			super::push(out, 5646);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46859);
							s.is_empty()
						}
						['е', 'т', s @ ..] => {
							super::push(out, 46860);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 75285);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99717);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 117373);
													s.is_empty()
												}
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
							super::push(out, 46861);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 46862);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 75286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 99718);
											s.is_empty()
										}
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
					super::push(out, 21484);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 46863);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 75287);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 99719);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 117374);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46864);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 75288);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99720);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 117375);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t30::n129858(s, out),
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
									super::push(out, 75289);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 99721);
											match s {
												[] => true,
												['о', s @ ..] => super::t30::n118638(s, out),
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
							super::push(out, 46865);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 46866);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 46867);
							s.is_empty()
						}
						['и', s @ ..] => super::t30::n47329(s, out),
						['о', s @ ..] => super::t30::n47330(s, out),
						['с', s @ ..] => super::t30::n47331(s, out),
						['ы', s @ ..] => super::t30::n47332(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t30::n21605(s, out),
				['м', s @ ..] => super::t30::n21606(s, out),
				['п', s @ ..] => super::t30::n21607(s, out),
				['р', s @ ..] => super::t30::n21608(s, out),
				['с', s @ ..] => super::t30::n21609(s, out),
				['т', s @ ..] => super::t30::n21610(s, out),
				['ц', '\u{1}', s @ ..] => super::t30::n21611(s, out),
				['ч', s @ ..] => super::t30::n21612(s, out),
				['щ', s @ ..] => super::t30::n21613(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t30::n5661(s, out),
		['л', s @ ..] => super::t30::n5662(s, out),
		['о', s @ ..] => super::t30::n5663(s, out),
		['у', s @ ..] => super::t30::n5664(s, out),
		['ы', s @ ..] => super::t30::n5665(s, out),
		['э', s @ ..] => super::t30::n5666(s, out),
		['ю', s @ ..] => super::t30::n5667(s, out),
		['я', s @ ..] => super::t30::n5668(s, out),
		_ => false,
	}
}

/// Words ending -мка.
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21546);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 46969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 21547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 46970);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 75406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 46971);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 75407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21548);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 21549);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21550);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 46972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46973);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 46974);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 75408);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 99807);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 117425);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21552);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 46975);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 75409);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 99808);
											s.is_empty()
										}
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
					super::push(out, 21553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 21554);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 46977);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 75410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 46978);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 75411);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 99809);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 117426);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21555);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 46979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21556);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 46980);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 46981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 21557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21558);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 46983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21559);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21560);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 46984);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 75412);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 99810);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 117427);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21561);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 46985);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 75413);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 99811);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 117428);
													match s {
														[] => true,
														['р', s @ ..] => super::t30::n129890(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t30::n21681(s, out),
				['и', s @ ..] => {
					super::push(out, 21563);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 46990);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 75419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 21564);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 21565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 46991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t30::n21685(s, out),
				['с', s @ ..] => {
					super::push(out, 21567);
					s.is_empty()
				}
				['ф', s @ ..] => super::t30::n21687(s, out),
				['ш', s @ ..] => {
					super::push(out, 21569);
					match s {
						[] => true,
						['е', s @ ..] => super::t30::n47459(s, out),
						_ => false,
					}
				}
				['ъ', s @ ..] => super::t30::n21689(s, out),
				['ы', s @ ..] => super::t30::n21690(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t30::n5671(s, out),
		['м', s @ ..] => super::t30::n5672(s, out),
		['о', s @ ..] => super::t30::n5673(s, out),
		['р', s @ ..] => super::t30::n5674(s, out),
		['у', s @ ..] => super::t30::n5675(s, out),
		['ы', s @ ..] => super::t30::n5676(s, out),
		['э', s @ ..] => {
			super::push(out, 5663);
			s.is_empty()
		}
		['ю', s @ ..] => super::t30::n5678(s, out),
		['я', s @ ..] => super::t30::n5679(s, out),
		_ => false,
	}
}

/// Words ending -нка.
pub(crate) fn n739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 738);
	match s {
		[] => true,
		['а', s @ ..] => super::t30::n5680(s, out),
		['е', s @ ..] => super::t30::n5681(s, out),
		['и', s @ ..] => super::t30::n5682(s, out),
		['й', s @ ..] => {
			super::push(out, 5669);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21670);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 47437);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 76040);
									s.is_empty()
								}
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
			super::push(out, 5670);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21671);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 47438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t30::n5685(s, out),
		['р', s @ ..] => {
			super::push(out, 5672);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21691);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 47512);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 47513);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21692);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 47514);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 76118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 21693);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 47515);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 76119);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 100383);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 117749);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5673);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21694);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 47516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 21695);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47517);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76120);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 100384);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 117750);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21696);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 21697);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 21698);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47518);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76121);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 100385);
											s.is_empty()
										}
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
					super::push(out, 21699);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 47519);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 76122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100386);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 47520);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 76123);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 100387);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 117751);
													s.is_empty()
												}
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
							super::push(out, 47521);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 76124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21700);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 47522);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 76125);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 100388);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117752);
													s.is_empty()
												}
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
							super::push(out, 47523);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 76126);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 100389);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 117753);
													match s {
														[] => true,
														['с', s @ ..] => super::t30::n130044(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'ш', s @ ..] => {
					super::push(out, 21701);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 47524);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 76127);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100390);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5674);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21702);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 21703);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 47526);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21704);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21705);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 5675);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5676);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21706);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 47528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t30::n5691(s, out),
		_ => false,
	}
}

/// Words ending -ока.
pub(crate) fn n740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 739);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5678);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5679);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21723);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 47606);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 76226);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 100465);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5680);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21724);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21725);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 47607);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 76227);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 100466);
											s.is_empty()
										}
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
					super::push(out, 21726);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 47608);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 76228);
									s.is_empty()
								}
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
			super::push(out, 5681);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 21727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47609);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 76229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5682);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5683);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21729);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 47611);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 76230);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 100467);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 117785);
													match s {
														[] => true,
														['п', s @ ..] => super::t30::n130062(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21730);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 47612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21731);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 47613);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76231);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 100468);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 100469);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 100470);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 76232);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 100471);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 117786);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 76233);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 100472);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 117787);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 76234);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 100473);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 117788);
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
										['п', s @ ..] => {
											super::push(out, 100474);
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 100475);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 76235);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 100476);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 47614);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 47615);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 76236);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 100477);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 117789);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 76237);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 100478);
											s.is_empty()
										}
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
					super::push(out, 21732);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47616);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 47617);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 76238);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 100479);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117790);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5684);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21734);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5685);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 21735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 47619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5686);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t30::n5701(s, out),
		['с', s @ ..] => super::t30::n5702(s, out),
		['т', s @ ..] => super::t30::n5703(s, out),
		['ф', s @ ..] => {
			super::push(out, 5690);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

