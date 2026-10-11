use flexion::Path;

/// Words ending -пнешь.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 3997);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 3998);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 3999);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9869);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 9870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4000);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9871);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17028);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 9872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9873);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 9874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17030);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 9875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9876);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4004);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 9878);
							s.is_empty()
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17032);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17033);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9879);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9881);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17034);
									s.is_empty()
								}
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
			super::push(out, 1096);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4008);
					match s {
						[] => true,
						['х', 'ы', s @ ..] => {
							super::push(out, 9882);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9883);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 9884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17036);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17037);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 9886);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 17039);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 23586);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17040);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 17041);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23587);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4012);
					match s {
						[] => true,
						['о', 'к', 'ы', s @ ..] => {
							super::push(out, 9887);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4013);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 9888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4014);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1101);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнешь.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4016);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4017);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9890);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9891);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9892);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17043);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4018);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4019);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9893);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9894);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4020);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4021);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1104);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4023);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4024);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9895);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4025);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1105);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1106);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4026);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9897);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 17046);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23588);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', 'к', 'ы', s @ ..] => {
							super::push(out, 9898);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17047);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 9899);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 17048);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23589);
											s.is_empty()
										}
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
					super::push(out, 4027);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4028);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9900);
							s.is_empty()
						}
						['ш', 'ы', s @ ..] => {
							super::push(out, 9901);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 9902);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снешь.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1107);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1108);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9903);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9904);
							s.is_empty()
						}
						['у', 'л', s @ ..] => {
							super::push(out, 9905);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4030);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4031);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4033);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9907);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17052);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 17053);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23590);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', 'ы', s @ ..] => {
							super::push(out, 9908);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9911);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9912);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9913);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9914);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4037);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9915);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17056);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4038);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4039);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9916);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17057);
									s.is_empty()
								}
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
			super::push(out, 1111);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4041);
					match s {
						[] => true,
						['о', 'п', 'ы', s @ ..] => {
							super::push(out, 9917);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17058);
									s.is_empty()
								}
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
			super::push(out, 1113);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4042);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4043);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1114);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4045);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4046);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9919);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4047);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 9920);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17060);
									s.is_empty()
								}
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
			super::push(out, 1116);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4048);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 9921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тнешь.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1118);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4049);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9922);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4050);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9923);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17062);
									s.is_empty()
								}
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
			super::push(out, 1120);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['о', 'б', 'ы', s @ ..] => {
					super::push(out, 4052);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4053);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4054);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4055);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9925);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17063);
									s.is_empty()
								}
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
			super::push(out, 1123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4056);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4057);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9926);
							match s {
								[] => true,
								['х', 'ы', s @ ..] => {
									super::push(out, 17064);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23591);
											s.is_empty()
										}
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
					super::push(out, 4058);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9927);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 9928);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 9929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9931);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 9932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'б', 'о', 'л', 'х', 'с', s @ ..] => {
					super::push(out, 4060);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4061);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9934);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 17066);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 9935);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 17067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'б', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унешь.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4064);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4065);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4066);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4069);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4070);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4071);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4072);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4073);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9937);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 9938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4076);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9940);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17068);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23592);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнешь.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4077);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4079);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9941);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9942);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9943);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9944);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17070);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23593);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9945);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9946);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9947);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4081);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9948);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9949);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9950);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1130);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4084);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9952);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9953);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9954);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9955);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4085);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9956);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9957);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9958);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 4087);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9960);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17075);
									s.is_empty()
								}
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
			super::push(out, 1133);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4088);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9961);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 9962);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 9963);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17076);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 17077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 9964);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9965);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17078);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 17079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9966);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9967);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17080);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 9968);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4090);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9969);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17082);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17083);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 9970);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 17085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9971);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4091);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9972);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9973);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9974);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9975);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9976);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9977);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9978);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17088);
									s.is_empty()
								}
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
			super::push(out, 1134);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4093);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 9979);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4094);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => super::t5::n10500(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t5::n4231(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t5::n1165(s, out),
		['ч', s @ ..] => super::t5::n1166(s, out),
		['ы', s @ ..] => super::t5::n1167(s, out),
		['э', s @ ..] => super::t5::n1168(s, out),
		['ю', s @ ..] => super::t5::n1169(s, out),
		['я', s @ ..] => super::t5::n1170(s, out),
		_ => false,
	}
}

/// Words ending -ацнешь.
pub(crate) fn n1171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1141);
	s.is_empty()
}

/// Words ending -чнешь.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шнешь.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1143);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынешь.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1144);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', 'о', s @ ..] => {
							super::push(out, 10006);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10007);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17108);
									s.is_empty()
								}
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
			super::push(out, 1146);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10008);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10009);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10010);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17109);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23596);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 10011);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьнешь.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4111);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнешь.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10012);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10013);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 10014);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10015);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10016);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10017);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10018);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10019);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10020);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10021);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 10022);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10023);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 10024);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17114);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23597);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янешь.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4115);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4116);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10025);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10026);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10027);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17115);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23598);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 10028);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 10029);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10030);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10031);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10032);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10033);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 17117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4120);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10035);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23599);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 23600);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 23601);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23602);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 23603);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 23604);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 28441);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1151);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4121);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17118);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23605);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'в', s @ ..] => {
									super::push(out, 17119);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 17120);
									s.is_empty()
								}
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
			super::push(out, 1152);
			match s {
				[] => true,
				['г', 'т', s @ ..] => {
					super::push(out, 4122);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4123);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10037);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10038);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 17122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17123);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 17124);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1153);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4125);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4126);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 17126);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23606);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17127);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23607);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'д', s @ ..] => {
							super::push(out, 10040);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 4127);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4128);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4129);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4130);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4131);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4132);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4133);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10041);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 10042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4134);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4135);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 10043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вуешь.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1223);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1224);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t5::n1260(s, out),
		['х', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4218);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гуешь.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 1226);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4219);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10094);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23796);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23797);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23798);
											s.is_empty()
										}
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
					super::push(out, 4220);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 10095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17214);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 17215);
									s.is_empty()
								}
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
			super::push(out, 1227);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4221);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4222);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17216);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10096);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10097);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 17218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4223);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4224);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 10100);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4225);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10101);
							match s {
								[] => true,
								['е', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23799);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23800);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 28663);
													s.is_empty()
												}
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
							super::push(out, 10102);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 17220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4226);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 10103);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 10104);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 10105);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17221);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 23801);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 10106);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10107);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17222);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 23802);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 28664);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 28665);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 17223);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17224);
									s.is_empty()
								}
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
			super::push(out, 1229);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10108);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1230);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10110);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10111);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 17225);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 17226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10112);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10113);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10114);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4227);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 10115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1231);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4228);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 4229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10117);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 10118);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1232);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4230);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дуешь.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1233);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4231);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4232);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10120);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10121);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10122);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10123);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17230);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23803);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 23804);
											s.is_empty()
										}
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
					super::push(out, 4234);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10124);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23805);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 23806);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23807);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1235);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1238);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10125);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10126);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4237);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10128);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 17231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10129);
							match s {
								[] => true,
								['п', 'с', s @ ..] => {
									super::push(out, 17232);
									match s {
										[] => true,
										['и', 'о', s @ ..] => {
											super::push(out, 23808);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28666);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4238);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17233);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17234);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28667);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 28668);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 28669);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['б', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23809);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23810);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 17235);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23811);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17236);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17237);
									match s {
										[] => true,
										['а', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28670);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 28671);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 28672);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 23812);
											s.is_empty()
										}
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
					super::push(out, 4239);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10130);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17238);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 17239);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 23813);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 28673);
													s.is_empty()
												}
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
							super::push(out, 10131);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4240);
					match s {
						[] => true,
						['е', s @ ..] => super::t5::n10660(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t5::n1275(s, out),
		['з', s @ ..] => super::t5::n1276(s, out),
		['и', s @ ..] => super::t5::n1277(s, out),
		['л', s @ ..] => super::t5::n1278(s, out),
		['н', s @ ..] => super::t5::n1279(s, out),
		['о', s @ ..] => super::t5::n1280(s, out),
		['р', s @ ..] => super::t5::n1281(s, out),
		['с', s @ ..] => {
			super::push(out, 1246);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t5::n1284(s, out),
		['ы', s @ ..] => super::t5::n1285(s, out),
		['я', s @ ..] => super::t5::n1286(s, out),
		_ => false,
	}
}

/// Words ending -вежуешь.
pub(crate) fn n4411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4265);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17270);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17271);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17272);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17273);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 23827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word межуешь.
pub(crate) fn n10688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10150);
	s.is_empty()
}

/// Words ending -амежуешь.
pub(crate) fn n10689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10151);
	s.is_empty()
}

/// Words ending -змежуешь.
pub(crate) fn n10690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10152);
	s.is_empty()
}

/// Words ending -ымежуешь.
pub(crate) fn n10691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10153);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -режуешь.
pub(crate) fn n4413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4267);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10154);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17275);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жжуешь.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1255);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зжуешь.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	s.is_empty()
}

/// Words ending -мжуешь.
pub(crate) fn n1293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1257);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4269);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10155);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нжуешь.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1258);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ожуешь.
pub(crate) fn n1295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1259);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 4271);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4272);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10156);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржуешь.
pub(crate) fn n1296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1260);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сжуешь.
pub(crate) fn n1297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1261);
	s.is_empty()
}

/// Words ending -тжуешь.
pub(crate) fn n1298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1262);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ужуешь.
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1263);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыжуешь.
pub(crate) fn n1300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1264);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яжуешь.
pub(crate) fn n1301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1265);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зуешь.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1266);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4279);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17277);
									s.is_empty()
								}
								['е', 'р', 'п', s @ ..] => {
									super::push(out, 17278);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23828);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17279);
									s.is_empty()
								}
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
			super::push(out, 1267);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10159);
							match s {
								[] => true,
								['р', 'т', 'о', s @ ..] => {
									super::push(out, 17280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4281);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 10160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4282);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4283);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4284);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t5::n10700(s, out),
						['е', s @ ..] => {
							super::push(out, 10161);
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', 'м', s @ ..] => super::t5::n18247(s, out),
								['в', s @ ..] => {
									super::push(out, 17286);
									s.is_empty()
								}
								['р', 'е', 'т', 'с', 'о', s @ ..] => {
									super::push(out, 17287);
									match s {
										[] => true,
										['р', s @ ..] => super::t5::n25224(s, out),
										_ => false,
									}
								}
								['т', s @ ..] => super::t5::n18250(s, out),
								_ => false,
							}
						}
						['л', s @ ..] => super::t5::n10703(s, out),
						['о', s @ ..] => super::t5::n10704(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t5::n4432(s, out),
				['н', s @ ..] => super::t5::n4433(s, out),
				['р', s @ ..] => super::t5::n4434(s, out),
				['т', s @ ..] => super::t5::n4435(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t5::n1305(s, out),
		['н', s @ ..] => super::t5::n1306(s, out),
		['у', s @ ..] => super::t5::n1307(s, out),
		['ь', s @ ..] => super::t5::n1308(s, out),
		['ю', s @ ..] => super::t5::n1309(s, out),
		_ => false,
	}
}

/// Words ending -куешь.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1273);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4290);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 10174);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4291);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 10175);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4292);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10176);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 17319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4293);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							super::push(out, 10177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4294);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17320);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 17321);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 10178);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 17323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4295);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10179);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10180);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4297);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10182);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23864);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 10183);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17324);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17325);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 17326);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10184);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17327);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 23867);
											s.is_empty()
										}
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
					super::push(out, 4298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1275);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4299);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 4300);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1276);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10187);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10188);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4303);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4305);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10191);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10192);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10193);
							match s {
								[] => true,
								['у', 'п', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23868);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23869);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 23870);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 10194);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10195);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 10196);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 17328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4307);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4308);
					match s {
						[] => true,
						['а', s @ ..] => super::t5::n10749(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t5::n4460(s, out),
				['р', s @ ..] => super::t5::n4461(s, out),
				['т', s @ ..] => super::t5::n4462(s, out),
				['ф', s @ ..] => super::t5::n4463(s, out),
				['ч', s @ ..] => super::t5::n4464(s, out),
				['ш', s @ ..] => {
					super::push(out, 4314);
					s.is_empty()
				}
				['щ', s @ ..] => super::t5::n4466(s, out),
				_ => false,
			}
		}
		['к', 'и', 'х', '\u{1}', s @ ..] => super::t5::n1317(s, out),
		['л', s @ ..] => super::t5::n1318(s, out),
		['н', s @ ..] => super::t5::n1319(s, out),
		['о', s @ ..] => super::t5::n1320(s, out),
		['п', s @ ..] => super::t5::n1321(s, out),
		['р', s @ ..] => super::t5::n1322(s, out),
		['с', s @ ..] => super::t5::n1323(s, out),
		['т', s @ ..] => super::t5::n1324(s, out),
		['у', s @ ..] => super::t5::n1325(s, out),
		['ч', s @ ..] => super::t5::n1326(s, out),
		['ш', s @ ..] => super::t5::n1327(s, out),
		['ы', s @ ..] => super::t5::n1328(s, out),
		['ь', s @ ..] => super::t5::n1329(s, out),
		['ю', s @ ..] => super::t5::n1330(s, out),
		['я', s @ ..] => super::t5::n1331(s, out),
		_ => false,
	}
}

/// Words ending -луешь.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10244);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10245);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4351);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10247);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10248);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 10249);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10250);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10251);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4353);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 4354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10254);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10255);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4358);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10257);
							match s {
								[] => true,
								['з', 'и', 'е', s @ ..] => {
									super::push(out, 17365);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23889);
											s.is_empty()
										}
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
					super::push(out, 4359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10258);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10259);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10260);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4360);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10261);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23890);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 23891);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 28699);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1299);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4361);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4362);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 10262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 4363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1301);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1303);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 4366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -муешь.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1304);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4367);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1305);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4368);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10264);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10265);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10266);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17368);
									s.is_empty()
								}
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
			super::push(out, 1306);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 10267);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23892);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23893);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 10268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17369);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4369);
					match s {
						[] => true,
						['т', 'ш', 'т', s @ ..] => {
							super::push(out, 10269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1307);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 4371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 1309);
			match s {
				[] => true,
				['и', 'р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10270);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['л', 'е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10272);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10273);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10274);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нуешь.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4374);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17371);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10277);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10278);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4376);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1313);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10281);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10282);
							s.is_empty()
						}
						['о', 'с', s @ ..] => {
							super::push(out, 10283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1314);
			match s {
				[] => true,
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10284);
							s.is_empty()
						}
						['с', 'а', 'з', s @ ..] => {
							super::push(out, 10285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1315);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4377);
					match s {
						[] => true,
						['а', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17373);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 17374);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23894);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17375);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 17376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4378);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1316);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'к', 'э', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23895);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23896);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 17377);
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 28700);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 28701);
													s.is_empty()
												}
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
								['\u{1}', s @ ..] => {
									super::push(out, 17378);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17379);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 17380);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17381);
									s.is_empty()
								}
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
			super::push(out, 1317);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4379);
					match s {
						[] => true,
						['л', 'о', 'б', 'о', 'с', 'о', s @ ..] => {
							super::push(out, 10287);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17382);
									s.is_empty()
								}
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
			super::push(out, 1318);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4382);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17383);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4383);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10290);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4384);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10292);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17385);
									s.is_empty()
								}
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
			super::push(out, 1319);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1320);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10293);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1321);
			match s {
				[] => true,
				['п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 4385);
					s.is_empty()
				}
				['р', 'о', 'б', '\u{1}', s @ ..] => {
					super::push(out, 4386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4387);
					s.is_empty()
				}
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10295);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4389);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4390);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4391);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10298);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 10299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пуешь.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4393);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4394);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4395);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'о', s @ ..] => {
					super::push(out, 4396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['а', 'х', 'о', s @ ..] => {
					super::push(out, 4397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1328);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10302);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 17387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10303);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23897);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23898);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 23899);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 28702);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1329);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4398);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17388);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 17389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 10304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['л', 'г', 'с', s @ ..] => {
					super::push(out, 4400);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4401);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -руешь.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4402);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4403);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10305);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 4404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10307);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4405);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 10309);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4406);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4407);
					match s {
						[] => true,
						['д', 'с', 'е', s @ ..] => {
							super::push(out, 10312);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 17390);
									s.is_empty()
								}
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
			super::push(out, 1335);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4408);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4409);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 10314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4411);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4412);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17391);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 23900);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 17392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4413);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 10315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1337);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10316);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10317);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4415);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4416);
					s.is_empty()
				}
				['з', 'е', 'р', 'ф', 'т', s @ ..] => {
					super::push(out, 4417);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'т', 'о', s @ ..] => {
					super::push(out, 4418);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4419);
					match s {
						[] => true,
						['и', 'т', 'с', 'о', s @ ..] => {
							super::push(out, 10321);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 10322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4420);
					match s {
						[] => true,
						['к', 'а', 'м', 'с', s @ ..] => {
							super::push(out, 10323);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4421);
					match s {
						[] => true,
						['о', 'н', '\u{1}', s @ ..] => {
							super::push(out, 10325);
							s.is_empty()
						}
						['у', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17394);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17395);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 17396);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4422);
					match s {
						[] => true,
						['а', 'ф', 'т', s @ ..] => {
							super::push(out, 10326);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 10327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4423);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4424);
					match s {
						[] => true,
						['у', 'ф', 'о', s @ ..] => {
							super::push(out, 10329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 4425);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10330);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n1375(s, out),
		['о', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4449);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4451);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 10501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1340);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4452);
					match s {
						[] => true,
						['т', 'т', s @ ..] => {
							super::push(out, 10502);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4453);
					match s {
						[] => true,
						['е', 'ц', '\u{1}', s @ ..] => {
							super::push(out, 10503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4454);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17731);
									s.is_empty()
								}
								['к', 'р', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24291);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 24292);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 10504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => super::t5::n4618(s, out),
				['ь', s @ ..] => super::t5::n4619(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t5::n1378(s, out),
		['ф', s @ ..] => super::t5::n1379(s, out),
		['ю', s @ ..] => super::t5::n1380(s, out),
		_ => false,
	}
}

/// Words ending -суешь.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1344);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4463);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4464);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4465);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10514);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4467);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 4469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10517);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10518);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4470);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 10520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4471);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10521);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4472);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 10522);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24295);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24296);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 24297);
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
								['п', s @ ..] => {
									super::push(out, 17754);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 17755);
									match s {
										[] => true,
										['н', 'и', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29099);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 29100);
													match s {
														[] => true,
														['з', s @ ..] => super::t5::n33814(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1349);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4474);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10524);
							match s {
								[] => true,
								['в', 'и', 'ж', '\u{1}', s @ ..] => {
									super::push(out, 17756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10525);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10526);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10527);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10528);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10529);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17757);
									s.is_empty()
								}
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
			super::push(out, 1350);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4476);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 10530);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17758);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 10531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 10532);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 17760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 10533);
							match s {
								[] => true,
								['п', 'м', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24298);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24299);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24300);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 10534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 10535);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 10536);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17762);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24301);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 17763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4477);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10537);
							s.is_empty()
						}
						['к', 'а', s @ ..] => {
							super::push(out, 10538);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 17764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4478);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10539);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17765);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t5::n1388(s, out),
		['п', s @ ..] => {
			super::push(out, 1352);
			match s {
				[] => true,
				['и', 'г', s @ ..] => super::t5::n4646(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t5::n1390(s, out),
		['с', s @ ..] => super::t5::n1391(s, out),
		['у', s @ ..] => super::t5::n1392(s, out),
		['ю', s @ ..] => super::t5::n1393(s, out),
		_ => false,
	}
}

/// Words ending -туешь.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4486);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17777);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10547);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10548);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4488);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10550);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10552);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10553);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10554);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 10555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1358);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10556);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 17780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10557);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24311);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24312);
											s.is_empty()
										}
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
					super::push(out, 4491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10558);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10559);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4492);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10560);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24313);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24314);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 24315);
											s.is_empty()
										}
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
					super::push(out, 4493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10561);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10562);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17782);
									s.is_empty()
								}
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
			super::push(out, 1359);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4494);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10563);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 17783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4495);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10564);
							match s {
								[] => true,
								['р', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24316);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 24317);
											s.is_empty()
										}
										['к', 'а', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29103);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 29104);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 24318);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10565);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'а', s @ ..] => {
					super::push(out, 4496);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 4497);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 4498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4499);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17785);
									s.is_empty()
								}
								['н', 'о', 'к', 'а', s @ ..] => {
									super::push(out, 17786);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4500);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10567);
							match s {
								[] => true,
								['п', 'м', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24319);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 24320);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 10568);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17788);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 10569);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t5::n4671(s, out),
				['н', s @ ..] => {
					super::push(out, 4502);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 10572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t5::n1398(s, out),
		['н', s @ ..] => super::t5::n1399(s, out),
		['о', s @ ..] => super::t5::n1400(s, out),
		['р', s @ ..] => super::t5::n1401(s, out),
		['с', s @ ..] => super::t5::n1402(s, out),
		['т', s @ ..] => super::t5::n1403(s, out),
		['у', s @ ..] => super::t5::n1404(s, out),
		['ф', s @ ..] => super::t5::n1405(s, out),
		['х', s @ ..] => super::t5::n1406(s, out),
		['ч', s @ ..] => super::t5::n1407(s, out),
		['ы', s @ ..] => super::t5::n1408(s, out),
		['ь', s @ ..] => super::t5::n1409(s, out),
		['э', s @ ..] => super::t5::n1410(s, out),
		['ю', s @ ..] => super::t5::n1411(s, out),
		['я', s @ ..] => super::t5::n1412(s, out),
		_ => false,
	}
}

/// Words ending -фуешь.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1376);
			match s {
				[] => true,
				['р', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10619);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'л', 'б', '\u{1}', s @ ..] => {
			super::push(out, 1377);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10621);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10622);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10623);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17848);
									s.is_empty()
								}
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
			super::push(out, 1379);
			match s {
				[] => true,
				['а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 4533);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 10624);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 10625);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 17849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10626);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24343);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24344);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1380);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 4534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1381);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4535);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 10627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4536);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 10628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хуешь.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1382);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4537);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17850);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17851);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 17852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1383);
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10630);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'т', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10632);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4539);
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 10634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4541);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цуешь.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['б', 'у', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4544);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1386);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4546);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1387);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10636);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 10637);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4547);
					s.is_empty()
				}
				['р', 'п', 'ш', 'о', s @ ..] => {
					super::push(out, 4548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1388);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1389);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4549);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 10639);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 10640);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 17855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17856);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17857);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17858);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 17859);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 17860);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24345);
											s.is_empty()
										}
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
					super::push(out, 4550);
					match s {
						[] => true,
						['в', 'с', 'о', s @ ..] => {
							super::push(out, 10641);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'л', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10643);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4551);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 10645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', 'л', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10646);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10647);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1390);
			match s {
				[] => true,
				['у', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10649);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1391);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4552);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 10651);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24346);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 10652);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 10653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4553);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 10654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 10655);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17862);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17863);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 17864);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 10656);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 10657);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17866);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4555);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 10658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10659);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10660);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуешь.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1393);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1394);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 4556);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4557);
					match s {
						[] => true,
						['в', 'у', s @ ..] => {
							super::push(out, 10662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1395);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10663);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10664);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1397);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4560);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4561);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 4562);
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 4563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4564);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4565);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17868);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 24348);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 17869);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24349);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1400);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4566);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4567);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17870);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17871);
									s.is_empty()
								}
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
			super::push(out, 1402);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуешь.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['р', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10669);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4571);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4572);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 10672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 4573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10673);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10674);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10676);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10677);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10678);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10679);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17872);
									s.is_empty()
								}
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
			super::push(out, 1410);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щуешь.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остечешь.
pub(crate) fn n11299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10701);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17884);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытечешь.
pub(crate) fn n4789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4602);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10702);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ичешь.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10703);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10704);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10705);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кчешь.
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['л', 'к', 'а', s @ ..] => {
					super::push(out, 4603);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лчешь.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1424);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10707);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очешь.
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['л', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10709);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['е', 'р', 'а', s @ ..] => {
					super::push(out, 4604);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4605);
					s.is_empty()
				}
				['о', 'г', 'о', s @ ..] => {
					super::push(out, 4606);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4608);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10712);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 17886);
									s.is_empty()
								}
								['к', 'а', s @ ..] => {
									super::push(out, 17887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17888);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17889);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24351);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 24352);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29111);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 10713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17891);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17892);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4609);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17894);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17896);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17897);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 17898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17899);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17900);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17901);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 17902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 10714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4611);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17903);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17904);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17905);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 17906);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24353);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1430);
			match s {
				[] => true,
				['р', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10715);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10716);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10717);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10718);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17907);
									s.is_empty()
								}
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
			super::push(out, 1431);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10719);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17908);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 17909);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17910);
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24354);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24355);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10720);
							s.is_empty()
						}
						['т', '\u{1}', s @ ..] => {
							super::push(out, 10721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4612);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4613);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4614);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17911);
									s.is_empty()
								}
								['а', s @ ..] => super::t5::n18943(s, out),
								['о', s @ ..] => super::t5::n18944(s, out),
								_ => false,
							}
						}
						['х', s @ ..] => super::t5::n11327(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t5::n4807(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пчешь.
pub(crate) fn n235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10724);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10725);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10726);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10727);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17917);
									s.is_empty()
								}
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
			super::push(out, 1434);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10728);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10729);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10730);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10731);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10732);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хчешь.
pub(crate) fn n236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4617);
					s.is_empty()
				}
				['л', 'к', 'а', s @ ..] => {
					super::push(out, 4618);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычешь.
pub(crate) fn n237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4620);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 4621);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							super::push(out, 10734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4622);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячешь.
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1440);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 4624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -опьешь.
pub(crate) fn n1533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1490);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4693);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4694);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4695);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10787);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10788);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10789);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 17932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4696);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4697);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4698);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4699);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4700);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпьешь.
pub(crate) fn n1535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1492);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4701);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шьешь.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1493);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1494);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1495);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1496);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1498);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4703);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4704);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10794);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10795);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10796);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4706);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4707);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4708);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 10799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4709);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4710);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1499);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4711);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атлюешь.
pub(crate) fn n4947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4745);
	match s {
		[] => true,
		['п', 'ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17942);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флюешь.
pub(crate) fn n1553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1510);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4747);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хлюешь.
pub(crate) fn n1554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1511);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4748);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4749);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17944);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 17945);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нюешь.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4750);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4751);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1514);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10818);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10819);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4754);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оюешь.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4755);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4757);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рюешь.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10821);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 17947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10822);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10823);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10824);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10825);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10826);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тюешь.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['о', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10828);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ляешь.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1525);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4771);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10832);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10833);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 10834);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 10835);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10836);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 10837);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10838);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17951);
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
						['т', s @ ..] => {
							super::push(out, 10839);
							s.is_empty()
						}
						['у', 'о', s @ ..] => {
							super::push(out, 10840);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4773);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4774);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1526);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4776);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4777);
					match s {
						[] => true,
						['р', 'т', 'о', 'п', 'у', s @ ..] => {
							super::push(out, 10841);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17953);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 24359);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'с', s @ ..] => {
					super::push(out, 4778);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 10842);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 17955);
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									super::push(out, 17956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', 'у', s @ ..] => {
							super::push(out, 10843);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17957);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24360);
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
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10845);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24361);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 24362);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29112);
													match s {
														[] => true,
														['п', s @ ..] => super::t5::n33815(s, out),
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
									super::push(out, 17958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17959);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24363);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 24364);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 17960);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24365);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29113);
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 24366);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 29114);
													s.is_empty()
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 24367);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 24368);
											s.is_empty()
										}
										['с', 'о', s @ ..] => {
											super::push(out, 24369);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29115);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4780);
					match s {
						[] => true,
						['ш', 'е', 'д', 'е', s @ ..] => {
							super::push(out, 10846);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17961);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24370);
											s.is_empty()
										}
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
						['ж', s @ ..] => {
							super::push(out, 10847);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10848);
							match s {
								[] => true,
								['к', 'о', s @ ..] => {
									super::push(out, 17962);
									match s {
										[] => true,
										['п', s @ ..] => {
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
				['о', s @ ..] => super::t5::n4991(s, out),
				['т', 'с', s @ ..] => super::t5::n4992(s, out),
				['я', s @ ..] => {
					super::push(out, 4782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1528);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1529);
			s.is_empty()
		}
		['е', s @ ..] => super::t5::n1573(s, out),
		['и', s @ ..] => super::t5::n1574(s, out),
		['к', s @ ..] => super::t5::n1575(s, out),
		['м', s @ ..] => super::t5::n1576(s, out),
		['о', s @ ..] => super::t5::n1577(s, out),
		['п', s @ ..] => super::t5::n1578(s, out),
		['р', s @ ..] => super::t5::n1579(s, out),
		['с', s @ ..] => super::t5::n1580(s, out),
		['т', s @ ..] => super::t5::n1581(s, out),
		['у', s @ ..] => super::t5::n1582(s, out),
		['ф', s @ ..] => super::t5::n1583(s, out),
		['х', s @ ..] => super::t5::n1584(s, out),
		['ш', s @ ..] => super::t5::n1585(s, out),
		['ы', s @ ..] => super::t5::n1586(s, out),
		_ => false,
	}
}

/// Words ending -вняешь.
pub(crate) fn n1588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1545);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10898);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 10899);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10900);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 10901);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10902);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4820);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 10904);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18006);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18007);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18008);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18009);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18010);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18012);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18013);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18014);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18015);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18016);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24400);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еняешь.
pub(crate) fn n1590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1547);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4822);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 4823);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18018);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 18019);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 18020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 10908);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10909);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10910);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10911);
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18022);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 18023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10912);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10913);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10914);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10915);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10916);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4824);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10917);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10918);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10919);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4825);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4826);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4827);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 4828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жняешь.
pub(crate) fn n1591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1548);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 4829);
			match s {
				[] => true,
				['п', 'у', 'о', s @ ..] => {
					super::push(out, 10920);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иняешь.
pub(crate) fn n1593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1550);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4831);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4832);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10921);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10922);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10923);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18027);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10924);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 10925);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10926);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 10927);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 10928);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10929);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4834);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 4835);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10930);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10931);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 24403);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29123);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 24404);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29124);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 29125);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 29126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мняешь.
pub(crate) fn n1595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1552);
	s.is_empty()
}

/// Words ending -оняешь.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1553);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10932);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10933);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 10934);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4837);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18030);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 18031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'р', 'о', s @ ..] => {
					super::push(out, 10936);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', s @ ..] => {
					super::push(out, 10937);
					s.is_empty()
				}
				['ы', 'в', 'о', s @ ..] => {
					super::push(out, 10938);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4838);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					super::push(out, 10939);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18034);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24405);
									s.is_empty()
								}
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
			super::push(out, 4839);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					super::push(out, 10940);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 10941);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18036);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 18037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рняешь.
pub(crate) fn n1597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1554);
	s.is_empty()
}

/// Words ending -сняешь.
pub(crate) fn n1598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1555);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4841);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					super::push(out, 10942);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18038);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24406);
									s.is_empty()
								}
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
			super::push(out, 4842);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 10943);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тняешь.
pub(crate) fn n1599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1556);
	s.is_empty()
}

/// Words ending -уняешь.
pub(crate) fn n1600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1557);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 4844);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 4845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чняешь.
pub(crate) fn n1601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1558);
	s.is_empty()
}

/// Words ending -ыняешь.
pub(crate) fn n1602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1559);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4846);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4847);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18040);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18041);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оувольняешь.
pub(crate) fn n1603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1560);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 4848);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ояешь.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	s.is_empty()
}

/// Words ending -ряешь.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4849);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 10945);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18043);
									s.is_empty()
								}
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
			super::push(out, 1562);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1563);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4850);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4851);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10946);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10947);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 10948);
							s.is_empty()
						}
						['е', 'р', s @ ..] => {
							super::push(out, 10949);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 18044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 10950);
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 10951);
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							super::push(out, 10952);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10953);
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							super::push(out, 10954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4852);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4853);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10955);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10956);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10957);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10958);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10959);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18045);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 18046);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 24407);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 10960);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 10961);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 4854);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10962);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18048);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1565);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1566);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1567);
			match s {
				[] => true,
				['с', 'о', 'в', s @ ..] => {
					super::push(out, 4857);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							super::push(out, 10965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1568);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4858);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 1570);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4860);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10967);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 18049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10968);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24408);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 24409);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24410);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 24411);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29127);
													s.is_empty()
												}
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
							super::push(out, 10969);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18050);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18051);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18052);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18053);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 4861);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10970);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10971);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10972);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18054);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18055);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24413);
											s.is_empty()
										}
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
					super::push(out, 4863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10973);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сяешь.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1573);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1574);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10975);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уяешь.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъяешь.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 1577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -здолбишь.
pub(crate) fn n11616(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18069);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 18070);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сдолбишь.
pub(crate) fn n11618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10998);
	s.is_empty()
}

/// Words ending -ыдолбишь.
pub(crate) fn n11619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10999);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -толбишь.
pub(crate) fn n5105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4884);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 11000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18072);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбишь.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1592);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11001);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11002);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11003);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11004);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обишь.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4885);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11005);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24416);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24417);
											s.is_empty()
										}
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
					super::push(out, 4886);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11006);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18074);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4887);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4888);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11007);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 18076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4889);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 11008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1595);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4891);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 11009);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18077);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18078);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24418);
											s.is_empty()
										}
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
					super::push(out, 4892);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 11010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4893);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11011);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1596);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11013);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11015);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11016);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11017);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11018);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18079);
									s.is_empty()
								}
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
			super::push(out, 1597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4896);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4897);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11019);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11021);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11022);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 11023);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11024);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11025);
							match s {
								[] => true,
								['п', 'у', s @ ..] => {
									super::push(out, 18080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4900);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4901);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 11026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18081);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4902);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4903);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 11027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18083);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4904);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4905);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4906);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4907);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11030);
							s.is_empty()
						}
						['п', 'с', s @ ..] => {
							super::push(out, 11031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбишь.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1599);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4908);
					match s {
						[] => true,
						['с', 'а', s @ ..] => {
							super::push(out, 11032);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 4909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11033);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11034);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11035);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11036);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18086);
									s.is_empty()
								}
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
			super::push(out, 1600);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11037);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11038);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 11039);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18088);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11040);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4911);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11041);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убишь.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1601);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4913);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4914);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4915);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4917);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11043);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 18091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11044);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 11045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4919);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4920);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11046);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4921);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11047);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18093);
									s.is_empty()
								}
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
			super::push(out, 1602);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4922);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4923);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4924);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4925);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4926);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4927);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 11051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1604);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4929);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11052);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18094);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18095);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4930);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11053);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11054);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4932);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18097);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18098);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24419);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 18099);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24420);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18100);
									s.is_empty()
								}
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
			super::push(out, 1606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4933);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4934);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4935);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4936);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11057);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11058);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4938);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4939);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4940);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4942);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4943);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11060);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11061);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11062);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18101);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 18102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4945);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4946);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбишь.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4947);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юбишь.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4950);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4951);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11065);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4952);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябишь.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11066);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4954);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4955);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4956);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кавишь.
pub(crate) fn n1661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1618);
	match s {
		[] => true,
		['е', 'р', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11080);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4973);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18109);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18110);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24421);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лавишь.
pub(crate) fn n1662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1619);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4974);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4975);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11082);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11083);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 11084);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11085);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18113);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18114);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11086);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11087);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11088);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11089);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11090);
					match s {
						[] => true,
						['е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 18116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -равишь.
pub(crate) fn n1663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1620);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 4977);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4978);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4979);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11093);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11094);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11095);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11096);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11097);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11098);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11099);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11100);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18118);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 24424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11101);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18119);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24425);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29128);
											s.is_empty()
										}
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
					super::push(out, 11102);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11103);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18120);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 24426);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 29129);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4981);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18121);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 18122);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18123);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18124);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18125);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18126);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24429);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18127);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 24430);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18128);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 24431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -савишь.
pub(crate) fn n1664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1621);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 4982);
			match s {
				[] => true,
				['н', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18129);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18130);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18131);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 24433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тавишь.
pub(crate) fn n1665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1622);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4983);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18132);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18133);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18134);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 24435);
									s.is_empty()
								}
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
			super::push(out, 4984);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11106);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11107);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 11108);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11109);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18135);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
									super::push(out, 24436);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 18136);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 24437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11110);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 24438);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 24439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -курчавишь.
pub(crate) fn n5211(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11111);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шавишь.
pub(crate) fn n1667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1624);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4985);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евишь.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 1625);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11114);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1626);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 1627);
			match s {
				[] => true,
				['е', 'д', '\u{1}', s @ ..] => {
					super::push(out, 4987);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -звишь.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1628);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11116);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11117);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11118);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18138);
									s.is_empty()
								}
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
			super::push(out, 1629);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4989);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4990);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 4991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивишь.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4993);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4995);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4996);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11119);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11120);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1633);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4999);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5000);
					match s {
						[] => true,
						['с', 'а', 'ч', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18140);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18141);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24440);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1634);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5001);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11122);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11123);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11124);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11125);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18142);
									s.is_empty()
								}
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
			super::push(out, 1635);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5002);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18143);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18144);
									s.is_empty()
								}
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
			super::push(out, 1636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5003);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11126);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 11127);
							match s {
								[] => true,
								['р', 'к', 'о', s @ ..] => {
									super::push(out, 18145);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24441);
											s.is_empty()
										}
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
					super::push(out, 5004);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 1637);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5005);
					s.is_empty()
				}
				['р', 'а', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11129);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 5006);
					match s {
						[] => true,
						['л', 'а', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18146);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18147);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лвишь.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11131);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11132);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11133);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овишь.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5008);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5009);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5010);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5012);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5013);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 11138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5014);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5015);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5016);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11141);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11142);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18150);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 18151);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24442);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 29130);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 24443);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 18152);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 18153);
									match s {
										[] => true,
										['з', 'о', s @ ..] => {
											super::push(out, 24444);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 29131);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 18154);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 24445);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 24446);
											match s {
												[] => true,
												['е', 'в', 'к', 'с', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n33819(s, out),
														['а', s @ ..] => super::t5::n33820(s, out),
														['о', s @ ..] => super::t5::n33821(s, out),
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
									super::push(out, 18155);
									match s {
										[] => true,
										['с', 'у', 'п', 'а', s @ ..] => {
											super::push(out, 24447);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29132);
													s.is_empty()
												}
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
							super::push(out, 11143);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 18156);
									match s {
										[] => true,
										['о', 'д', s @ ..] => {
											super::push(out, 24448);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 29133);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5017);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5018);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5019);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5020);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5021);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 11146);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 18157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5022);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5023);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5024);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 11149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 5025);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5026);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5027);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11152);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5028);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11153);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 11154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18159);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 18160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t5::n5260(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t6::n1686(s, out),
		_ => false,
	}
}

/// Words ending -рвишь.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5034);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твишь.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1645);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11157);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5036);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1647);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11160);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11161);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11162);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18175);
									s.is_empty()
								}
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
			super::push(out, 1648);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5037);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 11163);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 18176);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24450);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 18177);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24451);
											s.is_empty()
										}
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
					super::push(out, 5039);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11164);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 18178);
									match s {
										[] => true,
										['у', 'с', 'о', 'г', s @ ..] => {
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 29134);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 29135);
													s.is_empty()
												}
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
							super::push(out, 11165);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24452);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24453);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увишь.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1649);
			match s {
				[] => true,
				['п', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11166);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явишь.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1650);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1651);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1652);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['а', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11168);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'п', 'е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11170);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11171);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5041);
					s.is_empty()
				}
				['х', 'у', 'р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11173);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1654);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 5042);
					match s {
						[] => true,
						['л', 'с', '\u{1}', s @ ..] => {
							super::push(out, 11175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1655);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1656);
			match s {
				[] => true,
				['д', 'у', 'к', '\u{1}', s @ ..] => {
					super::push(out, 5044);
					s.is_empty()
				}
				['ы', 'д', '\u{1}', s @ ..] => {
					super::push(out, 5045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 1657);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5046);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5047);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -радишь.
pub(crate) fn n1708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1665);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 5078);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 5079);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5080);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 11198);
					match s {
						[] => true,
						['и', 'л', 'а', s @ ..] => {
							super::push(out, 18184);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 24455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -садишь.
pub(crate) fn n1709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5081);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5082);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5083);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5084);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5085);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5086);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5087);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18185);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 24456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11201);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11202);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5088);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5089);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5090);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ачадишь.
pub(crate) fn n5329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5092);
	s.is_empty()
}

/// Words ending -очадишь.
pub(crate) fn n5330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5093);
	s.is_empty()
}

/// Words ending -шадишь.
pub(crate) fn n1711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1668);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5094);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11204);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щадишь.
pub(crate) fn n1712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1669);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5095);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5096);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бдишь.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1670);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1671);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -едишь.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1673);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 5097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1674);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11206);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11207);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11208);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 11209);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11210);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11211);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18187);
									s.is_empty()
								}
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
			super::push(out, 1675);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5099);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1676);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5100);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11213);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11214);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11215);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11217);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11218);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11219);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 11220);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5103);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5104);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18190);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18191);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24457);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 11222);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18192);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 11223);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18194);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18195);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 11224);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18196);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18197);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5105);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5106);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5107);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 11225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5108);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11226);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1677);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5110);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 1678);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5111);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5112);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5113);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5114);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5115);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ждишь.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1679);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 5117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здишь.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1681);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5118);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11230);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5119);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1683);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5121);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5122);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5123);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18198);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18199);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24459);
											s.is_empty()
										}
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
					super::push(out, 5124);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11233);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5125);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 5126);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5127);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5128);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5129);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 11235);
							match s {
								[] => true,
								['м', 'м', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24460);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24461);
											s.is_empty()
										}
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
						['\u{1}', s @ ..] => {
							super::push(out, 11236);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11237);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 11238);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 11239);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11240);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 18202);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 18203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11241);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18204);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 18205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11242);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 11243);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11244);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1685);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5130);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18207);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5131);
					match s {
						[] => true,
						['о', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18209);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18210);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5132);
					match s {
						[] => true,
						['о', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18212);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 18213);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18214);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18215);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24463);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1686);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1687);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5134);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 11245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идишь.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5136);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5137);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11246);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 11247);
							match s {
								[] => true,
								['е', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24464);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 24465);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29136);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5138);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5139);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11249);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 11250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5141);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11251);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 11252);
							match s {
								[] => true,
								['с', 'я', s @ ..] => {
									super::push(out, 18216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 11253);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1690);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5143);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1691);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5145);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5146);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5147);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5148);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5149);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 11255);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5150);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 1692);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 1693);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5151);
					match s {
						[] => true,
						['н', 'е', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18218);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18219);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24466);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йдишь.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5152);
					match s {
						[] => true,
						['г', 'п', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18220);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдишь.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5153);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 11257);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11258);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11259);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1696);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 5156);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндишь.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 5158);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1698);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11264);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11265);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 11266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18223);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18224);
									s.is_empty()
								}
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
			super::push(out, 1699);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5159);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5160);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1700);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5161);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11269);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5162);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 5164);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5165);
					match s {
						[] => true,
						['е', 'а', s @ ..] => {
							super::push(out, 11273);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 18225);
									s.is_empty()
								}
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
			super::push(out, 1702);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5166);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одишь.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1703);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5167);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18226);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18227);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24467);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1704);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5168);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11274);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18228);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24468);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5169);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 11276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5170);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5171);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11277);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5172);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 11279);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11280);
							match s {
								[] => true,
								['е', 'п', 'а', s @ ..] => {
									super::push(out, 18229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11281);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5173);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11282);
							match s {
								[] => true,
								['р', 'о', s @ ..] => {
									super::push(out, 18231);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 11283);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 11284);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 18232);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24470);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 11285);
							match s {
								[] => true,
								['о', 'р', 'п', 'с', s @ ..] => {
									super::push(out, 18233);
									match s {
										[] => true,
										['о', 'в', 'о', s @ ..] => {
											super::push(out, 24471);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 29137);
													s.is_empty()
												}
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
							super::push(out, 11286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5174);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5175);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11288);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 11289);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 18234);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29138);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 29139);
													match s {
														[] => true,
														['п', s @ ..] => super::t6::n33822(s, out),
														_ => false,
													}
												}
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
							super::push(out, 11290);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 18235);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 24472);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11291);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18236);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 24473);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 11292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11293);
							match s {
								[] => true,
								['о', s @ ..] => super::t6::n19281(s, out),
								['п', s @ ..] => super::t6::n19282(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t6::n11931(s, out),
						['х', s @ ..] => super::t6::n11932(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5176);
					match s {
						[] => true,
						['е', s @ ..] => super::t6::n11933(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t6::n5416(s, out),
				['у', s @ ..] => super::t6::n5417(s, out),
				['ы', s @ ..] => super::t6::n5418(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t6::n1748(s, out),
		['к', s @ ..] => super::t6::n1749(s, out),
		['л', s @ ..] => super::t6::n1750(s, out),
		['р', s @ ..] => super::t6::n1751(s, out),
		['х', s @ ..] => super::t6::n1752(s, out),
		_ => false,
	}
}

/// Words ending -рдишь.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5210);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1711);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5211);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 11342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18291);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18292);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18293);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18294);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 18295);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18296);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24502);
											s.is_empty()
										}
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
					super::push(out, 5212);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18297);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 18298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11344);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 11345);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11346);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 11347);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 11348);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 11349);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 18300);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 24503);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 18301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11350);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18302);
									s.is_empty()
								}
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
			super::push(out, 1712);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5215);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 11351);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 18303);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 11352);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18304);
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
						['з', s @ ..] => {
							super::push(out, 11353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

