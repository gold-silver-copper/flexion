use flexion::Path;

/// Words ending -хтон.
pub(crate) fn n13406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13333);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34821);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62927);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 62928);
					match s {
						[] => true,
						['в', 'а', 'р', s @ ..] => {
							super::push(out, 89823);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110363);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123861);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 89824);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штон.
pub(crate) fn n13407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13334);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 34822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтон.
pub(crate) fn n13408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13335);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34823);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 62930);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89825);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110365);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 110366);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123862);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132023);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 62931);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этон.
pub(crate) fn n13409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13336);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34824);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 62932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютон.
pub(crate) fn n13410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13337);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 34825);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 62933);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89827);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110367);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123863);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132024);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word франкофон.
pub(crate) fn n133385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132025);
	s.is_empty()
}

/// Words ending -едь-кликун.
pub(crate) fn n133390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132030);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 136848);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жук-щелкун.
pub(crate) fn n133391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132031);
	s.is_empty()
}

/// Words ending -от-полоскун.
pub(crate) fn n133392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132032);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136849);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыба-клоун.
pub(crate) fn n133393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132033);
	s.is_empty()
}

/// Words ending -бедь-шипун.
pub(crate) fn n133394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132034);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136850);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139740);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ка-хохотун.
pub(crate) fn n133395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132035);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 136851);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139741);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 141441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кино-дулево.
pub(crate) fn n138266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136853);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ехово-зуево.
pub(crate) fn n138267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136854);
	s.is_empty()
}

/// Words ending -шереметьево.
pub(crate) fn n138268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136855);
	s.is_empty()
}

/// Words ending -клементьево.
pub(crate) fn n138269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136856);
	s.is_empty()
}

/// Words ending -емиум-пиво.
pub(crate) fn n133403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132043);
	s.is_empty()
}

/// Words ending -статус-кво.
pub(crate) fn n133404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132044);
	s.is_empty()
}

/// Words ending -ьниково.
pub(crate) fn n90953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89993);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 110462);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 123913);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 132045);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136857);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139744);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 132046);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 136858);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 139745);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риково.
pub(crate) fn n63892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63231);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 89994);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 110463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лково.
pub(crate) fn n35555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35214);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63232);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 89995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 63233);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 89996);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63234);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 89997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сково.
pub(crate) fn n35556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35215);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 63235);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уково.
pub(crate) fn n35557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35216);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 63236);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 63237);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 89999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 63238);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90000);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 110466);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 123914);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132047);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 136859);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шково.
pub(crate) fn n35558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35217);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63239);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 90001);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110467);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 123915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьково.
pub(crate) fn n35559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35218);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 63240);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90002);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 110468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 63241);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90003);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 110469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 90004);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яково.
pub(crate) fn n35560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35219);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 63242);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 90005);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110471);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 123916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лово.
pub(crate) fn n13712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35220);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63243);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90006);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110472);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123917);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ъ', s @ ..] => {
									super::push(out, 110473);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 123918);
											s.is_empty()
										}
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
					super::push(out, 63244);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 63245);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35221);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35222);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63247);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 63248);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90010);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110474);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123919);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132048);
													match s {
														[] => true,
														['о', s @ ..] => super::t47::n138273(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 63249);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 90011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 35223);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63250);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 90012);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 110475);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 123920);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 90013);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110476);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123921);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132049);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35224);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 35225);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 63251);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 35226);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 63252);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90015);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110477);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123922);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132050);
													s.is_empty()
												}
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
							super::push(out, 90016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 63253);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 90017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 63254);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 90018);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110479);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123923);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35227);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 63255);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 90019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35228);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 63256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 35229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ново.
pub(crate) fn n13713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13639);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35230);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63257);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90020);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110480);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 110481);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 110482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 63258);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 63259);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90022);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 63260);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90023);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 110484);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123924);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132051);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 63261);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90024);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110485);
									s.is_empty()
								}
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
			super::push(out, 35231);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63262);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 90025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 63263);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 90026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35232);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63264);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90027);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 63265);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90028);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110487);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123925);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35233);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 63266);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 90029);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 110488);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123926);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132052);
													match s {
														[] => true,
														['и', s @ ..] => super::t47::n138274(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35234);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 63267);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 90030);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110489);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123927);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пово.
pub(crate) fn n13714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13640);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 35235);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63268);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90031);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 110490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рово.
pub(crate) fn n13715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13641);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63269);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90032);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 110491);
									s.is_empty()
								}
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
			super::push(out, 35237);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 63270);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90033);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 63271);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35238);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63272);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90035);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 110493);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123928);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132053);
													s.is_empty()
												}
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
							super::push(out, 90036);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110494);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 123929);
											s.is_empty()
										}
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
					super::push(out, 63273);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90037);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 63274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35239);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63275);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 63276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 35240);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 35241);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63277);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 90039);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110496);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123930);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 132054);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сово.
pub(crate) fn n13716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13642);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35242);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35243);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63279);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90040);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110497);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 123931);
											s.is_empty()
										}
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
					super::push(out, 63280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 63281);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90042);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 110498);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123932);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35244);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63282);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35245);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 63283);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 90044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 63284);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 90045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тово.
pub(crate) fn n13717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13643);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35246);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63285);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 90046);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 63286);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 63287);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110500);
									s.is_empty()
								}
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
			super::push(out, 35247);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 35248);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63289);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 90049);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110501);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123933);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35249);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35250);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хово.
pub(crate) fn n13718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13644);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35251);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63292);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 90050);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110502);
									s.is_empty()
								}
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
			super::push(out, 35252);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63293);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35253);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63294);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90052);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110503);
									s.is_empty()
								}
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
			super::push(out, 35254);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 63295);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90053);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ство.
pub(crate) fn n13721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13647);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 35258);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 63298);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90054);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 110505);
									s.is_empty()
								}
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
			super::push(out, 35259);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63299);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 63300);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90055);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 90056);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 110506);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123934);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132055);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t47::n138275(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 63301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 35260);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63302);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90057);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 110507);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 110508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 63303);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 90058);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 90059);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 63304);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 63305);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 90060);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 110510);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 110511);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 110512);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123935);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 90061);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 110513);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123936);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 110514);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 123937);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 90062);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110515);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123938);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 110516);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 123939);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 132056);
													s.is_empty()
												}
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
							super::push(out, 90063);
							match s {
								[] => true,
								['у', s @ ..] => super::t47::n111686(s, out),
								['ь', s @ ..] => {
									super::push(out, 110518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t47::n91024(s, out),
						['п', s @ ..] => super::t47::n91025(s, out),
						['р', s @ ..] => super::t47::n91026(s, out),
						['с', s @ ..] => super::t47::n91027(s, out),
						['т', s @ ..] => super::t47::n91028(s, out),
						['х', s @ ..] => super::t47::n91029(s, out),
						['ц', s @ ..] => super::t47::n91030(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t47::n63967(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t47::n35602(s, out),
		['д', s @ ..] => super::t47::n35603(s, out),
		['е', s @ ..] => super::t47::n35604(s, out),
		['ж', s @ ..] => super::t47::n35605(s, out),
		['й', s @ ..] => super::t47::n35606(s, out),
		['к', s @ ..] => super::t47::n35607(s, out),
		['м', s @ ..] => super::t47::n35608(s, out),
		['н', s @ ..] => super::t47::n35609(s, out),
		['о', s @ ..] => super::t47::n35610(s, out),
		['п', s @ ..] => super::t47::n35611(s, out),
		['р', s @ ..] => super::t47::n35612(s, out),
		['с', s @ ..] => super::t47::n35613(s, out),
		['т', s @ ..] => super::t48::n35614(s, out),
		['у', s @ ..] => super::t48::n35615(s, out),
		['ф', s @ ..] => super::t48::n35616(s, out),
		['ь', s @ ..] => super::t48::n35617(s, out),
		['я', s @ ..] => super::t48::n35618(s, out),
		_ => false,
	}
}

/// Words ending -эуэтенанго.
pub(crate) fn n133607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132245);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 137002);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -то-доминго.
pub(crate) fn n133608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132246);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137003);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суперкарго.
pub(crate) fn n133609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132247);
	s.is_empty()
}

/// Words ending -компримарио.
pub(crate) fn n138420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137004);
	s.is_empty()
}

/// Words ending -бальзамико.
pub(crate) fn n133613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132251);
	s.is_empty()
}

/// Words ending -ью-мексико.
pub(crate) fn n133614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132252);
	s.is_empty()
}

/// Words ending -вильянсико.
pub(crate) fn n133615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132253);
	s.is_empty()
}

/// Words ending -рто-франко.
pub(crate) fn n133617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132255);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онанищенко.
pub(crate) fn n133618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132256);
	s.is_empty()
}

/// Words ending -платереско.
pub(crate) fn n133619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132257);
	s.is_empty()
}

/// Words ending -ишко.
pub(crate) fn n13954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13880);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35582);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63619);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90399);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110876);
									s.is_empty()
								}
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
			super::push(out, 35583);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63620);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90400);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110877);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 110878);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124226);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132258);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35584);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 63621);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90401);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 63622);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 90402);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 90403);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110880);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 110881);
									match s {
										[] => true,
										['г', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132259);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 132260);
													s.is_empty()
												}
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
							super::push(out, 90404);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110882);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 63623);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90405);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110883);
									s.is_empty()
								}
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
			super::push(out, 35585);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63624);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90406);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 110884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 63625);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 90407);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110885);
									s.is_empty()
								}
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
			super::push(out, 35586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63626);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90408);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110886);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124227);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132261);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 63627);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 90409);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110887);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124228);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132262);
													match s {
														[] => true,
														['п', s @ ..] => super::t48::n138422(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35587);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63628);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 90410);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110888);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 124229);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 132263);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 63629);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90411);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 90412);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 90413);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 63630);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90414);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 63631);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 90415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 63632);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90416);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 110892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 63633);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90417);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110893);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124230);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132264);
													s.is_empty()
												}
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
							super::push(out, 90418);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t48::n64296(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t48::n35929(s, out),
		['н', s @ ..] => super::t48::n35930(s, out),
		['п', s @ ..] => super::t48::n35931(s, out),
		['р', s @ ..] => super::t48::n35932(s, out),
		['с', s @ ..] => super::t48::n35933(s, out),
		['т', s @ ..] => super::t48::n35934(s, out),
		['у', s @ ..] => super::t48::n35935(s, out),
		['ч', s @ ..] => super::t48::n35936(s, out),
		['щ', s @ ..] => super::t48::n35937(s, out),
		['ь', s @ ..] => super::t48::n35938(s, out),
		_ => false,
	}
}

/// Words ending -олотенушко.
pub(crate) fn n133646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132284);
	s.is_empty()
}

/// Words ending -словечушко.
pub(crate) fn n133647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132285);
	s.is_empty()
}

/// Words ending -сердечушко.
pub(crate) fn n133648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132286);
	s.is_empty()
}

/// Words ending -стеколышко.
pub(crate) fn n133649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юшко.
pub(crate) fn n13958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13884);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35608);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63690);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90489);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110955);
									s.is_empty()
								}
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
			super::push(out, 35609);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63691);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 35610);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63692);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 63693);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 90491);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 90492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 35611);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63694);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90493);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 110956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word несколько.
pub(crate) fn n133650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132288);
	s.is_empty()
}

/// Words ending -келанджело.
pub(crate) fn n133651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132289);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етаправило.
pub(crate) fn n133652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132290);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лимончелло.
pub(crate) fn n133653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132291);
	s.is_empty()
}

/// Words ending -та-пикколо.
pub(crate) fn n133654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132292);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 137019);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139837);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вода-масло.
pub(crate) fn n133655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132293);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ашют-крыло.
pub(crate) fn n133656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132294);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137021);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139838);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --наполетано.
pub(crate) fn n138438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137022);
	s.is_empty()
}

/// Words ending -ождествено.
pub(crate) fn n133659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132297);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 137023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -луб-казино.
pub(crate) fn n133660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132298);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нкино.
pub(crate) fn n36217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35876);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63908);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90619);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окино.
pub(crate) fn n36218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35877);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63909);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 90620);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111036);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124300);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132299);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 63910);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 90621);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111037);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 124301);
									s.is_empty()
								}
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
			super::push(out, 63911);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 90622);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркино.
pub(crate) fn n36219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35878);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 63912);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 90623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ткино.
pub(crate) fn n36220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35879);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63913);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкино.
pub(crate) fn n36221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35880);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 63914);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90625);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63915);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 90626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкино.
pub(crate) fn n36222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35881);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 63916);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90627);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 111038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лино.
pub(crate) fn n14138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14064);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35882);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 35883);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63917);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90628);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111039);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 124302);
											s.is_empty()
										}
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
					super::push(out, 63918);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 90629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 35884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90630);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111040);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124303);
											s.is_empty()
										}
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
					super::push(out, 63920);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 90631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 35885);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63921);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90632);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111041);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 124304);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35886);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63922);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90633);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111042);
									s.is_empty()
								}
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
			super::push(out, 35887);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63923);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90634);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 63924);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90635);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111044);
									s.is_empty()
								}
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
			super::push(out, 35888);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 63925);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 90636);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111045);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 124305);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35889);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 63926);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90637);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111046);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 124306);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35890);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63927);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мино.
pub(crate) fn n14139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14065);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35891);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 63928);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 90639);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111047);
									s.is_empty()
								}
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
			super::push(out, 35892);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63929);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90640);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111048);
									s.is_empty()
								}
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
			super::push(out, 35893);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нино.
pub(crate) fn n14140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14066);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 35894);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 35895);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 63931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 63932);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35896);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 63933);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 90643);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111049);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124307);
											s.is_empty()
										}
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
					super::push(out, 63934);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 90644);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111050);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 124308);
											s.is_empty()
										}
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
					super::push(out, 63935);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 90645);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111051);
									s.is_empty()
								}
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
			super::push(out, 35897);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 63936);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пино.
pub(crate) fn n14141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14067);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 35898);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63937);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35899);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63938);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рино.
pub(crate) fn n14142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14068);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35900);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63939);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 90648);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 111052);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 124309);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63940);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 90649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 35902);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63941);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 90650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 35903);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 63942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90651);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 63943);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90652);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111054);
									s.is_empty()
								}
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
			super::push(out, 35904);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сино.
pub(crate) fn n14143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14069);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35905);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 35906);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90653);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 90654);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111055);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124310);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132300);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тино.
pub(crate) fn n14144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14070);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35907);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63946);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 63947);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 90655);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111056);
									s.is_empty()
								}
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
			super::push(out, 35908);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 63948);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90656);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111057);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 124311);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35909);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63949);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 90657);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111058);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124312);
											s.is_empty()
										}
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
					super::push(out, 63950);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 90658);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111059);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 124313);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 90659);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111060);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 124314);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35910);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 35911);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63952);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90661);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 90662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 35912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63953);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 90663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 35913);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63954);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хино.
pub(crate) fn n14145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14071);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 35914);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63955);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чино.
pub(crate) fn n14146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35915);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63956);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90666);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111062);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 124315);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35916);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 63957);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90667);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 90668);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111064);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124316);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 132301);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35917);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 63958);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90669);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111065);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124317);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шино.
pub(crate) fn n14147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14073);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35918);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 63959);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90670);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111066);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124318);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132302);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35919);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63960);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35920);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63961);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90672);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 63962);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90673);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111068);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124319);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьино.
pub(crate) fn n14148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14074);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35921);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63963);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 90674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -редственно.
pub(crate) fn n133665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132303);
	s.is_empty()
}

/// Words ending -заготзерно.
pub(crate) fn n133666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132304);
	s.is_empty()
}

/// Words ending -ворительно.
pub(crate) fn n133667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132305);
	s.is_empty()
}

/// Words ending -монтенегро.
pub(crate) fn n133670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132308);
	s.is_empty()
}

/// Words ending -жак-болеро.
pub(crate) fn n133671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132309);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 137025);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -джорнаменто.
pub(crate) fn n133675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132313);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нанозолото.
pub(crate) fn n133676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132314);
	s.is_empty()
}

/// Words ending -онтекристо.
pub(crate) fn n133677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132315);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137029);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лащ-пальто.
pub(crate) fn n133678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132316);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 137030);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цедиантроп.
pub(crate) fn n133682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132320);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -риббентроп.
pub(crate) fn n133683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132321);
	s.is_empty()
}

/// Words ending -кешфехервар.
pub(crate) fn n138450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137034);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -александар.
pub(crate) fn n133686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132324);
	s.is_empty()
}

/// Words ending -атеринодар.
pub(crate) fn n133687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132325);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 137035);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 139843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жмудар.
pub(crate) fn n65045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64381);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 90930);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 111198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тудар.
pub(crate) fn n36801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36460);
	s.is_empty()
}

/// Words ending -уудар.
pub(crate) fn n36802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36461);
	s.is_empty()
}

/// Words ending -ьдар.
pub(crate) fn n14626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14552);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36462);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 64383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алун-чакар.
pub(crate) fn n133688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132326);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 137036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -халунчакар.
pub(crate) fn n133689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132327);
	s.is_empty()
}

/// The word вальдемар.
pub(crate) fn n133690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132328);
	s.is_empty()
}

/// Words ending -тимакассар.
pub(crate) fn n133691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132329);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137037);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аминосахар.
pub(crate) fn n133692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132330);
	s.is_empty()
}

/// Words ending -кантилевер.
pub(crate) fn n133695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132333);
	s.is_empty()
}

/// Words ending -р-ретривер.
pub(crate) fn n133696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132334);
	match s {
		[] => true,
		['о', 'д', 'а', 'р', 'б', 'а', 'л', '\u{1}', s @ ..] => {
			super::push(out, 137039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -от-ресивер.
pub(crate) fn n133697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132335);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137040);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 139846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -айвер.
pub(crate) fn n37065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36720);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 64582);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 91045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 64583);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 91046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейвер.
pub(crate) fn n37066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36721);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 64584);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 64585);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 91047);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111246);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124394);
									s.is_empty()
								}
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
			super::push(out, 64586);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сквер.
pub(crate) fn n37067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36722);
	s.is_empty()
}

/// Words ending -лвер.
pub(crate) fn n14817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14740);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 36723);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 64587);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нвер.
pub(crate) fn n14818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14741);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36724);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 64588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овер.
pub(crate) fn n14819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14742);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 36725);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64589);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 36726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64590);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 64591);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91050);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 64592);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91051);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111248);
									s.is_empty()
								}
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
			super::push(out, 36727);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64593);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91052);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 64594);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 64595);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91054);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111250);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 111251);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124395);
											s.is_empty()
										}
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
					super::push(out, 64596);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 36728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64597);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 64598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36729);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 64599);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91056);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 64600);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 64601);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91057);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64602);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91058);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111254);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124396);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36730);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 36731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64603);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91059);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 64604);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91060);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111256);
									s.is_empty()
								}
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
			super::push(out, 36732);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 64605);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 64606);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвер.
pub(crate) fn n14820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14743);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36733);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64607);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 64608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36734);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64609);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91063);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свер.
pub(crate) fn n14821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14744);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36735);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 64610);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91064);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111258);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 124397);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36736);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 64611);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91065);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111259);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твер.
pub(crate) fn n14822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14745);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 36737);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64612);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91066);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111260);
									s.is_empty()
								}
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
			super::push(out, 36738);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64613);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увер.
pub(crate) fn n14823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14746);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 36739);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 36740);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64615);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 64616);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91069);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111261);
									s.is_empty()
								}
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
			super::push(out, 36741);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьвер.
pub(crate) fn n14824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14747);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36742);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64618);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91070);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111262);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124398);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ингер.
pub(crate) fn n37107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36762);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 64641);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 91086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 64642);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 91087);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 111270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 91088);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 64643);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91089);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 111272);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124399);
									s.is_empty()
								}
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
			super::push(out, 64644);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 91090);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124400);
									s.is_empty()
								}
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
			super::push(out, 64645);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 91091);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 111274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 64646);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 91092);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 91093);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 64647);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 91094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хэнгер.
pub(crate) fn n65314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64649);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 91096);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 111277);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 124401);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 132336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -логер.
pub(crate) fn n37110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36765);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 64650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ргер.
pub(crate) fn n14832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14755);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36766);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64651);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91097);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111278);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124402);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132337);
													s.is_empty()
												}
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
							super::push(out, 91098);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111279);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124403);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132338);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64652);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91099);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111280);
									s.is_empty()
								}
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
			super::push(out, 36767);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64653);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91100);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111281);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 124404);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132339);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 36768);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91101);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 91102);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 91103);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111282);
									s.is_empty()
								}
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
			super::push(out, 36769);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64655);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -югер.
pub(crate) fn n14834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14757);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36770);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 64656);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягер.
pub(crate) fn n14835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14758);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36771);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 64657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дебаркадер.
pub(crate) fn n133702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132340);
	s.is_empty()
}

/// Words ending -суперспредер.
pub(crate) fn n133703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132341);
	s.is_empty()
}

/// Words ending -йдер.
pub(crate) fn n14841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14764);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36790);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64680);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91120);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111293);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124409);
											s.is_empty()
										}
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
					super::push(out, 64681);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91121);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 91122);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 91123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64682);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 91124);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111295);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 124410);
											s.is_empty()
										}
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
						['н', s @ ..] => {
							super::push(out, 91125);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 91126);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111297);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124411);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91127);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111298);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124412);
											s.is_empty()
										}
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
					super::push(out, 64683);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91128);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 64684);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91129);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111300);
									s.is_empty()
								}
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
			super::push(out, 36791);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64685);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91130);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 91131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111301);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 111302);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124413);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132342);
													s.is_empty()
												}
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
							super::push(out, 91132);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 64686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', '\u{1}', s @ ..] => {
					super::push(out, 64687);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдер.
pub(crate) fn n14842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14765);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 36792);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 64688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36793);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 64689);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 64690);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36794);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64691);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндер.
pub(crate) fn n14843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14766);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36795);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64692);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91137);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111305);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124414);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 111306);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 124415);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132343);
													s.is_empty()
												}
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
							super::push(out, 91138);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111307);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124416);
											s.is_empty()
										}
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
					super::push(out, 64693);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91139);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91140);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 111309);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124417);
											s.is_empty()
										}
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
					super::push(out, 64694);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91141);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111310);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124418);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 91142);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 64695);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64696);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 64697);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91145);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 91146);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111313);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124419);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132344);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 36796);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 64699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91148);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 91149);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 91150);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 91151);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111314);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124420);
											s.is_empty()
										}
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
					super::push(out, 64700);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 91152);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 64701);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91153);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 91154);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111316);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124421);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91155);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111317);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 124422);
											s.is_empty()
										}
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
					super::push(out, 64702);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 64703);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91156);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 91157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36797);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64704);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 64705);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 64706);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							super::push(out, 91158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36798);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64707);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 64708);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91159);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111319);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124423);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132345);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64709);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t48::n37144(s, out),
		['ю', s @ ..] => {
			super::push(out, 36800);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64714);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одер.
pub(crate) fn n14844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14767);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36801);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 36802);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64715);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 91167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 36803);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64716);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 91168);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 91169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111324);
									s.is_empty()
								}
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
			super::push(out, 36804);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 64717);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91170);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111325);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 124425);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36805);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64718);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 64719);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 64720);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91172);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 91173);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 64721);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91174);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111327);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124426);
											s.is_empty()
										}
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
					super::push(out, 64722);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91175);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 64723);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 36806);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 64724);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91177);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64725);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91178);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111330);
									s.is_empty()
								}
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
			super::push(out, 36807);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36808);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64727);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91179);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 111331);
									s.is_empty()
								}
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
			super::push(out, 36809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64728);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 64729);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 64730);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91181);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 111332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдер.
pub(crate) fn n14845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14768);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36810);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64731);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91182);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111333);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 124427);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132347);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64732);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91183);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111334);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124428);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132348);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64733);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 64734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36811);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64735);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 64736);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91184);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111335);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124429);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132349);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 111336);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124430);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132350);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138458(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91185);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 111337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 64737);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91186);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124431);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 124432);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 91187);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 64738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удер.
pub(crate) fn n14846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14769);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36812);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 64739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 36813);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 64740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 36814);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64741);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 64742);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдер.
pub(crate) fn n14847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14770);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36815);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91190);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124433);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 132351);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138459(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 124434);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 111339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 91191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдер.
pub(crate) fn n14848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14771);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36816);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64743);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -д-тренажер.
pub(crate) fn n133714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132352);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 137044);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 139847);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141494);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 142453);
									match s {
										[] => true,
										['о', 'л', 'е', 'в', '\u{1}', s @ ..] => {
											super::push(out, 142979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -и-тренажер.
pub(crate) fn n133715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132353);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 137045);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139848);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 141495);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-тренажер.
pub(crate) fn n133716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132354);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137046);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 139849);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141496);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 142455);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142980);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арбитражер.
pub(crate) fn n133717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132355);
	s.is_empty()
}

/// Words ending -аджер.
pub(crate) fn n37170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36825);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 64752);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 91200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 64753);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 91201);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 111346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еджер.
pub(crate) fn n37171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36826);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 64754);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 91202);
					match s {
						[] => true,
						['м', '-', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124439);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132356);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137047);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n141291(s, out),
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
									super::push(out, 124440);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 132357);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137048);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n141292(s, out),
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
									super::push(out, 124441);
									match s {
										[] => true,
										['о', 'т', '\u{1}', s @ ..] => {
											super::push(out, 132358);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 124442);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 124443);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132359);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137049);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n141293(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 64755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иджер.
pub(crate) fn n37172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36827);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 64756);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йджер.
pub(crate) fn n37173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36828);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 64757);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 91203);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 91204);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111348);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 91205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 64758);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 91206);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 111349);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 124445);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132360);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нджер.
pub(crate) fn n37174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36829);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 64759);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91207);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111350);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124446);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 132361);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 111351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 91208);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111352);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 124447);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 132362);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 64760);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 91209);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111353);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 124448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 111354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 64761);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91210);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 111355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ежер.
pub(crate) fn n14854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14777);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36830);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64762);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91211);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111356);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 124449);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ижер.
pub(crate) fn n14855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14778);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 36831);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64763);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 36832);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64764);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 64765);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжер.
pub(crate) fn n14856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14779);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36833);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64766);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91213);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 91214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64767);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 64768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36834);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64769);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36835);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64770);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 36836);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64771);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржер.
pub(crate) fn n14857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14780);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 36837);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 64772);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91220);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужер.
pub(crate) fn n14858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14781);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 36838);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжер.
pub(crate) fn n14859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14782);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 36839);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64773);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91221);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ран-призер.
pub(crate) fn n133725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132363);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 137050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 139853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -климатизер.
pub(crate) fn n133726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132364);
	s.is_empty()
}

/// Words ending -видеотизер.
pub(crate) fn n133727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132365);
	s.is_empty()
}

/// Words ending -гипнотизер.
pub(crate) fn n133728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132366);
	s.is_empty()
}

/// Words ending -стриптизер.
pub(crate) fn n133729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132367);
	s.is_empty()
}

/// Words ending -йзер.
pub(crate) fn n14864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14787);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64792);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 64793);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91231);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111368);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 91232);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111369);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124456);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 132368);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64794);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91233);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111370);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 124457);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132369);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64795);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91234);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111371);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124458);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132370);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 64797);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 64798);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91236);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 91237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64799);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91238);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111372);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124459);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 132371);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64800);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36857);
			match s {
				[] => true,
				['г', 'у', s @ ..] => {
					super::push(out, 64801);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 91240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 64802);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91242);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 91243);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111374);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124460);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 132372);
													s.is_empty()
												}
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
							super::push(out, 91244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озер.
pub(crate) fn n14866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14789);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 36860);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64804);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 64805);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91245);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111375);
									s.is_empty()
								}
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
			super::push(out, 36861);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64806);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 36862);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 64807);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91247);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узер.
pub(crate) fn n14867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14790);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36863);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 64808);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 64809);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 36864);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 36865);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 36866);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64811);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91249);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 64812);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 64813);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьюзер.
pub(crate) fn n37213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36868);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 64814);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 91251);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бидермейер.
pub(crate) fn n133735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132373);
	s.is_empty()
}

/// Words ending -крем-ликер.
pub(crate) fn n133736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132374);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хроникер.
pub(crate) fn n112571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111389);
	s.is_empty()
}

/// Words ending -сникер.
pub(crate) fn n65520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64852);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 91269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пикер.
pub(crate) fn n37247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36900);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 64853);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 91270);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 111390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рикер.
pub(crate) fn n37248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36901);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 64854);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 64855);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тикер.
pub(crate) fn n37249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36902);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 64856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йкер.
pub(crate) fn n14877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14800);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36903);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 36904);
			match s {
				[] => true,
				['м', 'с', 'й', s @ ..] => {
					super::push(out, 64857);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91271);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111391);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124467);
											s.is_empty()
										}
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
					super::push(out, 64858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ккер.
pub(crate) fn n14878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14801);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 36905);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 64859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64860);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 64861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36906);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64862);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкер.
pub(crate) fn n14879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14802);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64863);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 64864);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91274);
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
				['р', s @ ..] => {
					super::push(out, 64865);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 64866);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91275);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкер.
pub(crate) fn n14880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14803);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36907);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64867);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 64868);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64869);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 64870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36908);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 64871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36909);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 64872);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111394);
									s.is_empty()
								}
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
			super::push(out, 36910);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91279);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 91280);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111395);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124468);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36911);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64874);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 64875);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91281);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 111396);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124469);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 132375);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окер.
pub(crate) fn n14881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14804);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 36912);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 64876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 36913);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64877);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 64878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 64879);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 36914);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 64880);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 36915);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 64881);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 64882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'с', s @ ..] => {
			super::push(out, 36916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36917);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 36918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 36919);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91285);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111397);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124470);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркер.
pub(crate) fn n14882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14805);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36920);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91286);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111398);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 124471);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 91287);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111399);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124472);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132376);
													s.is_empty()
												}
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
							super::push(out, 91288);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111400);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 124473);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 124474);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132377);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138468(s, out),
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
									super::push(out, 111401);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 124475);
											s.is_empty()
										}
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
					super::push(out, 64887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36921);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64888);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91289);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 64889);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91290);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111403);
									s.is_empty()
								}
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
			super::push(out, 36922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скер.
pub(crate) fn n14883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14806);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64890);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 64891);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91291);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111404);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 124476);
											s.is_empty()
										}
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
					super::push(out, 64892);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91292);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 64893);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91293);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111406);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 124477);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36924);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64894);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 36925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64895);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укер.
pub(crate) fn n14884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14807);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 36926);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 36927);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64896);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цкер.
pub(crate) fn n14885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14808);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36928);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 64897);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экер.
pub(crate) fn n14886(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36929);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 36930);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64898);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкер.
pub(crate) fn n14887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14809);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 36931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -якер.
pub(crate) fn n14888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14810);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36932);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64899);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтитейблер.
pub(crate) fn n133740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132378);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --ассемблер.
pub(crate) fn n133741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132379);
	s.is_empty()
}

/// Words ending -зассемблер.
pub(crate) fn n133742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132380);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137054);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 139854);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -белер.
pub(crate) fn n37299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36949);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 64920);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 91310);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 111412);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 124479);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132381);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137055);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n141296(s, out),
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
									super::push(out, 124480);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132382);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оделер.
pub(crate) fn n65590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64921);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 91311);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -келер.
pub(crate) fn n37301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36951);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 64922);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 91312);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 111413);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илер.
pub(crate) fn n14894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14816);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 36952);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 64923);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91313);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111414);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124482);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132383);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138472(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91314);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111415);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124483);
											s.is_empty()
										}
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
					super::push(out, 64924);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 64925);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91315);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111416);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91316);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111417);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124485);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36953);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64926);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 36954);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64927);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91318);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111418);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124486);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36955);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 64928);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91319);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111419);
									s.is_empty()
								}
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
			super::push(out, 36956);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 36957);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 36958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йлер.
pub(crate) fn n14895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14817);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36959);
			match s {
				[] => true,
				['т', 'с', 'и', s @ ..] => {
					super::push(out, 64930);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91320);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 111420);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124487);
											s.is_empty()
										}
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
					super::push(out, 64931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36960);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64932);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91321);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111421);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124488);
											s.is_empty()
										}
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
					super::push(out, 64933);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91322);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 91323);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111422);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124489);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 132384);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 64934);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 64935);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91325);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 64936);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91326);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 91327);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111425);
									s.is_empty()
								}
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
			super::push(out, 36961);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64937);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 64938);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64939);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клер.
pub(crate) fn n14896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14818);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36962);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 64940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36963);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64941);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91330);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111427);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 124490);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36964);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 64942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36965);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64943);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91331);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111428);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124491);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36966);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 64944);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 91332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 36967);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64945);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллер.
pub(crate) fn n14897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14819);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36968);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64946);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91333);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111429);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 91334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 64947);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 64948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91335);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 91336);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111430);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91337);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111431);
									s.is_empty()
								}
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
			super::push(out, 36969);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64949);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91338);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111432);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 124494);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132385);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138473(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 64950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91339);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 91340);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111433);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124495);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132386);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138474(s, out),
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
									super::push(out, 111434);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 111435);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 91341);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111436);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124496);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 132387);
													match s {
														[] => true,
														['с', s @ ..] => super::t48::n138475(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 64951);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 64952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36970);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 64954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64955);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91346);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111438);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 124497);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132388);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138476(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91347);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111439);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 124498);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132389);
													s.is_empty()
												}
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
							super::push(out, 91348);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111440);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91349);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111441);
									s.is_empty()
								}
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
			super::push(out, 36971);
			match s {
				[] => true,
				['б', s @ ..] => super::t48::n65626(s, out),
				['п', s @ ..] => {
					super::push(out, 64957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олер.
pub(crate) fn n14898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14820);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 36972);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 36973);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64959);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91352);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111443);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124500);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 132390);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138477(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111444);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 124501);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 132391);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138478(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плер.
pub(crate) fn n14899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14821);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36974);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64960);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36975);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 64961);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 64962);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111445);
									s.is_empty()
								}
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
			super::push(out, 36976);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64963);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 64964);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлер.
pub(crate) fn n14900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14822);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36977);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 64965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слер.
pub(crate) fn n14901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14823);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 36978);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлер.
pub(crate) fn n14902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14824);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36979);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36980);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 64968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 36981);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64969);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 64970);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 36982);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64971);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улер.
pub(crate) fn n14903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14825);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36983);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64972);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91358);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111446);
									s.is_empty()
								}
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
			super::push(out, 36984);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64973);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91359);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111447);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 124502);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36985);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64975);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 36986);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64976);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 36987);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64977);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 36988);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флер.
pub(crate) fn n14904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14826);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36989);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 36990);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36991);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36992);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64981);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цлер.
pub(crate) fn n14905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14827);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 36993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64982);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таймер.
pub(crate) fn n65673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65003);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 91379);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 111456);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 124505);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 132392);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 137063);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 139861);
													match s {
														[] => true,
														['ч', s @ ..] => super::t48::n142950(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 91380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еймер.
pub(crate) fn n37362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37012);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 65004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91381);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 91382);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111457);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124506);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132393);
											s.is_empty()
										}
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
					super::push(out, 91383);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 111458);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124507);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132394);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['к', s @ ..] => {
					super::push(out, 91384);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111459);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 91385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 65005);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 91386);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111460);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 124509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лмер.
pub(crate) fn n14912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37013);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ммер.
pub(crate) fn n14913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14835);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37014);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65007);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 65008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37015);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 37016);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 65010);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омер.
pub(crate) fn n14914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14836);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37017);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 37018);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 37019);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 37020);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65012);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91389);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 65013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37021);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 37022);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 37023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65014);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 65015);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 65016);
					s.is_empty()
				}
				['о', 'т', s @ ..] => {
					super::push(out, 65017);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91390);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111463);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124510);
											s.is_empty()
										}
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
					super::push(out, 65018);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91391);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111464);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124511);
											s.is_empty()
										}
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
					super::push(out, 65019);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91392);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111465);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124512);
											s.is_empty()
										}
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
					super::push(out, 65020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 37024);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 37025);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65021);
					match s {
						[] => true,
						['с', 'ы', 'в', '-', s @ ..] => {
							super::push(out, 91393);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111466);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124513);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132395);
													match s {
														[] => true,
														['д', s @ ..] => super::t48::n138480(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37026);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмер.
pub(crate) fn n14915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14837);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37027);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 65023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37028);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65024);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 91394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 65025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37029);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 65026);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91395);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 91396);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 91397);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111469);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 124514);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132396);
													s.is_empty()
												}
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
							super::push(out, 91398);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111470);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124515);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 91399);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111471);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124516);
											match s {
												[] => true,
												['р', 'т', '-', s @ ..] => {
													super::push(out, 132397);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138481(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91400);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умер.
pub(crate) fn n14917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14839);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37030);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65027);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 65028);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91401);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111473);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124517);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37031);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 37032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37033);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 37034);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 37035);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хмер.
pub(crate) fn n14918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14840);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37036);
			match s {
				[] => true,
				['р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 65033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эмер.
pub(crate) fn n14919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14841);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37038);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65035);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 91403);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмер.
pub(crate) fn n14920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14842);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 37039);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 37040);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65037);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91404);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111475);
									s.is_empty()
								}
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
			super::push(out, 37041);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65038);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91405);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едиапланер.
pub(crate) fn n133760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132398);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137066);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -идропланер.
pub(crate) fn n133761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132399);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 137067);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мотопланер.
pub(crate) fn n133762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132400);
	s.is_empty()
}

/// Words ending -денер.
pub(crate) fn n37400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37050);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 65050);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 111486);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124526);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132401);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женер.
pub(crate) fn n37401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37051);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 65051);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 91416);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 111487);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124527);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 132402);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137068);
													match s {
														[] => true,
														['в', s @ ..] => super::t48::n141305(s, out),
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
									super::push(out, 124528);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132403);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137069);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n141306(s, out),
														_ => false,
													}
												}
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
							super::push(out, 111488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренер.
pub(crate) fn n37402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37052);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 65052);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 91417);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 111489);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124529);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132404);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 65053);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 91418);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 111490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 91419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенер.
pub(crate) fn n37403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37053);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 65054);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 91420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченер.
pub(crate) fn n37404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37054);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 65055);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 91421);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знер.
pub(crate) fn n14924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14846);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37055);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инер.
pub(crate) fn n14925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14847);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91422);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111493);
									s.is_empty()
								}
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
			super::push(out, 37057);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65058);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91423);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111494);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124530);
											s.is_empty()
										}
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
					super::push(out, 65059);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91424);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111495);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124531);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37058);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65060);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 37059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65061);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 65062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 37060);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 65063);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 37061);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65064);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 65065);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91428);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111496);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124532);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37062);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65066);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91429);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111497);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124533);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132405);
													s.is_empty()
												}
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
							super::push(out, 91430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 65067);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 37063);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65068);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 65069);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йнер.
pub(crate) fn n14926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14848);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37064);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65070);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91434);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111498);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124534);
											s.is_empty()
										}
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
					super::push(out, 65071);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 65072);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91435);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111499);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 124535);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132406);
													match s {
														[] => true,
														['и', s @ ..] => super::t48::n138486(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 132407);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 124536);
											s.is_empty()
										}
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
					super::push(out, 65073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91436);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 91437);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111500);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 124537);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132408);
													s.is_empty()
												}
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
							super::push(out, 91438);
							s.is_empty()
						}
						['й', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111501);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 111502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111503);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124538);
											s.is_empty()
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 111504);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 124539);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132409);
													s.is_empty()
												}
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
							super::push(out, 91439);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111505);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124540);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 91440);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 91441);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111507);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 111508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 91442);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111509);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 124541);
											s.is_empty()
										}
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
					super::push(out, 65074);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37065);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65076);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 65077);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 65078);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91443);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 65079);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 65080);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 111511);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124542);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132410);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t48::n138487(s, out),
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
									super::push(out, 111512);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124543);
											s.is_empty()
										}
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
					super::push(out, 65081);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91444);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111513);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 124544);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнер.
pub(crate) fn n14927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14849);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 37066);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 65082);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ннер.
pub(crate) fn n14928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14850);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37067);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65083);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 65084);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37068);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65086);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37069);
			match s {
				[] => true,
				['п', 'с', '\u{1}', s @ ..] => {
					super::push(out, 65087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онер.
pub(crate) fn n14929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14851);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37070);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 37071);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65088);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65089);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 91446);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124545);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37072);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65090);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 65091);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91448);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111515);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124546);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132411);
													match s {
														[] => true,
														['г', s @ ..] => super::t48::n138488(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65092);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91449);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111516);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124547);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132412);
													s.is_empty()
												}
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
							super::push(out, 91450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 65093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91451);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111517);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124548);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132413);
													s.is_empty()
												}
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
							super::push(out, 91452);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111518);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 124549);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132414);
													match s {
														[] => true,
														['к', s @ ..] => super::t48::n138489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', 'о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 111519);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 111520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 91453);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111521);
									match s {
										[] => true,
										['л', 'е', s @ ..] => {
											super::push(out, 124550);
											match s {
												[] => true,
												['с', '-', s @ ..] => {
													super::push(out, 132415);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138490(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91454);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 91455);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111522);
									match s {
										[] => true,
										['о', 'в', 'е', 'р', '-', s @ ..] => {
											super::push(out, 124551);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132416);
													match s {
														[] => true,
														['с', s @ ..] => super::t48::n138491(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37073);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 37074);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65094);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91456);
							match s {
								[] => true,
								['к', s @ ..] => super::t48::n112705(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t48::n37425(s, out),
		['т', s @ ..] => super::t48::n37426(s, out),
		_ => false,
	}
}

/// Words ending -рнер.
pub(crate) fn n14930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14852);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 37077);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65097);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 91458);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 65098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91459);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 91460);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111526);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65099);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 65100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37078);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнер.
pub(crate) fn n14931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14853);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 37079);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65102);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 91462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 37080);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65103);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унер.
pub(crate) fn n14932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14854);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 37081);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 65104);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 37082);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91465);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111527);
									s.is_empty()
								}
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
			super::push(out, 37083);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шнер.
pub(crate) fn n14933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14855);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37084);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65107);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьнер.
pub(crate) fn n14934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14856);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37085);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65108);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнер.
pub(crate) fn n14935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14857);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 37086);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 37087);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 65109);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -акваскипер.
pub(crate) fn n133779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132417);
	s.is_empty()
}

/// Words ending -тереотипер.
pub(crate) fn n133780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132418);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 137076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ампер.
pub(crate) fn n37457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37107);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 65132);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 65133);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 91480);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 111532);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124555);
									s.is_empty()
								}
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
			super::push(out, 65134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 65135);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 91482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111533);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 111534);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124556);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 132419);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 111535);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 124557);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 132420);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 137077);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 65136);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91483);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 111536);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124558);
									s.is_empty()
								}
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
			super::push(out, 65137);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 91484);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 111537);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 91485);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 111538);
							match s {
								[] => true,
								['и', 'м', '\u{1}', s @ ..] => {
									super::push(out, 124560);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емпер.
pub(crate) fn n37458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37108);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 65138);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 91486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 65139);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 91487);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 111539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лимпер.
pub(crate) fn n65811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65140);
	s.is_empty()
}

/// Words ending -умпер.
pub(crate) fn n37460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37110);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 65141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -опер.
pub(crate) fn n14944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14866);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37111);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 37112);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65142);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 37113);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 37114);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91489);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 65145);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91490);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111541);
									s.is_empty()
								}
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
			super::push(out, 37115);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 65146);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 91491);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111542);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 124561);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37116);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65147);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37117);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65148);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 65149);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91493);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 111543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 65150);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91494);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111544);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124562);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132421);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37118);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65151);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91495);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111545);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124563);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 132422);
													match s {
														[] => true,
														['д', s @ ..] => super::t48::n138494(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37119);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 65152);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 91496);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111546);
									s.is_empty()
								}
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
			super::push(out, 37120);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65153);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91497);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111547);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124564);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132423);
													s.is_empty()
												}
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
							super::push(out, 91498);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 111548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 91499);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111549);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 124565);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132424);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65154);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 65155);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91500);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 65156);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 37122);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65157);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91502);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ппер.
pub(crate) fn n14945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14867);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37123);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 65158);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65159);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37124);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 91504);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 91505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 65160);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 65161);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37125);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65162);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 65163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37126);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91509);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 91510);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91511);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 91512);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 91513);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 65164);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91514);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111555);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 124566);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 132425);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138495(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91515);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 111556);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124567);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132426);
													match s {
														[] => true,
														['т', s @ ..] => super::t48::n138496(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 37127);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 37128);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рпер.
pub(crate) fn n14946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14868);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37129);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 65168);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спер.
pub(crate) fn n14947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14869);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37130);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37131);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65170);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65171);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91517);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 111557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упер.
pub(crate) fn n14948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14870);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37132);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65172);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 65173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 37133);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37134);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 37135);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 37136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шпер.
pub(crate) fn n14949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14871);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37137);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37138);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65178);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91519);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 111558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьпер.
pub(crate) fn n14950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14872);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37139);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65179);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эпер.
pub(crate) fn n14951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14873);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 37140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -штурмфюрер.
pub(crate) fn n133789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132427);
	s.is_empty()
}

/// Words ending -гауптфюрер.
pub(crate) fn n133790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132428);
	s.is_empty()
}

/// Words ending -виакрейсер.
pub(crate) fn n133791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132429);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 137081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дрокрейсер.
pub(crate) fn n133792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132430);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 137082);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кик-боксер.
pub(crate) fn n133793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132431);
	s.is_empty()
}

/// Words ending -нсер.
pub(crate) fn n14963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14885);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37159);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 65205);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91546);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111576);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 124577);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132432);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138499(s, out),
														_ => false,
													}
												}
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
							super::push(out, 91547);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111577);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124578);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132433);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138500(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65206);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 65207);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91549);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 91550);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111579);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 124579);
											s.is_empty()
										}
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
					super::push(out, 65208);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91551);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111580);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124580);
											s.is_empty()
										}
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
					super::push(out, 65209);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 65210);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91553);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 111581);
									s.is_empty()
								}
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
			super::push(out, 37160);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65211);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91554);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111582);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124581);
											s.is_empty()
										}
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
					super::push(out, 65212);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 91555);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111583);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124582);
											s.is_empty()
										}
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
					super::push(out, 65213);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 37161);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 65214);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91557);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 111584);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 124583);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word косер.
pub(crate) fn n65889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65215);
	s.is_empty()
}

/// Words ending -псер.
pub(crate) fn n14965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14887);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37163);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 65216);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсер.
pub(crate) fn n14966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14888);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37164);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65217);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 65218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37165);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37166);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65220);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 65221);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91560);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 111585);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124584);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 132434);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссер.
pub(crate) fn n14967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14889);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37167);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65222);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 91561);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111586);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124585);
											s.is_empty()
										}
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
					super::push(out, 65223);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91562);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 111587);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 91563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37168);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 65224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65225);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91565);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111588);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124586);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132435);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65226);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91566);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111589);
									s.is_empty()
								}
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
			super::push(out, 37169);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 65227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 65228);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91568);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 65229);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 91569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 65230);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 37170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65231);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91571);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 111591);
									s.is_empty()
								}
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
			super::push(out, 37171);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65232);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 91572);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 111592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65233);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91573);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111593);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 111594);
									s.is_empty()
								}
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
			super::push(out, 37172);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 37173);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65235);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 37174);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65236);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тсер.
pub(crate) fn n14968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14890);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 37175);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 65237);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усер.
pub(crate) fn n14969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14891);
	s.is_empty()
}

/// Words ending -ысер.
pub(crate) fn n14970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14892);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 37176);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эсер.
pub(crate) fn n14971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14893);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юсер.
pub(crate) fn n14972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14894);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 37178);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65238);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -микрокатер.
pub(crate) fn n133798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132436);
	s.is_empty()
}

/// Words ending -итер.
pub(crate) fn n14975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14897);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37197);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 37198);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91595);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 91596);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111604);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124588);
											s.is_empty()
										}
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
					super::push(out, 65262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 37199);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 65263);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91597);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111605);
									s.is_empty()
								}
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
			super::push(out, 37200);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65264);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65265);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91599);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111606);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 124589);
											s.is_empty()
										}
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
					super::push(out, 65266);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91600);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111607);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124590);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37201);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65267);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 37202);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65268);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111608);
									s.is_empty()
								}
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
			super::push(out, 37203);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65269);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 65270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 37204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йтер.
pub(crate) fn n14976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14898);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37205);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65271);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91604);
							match s {
								[] => true,
								['а', 'т', 'с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124591);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 124592);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132437);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65272);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 91605);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 91606);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111609);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 124593);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132438);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65273);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91607);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111610);
									match s {
										[] => true,
										['д', 'н', 'а', '-', s @ ..] => {
											super::push(out, 124594);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132439);
													match s {
														[] => true,
														['н', s @ ..] => super::t48::n138502(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65274);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 91608);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111611);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124595);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132440);
													match s {
														[] => true,
														['с', s @ ..] => super::t48::n138503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37206);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65275);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 37207);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктер.
pub(crate) fn n14977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14899);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37208);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65278);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91610);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111612);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124596);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132441);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138504(s, out),
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
									super::push(out, 111613);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124597);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 132442);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138505(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65279);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 65280);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91612);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65281);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91613);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 111615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 65282);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91614);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111616);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124598);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37209);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 65283);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 65284);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 37210);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 65285);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 91617);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111617);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елтер.
pub(crate) fn n37562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37212);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 65287);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 65288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олтер.
pub(crate) fn n37563(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 65289);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 65290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 91620);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элтер.
pub(crate) fn n37564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37213);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 65291);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 91621);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтер.
pub(crate) fn n14979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37214);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65292);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91622);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 65293);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 65294);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91623);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111620);
									s.is_empty()
								}
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
			super::push(out, 37215);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65295);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91624);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111621);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124599);
											s.is_empty()
										}
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
					super::push(out, 65296);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91625);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111622);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124600);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132443);
													s.is_empty()
												}
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
							super::push(out, 91626);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111623);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124601);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 132444);
													s.is_empty()
												}
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
							super::push(out, 91627);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111624);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 124602);
											s.is_empty()
										}
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
					super::push(out, 65297);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91628);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111625);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 124603);
											s.is_empty()
										}
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
					super::push(out, 65298);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91629);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111626);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 124604);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 132445);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 37216);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65299);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 65300);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 65301);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91630);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111627);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124605);
											s.is_empty()
										}
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
					super::push(out, 65302);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 37217);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65303);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91632);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 111628);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 124606);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 37218);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 65304);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91633);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 65305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 37219);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65306);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 91634);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 111630);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124607);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 132446);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65307);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 65308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 37220);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 65309);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отер.
pub(crate) fn n14980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14902);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 37221);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65310);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91635);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 111631);
									s.is_empty()
								}
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
			super::push(out, 37222);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 65311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 37223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 37224);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 65313);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37225);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 65314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 37226);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 65315);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91638);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111632);
									s.is_empty()
								}
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
			super::push(out, 37227);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 65316);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птер.
pub(crate) fn n14981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14903);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37228);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 65317);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37229);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 37230);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 37231);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 65320);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91641);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65321);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 65322);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91643);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111634);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124608);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 132447);
													s.is_empty()
												}
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
							super::push(out, 91644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 65323);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 65324);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 91645);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111635);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124609);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132448);
													s.is_empty()
												}
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
							super::push(out, 91646);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111636);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124610);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 132449);
													s.is_empty()
												}
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
							super::push(out, 91647);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111637);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 124611);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртер.
pub(crate) fn n14982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14904);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37232);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 65325);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 65326);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 65327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 65328);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111638);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 111639);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 124612);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132450);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 132451);
													match s {
														[] => true,
														['о', s @ ..] => super::t48::n138506(s, out),
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
									super::push(out, 111640);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124613);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 132452);
													match s {
														[] => true,
														['а', s @ ..] => super::t48::n138507(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 65329);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 91650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 37233);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65330);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 91651);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111641);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124614);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 132453);
													match s {
														[] => true,
														['р', s @ ..] => super::t48::n138508(s, out),
														_ => false,
													}
												}
												['а', s @ ..] => {
													super::push(out, 132454);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91652);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111642);
									s.is_empty()
								}
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
			super::push(out, 37234);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91653);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 91654);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111643);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 124615);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 132455);
													match s {
														[] => true,
														['е', s @ ..] => super::t48::n138509(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 124616);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 91655);
							match s {
								[] => true,
								['и', s @ ..] => super::t48::n112827(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t48::n92629(s, out),
						['с', s @ ..] => super::t48::n92630(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t48::n66007(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

