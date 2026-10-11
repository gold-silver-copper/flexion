use flexion::Path;

/// Words ending -эмо.
pub(crate) fn n3061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3051);
	s.is_empty()
}

/// Words ending -юмо.
pub(crate) fn n3062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3052);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14027);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35814);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 63850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14028);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -боливиано.
pub(crate) fn n125578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124292);
	s.is_empty()
}

/// Words ending -милисиано.
pub(crate) fn n125579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124293);
	s.is_empty()
}

/// Words ending -американо.
pub(crate) fn n125580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124294);
	s.is_empty()
}

/// Words ending -сопрано.
pub(crate) fn n91558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90589);
	s.is_empty()
}

/// Words ending -трано.
pub(crate) fn n36172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35831);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 63866);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 90590);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 111016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урано.
pub(crate) fn n36173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35832);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 63867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тано.
pub(crate) fn n14109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14035);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35833);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63868);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90591);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111017);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124295);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132296);
													match s {
														[] => true,
														['-', s @ ..] => super::t37::n138438(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уано.
pub(crate) fn n14110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14036);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35834);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63869);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 90592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 35835);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фано.
pub(crate) fn n14111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14037);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35836);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 63870);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 90593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вно.
pub(crate) fn n3064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3054);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14038);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35837);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14039);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35838);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63872);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90594);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111019);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124296);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35839);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63873);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 63874);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90595);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111020);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14040);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35840);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63875);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гно.
pub(crate) fn n3065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3055);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14041);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35841);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дно.
pub(crate) fn n3066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3056);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14042);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14043);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14044);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35843);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14045);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35844);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63878);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 63879);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90598);
									s.is_empty()
								}
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
			super::push(out, 14046);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 35845);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63880);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90599);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111021);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14047);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ено.
pub(crate) fn n3067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3057);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14048);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35847);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 35848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63881);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90600);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111022);
											s.is_empty()
										}
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
					super::push(out, 35849);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63882);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90601);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111023);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 124297);
													match s {
														[] => true,
														['о', s @ ..] => super::t37::n133659(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35850);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63883);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 90602);
									s.is_empty()
								}
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
			super::push(out, 14050);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35851);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14051);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35852);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 63886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14053);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35854);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63887);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90604);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111024);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14054);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жно.
pub(crate) fn n3068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3058);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14055);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зно.
pub(crate) fn n3069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3059);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14056);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35857);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90605);
									s.is_empty()
								}
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
			super::push(out, 14057);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ино.
pub(crate) fn n3070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3060);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14058);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35859);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63889);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90606);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111025);
											s.is_empty()
										}
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
					super::push(out, 35860);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63890);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90607);
									s.is_empty()
								}
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
			super::push(out, 14059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35861);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35862);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63891);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90608);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111026);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124298);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35863);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14060);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35864);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 63893);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90609);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111027);
											s.is_empty()
										}
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
					super::push(out, 35865);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63894);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90610);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111028);
											s.is_empty()
										}
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
					super::push(out, 35866);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 63895);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 90611);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 111029);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14061);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35867);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 35868);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63897);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90612);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111030);
											s.is_empty()
										}
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
					super::push(out, 35869);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 63898);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 63899);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90613);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111031);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14062);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35870);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63900);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 90614);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111032);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 124299);
													match s {
														[] => true,
														['л', s @ ..] => super::t37::n133660(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35871);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 63901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14063);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 35872);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63902);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90615);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111033);
											s.is_empty()
										}
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
					super::push(out, 35873);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63903);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90616);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 111034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 63904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 35874);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 63905);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 90617);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 35875);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63906);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 63907);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t37::n36217(s, out),
				['о', s @ ..] => super::t37::n36218(s, out),
				['р', s @ ..] => super::t37::n36219(s, out),
				['т', s @ ..] => super::t37::n36220(s, out),
				['ш', s @ ..] => super::t37::n36221(s, out),
				['ю', s @ ..] => super::t37::n36222(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t37::n14138(s, out),
		['м', s @ ..] => super::t37::n14139(s, out),
		['н', s @ ..] => super::t37::n14140(s, out),
		['п', s @ ..] => super::t37::n14141(s, out),
		['р', s @ ..] => super::t37::n14142(s, out),
		['с', s @ ..] => super::t37::n14143(s, out),
		['т', s @ ..] => super::t37::n14144(s, out),
		['х', s @ ..] => super::t37::n14145(s, out),
		['ч', s @ ..] => super::t37::n14146(s, out),
		['ш', s @ ..] => super::t37::n14147(s, out),
		['ь', s @ ..] => super::t37::n14148(s, out),
		_ => false,
	}
}

/// Words ending -йно.
pub(crate) fn n3071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3061);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14075);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35922);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кно.
pub(crate) fn n3072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3062);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14076);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63964);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90675);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111069);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 90676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14077);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35925);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мно.
pub(crate) fn n3073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3063);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14078);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14079);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14080);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35928);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нно.
pub(crate) fn n3074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3064);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14081);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14082);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35930);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63966);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90677);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111070);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124320);
													match s {
														[] => true,
														['р', s @ ..] => super::t37::n133665(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оно.
pub(crate) fn n3075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3065);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 14083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35931);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35932);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63968);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 90678);
									s.is_empty()
								}
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
			super::push(out, 14084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14085);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35934);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 35935);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63970);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90679);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111071);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14086);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35936);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63971);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 90680);
									s.is_empty()
								}
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
			super::push(out, 14087);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35937);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рно.
pub(crate) fn n3076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3066);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14088);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63973);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 63974);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 63975);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90682);
									s.is_empty()
								}
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
			super::push(out, 14089);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 35940);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63977);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90683);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111072);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124321);
													match s {
														[] => true,
														['з', s @ ..] => super::t37::n133666(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63978);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 63979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 35942);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63980);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90685);
									s.is_empty()
								}
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
			super::push(out, 14090);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35943);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 35944);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 63982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35945);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63983);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сно.
pub(crate) fn n3077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3067);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35946);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тно.
pub(crate) fn n3078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3068);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14092);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35947);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14093);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35948);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63986);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90686);
									s.is_empty()
								}
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
			super::push(out, 14094);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уно.
pub(crate) fn n3079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3069);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14095);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14096);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35951);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 35952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14097);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35953);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63987);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 90687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хно.
pub(crate) fn n3080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3070);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14098);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чно.
pub(crate) fn n3081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3071);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14099);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35955);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63988);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90688);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111073);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124322);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шно.
pub(crate) fn n3082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14100);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35956);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14101);
			match s {
				[] => true,
				['р', 'о', 'б', '\u{1}', s @ ..] => {
					super::push(out, 35957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыно.
pub(crate) fn n3083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3073);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 14102);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35958);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63990);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90689);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 63991);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90690);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 111075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьно.
pub(crate) fn n3084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3074);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14103);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35959);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63992);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90691);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111076);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124323);
													match s {
														[] => true,
														['в', s @ ..] => super::t37::n133667(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юно.
pub(crate) fn n3085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3075);
	s.is_empty()
}

/// Words ending -яно.
pub(crate) fn n3086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3076);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 14104);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35960);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63993);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90692);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111077);
											s.is_empty()
										}
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
					super::push(out, 35961);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63994);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90693);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111078);
											s.is_empty()
										}
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
					super::push(out, 35962);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63995);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90694);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111079);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124324);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иманджаро.
pub(crate) fn n125611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124325);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 132306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атандзаро.
pub(crate) fn n125612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124326);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 132307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пиффераро.
pub(crate) fn n125613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124327);
	s.is_empty()
}

/// Words ending -полуребро.
pub(crate) fn n125614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124328);
	s.is_empty()
}

/// Words ending -гро.
pub(crate) fn n3101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3091);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35994);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64018);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 90708);
									s.is_empty()
								}
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
			super::push(out, 14135);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35995);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64019);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35996);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64020);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90710);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111089);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124329);
													match s {
														[] => true,
														['м', s @ ..] => super::t37::n133670(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дро.
pub(crate) fn n3102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3092);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14136);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35997);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64021);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 64022);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 35998);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64023);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90713);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111090);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14137);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35999);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14138);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36000);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 36001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14139);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14140);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14141);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36004);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64026);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36005);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64027);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еро.
pub(crate) fn n3103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3093);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14142);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36006);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90716);
									s.is_empty()
								}
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
			super::push(out, 14143);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36007);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64029);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90717);
									s.is_empty()
								}
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
			super::push(out, 14144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36008);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36009);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64030);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90718);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111091);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 124330);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14145);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36010);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90719);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 90720);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111092);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124331);
													match s {
														[] => true,
														['ж', s @ ..] => super::t37::n133671(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36011);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36012);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64033);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90721);
									s.is_empty()
								}
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
			super::push(out, 14147);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36013);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64034);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90722);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111093);
											s.is_empty()
										}
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
					super::push(out, 36014);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36015);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36016);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64037);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 90723);
									s.is_empty()
								}
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
			super::push(out, 14148);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36017);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64038);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 64039);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90725);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111094);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14149);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 36018);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64040);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90726);
									s.is_empty()
								}
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
			super::push(out, 14150);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36019);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 64041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14151);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36020);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64042);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90727);
									s.is_empty()
								}
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
			super::push(out, 14152);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36021);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64043);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90728);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 64044);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 36022);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64045);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90730);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 111095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иро.
pub(crate) fn n3104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3094);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 14153);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 14154);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14155);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 36024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14156);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36025);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 14157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ейро.
pub(crate) fn n14232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14158);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 36026);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36027);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 36028);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кро.
pub(crate) fn n3106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3096);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14159);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 36029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14160);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36030);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64050);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нро.
pub(crate) fn n3107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3097);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14161);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14162);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36031);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оро.
pub(crate) fn n3108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3098);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14163);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36032);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64051);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 90732);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111096);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14164);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 14165);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36033);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64052);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рро.
pub(crate) fn n3109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3099);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14166);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 36034);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64053);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90734);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 111097);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14167);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64054);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 90735);
									s.is_empty()
								}
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
			super::push(out, 14168);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 36036);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тро.
pub(crate) fn n3110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3100);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14169);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 36037);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14170);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36038);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64056);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90736);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111098);
											s.is_empty()
										}
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
					super::push(out, 36039);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 36040);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 64057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14171);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36041);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14172);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36043);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64059);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 90737);
									s.is_empty()
								}
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
			super::push(out, 14173);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36044);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36045);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64060);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90738);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111099);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14174);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36046);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 64061);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90739);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111100);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124332);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36047);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36048);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 36049);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64064);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 36050);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 36052);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64067);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90741);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111101);
											s.is_empty()
										}
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
					super::push(out, 36053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36054);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уро.
pub(crate) fn n3111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3101);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14177);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14178);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36055);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64068);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90742);
									s.is_empty()
								}
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
			super::push(out, 14179);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36056);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 64069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14180);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фро.
pub(crate) fn n3112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3102);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14181);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юро.
pub(crate) fn n3113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3103);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14182);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 36058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кина-фасо.
pub(crate) fn n125619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124333);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 132310);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 137026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор-колесо.
pub(crate) fn n125620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124334);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 132311);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137027);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 139840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясо.
pub(crate) fn n3125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3115);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уанахуато.
pub(crate) fn n125621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124335);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 132312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инкогнито.
pub(crate) fn n125622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124336);
	s.is_empty()
}

/// Words ending -инито.
pub(crate) fn n36437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36096);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 64092);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 90755);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 111109);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пито.
pub(crate) fn n14300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14226);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36097);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 64093);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рито.
pub(crate) fn n14301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14227);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36098);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 64094);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90757);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 111110);
									s.is_empty()
								}
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
			super::push(out, 36099);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64095);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сито.
pub(crate) fn n14302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14228);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36100);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 36101);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64096);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90759);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фито.
pub(crate) fn n14304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14230);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 36102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64097);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90760);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 111112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хито.
pub(crate) fn n14305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14231);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 36103);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 64098);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36104);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 64099);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 64100);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90762);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 111113);
									s.is_empty()
								}
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
			super::push(out, 36105);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 90763);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кто.
pub(crate) fn n3132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3122);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14232);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36106);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64102);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90764);
									s.is_empty()
								}
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
			super::push(out, 14233);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14234);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нто.
pub(crate) fn n3133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3123);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14235);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36109);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64104);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 64105);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90765);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111115);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124338);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14236);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 36113);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64107);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90766);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111116);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 124339);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36114);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 64108);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90767);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111117);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 124340);
													match s {
														[] => true,
														['д', s @ ..] => super::t37::n133675(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36115);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 36116);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64109);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90768);
									s.is_empty()
								}
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
			super::push(out, 14237);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14238);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36119);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36120);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64112);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90769);
									s.is_empty()
								}
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
			super::push(out, 14239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ото.
pub(crate) fn n3134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3124);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14240);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14241);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 14242);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36122);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36123);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36124);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64113);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 64114);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 64115);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90770);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111118);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124341);
													match s {
														[] => true,
														['н', s @ ..] => super::t37::n133676(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14243);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36125);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64116);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 64117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14244);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рто.
pub(crate) fn n3135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3125);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14245);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36127);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64118);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 90771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36128);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64119);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90772);
									s.is_empty()
								}
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
			super::push(out, 14246);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сто.
pub(crate) fn n3136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3126);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14247);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36130);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64120);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90773);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111119);
											s.is_empty()
										}
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
					super::push(out, 36131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36132);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 64122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14248);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36134);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64123);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36135);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64124);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90775);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 90776);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111120);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124342);
													match s {
														[] => true,
														['о', s @ ..] => super::t37::n133677(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14249);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тто.
pub(crate) fn n3137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3127);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14250);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36137);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64125);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36138);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64126);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90778);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111121);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124343);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', 'и', s @ ..] => {
							super::push(out, 64127);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90779);
									s.is_empty()
								}
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
			super::push(out, 14251);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 36139);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уто.
pub(crate) fn n3138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3128);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14254);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36142);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64128);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90780);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111122);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14256);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36144);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64129);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90781);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хто.
pub(crate) fn n3139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3129);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14257);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14258);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 36145);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64130);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90782);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -что.
pub(crate) fn n3140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3130);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14259);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36146);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64131);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90783);
									s.is_empty()
								}
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
			super::push(out, 14260);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыто.
pub(crate) fn n3141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3131);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14261);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 14262);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36148);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14263);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36149);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьто.
pub(crate) fn n3142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3132);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14265);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36150);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 64134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90784);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 90785);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 111125);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124344);
													match s {
														[] => true,
														['л', s @ ..] => super::t37::n133678(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64135);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90786);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111126);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124345);
													s.is_empty()
												}
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
							super::push(out, 64136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -это.
pub(crate) fn n3143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3133);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14266);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -акамакафо.
pub(crate) fn n125632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124346);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 132317);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эн-дэн-цо.
pub(crate) fn n125633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124347);
	s.is_empty()
}

/// Words ending -серебрецо.
pub(crate) fn n125634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124348);
	s.is_empty()
}

/// Words ending -йцо.
pub(crate) fn n3166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3156);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 14305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мцо.
pub(crate) fn n3167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3157);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14306);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36190);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нцо.
pub(crate) fn n3168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3158);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14307);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36191);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 36192);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 64168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14308);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36193);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14309);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36195);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рцо.
pub(crate) fn n3169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3159);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14310);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36196);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 36197);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36198);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90807);
									s.is_empty()
								}
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
			super::push(out, 14311);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сцо.
pub(crate) fn n3170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3160);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14312);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36200);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64172);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90808);
									s.is_empty()
								}
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
			super::push(out, 14313);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36201);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тцо.
pub(crate) fn n3171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3161);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14314);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 36202);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64173);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90809);
									s.is_empty()
								}
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
			super::push(out, 14315);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36203);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64174);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ццо.
pub(crate) fn n3172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3162);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14316);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36204);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64175);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90811);
									s.is_empty()
								}
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
			super::push(out, 14317);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36205);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64176);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90812);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111134);
											s.is_empty()
										}
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
					super::push(out, 36206);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14318);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36207);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64178);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьцо.
pub(crate) fn n3173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3163);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14319);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36208);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 36209);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36210);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64181);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90814);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111135);
											s.is_empty()
										}
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
					super::push(out, 36211);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64182);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эль-ниньо.
pub(crate) fn n125635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124349);
	s.is_empty()
}

/// Words ending -ик-прицеп.
pub(crate) fn n125636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124350);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 132318);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137031);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 139841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улмуталип.
pub(crate) fn n125637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124351);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 132319);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word привсхлип.
pub(crate) fn n125638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124352);
	s.is_empty()
}

/// Words ending -окшип.
pub(crate) fn n36623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36282);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 64235);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 111147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ншип.
pub(crate) fn n14483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14409);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 36283);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64236);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 90841);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ошип.
pub(crate) fn n14484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14410);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 36284);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щип.
pub(crate) fn n3224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3214);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 14411);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хиепископ.
pub(crate) fn n125639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124353);
	s.is_empty()
}

/// The word пирископ.
pub(crate) fn n125640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124354);
	s.is_empty()
}

/// Words ending -толоп.
pub(crate) fn n36649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36308);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 64254);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90852);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слоп.
pub(crate) fn n14506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14432);
	s.is_empty()
}

/// Words ending -флоп.
pub(crate) fn n14507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14433);
	s.is_empty()
}

/// Words ending -хлоп.
pub(crate) fn n14508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14434);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 36310);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64255);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 36311);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ноп.
pub(crate) fn n3241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3231);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 14435);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поп.
pub(crate) fn n3242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3232);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14436);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 14437);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36313);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64257);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90855);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111157);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124355);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36314);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64258);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 90856);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111158);
											s.is_empty()
										}
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
					super::push(out, 36315);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36316);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 36317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14438);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 36318);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64259);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90857);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111159);
											s.is_empty()
										}
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
					super::push(out, 36319);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64260);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90858);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111160);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124356);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14439);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 14440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36320);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90859);
									s.is_empty()
								}
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
			super::push(out, 14441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роп.
pub(crate) fn n3243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3233);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36321);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 36322);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64262);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 90860);
									s.is_empty()
								}
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
			super::push(out, 14443);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14444);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 36324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14445);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 36325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36326);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 64264);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90861);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111161);
											s.is_empty()
										}
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
					super::push(out, 36327);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64265);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90862);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 90863);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111162);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124357);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 90864);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111163);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124358);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 90865);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111164);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124359);
													match s {
														[] => true,
														['ц', s @ ..] => super::t37::n133682(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', 'и', 'л', s @ ..] => {
									super::push(out, 90866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111165);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', 'и', s @ ..] => {
									super::push(out, 90867);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111166);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'и', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111167);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124360);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 111168);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 90868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 64266);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90869);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111169);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124361);
													match s {
														[] => true,
														['р', s @ ..] => super::t37::n133683(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36328);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64267);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90870);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111170);
											s.is_empty()
										}
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
					super::push(out, 36329);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соп.
pub(crate) fn n3244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3234);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14447);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -топ.
pub(crate) fn n3245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3235);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14448);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36331);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64270);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 90871);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111171);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14449);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14450);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64271);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90872);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 64272);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90873);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111173);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124362);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хоп.
pub(crate) fn n3246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3236);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14452);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36333);
					match s {
						[] => true,
						['и', 'х', s @ ..] => {
							super::push(out, 64273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шоп.
pub(crate) fn n3247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3237);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кингисепп.
pub(crate) fn n125649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124363);
	s.is_empty()
}

/// Words ending -ступ.
pub(crate) fn n14566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14492);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36366);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 64293);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 64294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 36367);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64295);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36368);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64296);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90883);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111177);
									s.is_empty()
								}
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
			super::push(out, 36369);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 64297);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90884);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111178);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124364);
											s.is_empty()
										}
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
						['\u{1}', s @ ..] => {
							super::push(out, 90885);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 90886);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 64298);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 90887);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 111180);
									s.is_empty()
								}
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
			super::push(out, 36370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36371);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 36372);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -головотяп.
pub(crate) fn n125651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124365);
	s.is_empty()
}

/// Words ending -квидамбар.
pub(crate) fn n125652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124366);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 132322);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 137033);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -звар.
pub(crate) fn n14593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14519);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 36396);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 36397);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ивар.
pub(crate) fn n14594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14520);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36398);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64320);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90898);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 90899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 36399);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 64321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -квар.
pub(crate) fn n14595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14521);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 36400);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64322);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 90900);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111184);
									s.is_empty()
								}
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
			super::push(out, 36401);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64323);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 90901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инвар.
pub(crate) fn n36743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36402);
	s.is_empty()
}

/// Words ending -овар.
pub(crate) fn n14597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14523);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 36403);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64324);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 36404);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64325);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 90903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 36405);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64326);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 36406);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 64327);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 90905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 36407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64328);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 64329);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 64330);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 64331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 36408);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 36409);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64332);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 90906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 36410);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64333);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 64334);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 90907);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 111185);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 124367);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64335);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 90908);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111186);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 124368);
											s.is_empty()
										}
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
					super::push(out, 64336);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 36412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64337);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 64338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 36413);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 64339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвар.
pub(crate) fn n14598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36414);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 64340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36415);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 64341);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90910);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 111187);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 124369);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132323);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n138450(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твар.
pub(crate) fn n14599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14525);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 36416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64342);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увар.
pub(crate) fn n14600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14526);
	s.is_empty()
}

/// Words ending -ьвар.
pub(crate) fn n14601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14527);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36417);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 64343);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ювар.
pub(crate) fn n14602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14528);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 36418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гар.
pub(crate) fn n3278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3267);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14529);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 36419);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36420);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 64344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36421);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64345);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90913);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111188);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124370);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36422);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 64346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90914);
									s.is_empty()
								}
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
			super::push(out, 14531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36423);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64347);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 64348);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90915);
									s.is_empty()
								}
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
			super::push(out, 14532);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36424);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 36425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14533);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36426);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 36427);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64351);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90916);
									s.is_empty()
								}
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
			super::push(out, 14534);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36428);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14535);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14536);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36430);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64353);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36431);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90918);
									s.is_empty()
								}
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
			super::push(out, 14537);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36432);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 36433);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 64355);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90919);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111189);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14538);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14539);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 14540);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дар.
pub(crate) fn n3279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3268);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14541);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14542);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 36436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14543);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36439);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64359);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 64360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 36440);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64361);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90920);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111190);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36441);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 36442);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 36443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64362);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 64363);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90921);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111191);
											s.is_empty()
										}
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
					super::push(out, 36444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36445);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'г', '\u{1}', s @ ..] => {
					super::push(out, 36446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14546);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64366);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 64367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 36448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36449);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 64368);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90922);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 64369);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90923);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111193);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124371);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133686(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36450);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14548);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36451);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64371);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90924);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 111194);
											s.is_empty()
										}
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
					super::push(out, 36452);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 64372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36453);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64373);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90925);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111195);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 90926);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111196);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124372);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133687(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64374);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90927);
									s.is_empty()
								}
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
			super::push(out, 14549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36454);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 36455);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36456);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64377);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90928);
									s.is_empty()
								}
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
			super::push(out, 14551);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36457);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64378);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90929);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111197);
											s.is_empty()
										}
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
					super::push(out, 36458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 36459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64380);
							s.is_empty()
						}
						['ж', s @ ..] => super::t37::n65045(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t37::n36801(s, out),
				['у', s @ ..] => super::t37::n36802(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t37::n14626(s, out),
		_ => false,
	}
}

/// Words ending -еар.
pub(crate) fn n3280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3269);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14553);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жар.
pub(crate) fn n3281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3270);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14554);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14555);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зар.
pub(crate) fn n3282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3271);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36464);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 36465);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 36466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 36467);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 36468);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64386);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36469);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 36470);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36471);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 36472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14557);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14558);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36475);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64390);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 64391);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 90933);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111199);
											s.is_empty()
										}
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
					super::push(out, 36476);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64392);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90934);
									s.is_empty()
								}
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
			super::push(out, 14559);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 36477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14560);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36478);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иар.
pub(crate) fn n3283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3272);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 14561);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36480);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64394);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 90935);
									s.is_empty()
								}
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
			super::push(out, 14562);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14563);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 14564);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36482);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 64395);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кар.
pub(crate) fn n3284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3273);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14565);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 36484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 14566);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64396);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 64397);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90938);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111200);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124373);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36486);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36488);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64399);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36489);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64400);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36490);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64401);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36491);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64402);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 36492);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64403);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90943);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111201);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124374);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133688(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64404);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90944);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 111202);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124375);
													match s {
														[] => true,
														['х', s @ ..] => super::t37::n133689(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14567);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14568);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 14569);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36494);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64405);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14570);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36496);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 36497);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64406);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90946);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111203);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124376);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14571);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 36498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14572);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36499);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64407);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90947);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111204);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14573);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14574);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64408);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 64409);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14575);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 36504);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64412);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90949);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 111205);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14576);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64413);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 64414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лар.
pub(crate) fn n3285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14577);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14578);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36505);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 36506);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64415);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90951);
									s.is_empty()
								}
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
			super::push(out, 14579);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36507);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14580);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 36509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36510);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14582);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36511);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14583);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 36512);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64421);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90952);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111206);
											s.is_empty()
										}
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
					super::push(out, 36513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36514);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мар.
pub(crate) fn n3286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3275);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 14585);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36515);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 64423);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 90953);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111207);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14586);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36516);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 64424);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36517);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90955);
									s.is_empty()
								}
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
			super::push(out, 14587);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 36518);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 64426);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90956);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111208);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124377);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t37::n133690(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36519);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64427);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90957);
									s.is_empty()
								}
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
			super::push(out, 14588);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36520);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 36521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36522);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14590);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36523);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90958);
									s.is_empty()
								}
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
			super::push(out, 14591);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36524);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14592);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14593);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 36527);
					s.is_empty()
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64432);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 64433);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 90959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 64434);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 14594);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36529);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14595);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64437);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 64438);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нар.
pub(crate) fn n3287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3276);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14596);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14597);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14598);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36532);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14599);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14600);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 36533);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 36534);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64440);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 36535);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64441);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36536);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64442);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90965);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111209);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124378);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14601);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14602);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36538);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64443);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90966);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111210);
											s.is_empty()
										}
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
						['с', s @ ..] => {
							super::push(out, 64444);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90967);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111211);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 124379);
													s.is_empty()
												}
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
							super::push(out, 64445);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36539);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 36540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14603);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36541);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14604);
			match s {
				[] => true,
				['м', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 36542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оар.
pub(crate) fn n3288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3277);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 14605);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36543);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пар.
pub(crate) fn n3289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3278);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14606);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 14607);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14608);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 36545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14609);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36546);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14610);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36547);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14611);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36548);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64451);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90970);
									s.is_empty()
								}
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
			super::push(out, 14612);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36549);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 64452);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90971);
									s.is_empty()
								}
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
			super::push(out, 14613);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рар.
pub(crate) fn n3290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3279);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14614);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 36551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14615);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36552);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64455);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 90972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сар.
pub(crate) fn n3291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3280);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14616);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36553);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 64456);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90973);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111212);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14617);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36554);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14618);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 36555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36556);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14620);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14621);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36558);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64460);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90974);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111213);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124380);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14622);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36559);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14623);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36560);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64462);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90975);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111214);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124381);
													match s {
														[] => true,
														['т', s @ ..] => super::t37::n133691(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36561);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14624);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36562);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 64464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 36563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64465);
							s.is_empty()
						}
						['-', 'б', 'й', 'е', 'л', '\u{1}', s @ ..] => {
							super::push(out, 64466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14625);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14626);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36565);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64467);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тар.
pub(crate) fn n3292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3281);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14627);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36566);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14628);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36568);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14629);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64469);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 64470);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90979);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111215);
											s.is_empty()
										}
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
					super::push(out, 36570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14630);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36571);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64471);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90980);
									s.is_empty()
								}
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
			super::push(out, 14631);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14632);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 14633);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14634);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 14635);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36573);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 64472);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90981);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111216);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36574);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90982);
									s.is_empty()
								}
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
			super::push(out, 14637);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 36575);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64474);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90983);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 64475);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90984);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111218);
											s.is_empty()
										}
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
					super::push(out, 36576);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64476);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90985);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111219);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124382);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14638);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 14639);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 36577);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64477);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90986);
									s.is_empty()
								}
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
			super::push(out, 14640);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 14641);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уар.
pub(crate) fn n3293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3282);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14642);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14643);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 36580);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 36581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 36582);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 36583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14644);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36584);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90987);
									s.is_empty()
								}
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
			super::push(out, 14645);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 36586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36587);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'п', '\u{1}', s @ ..] => {
					super::push(out, 36588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14646);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фар.
pub(crate) fn n3294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3283);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36590);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 36591);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36592);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 36593);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 36594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14648);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36595);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14649);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36596);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64485);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90988);
									s.is_empty()
								}
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
			super::push(out, 14650);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 36597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14651);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36598);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 64486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 14652);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36599);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64487);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хар.
pub(crate) fn n3295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3284);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14653);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36600);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 36601);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36602);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36603);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 36604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64489);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 64490);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90990);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111220);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124383);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133692(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64491);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90991);
									s.is_empty()
								}
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
			super::push(out, 14654);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 36605);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64492);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90992);
									s.is_empty()
								}
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
			super::push(out, 14655);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 36606);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36608);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64494);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 36609);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цар.
pub(crate) fn n3296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3285);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 14658);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36610);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 64495);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 90994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чар.
pub(crate) fn n3297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3286);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14659);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36611);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64496);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36612);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64497);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90996);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111221);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124384);
													s.is_empty()
												}
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
							super::push(out, 64498);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90997);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 111222);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124385);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14660);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14661);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 36614);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 36615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14662);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36616);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36617);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64500);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90998);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 64501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14663);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14664);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 36619);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64503);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90999);
									s.is_empty()
								}
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
			super::push(out, 14665);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36620);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 64504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шар.
pub(crate) fn n3298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14666);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14667);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14668);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14669);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36622);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14670);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 14672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щар.
pub(crate) fn n3299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3288);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 14673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юар.
pub(crate) fn n3300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3289);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 14674);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ихтиандр.
pub(crate) fn n125672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124386);
	s.is_empty()
}

/// Words ending -оид-додекаэдр.
pub(crate) fn n125673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124387);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 132331);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 137038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139845);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алибер.
pub(crate) fn n65206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64542);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 91018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шибер.
pub(crate) fn n37026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36681);
	s.is_empty()
}

/// Words ending -йбер.
pub(crate) fn n14800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14723);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36682);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 36683);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 64543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лбер.
pub(crate) fn n14801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14724);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36684);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 64544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбер.
pub(crate) fn n14802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14725);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36685);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 64545);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91020);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111230);
									s.is_empty()
								}
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
			super::push(out, 36686);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 64546);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 91021);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 111231);
									s.is_empty()
								}
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
			super::push(out, 36687);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64547);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 64548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 36688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64549);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обер.
pub(crate) fn n14803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14726);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 36689);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64550);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91022);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 111232);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 124388);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132332);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 36690);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 64551);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91023);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 111233);
									s.is_empty()
								}
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
			super::push(out, 36691);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64552);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 64553);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91024);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111234);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124389);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбер.
pub(crate) fn n14804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14727);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36692);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36693);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 64556);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91026);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 64557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36694);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 64558);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 91028);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 111236);
									s.is_empty()
								}
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
			super::push(out, 36695);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 64559);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91029);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 111237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сбер.
pub(crate) fn n14805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14728);
	s.is_empty()
}

/// Words ending -убер.
pub(crate) fn n14806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14729);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 36696);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 36697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 36698);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 64561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 36699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьбер.
pub(crate) fn n14807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14730);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36700);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 64562);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 91030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбер.
pub(crate) fn n14808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14731);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 64563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 36702);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 64564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябер.
pub(crate) fn n14809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14732);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 36703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вер.
pub(crate) fn n3330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3319);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14733);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 36704);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 36705);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91031);
									s.is_empty()
								}
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
			super::push(out, 14734);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14735);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36707);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64566);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36708);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64567);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 91033);
									s.is_empty()
								}
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
			super::push(out, 14736);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36709);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64568);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91034);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111238);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124390);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n133695(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64569);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36710);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36712);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 36713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14737);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36714);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 36716);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64574);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91037);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111239);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 64575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 64576);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 91039);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111240);
											s.is_empty()
										}
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
					super::push(out, 36717);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64577);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36718);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64578);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91041);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111241);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124391);
													s.is_empty()
												}
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
							super::push(out, 64579);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91042);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111242);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 124392);
													match s {
														[] => true,
														['р', s @ ..] => super::t37::n133696(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36719);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64580);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91043);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 111243);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124393);
													match s {
														[] => true,
														['о', s @ ..] => super::t37::n133697(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 111244);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 64581);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91044);
									s.is_empty()
								}
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
			super::push(out, 14738);
			match s {
				[] => true,
				['а', s @ ..] => super::t37::n37065(s, out),
				['е', s @ ..] => super::t37::n37066(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14739);
			match s {
				[] => true,
				['с', s @ ..] => super::t37::n37067(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t37::n14817(s, out),
		['н', s @ ..] => super::t37::n14818(s, out),
		['о', s @ ..] => super::t37::n14819(s, out),
		['р', s @ ..] => super::t37::n14820(s, out),
		['с', s @ ..] => super::t37::n14821(s, out),
		['т', s @ ..] => super::t37::n14822(s, out),
		['у', s @ ..] => super::t37::n14823(s, out),
		['ь', s @ ..] => super::t37::n14824(s, out),
		_ => false,
	}
}

/// Words ending -гер.
pub(crate) fn n3331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3320);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14748);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36743);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64619);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 36744);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 36745);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36746);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64622);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91073);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111263);
											s.is_empty()
										}
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
					super::push(out, 36747);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14749);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36748);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64624);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91074);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111264);
											s.is_empty()
										}
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
					super::push(out, 36749);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 36750);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 64626);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 64627);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 64628);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 64629);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91075);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111265);
											s.is_empty()
										}
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
					super::push(out, 36751);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91076);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 64631);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91077);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 91078);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 91079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 36752);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 36753);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14750);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36754);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36755);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14751);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36756);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64636);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 91080);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111266);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 91081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111267);
											s.is_empty()
										}
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
					super::push(out, 36757);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 36758);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64637);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 91082);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111268);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14752);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36759);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64638);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 91083);
									s.is_empty()
								}
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
			super::push(out, 14753);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36760);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64639);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 91084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 36761);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64640);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 91085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t37::n37107(s, out),
				['о', s @ ..] => {
					super::push(out, 36763);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64648);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 36764);
					match s {
						[] => true,
						['х', s @ ..] => super::t37::n65314(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14754);
			match s {
				[] => true,
				['л', s @ ..] => super::t37::n37110(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t37::n14832(s, out),
		['э', s @ ..] => {
			super::push(out, 14756);
			s.is_empty()
		}
		['ю', s @ ..] => super::t37::n14834(s, out),
		['я', s @ ..] => super::t37::n14835(s, out),
		_ => false,
	}
}

/// Words ending -дер.
pub(crate) fn n3332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3321);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14759);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14760);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36773);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64659);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91105);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111283);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124405);
													match s {
														[] => true,
														['д', s @ ..] => super::t37::n133702(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64660);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91106);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111284);
											s.is_empty()
										}
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
					super::push(out, 36774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64661);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91107);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111285);
											s.is_empty()
										}
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
					super::push(out, 36775);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64662);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91108);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111286);
											s.is_empty()
										}
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
					super::push(out, 36776);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64663);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91109);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111287);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14761);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36777);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 64664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 36778);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64665);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36779);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14762);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36780);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 64667);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91111);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111288);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 124406);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36781);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64668);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36782);
					match s {
						[] => true,
						['п', 'с', 'р', s @ ..] => {
							super::push(out, 64669);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91113);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111289);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 124407);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133703(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64670);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91114);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111290);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 124408);
													s.is_empty()
												}
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
							super::push(out, 64671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 36783);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14763);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36785);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64673);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91115);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 64674);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 64675);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91116);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111291);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 91117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 64676);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 64677);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91118);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111292);
											s.is_empty()
										}
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
					super::push(out, 36786);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64678);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36788);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 36789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t37::n14841(s, out),
		['л', s @ ..] => super::t37::n14842(s, out),
		['н', s @ ..] => super::t37::n14843(s, out),
		['о', s @ ..] => super::t37::n14844(s, out),
		['р', s @ ..] => super::t37::n14845(s, out),
		['у', s @ ..] => super::t37::n14846(s, out),
		['ь', s @ ..] => super::t37::n14847(s, out),
		['ю', s @ ..] => super::t37::n14848(s, out),
		_ => false,
	}
}

/// Words ending -еер.
pub(crate) fn n3333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3322);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36818);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 36819);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64744);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 64745);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91194);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111341);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 91195);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111342);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124435);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'ш', 'р', 'е', 'ф', 'н', 'и', 'т', 'у', 'п', '\u{1}', s @ ..] => {
			super::push(out, 14774);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жер.
pub(crate) fn n3334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3323);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14775);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36820);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64746);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91196);
									match s {
										[] => true,
										['т', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 124436);
													match s {
														[] => true,
														['д', s @ ..] => super::t37::n133714(s, out),
														['и', s @ ..] => super::t37::n133715(s, out),
														['н', s @ ..] => super::t37::n133716(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 124437);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36821);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 64747);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91197);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111343);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124438);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133717(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36822);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64748);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91198);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111344);
											s.is_empty()
										}
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
					super::push(out, 36823);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64749);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91199);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111345);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 64750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36824);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 64751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14776);
			match s {
				[] => true,
				['а', s @ ..] => super::t37::n37170(s, out),
				['е', s @ ..] => super::t37::n37171(s, out),
				['и', s @ ..] => super::t37::n37172(s, out),
				['й', s @ ..] => super::t37::n37173(s, out),
				['н', s @ ..] => super::t37::n37174(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t37::n14854(s, out),
		['и', s @ ..] => super::t37::n14855(s, out),
		['н', s @ ..] => super::t37::n14856(s, out),
		['р', s @ ..] => super::t37::n14857(s, out),
		['у', s @ ..] => super::t37::n14858(s, out),
		['я', s @ ..] => super::t37::n14859(s, out),
		_ => false,
	}
}

/// Words ending -зер.
pub(crate) fn n3335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3324);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14783);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36840);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 36841);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 36842);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 36843);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 36844);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 64775);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91222);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111360);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124450);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36845);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 64776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36846);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64777);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91223);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111361);
											s.is_empty()
										}
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
					super::push(out, 36847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14784);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 36848);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36849);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 64780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14785);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36851);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 64782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14786);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 36852);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 36853);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64783);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 64784);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91224);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111362);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124451);
													match s {
														[] => true,
														['р', s @ ..] => super::t37::n133725(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36854);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64786);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 64787);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91225);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111363);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124452);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n133726(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64788);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91226);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111364);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 64789);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91227);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111365);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124453);
													match s {
														[] => true,
														['в', s @ ..] => super::t37::n133727(s, out),
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
									super::push(out, 91228);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111366);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124454);
													match s {
														[] => true,
														['г', s @ ..] => super::t37::n133728(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64790);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91229);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111367);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124455);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133729(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36855);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64791);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t37::n14864(s, out),
		['н', s @ ..] => {
			super::push(out, 14788);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36858);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 36859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t37::n14866(s, out),
		['у', s @ ..] => super::t37::n14867(s, out),
		['ю', s @ ..] => {
			super::push(out, 14791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36867);
					s.is_empty()
				}
				['ь', s @ ..] => super::t37::n37213(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иер.
pub(crate) fn n3336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3325);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14792);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 36869);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64815);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91252);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111378);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14793);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 36870);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йер.
pub(crate) fn n3337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3326);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14794);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 36871);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64816);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91253);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111379);
											s.is_empty()
										}
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
					super::push(out, 36872);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 36873);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 64818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36874);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36875);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 64820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 36876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14795);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36877);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 64821);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91254);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111380);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 64822);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 91255);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111381);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124461);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36878);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 36879);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64823);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91256);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111382);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124462);
													match s {
														[] => true,
														['б', s @ ..] => super::t37::n133735(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 14796);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36881);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кер.
pub(crate) fn n3338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3327);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14797);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36882);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64825);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 36883);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64826);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 36884);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36885);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64828);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91259);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111383);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124463);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 36887);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 64830);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 64831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 36888);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64832);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91260);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111384);
											s.is_empty()
										}
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
					super::push(out, 36889);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14798);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64834);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 64835);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 64836);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 91262);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111385);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124464);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 36891);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 36892);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 36893);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64839);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 91263);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111386);
											s.is_empty()
										}
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
					super::push(out, 36894);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64840);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 64841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36895);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 64842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64843);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 64844);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 64845);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91265);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111387);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 124465);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36896);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36897);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 36898);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 64846);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91266);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111388);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124466);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n133736(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64847);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 64848);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 64849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36899);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64850);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91267);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 64851);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91268);
									match s {
										[] => true,
										['х', s @ ..] => super::t37::n112571(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t37::n65520(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t37::n37247(s, out),
				['р', s @ ..] => super::t37::n37248(s, out),
				['т', s @ ..] => super::t37::n37249(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t37::n14877(s, out),
		['к', s @ ..] => super::t37::n14878(s, out),
		['л', s @ ..] => super::t37::n14879(s, out),
		['н', s @ ..] => super::t37::n14880(s, out),
		['о', s @ ..] => super::t37::n14881(s, out),
		['р', s @ ..] => super::t37::n14882(s, out),
		['с', s @ ..] => super::t37::n14883(s, out),
		['у', s @ ..] => super::t37::n14884(s, out),
		['ц', s @ ..] => super::t37::n14885(s, out),
		['э', s @ ..] => super::t37::n14886(s, out),
		['ю', s @ ..] => super::t37::n14887(s, out),
		['я', s @ ..] => super::t37::n14888(s, out),
		_ => false,
	}
}

/// Words ending -лер.
pub(crate) fn n3339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3328);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14811);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 36933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64900);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 64901);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 36934);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 36935);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 64903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 36936);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64904);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 91297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 64905);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 91298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 36937);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14812);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 36938);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64907);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 36939);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64908);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91300);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111407);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124478);
													match s {
														[] => true,
														['ь', s @ ..] => super::t37::n133740(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 36940);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 64909);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91301);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 64910);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 91302);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 91303);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111410);
											match s {
												[] => true,
												['а', s @ ..] => {
													match s {
														[] => true,
														['-', s @ ..] => super::t37::n133741(s, out),
														['з', s @ ..] => super::t37::n133742(s, out),
														_ => false,
													}
												}
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
							super::push(out, 64911);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36941);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 64912);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 64913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36942);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 64914);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91306);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111411);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14813);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 36943);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64915);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 91307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 36944);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14814);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 36945);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 36946);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64917);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 36947);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 64918);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 91309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 36948);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 64919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14815);
			match s {
				[] => true,
				['б', s @ ..] => super::t37::n37299(s, out),
				['д', s @ ..] => {
					super::push(out, 36950);
					match s {
						[] => true,
						['о', s @ ..] => super::t37::n65590(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t37::n37301(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t37::n14894(s, out),
		['й', s @ ..] => super::t37::n14895(s, out),
		['к', s @ ..] => super::t37::n14896(s, out),
		['л', s @ ..] => super::t37::n14897(s, out),
		['о', s @ ..] => super::t37::n14898(s, out),
		['п', s @ ..] => super::t37::n14899(s, out),
		['р', s @ ..] => super::t37::n14900(s, out),
		['с', s @ ..] => super::t37::n14901(s, out),
		['т', s @ ..] => super::t37::n14902(s, out),
		['у', s @ ..] => super::t37::n14903(s, out),
		['ф', s @ ..] => super::t37::n14904(s, out),
		['ц', s @ ..] => super::t37::n14905(s, out),
		_ => false,
	}
}

/// Words ending -мер.
pub(crate) fn n3340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3329);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 36994);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 36995);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 64983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 36996);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 36997);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 36998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 64985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 36999);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 64986);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91365);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 91366);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111448);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124503);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37000);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64987);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91367);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111449);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 91368);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 111450);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 64988);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91369);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111451);
											s.is_empty()
										}
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
					super::push(out, 37001);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 64989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14829);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14830);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37004);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 64990);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91370);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 111452);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124504);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37005);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 64991);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 64992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37006);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 64993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14832);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37007);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37008);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 64994);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37009);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 64995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 64996);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91374);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111453);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 64997);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 64998);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91376);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111454);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 64999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37010);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65000);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91377);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111455);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 65001);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91378);
									s.is_empty()
								}
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
			super::push(out, 14833);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37011);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65002);
							s.is_empty()
						}
						['т', s @ ..] => super::t37::n65673(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t37::n37362(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t37::n14912(s, out),
		['м', s @ ..] => super::t37::n14913(s, out),
		['о', s @ ..] => super::t37::n14914(s, out),
		['р', s @ ..] => super::t37::n14915(s, out),
		['т', s @ ..] => {
			super::push(out, 14838);
			s.is_empty()
		}
		['у', s @ ..] => super::t37::n14917(s, out),
		['х', s @ ..] => super::t37::n14918(s, out),
		['э', s @ ..] => super::t37::n14919(s, out),
		['ю', s @ ..] => super::t37::n14920(s, out),
		_ => false,
	}
}

/// Words ending -нер.
pub(crate) fn n3341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3330);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14843);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37042);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65039);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91406);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111477);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124518);
													s.is_empty()
												}
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
							super::push(out, 65040);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91407);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111478);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 124519);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 111479);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124520);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37043);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65041);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37044);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65042);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91409);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111480);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124521);
													s.is_empty()
												}
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
							super::push(out, 65043);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91410);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 65044);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 37045);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65045);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91412);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111482);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 124522);
													match s {
														[] => true,
														['е', s @ ..] => super::t37::n133760(s, out),
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
									super::push(out, 91413);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111483);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 124523);
													match s {
														[] => true,
														['и', s @ ..] => super::t37::n133761(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 111484);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124524);
													match s {
														[] => true,
														['м', s @ ..] => super::t37::n133762(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 37047);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37048);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65048);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91414);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111485);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124525);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14844);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37049);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65049);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14845);
			match s {
				[] => true,
				['д', s @ ..] => super::t37::n37400(s, out),
				['ж', s @ ..] => super::t37::n37401(s, out),
				['р', s @ ..] => super::t37::n37402(s, out),
				['т', s @ ..] => super::t37::n37403(s, out),
				['ч', s @ ..] => super::t37::n37404(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t37::n14924(s, out),
		['и', s @ ..] => super::t37::n14925(s, out),
		['й', s @ ..] => super::t37::n14926(s, out),
		['л', s @ ..] => super::t37::n14927(s, out),
		['н', s @ ..] => super::t37::n14928(s, out),
		['о', s @ ..] => super::t37::n14929(s, out),
		['р', s @ ..] => super::t37::n14930(s, out),
		['т', s @ ..] => super::t37::n14931(s, out),
		['у', s @ ..] => super::t37::n14932(s, out),
		['ш', s @ ..] => super::t37::n14933(s, out),
		['ь', s @ ..] => super::t37::n14934(s, out),
		['ю', s @ ..] => super::t37::n14935(s, out),
		_ => false,
	}
}

/// Words ending -пер.
pub(crate) fn n3342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3331);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14858);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14859);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37088);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65110);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65111);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 65112);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37090);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65113);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 37091);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37092);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65114);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91470);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111528);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14860);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37093);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65116);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 65117);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 37095);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65118);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 91472);
									s.is_empty()
								}
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
			super::push(out, 14861);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 37097);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 37098);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65122);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 65123);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 65124);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 65125);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91475);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111529);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124553);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133779(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 37100);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65128);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91476);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111530);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124554);
													match s {
														[] => true,
														['т', s @ ..] => super::t37::n133780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 14862);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37103);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 37104);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91477);
									s.is_empty()
								}
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
			super::push(out, 14863);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37105);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65130);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91478);
									s.is_empty()
								}
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
			super::push(out, 14864);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37106);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 65131);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91479);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111531);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14865);
			match s {
				[] => true,
				['а', s @ ..] => super::t37::n37457(s, out),
				['е', s @ ..] => super::t37::n37458(s, out),
				['и', s @ ..] => {
					super::push(out, 37109);
					match s {
						[] => true,
						['л', s @ ..] => super::t37::n65811(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t37::n37460(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t37::n14944(s, out),
		['п', s @ ..] => super::t37::n14945(s, out),
		['р', s @ ..] => super::t37::n14946(s, out),
		['с', s @ ..] => super::t37::n14947(s, out),
		['у', s @ ..] => super::t37::n14948(s, out),
		['ш', s @ ..] => super::t37::n14949(s, out),
		['ь', s @ ..] => super::t37::n14950(s, out),
		['э', s @ ..] => super::t37::n14951(s, out),
		_ => false,
	}
}

/// Words ending -рер.
pub(crate) fn n3343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3332);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14874);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37141);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 65180);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91521);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111559);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14875);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65181);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 37143);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65182);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91523);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111560);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 65183);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91524);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111561);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14876);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37144);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65184);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91525);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111562);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 124568);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14877);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37145);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65185);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91526);
									s.is_empty()
								}
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
			super::push(out, 14878);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37146);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 14879);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 37147);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65187);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91527);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111563);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124569);
													match s {
														[] => true,
														['ш', s @ ..] => super::t37::n133789(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65188);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 65189);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 65190);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91530);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111564);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124570);
													match s {
														[] => true,
														['г', s @ ..] => super::t37::n133790(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сер.
pub(crate) fn n3344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3333);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14880);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37148);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37149);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65191);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91531);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111565);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14881);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14882);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37151);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 37152);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65192);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91532);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111566);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14883);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37153);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65193);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37154);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65194);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 65195);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91534);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 65196);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91535);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 111567);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 91536);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111568);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 111569);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124571);
													match s {
														[] => true,
														['в', s @ ..] => super::t37::n133791(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 111570);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124572);
													match s {
														[] => true,
														['д', s @ ..] => super::t37::n133792(s, out),
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
									super::push(out, 91537);
									s.is_empty()
								}
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
			super::push(out, 14884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37155);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65197);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91538);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111571);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124573);
													s.is_empty()
												}
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
							super::push(out, 65198);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 65199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37156);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37157);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65201);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 91539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 65202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91540);
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									super::push(out, 91541);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 91542);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111572);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 124574);
													s.is_empty()
												}
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
							super::push(out, 65203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37158);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65204);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91543);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111573);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124575);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n133793(s, out),
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
									super::push(out, 91544);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111574);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 91545);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111575);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 124576);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t37::n14963(s, out),
		['о', s @ ..] => {
			super::push(out, 14886);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t37::n65889(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t37::n14965(s, out),
		['р', s @ ..] => super::t37::n14966(s, out),
		['с', s @ ..] => super::t37::n14967(s, out),
		['т', s @ ..] => super::t37::n14968(s, out),
		['у', s @ ..] => super::t37::n14969(s, out),
		['ы', s @ ..] => super::t37::n14970(s, out),
		['э', s @ ..] => super::t37::n14971(s, out),
		['ю', s @ ..] => super::t37::n14972(s, out),
		_ => false,
	}
}

