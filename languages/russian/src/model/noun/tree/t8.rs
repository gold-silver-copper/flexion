use flexion::Path;

/// Words ending -пах.
pub(crate) fn n4110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4099);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17564);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17565);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 41096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41097);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17566);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41098);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69054);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94421);
									s.is_empty()
								}
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
			super::push(out, 17567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41099);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рах.
pub(crate) fn n4111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4100);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 41100);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 69056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 41101);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69057);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 94422);
									s.is_empty()
								}
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
			super::push(out, 17568);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17569);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 41102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17570);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41103);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69059);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94423);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113426);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125665);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17571);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 69060);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 69061);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94424);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 113427);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125666);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69062);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 69063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сах.
pub(crate) fn n4112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4101);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17572);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 17573);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17574);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41106);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 69065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тах.
pub(crate) fn n4113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4102);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17575);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17576);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 41107);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69066);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94425);
									s.is_empty()
								}
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
			super::push(out, 17577);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 17578);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41108);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69067);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хах.
pub(crate) fn n4114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4103);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17579);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цах.
pub(crate) fn n4115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4104);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 17580);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41109);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шах.
pub(crate) fn n4116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4105);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 17581);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41110);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69068);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94427);
									s.is_empty()
								}
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
			super::push(out, 17582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вх.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4106);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гх.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 4107);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дх.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 4108);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17585);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ех.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4109);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17586);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41112);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69069);
									s.is_empty()
								}
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
			super::push(out, 4110);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4111);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17587);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17588);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4112);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17589);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41115);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 69070);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94428);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17590);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17591);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 41116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17592);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41117);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69072);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94429);
											s.is_empty()
										}
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
					super::push(out, 17593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41118);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 69073);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94430);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17594);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17595);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 17596);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41120);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 69075);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 94431);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113428);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17597);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17598);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41122);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 69077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41123);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17599);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41124);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41125);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 69079);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94432);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41126);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41127);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 69081);
									s.is_empty()
								}
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
			super::push(out, 4115);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17601);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41128);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 69082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17602);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 41129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4116);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17603);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41130);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41131);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 69083);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94433);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 113429);
													match s {
														[] => true,
														['п', s @ ..] => super::t26::n126967(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41132);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41133);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 69084);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 94434);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113430);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17605);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 41134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4117);
			match s {
				[] => true,
				['о', s @ ..] => super::t26::n17707(s, out),
				_ => false,
			}
		}
		['у', 'о', 'п', '\u{1}', s @ ..] => {
			super::push(out, 4118);
			s.is_empty()
		}
		['ц', s @ ..] => super::t26::n4130(s, out),
		['ч', s @ ..] => super::t26::n4131(s, out),
		_ => false,
	}
}

/// Words ending -их.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4121);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17614);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41142);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69093);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94442);
											s.is_empty()
										}
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
					super::push(out, 17615);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 17616);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4122);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17617);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41144);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 69094);
									s.is_empty()
								}
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
			super::push(out, 4123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17618);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 41145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4124);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17619);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41146);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 41147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 17620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4125);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17621);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 41148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 17622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4126);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17623);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41149);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69095);
									s.is_empty()
								}
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
			super::push(out, 4127);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4128);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17624);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41150);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 69096);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 94443);
											s.is_empty()
										}
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
					super::push(out, 17625);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17626);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41152);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69097);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94444);
											s.is_empty()
										}
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
					super::push(out, 17627);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41153);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 69098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41154);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 69099);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94445);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113435);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126969(s, out),
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
									super::push(out, 69100);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94446);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 113436);
													match s {
														[] => true,
														['л', s @ ..] => super::t26::n126970(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41155);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 69101);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94447);
											s.is_empty()
										}
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
					super::push(out, 17628);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41156);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 69102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 17629);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 17630);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 41157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4129);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17631);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41158);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69103);
									s.is_empty()
								}
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
			super::push(out, 4130);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17632);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 41159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17633);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41160);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 69105);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 69106);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94448);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t26::n17734(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 4131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йх.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4132);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4133);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17636);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17637);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кх.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4134);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17638);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17639);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41163);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 69111);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94452);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4135);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17640);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 17641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4137);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17643);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лх.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4138);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нх.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4139);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4140);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41166);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41167);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 69112);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94453);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113438);
													match s {
														[] => true,
														['р', s @ ..] => super::t26::n126971(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4141);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17645);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17646);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ох.
pub(crate) fn n460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17647);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 41168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4143);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17648);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41169);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4144);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17650);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41170);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 69114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 17651);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 17652);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41171);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69115);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 94454);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113439);
													s.is_empty()
												}
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
							super::push(out, 41172);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69116);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94455);
											s.is_empty()
										}
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
					super::push(out, 17653);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17654);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 41174);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 69117);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94456);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113440);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17655);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4146);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17657);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41176);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69118);
									s.is_empty()
								}
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
			super::push(out, 4147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17658);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17659);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 41178);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 69120);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94457);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113441);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17660);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 41179);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 41180);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69121);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41181);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69122);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94459);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 113442);
													match s {
														[] => true,
														['п', s @ ..] => super::t26::n126972(s, out),
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
									super::push(out, 69123);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94460);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113443);
													match s {
														[] => true,
														['е', s @ ..] => super::t26::n126973(s, out),
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
									super::push(out, 69124);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94461);
											s.is_empty()
										}
										['в', s @ ..] => {
											super::push(out, 94462);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113444);
													s.is_empty()
												}
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
							super::push(out, 41182);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41183);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 69126);
									s.is_empty()
								}
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
			super::push(out, 4148);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17662);
					s.is_empty()
				}
				['а', s @ ..] => super::t26::n17764(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4149);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t26::n17765(s, out),
				['а', s @ ..] => super::t26::n17766(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t26::n4161(s, out),
		['с', s @ ..] => super::t26::n4162(s, out),
		['ч', s @ ..] => super::t26::n4163(s, out),
		_ => false,
	}
}

/// Words ending -рх.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4153);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17670);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41193);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94467);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113448);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 94468);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41194);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 69136);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94469);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 113449);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126976(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4154);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41195);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 41196);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 69137);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94470);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113450);
													s.is_empty()
												}
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
							super::push(out, 41197);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41198);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 69139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17673);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4155);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41200);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41201);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69140);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94471);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113451);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сх.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4156);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тх.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4157);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ух.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4158);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17677);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 41203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69142);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 69143);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94472);
											s.is_empty()
										}
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
					super::push(out, 17678);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41204);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 69144);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 94473);
											s.is_empty()
										}
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
					super::push(out, 17679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17680);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41206);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69145);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94474);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 113452);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17681);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41207);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69146);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94475);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 113453);
													match s {
														[] => true,
														['и', s @ ..] => super::t26::n126977(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17682);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 17683);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'д', '\u{1}', s @ ..] => {
					super::push(out, 17684);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17685);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41209);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 69147);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94476);
											s.is_empty()
										}
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
					super::push(out, 17686);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17687);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41211);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17688);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41212);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69149);
									s.is_empty()
								}
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
			super::push(out, 4160);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17689);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 41213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4161);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17690);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 41214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17691);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4162);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17692);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4163);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4164);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17693);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 41218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17694);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41219);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 41220);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 69151);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94477);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 41221);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 69152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41222);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17696);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4165);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17697);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17698);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41225);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 69154);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 94478);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 113454);
													match s {
														[] => true,
														['п', s @ ..] => super::t26::n126978(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4166);
			match s {
				[] => true,
				['р', s @ ..] => super::t26::n17800(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t26::n4178(s, out),
		['р', s @ ..] => super::t26::n4179(s, out),
		['с', s @ ..] => super::t26::n4180(s, out),
		['т', s @ ..] => super::t26::n4181(s, out),
		_ => false,
	}
}

/// Words ending -ых.
pub(crate) fn n465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 464);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4171);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4172);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4173);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41245);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17717);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41246);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69167);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41247);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17718);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41248);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41249);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 69170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17719);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41250);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17720);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41251);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4174);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4175);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 17723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4176);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17724);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юх.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 465);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4178);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17725);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69173);
									s.is_empty()
								}
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
			super::push(out, 4179);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17726);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69174);
									s.is_empty()
								}
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
			super::push(out, 4180);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17727);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4181);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17728);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41259);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 69176);
									s.is_empty()
								}
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
			super::push(out, 4182);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17729);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41260);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 69177);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94485);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ях.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 466);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4183);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17730);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41261);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69178);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 94486);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4184);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17731);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4185);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17733);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41263);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ец.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 468);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4194);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17751);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17752);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41278);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17754);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41280);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69189);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94491);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113459);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17755);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41281);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 69190);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94492);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113460);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17756);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17757);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17758);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4195);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17760);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 41285);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 69192);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 41286);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 69193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17761);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41287);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69194);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94493);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 113461);
													match s {
														[] => true,
														['д', s @ ..] => super::t26::n126980(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41288);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69195);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 94494);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113462);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 69196);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94495);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113463);
													match s {
														[] => true,
														['и', s @ ..] => super::t26::n126981(s, out),
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
									super::push(out, 69197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t26::n41704(s, out),
						['т', s @ ..] => {
							super::push(out, 41290);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 69200);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t26::n17863(s, out),
				['о', s @ ..] => super::t26::n17864(s, out),
				['у', s @ ..] => super::t26::n17865(s, out),
				['ы', s @ ..] => super::t26::n17866(s, out),
				['ь', s @ ..] => super::t26::n17867(s, out),
				['ю', s @ ..] => super::t26::n17868(s, out),
				['я', s @ ..] => super::t26::n17869(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t26::n4207(s, out),
		['г', s @ ..] => super::t26::n4208(s, out),
		['д', s @ ..] => super::t26::n4209(s, out),
		['е', s @ ..] => super::t26::n4210(s, out),
		['ж', s @ ..] => super::t26::n4211(s, out),
		['з', s @ ..] => super::t26::n4212(s, out),
		['и', s @ ..] => super::t26::n4213(s, out),
		['к', s @ ..] => super::t26::n4214(s, out),
		['л', s @ ..] => super::t26::n4215(s, out),
		['м', s @ ..] => super::t26::n4216(s, out),
		['н', s @ ..] => super::t26::n4217(s, out),
		['о', s @ ..] => super::t26::n4218(s, out),
		['п', s @ ..] => super::t26::n4219(s, out),
		['р', s @ ..] => super::t26::n4220(s, out),
		['с', s @ ..] => super::t26::n4221(s, out),
		['т', s @ ..] => super::t26::n4222(s, out),
		['у', s @ ..] => super::t26::n4223(s, out),
		['х', s @ ..] => super::t26::n4224(s, out),
		['ч', s @ ..] => super::t26::n4225(s, out),
		['ш', s @ ..] => super::t26::n4226(s, out),
		['щ', s @ ..] => super::t26::n4227(s, out),
		['ы', s @ ..] => super::t26::n4228(s, out),
		['я', s @ ..] => super::t26::n4229(s, out),
		_ => false,
	}
}

/// Words ending -каттовиц.
pub(crate) fn n115310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114098);
	s.is_empty()
}

/// Words ending -лиц.
pub(crate) fn n4232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4221);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17947);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17948);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41827);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70103);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 95402);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 114099);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 126055);
													match s {
														[] => true,
														['н', s @ ..] => super::t26::n134613(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -амиц.
pub(crate) fn n18051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17950);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 41828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пиц.
pub(crate) fn n4234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4223);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 17951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41829);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риц.
pub(crate) fn n4235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4224);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 17952);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 41830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17953);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 41831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 17954);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йц.
pub(crate) fn n471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4225);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17955);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 41832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нц.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4226);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17956);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 41833);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70105);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95403);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114100);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17957);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41834);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17958);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 41835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17959);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41836);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70107);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95404);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 114101);
													s.is_empty()
												}
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
							super::push(out, 41837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 17960);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41838);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70108);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 95405);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 114102);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4227);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17961);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17962);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41839);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70109);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95406);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 114103);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17963);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41840);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 41841);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 70111);
									s.is_empty()
								}
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
			super::push(out, 4228);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17964);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 41842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4229);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17965);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 41843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4230);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17966);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оц.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 472);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 4231);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4232);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рц.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4233);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4234);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17971);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41847);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 70112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17972);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41848);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70113);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95407);
											s.is_empty()
										}
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
					super::push(out, 17973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тц.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4235);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4236);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уц.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4237);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17976);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4238);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17977);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 41851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17978);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьц.
pub(crate) fn n477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4240);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17979);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17980);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41853);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 70114);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эц.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 4241);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17981);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яц.
pub(crate) fn n479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4242);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17982);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4243);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17984);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41854);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 70115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ач.
pub(crate) fn n480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 479);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4244);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17985);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17986);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 17987);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41855);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 70116);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 95409);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 114104);
													match s {
														[] => true,
														['л', s @ ..] => super::t26::n127356(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 95410);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 114105);
													match s {
														[] => true,
														['е', s @ ..] => super::t26::n127357(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41856);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70117);
									s.is_empty()
								}
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
			super::push(out, 4245);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17988);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17989);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 17990);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17991);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 41860);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 70118);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 95411);
											s.is_empty()
										}
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
					super::push(out, 17992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17993);
					s.is_empty()
				}
				['р', 'е', s @ ..] => {
					super::push(out, 17994);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 41862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70119);
									s.is_empty()
								}
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
			super::push(out, 4246);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17995);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41863);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70120);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95412);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114106);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17996);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41864);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 41865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17997);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41866);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 41867);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70121);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95413);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 114107);
													match s {
														[] => true,
														['в', s @ ..] => super::t26::n127358(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'т', s @ ..] => {
							super::push(out, 41868);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17998);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17999);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41870);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70123);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95414);
											s.is_empty()
										}
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
					super::push(out, 18000);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 41871);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70124);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95415);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114108);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t26::n18102(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t26::n4258(s, out),
		['ж', s @ ..] => super::t26::n4259(s, out),
		['з', s @ ..] => super::t26::n4260(s, out),
		['к', s @ ..] => super::t26::n4261(s, out),
		['л', s @ ..] => super::t26::n4262(s, out),
		['м', s @ ..] => super::t26::n4263(s, out),
		['н', s @ ..] => super::t26::n4264(s, out),
		['п', s @ ..] => super::t26::n4265(s, out),
		['р', s @ ..] => super::t26::n4266(s, out),
		['с', s @ ..] => super::t26::n4267(s, out),
		['т', s @ ..] => super::t26::n4268(s, out),
		['ф', s @ ..] => super::t26::n4269(s, out),
		['х', s @ ..] => super::t26::n4270(s, out),
		_ => false,
	}
}

/// Words ending -ич.
pub(crate) fn n482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 481);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4267);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18069);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4268);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18070);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 18071);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41940);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 18072);
					match s {
						[] => true,
						['н', s @ ..] => super::t26::n42362(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t26::n18174(s, out),
				['к', s @ ..] => {
					super::push(out, 18074);
					match s {
						[] => true,
						['с', s @ ..] => super::t26::n42369(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t26::n18176(s, out),
				['о', s @ ..] => super::t26::n18177(s, out),
				['с', s @ ..] => super::t26::n18178(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t26::n4280(s, out),
		['д', s @ ..] => super::t26::n4281(s, out),
		['е', s @ ..] => super::t26::n4282(s, out),
		['ж', s @ ..] => super::t26::n4283(s, out),
		['з', s @ ..] => super::t26::n4284(s, out),
		['к', s @ ..] => super::t26::n4285(s, out),
		['л', s @ ..] => super::t26::n4286(s, out),
		['м', s @ ..] => super::t26::n4287(s, out),
		['н', s @ ..] => super::t26::n4288(s, out),
		['п', s @ ..] => super::t26::n4289(s, out),
		['р', s @ ..] => super::t26::n4290(s, out),
		['с', s @ ..] => super::t26::n4291(s, out),
		['т', s @ ..] => super::t26::n4292(s, out),
		['у', s @ ..] => super::t26::n4293(s, out),
		['х', s @ ..] => super::t26::n4294(s, out),
		['ч', s @ ..] => super::t26::n4295(s, out),
		['ь', s @ ..] => super::t26::n4296(s, out),
		_ => false,
	}
}

/// Words ending -е-стрейч.
pub(crate) fn n115381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114169);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 126102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ак-френч.
pub(crate) fn n115382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114170);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 126103);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 133266);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 137540);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моч.
pub(crate) fn n4302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4291);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18123);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42006);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 70242);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роч.
pub(crate) fn n4303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4292);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 18124);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18125);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точ.
pub(crate) fn n4304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4293);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18126);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42008);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рч.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4294);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18127);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 18128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4295);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18129);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4296);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18130);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4297);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18132);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тч.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4298);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4299);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4300);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4301);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 4302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уч.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4303);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4304);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18134);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42012);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70244);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95504);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 42013);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70245);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 95505);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4305);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18135);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18136);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18137);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18138);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42016);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70246);
									s.is_empty()
								}
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
			super::push(out, 4307);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4308);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18140);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4309);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4310);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 4311);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4312);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18141);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 42018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4313);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4314);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4315);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18144);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 4316);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18145);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42022);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70247);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95506);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыч.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 488);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4317);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18146);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42023);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70248);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95507);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 114171);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4318);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18148);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42025);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70249);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 95508);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4319);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18149);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42026);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70250);
									s.is_empty()
								}
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
			super::push(out, 4320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18150);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 42027);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70251);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95509);
											s.is_empty()
										}
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
					super::push(out, 18151);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42028);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70252);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95510);
											s.is_empty()
										}
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
					super::push(out, 18152);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4321);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18153);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42030);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70253);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95511);
											s.is_empty()
										}
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
					super::push(out, 18154);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42031);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70254);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95512);
											match s {
												[] => true,
												['а', '\u{1}', s @ ..] => {
													super::push(out, 114172);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4322);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18155);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42032);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 42033);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42034);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70256);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 95513);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114173);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18156);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42035);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70257);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42036);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70258);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95515);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114174);
													match s {
														[] => true,
														['м', s @ ..] => super::t26::n127404(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18157);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70259);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 18158);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42038);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70260);
									s.is_empty()
								}
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
			super::push(out, 4323);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18159);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42039);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70261);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95516);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 70262);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95517);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t26::n42461(s, out),
						['н', s @ ..] => {
							super::push(out, 42041);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t26::n71011(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t26::n42463(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t26::n18261(s, out),
				['о', 'г', 'о', 'м', '\u{1}', s @ ..] => super::t26::n18262(s, out),
				['х', s @ ..] => super::t26::n18263(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t26::n4335(s, out),
		_ => false,
	}
}

/// Words ending -ьч.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4325);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42047);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 42048);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юч.
pub(crate) fn n491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4326);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4327);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18167);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70268);
									s.is_empty()
								}
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
			super::push(out, 4328);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 18168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яч.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4329);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4330);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18170);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42050);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70269);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95521);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -подъебаш.
pub(crate) fn n115388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114176);
	s.is_empty()
}

/// Words ending -либаш.
pub(crate) fn n42477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42056);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 70273);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 95524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кбаш.
pub(crate) fn n18275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18174);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42057);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70274);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лбаш.
pub(crate) fn n18276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18175);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 42058);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 70275);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 95525);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обаш.
pub(crate) fn n18277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18176);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 42059);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 70276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбаш.
pub(crate) fn n18278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18177);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42060);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 70277);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95526);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьбаш.
pub(crate) fn n18279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18178);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 42061);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 70278);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ваш.
pub(crate) fn n4343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4332);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18179);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18180);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18181);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 42064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гаш.
pub(crate) fn n4344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4333);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18182);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 18183);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 42066);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 42067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18184);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42068);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 18185);
			match s {
				[] => true,
				['л', 'а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 42069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -даш.
pub(crate) fn n4345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4334);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18186);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18187);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42071);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70284);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95528);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114179);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18188);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42072);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70285);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95529);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114180);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126106);
													s.is_empty()
												}
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
							super::push(out, 70286);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -каш.
pub(crate) fn n4346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4335);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18189);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42073);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70287);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95531);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114181);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18190);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18191);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42075);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 70289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18192);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лаш.
pub(crate) fn n4347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4336);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18193);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42077);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 42078);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 42079);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 42080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18194);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маш.
pub(crate) fn n4348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4337);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18195);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 42082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18196);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42083);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18197);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 42084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18198);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42085);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 18199);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наш.
pub(crate) fn n4349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4338);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18200);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18201);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 42088);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 42090);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70300);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95532);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 114182);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18202);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42091);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70301);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 95533);
									s.is_empty()
								}
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
			super::push(out, 18203);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 42092);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паш.
pub(crate) fn n4350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раш.
pub(crate) fn n4351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18205);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42093);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 42094);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70302);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95534);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114183);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126107);
													match s {
														[] => true,
														['р', s @ ..] => super::t26::n134637(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18206);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42095);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 70303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18207);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18208);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42097);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 42098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таш.
pub(crate) fn n4352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4341);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18209);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18210);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 18211);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18212);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42102);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70305);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95535);
									s.is_empty()
								}
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
			super::push(out, 18213);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18214);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 42104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шаш.
pub(crate) fn n4354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4343);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яаш.
pub(crate) fn n4355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4344);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вш.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4345);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18217);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42105);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еш.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4346);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18218);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42106);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95536);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4347);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 18219);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 42107);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70310);
									s.is_empty()
								}
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
			super::push(out, 4348);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18220);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42108);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 70311);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95537);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114184);
													match s {
														[] => true,
														['б', s @ ..] => super::t26::n127408(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4350);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18221);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42109);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70312);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95538);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 114185);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4351);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18222);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 42111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18223);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18224);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 18225);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42112);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 18226);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 42113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4352);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 4353);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 4354);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18227);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 18228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4355);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18229);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 18230);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18231);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42116);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70318);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95539);
											s.is_empty()
										}
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
					super::push(out, 18232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 4356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иш.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4357);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4358);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18234);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42117);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18235);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18236);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42119);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70320);
									s.is_empty()
								}
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
			super::push(out, 4359);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18237);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4360);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18238);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 42121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4361);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18239);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 18240);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4362);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18241);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 18242);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42125);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70321);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 18243);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 18244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4363);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18245);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 18246);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 18247);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 42127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 18248);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42128);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70323);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 95540);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 114186);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n127409(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4364);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18249);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 42129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4365);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18250);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4366);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18251);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42131);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 70324);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95541);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4367);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18252);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 42132);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 70325);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95542);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 114187);
													match s {
														[] => true,
														['ф', s @ ..] => super::t26::n127410(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18253);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42133);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70326);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95543);
											s.is_empty()
										}
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
					super::push(out, 18254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18255);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42135);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 70327);
									s.is_empty()
								}
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
			super::push(out, 4368);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 4369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18257);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 18258);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42136);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70328);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 95544);
											s.is_empty()
										}
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
					super::push(out, 18259);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42137);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70329);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95545);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114188);
													match s {
														[] => true,
														['б', s @ ..] => super::t26::n127411(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18260);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42138);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 70330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t26::n4382(s, out),
		['ш', s @ ..] => super::t26::n4383(s, out),
		_ => false,
	}
}

/// Words ending -нш.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4373);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18266);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42143);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70334);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 95547);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 114189);
													match s {
														[] => true,
														['т', s @ ..] => super::t26::n127412(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18267);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 18268);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42145);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70335);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95548);
											s.is_empty()
										}
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
					super::push(out, 18269);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18270);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42147);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 70336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95549);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4374);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18271);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 18272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42148);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70337);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95550);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 114190);
													match s {
														[] => true,
														['у', s @ ..] => super::t26::n127413(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4375);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4376);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ош.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4377);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18274);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4378);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18276);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4379);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42152);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 70338);
									s.is_empty()
								}
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
			super::push(out, 4380);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4381);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18279);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42154);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70339);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95551);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 114191);
													match s {
														[] => true,
														['в', s @ ..] => super::t26::n127414(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18280);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70340);
									s.is_empty()
								}
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
			super::push(out, 4382);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18281);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18282);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42157);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18283);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70342);
									s.is_empty()
								}
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
			super::push(out, 4383);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 18284);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42159);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70343);
									s.is_empty()
								}
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
			super::push(out, 4384);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 18285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4385);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18286);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42161);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 70344);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 95552);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114192);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 70345);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 95553);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114193);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18287);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42162);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 18288);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42164);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 70346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95554);
											s.is_empty()
										}
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
					super::push(out, 18289);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42165);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18290);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 42166);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18291);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4386);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18292);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42168);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70349);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95555);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пш.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4387);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18293);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4388);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18294);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18295);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4389);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18296);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42172);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рш.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4390);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18297);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 42173);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70351);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95556);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 114194);
													match s {
														[] => true,
														['о', s @ ..] => super::t26::n127415(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18298);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4391);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18299);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 18300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18301);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4392);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18303);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 18304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4393);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 18307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уш.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4394);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18308);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 18309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4395);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 18310);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42177);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18311);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42178);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70353);
									s.is_empty()
								}
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
			super::push(out, 4396);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4397);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18313);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42180);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70354);
									s.is_empty()
								}
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
			super::push(out, 4398);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4399);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18315);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42181);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42182);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70356);
									s.is_empty()
								}
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
			super::push(out, 4400);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4401);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4402);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18318);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 18319);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42185);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18320);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18321);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42187);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 70358);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95557);
											s.is_empty()
										}
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
					super::push(out, 18322);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4403);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18323);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 42190);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18324);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42191);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыш.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4404);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18325);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42192);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 42193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18326);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42194);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70363);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95558);
											s.is_empty()
										}
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
					super::push(out, 18327);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42195);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70364);
									s.is_empty()
								}
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
			super::push(out, 4405);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18328);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42196);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70365);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95559);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4406);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18329);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42197);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70366);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95560);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 114195);
													s.is_empty()
												}
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
							super::push(out, 42198);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 70367);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 70368);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 95561);
											s.is_empty()
										}
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
					super::push(out, 18330);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42199);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70369);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95562);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 114196);
													s.is_empty()
												}
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
							super::push(out, 42200);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70370);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95563);
											s.is_empty()
										}
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
					super::push(out, 18331);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42201);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70371);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95564);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 114197);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 70372);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 95565);
											s.is_empty()
										}
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
					super::push(out, 18332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42202);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18333);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42203);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 70374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42204);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95566);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 95567);
											s.is_empty()
										}
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
					super::push(out, 18334);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42205);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70375);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 70376);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 95568);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 114198);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18335);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42206);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70377);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 95569);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 114199);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4407);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18336);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42207);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 70378);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 95570);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 114200);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t26::n4419(s, out),
		['м', s @ ..] => super::t26::n4420(s, out),
		['н', s @ ..] => super::t26::n4421(s, out),
		['п', s @ ..] => super::t26::n4422(s, out),
		['р', s @ ..] => super::t26::n4423(s, out),
		['т', s @ ..] => super::t26::n4424(s, out),
		_ => false,
	}
}

/// Words ending -эш.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 4414);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4415);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 18361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4416);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18362);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 18363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 4417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юш.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 503);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4418);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18364);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42247);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4419);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яш.
pub(crate) fn n505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4420);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18366);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42248);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18367);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18368);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42250);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70418);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 18369);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 18370);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42252);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 70419);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95589);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4421);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 18371);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 42253);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 70420);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95590);
											s.is_empty()
										}
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
					super::push(out, 18372);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42254);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 70421);
									s.is_empty()
								}
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
			super::push(out, 4422);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18373);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42255);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70422);
									s.is_empty()
								}
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
			super::push(out, 4423);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18374);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70423);
									s.is_empty()
								}
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
			super::push(out, 4424);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 18375);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42257);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70424);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95591);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пол-избы.
pub(crate) fn n115425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114213);
	s.is_empty()
}

/// Words ending -замглавы.
pub(crate) fn n115426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114214);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 126118);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 133270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -л-москвы.
pub(crate) fn n115427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114215);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 126119);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 133271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онсервы.
pub(crate) fn n96628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95603);
	s.is_empty()
}

/// Words ending -червы.
pub(crate) fn n42706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42285);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 70441);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хвы.
pub(crate) fn n4472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4461);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18419);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чвы.
pub(crate) fn n4473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4462);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18420);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 42287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ды.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 517);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4463);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18421);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18422);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42289);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70443);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95604);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 42290);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70444);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95605);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4464);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18423);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42291);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70445);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95606);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 114216);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18424);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4465);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18425);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42293);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70446);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95607);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4466);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18426);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42294);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70447);
									s.is_empty()
								}
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
			super::push(out, 4467);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18427);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18428);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42296);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42297);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70449);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95608);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 114217);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18429);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42298);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70450);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95609);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 114218);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18430);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18431);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42300);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70452);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70453);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95610);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 70454);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95611);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 114219);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4468);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18432);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42301);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70455);
									s.is_empty()
								}
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
			super::push(out, 4469);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42302);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 42303);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70456);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 95612);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 114220);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42304);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42305);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70458);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95613);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 114221);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t26::n4481(s, out),
		['р', s @ ..] => super::t26::n4482(s, out),
		['у', s @ ..] => {
			super::push(out, 4472);
			match s {
				[] => true,
				['г', s @ ..] => super::t26::n18544(s, out),
				['м', s @ ..] => super::t26::n18545(s, out),
				['с', s @ ..] => super::t26::n18546(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t27::n4484(s, out),
		['я', s @ ..] => super::t27::n4485(s, out),
		_ => false,
	}
}

/// Words ending -зы.
pub(crate) fn n519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18445);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 18446);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 18447);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42321);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70472);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95623);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114227);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4476);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18448);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42322);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70473);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 95624);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114228);
													match s {
														[] => true,
														['б', s @ ..] => super::t27::n127423(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18449);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42323);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 42324);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70475);
									s.is_empty()
								}
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
			super::push(out, 4477);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18451);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70476);
									s.is_empty()
								}
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
			super::push(out, 4479);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18452);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42327);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 70477);
									s.is_empty()
								}
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
			super::push(out, 4480);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18453);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42328);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70478);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95625);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4481);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18454);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18455);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 42330);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70479);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95626);
											s.is_empty()
										}
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
					super::push(out, 18456);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42331);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70480);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95627);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 114229);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 4482);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18458);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 42333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70481);
									s.is_empty()
								}
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
			super::push(out, 4483);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кы.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4484);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лы.
pub(crate) fn n521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4485);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18461);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42336);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70482);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 95628);
											s.is_empty()
										}
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
					super::push(out, 18462);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42337);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70483);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95629);
											s.is_empty()
										}
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
					super::push(out, 18463);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42338);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 70484);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95630);
											s.is_empty()
										}
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
					super::push(out, 18464);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42339);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 18465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18466);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42341);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70486);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95631);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4486);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18467);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70487);
									s.is_empty()
								}
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
			super::push(out, 4487);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4488);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 18469);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 42344);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70488);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95632);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4489);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18470);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4490);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18471);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18472);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 18473);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42346);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 42347);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70490);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95633);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4491);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18474);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 42348);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70491);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95634);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 114230);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18475);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42349);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70492);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 95635);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114231);
													match s {
														[] => true,
														['р', s @ ..] => super::t27::n127424(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4492);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18476);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 42350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4493);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18477);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18478);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42352);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70493);
									s.is_empty()
								}
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
			super::push(out, 4494);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18479);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42353);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 42354);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70495);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42355);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70496);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95636);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 114232);
													match s {
														[] => true,
														['у', s @ ..] => super::t27::n127425(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t27::n18584(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4495);
			match s {
				[] => true,
				['п', s @ ..] => super::t27::n18585(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мы.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4496);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18482);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18483);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42358);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70498);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 95638);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4497);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18484);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42359);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70499);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95639);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 114233);
													match s {
														[] => true,
														['и', s @ ..] => super::t27::n127426(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4498);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18485);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42360);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70500);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 95640);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114234);
													match s {
														[] => true,
														['о', s @ ..] => super::t27::n127427(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4499);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18486);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42361);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 70501);
									s.is_empty()
								}
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
			super::push(out, 4500);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4501);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 4502);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ны.
pub(crate) fn n523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 522);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4503);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18489);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42362);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70502);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 95641);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 114235);
													match s {
														[] => true,
														['о', s @ ..] => super::t27::n127428(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18490);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 18491);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42363);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70503);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 95642);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114236);
													match s {
														[] => true,
														['б', s @ ..] => super::t27::n127429(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42364);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70504);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95643);
											s.is_empty()
										}
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
					super::push(out, 18492);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42365);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70505);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95644);
											s.is_empty()
										}
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
					super::push(out, 18493);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42366);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70506);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 95645);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 114237);
													match s {
														[] => true,
														['п', s @ ..] => super::t27::n127430(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18494);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 42367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 18495);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42368);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70507);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95646);
											s.is_empty()
										}
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
					super::push(out, 18496);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42369);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70508);
									s.is_empty()
								}
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
			super::push(out, 4504);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18497);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4505);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 18498);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42371);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70509);
									s.is_empty()
								}
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
			super::push(out, 4506);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18499);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42372);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 70510);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95647);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 114238);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18500);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18501);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4507);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18502);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4508);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18503);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t27::n4520(s, out),
		['н', s @ ..] => {
			super::push(out, 4510);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18515);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4511);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 18516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42390);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 70521);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95655);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t27::n18621(s, out),
				['н', s @ ..] => {
					super::push(out, 18518);
					s.is_empty()
				}
				['р', s @ ..] => super::t27::n18623(s, out),
				['с', s @ ..] => super::t27::n18624(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n4523(s, out),
		['у', s @ ..] => super::t27::n4524(s, out),
		['х', s @ ..] => super::t27::n4525(s, out),
		['я', s @ ..] => super::t27::n4526(s, out),
		_ => false,
	}
}

/// Words ending -пы.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4516);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4517);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18526);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4518);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18528);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 42401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4519);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18529);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42402);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70531);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 95665);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114256);
													match s {
														[] => true,
														['о', s @ ..] => super::t27::n127440(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4520);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42403);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 70532);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 95666);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 114257);
													match s {
														[] => true,
														['п', s @ ..] => super::t27::n127441(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4521);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18531);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ры.
pub(crate) fn n525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4522);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18532);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42405);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70533);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42406);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 18533);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42407);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70535);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95667);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 114258);
													s.is_empty()
												}
												_ => false,
											}
										}
										['б', s @ ..] => {
											super::push(out, 95668);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114259);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 70536);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95669);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 114260);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ц', s @ ..] => {
											super::push(out, 95670);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 114261);
													s.is_empty()
												}
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
							super::push(out, 42408);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70537);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95671);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 114262);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18534);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42409);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70538);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95672);
											s.is_empty()
										}
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
					super::push(out, 18535);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42410);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70539);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95673);
											s.is_empty()
										}
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
					super::push(out, 18536);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42411);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70540);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 95674);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 114263);
													match s {
														[] => true,
														['к', s @ ..] => super::t27::n127442(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18538);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18539);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42413);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 70541);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95675);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114264);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18540);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42414);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70542);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95676);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114265);
													match s {
														[] => true,
														['т', s @ ..] => super::t27::n127443(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18541);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 42415);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70543);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 95677);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 114266);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18542);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42416);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70544);
									match s {
										[] => true,
										['ш', s @ ..] => super::t27::n96704(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t27::n18647(s, out),
				['ч', s @ ..] => super::t27::n18648(s, out),
				['ш', s @ ..] => super::t27::n18649(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t27::n4534(s, out),
		['в', s @ ..] => super::t27::n4535(s, out),
		['г', s @ ..] => super::t27::n4536(s, out),
		['д', s @ ..] => super::t27::n4537(s, out),
		['е', s @ ..] => super::t27::n4538(s, out),
		['и', s @ ..] => super::t27::n4539(s, out),
		['к', s @ ..] => super::t27::n4540(s, out),
		['м', s @ ..] => super::t27::n4541(s, out),
		['о', s @ ..] => super::t27::n4542(s, out),
		['т', s @ ..] => super::t27::n4543(s, out),
		['у', s @ ..] => super::t27::n4544(s, out),
		['х', s @ ..] => super::t27::n4545(s, out),
		['ю', s @ ..] => super::t27::n4546(s, out),
		_ => false,
	}
}

/// Words ending -сы.
pub(crate) fn n526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 525);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4536);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18577);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42453);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70566);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95694);
											s.is_empty()
										}
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
					super::push(out, 18578);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 42454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 18579);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 18580);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18581);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42457);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 42458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42459);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 70568);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95695);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 42460);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70569);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95696);
											s.is_empty()
										}
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
					super::push(out, 18583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 18584);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 42462);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70570);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95697);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114273);
													match s {
														[] => true,
														['м', s @ ..] => super::t27::n127448(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42463);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70571);
									s.is_empty()
								}
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
			super::push(out, 4537);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18585);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 42464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4538);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18586);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18587);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42466);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70572);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95698);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4539);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18588);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42467);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 70573);
									s.is_empty()
								}
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
			super::push(out, 4540);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18589);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42468);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 18590);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42469);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 42470);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 70575);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95699);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 114274);
													match s {
														[] => true,
														['у', s @ ..] => super::t27::n127449(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18591);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42471);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70576);
									s.is_empty()
								}
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
			super::push(out, 4541);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18592);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18593);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42473);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70577);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 95700);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 42474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18594);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42475);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70578);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 95701);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t27::n18700(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t27::n4554(s, out),
		['р', s @ ..] => super::t27::n4555(s, out),
		['с', s @ ..] => super::t27::n4556(s, out),
		['т', s @ ..] => {
			super::push(out, 4546);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 18605);
					match s {
						[] => true,
						['а', s @ ..] => super::t27::n42914(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t27::n4558(s, out),
		['ь', s @ ..] => super::t27::n4559(s, out),
		['я', s @ ..] => super::t27::n4560(s, out),
		_ => false,
	}
}

/// Words ending -ты.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 526);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4550);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18612);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42499);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18613);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 18614);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42500);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70599);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95709);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 42501);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70600);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 95710);
											s.is_empty()
										}
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
					super::push(out, 18615);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42502);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 70601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42503);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18616);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42504);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70603);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 42505);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18617);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42506);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 42507);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70606);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 95711);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 114277);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18618);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 42508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4551);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18619);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 42509);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 18620);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42510);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18621);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42511);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 70609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42512);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70610);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95712);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114278);
													match s {
														[] => true,
														['о', s @ ..] => super::t27::n127452(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18623);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4552);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18624);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70611);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95713);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114279);
													match s {
														[] => true,
														['ф', s @ ..] => super::t27::n127453(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4553);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18625);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 18626);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42515);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70612);
									s.is_empty()
								}
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
			super::push(out, 4554);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18627);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42516);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 42517);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 70613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18628);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42518);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70614);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 70615);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 95714);
											s.is_empty()
										}
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
					super::push(out, 18629);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42519);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t27::n4566(s, out),
		['р', s @ ..] => super::t27::n4567(s, out),
		['с', s @ ..] => super::t27::n4568(s, out),
		['т', s @ ..] => {
			super::push(out, 4558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18645);
					match s {
						[] => true,
						['в', s @ ..] => super::t27::n42957(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t27::n18751(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t27::n4570(s, out),
		_ => false,
	}
}

/// Words ending -фы.
pub(crate) fn n528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4560);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18650);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4561);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18651);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4562);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4563);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18653);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42540);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70629);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цы.
pub(crate) fn n529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 528);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4564);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18654);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42541);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 70630);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95725);
											s.is_empty()
										}
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
					super::push(out, 18655);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42542);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 42543);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70632);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 95726);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114287);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4565);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18656);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42544);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70633);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 95727);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 114288);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4566);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18657);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 42545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18658);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 42546);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70634);
									s.is_empty()
								}
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
			super::push(out, 4567);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 18659);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42547);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70635);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 95728);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 114289);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4568);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 18660);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42548);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70636);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 95729);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114290);
													match s {
														[] => true,
														['-', s @ ..] => super::t27::n127456(s, out),
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
									super::push(out, 70637);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 95730);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114291);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42549);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70638);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95731);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 114292);
													match s {
														[] => true,
														['л', s @ ..] => super::t27::n127457(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42550);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70639);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95732);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42551);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70640);
									s.is_empty()
								}
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
			super::push(out, 4569);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18662);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4570);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18663);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 42553);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70641);
									s.is_empty()
								}
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
			super::push(out, 4571);
			match s {
				[] => true,
				['а', s @ ..] => super::t27::n18769(s, out),
				['е', s @ ..] => super::t27::n18770(s, out),
				['и', s @ ..] => super::t27::n18771(s, out),
				['о', s @ ..] => super::t27::n18772(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t27::n4583(s, out),
		['р', s @ ..] => super::t27::n4584(s, out),
		['с', s @ ..] => super::t27::n4585(s, out),
		['т', s @ ..] => super::t27::n4586(s, out),
		['ь', s @ ..] => super::t27::n4587(s, out),
		_ => false,
	}
}

/// Words ending -шы.
pub(crate) fn n530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4577);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18675);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42572);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70659);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95743);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 114298);
													match s {
														[] => true,
														['р', s @ ..] => super::t27::n127460(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вь.
pub(crate) fn n532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4586);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 18692);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 18693);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18695);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4587);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 18696);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42589);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70668);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 95747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4588);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 18697);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42590);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70669);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95748);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4589);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18698);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 42591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18699);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4590);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18700);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 42593);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70671);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 95749);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 114299);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18701);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42594);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70672);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 95750);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 114300);
													match s {
														[] => true,
														['ж', s @ ..] => super::t27::n127461(s, out),
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
									super::push(out, 70673);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95751);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 114301);
													match s {
														[] => true,
														['ь', s @ ..] => super::t27::n127462(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18702);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18703);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42595);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 42596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70674);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 70675);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 95752);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 114302);
													s.is_empty()
												}
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
							super::push(out, 42597);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 70676);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95753);
											s.is_empty()
										}
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
					super::push(out, 18704);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 42598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4591);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18705);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42599);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 42600);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70677);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95754);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 114303);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n127463(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42601);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70678);
									s.is_empty()
								}
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
			super::push(out, 4592);
			match s {
				[] => true,
				['е', s @ ..] => super::t27::n18811(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4593);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18707);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t27::n4605(s, out),
		['я', s @ ..] => super::t27::n4606(s, out),
		_ => false,
	}
}

/// Words ending -дь.
pub(crate) fn n533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 532);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4596);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 18710);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 18711);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 18712);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42606);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 42607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18714);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 42609);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70683);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95758);
											s.is_empty()
										}
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
					super::push(out, 18715);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42610);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 18716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42611);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70685);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95759);
											s.is_empty()
										}
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
					super::push(out, 18717);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42612);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70686);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95760);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42613);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70687);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95761);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 114306);
													match s {
														[] => true,
														['и', s @ ..] => super::t27::n127464(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 114307);
													match s {
														[] => true,
														['а', s @ ..] => super::t27::n127465(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4597);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18718);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42614);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70688);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95762);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42615);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 18719);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42616);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70690);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 95763);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42617);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70691);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 70692);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95764);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 114308);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 95765);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 114309);
													match s {
														[] => true,
														['-', s @ ..] => super::t27::n127466(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t27::n18825(s, out),
				['м', s @ ..] => super::t27::n18826(s, out),
				['н', s @ ..] => {
					super::push(out, 18722);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 18723);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 42624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t27::n18829(s, out),
				['с', s @ ..] => super::t27::n18830(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t27::n4609(s, out),
		['з', s @ ..] => super::t27::n4610(s, out),
		['и', s @ ..] => super::t27::n4611(s, out),
		['о', s @ ..] => super::t27::n4612(s, out),
		['р', s @ ..] => super::t27::n4613(s, out),
		['у', s @ ..] => super::t27::n4614(s, out),
		['ы', s @ ..] => super::t27::n4615(s, out),
		['ь', s @ ..] => super::t27::n4616(s, out),
		['ю', s @ ..] => super::t27::n4617(s, out),
		['я', s @ ..] => super::t27::n4618(s, out),
		_ => false,
	}
}

/// Words ending -одежь.
pub(crate) fn n43092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42670);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 70739);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95796);
					match s {
						[] => true,
						['м', 'с', s @ ..] => {
							super::push(out, 114326);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 126175);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133301);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алежь.
pub(crate) fn n43093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42671);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 70740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -режь.
pub(crate) fn n18862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18757);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42672);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 70741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 42673);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 70742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тежь.
pub(crate) fn n18863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18758);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 42674);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 70743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 42675);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 70744);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожь.
pub(crate) fn n4621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4610);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18759);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 18760);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42676);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18761);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42677);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 42678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18762);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 42679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжь.
pub(crate) fn n4622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4611);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18763);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42680);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70747);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжь.
pub(crate) fn n4623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4612);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18764);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18765);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42682);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зь.
pub(crate) fn n535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4613);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42683);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42684);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70749);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95798);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 114327);
													s.is_empty()
												}
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
							super::push(out, 42685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 18767);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42686);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70750);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 70751);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 95799);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 114328);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4614);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18768);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42687);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70752);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95800);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42688);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 70753);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95801);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 114329);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42689);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 70754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 42690);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 42691);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 42692);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42693);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70758);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95802);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4615);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18770);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18771);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42695);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 42696);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70760);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95803);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42697);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 70761);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 95804);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4616);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18772);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42698);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 70762);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 95805);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 114330);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4617);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18773);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42699);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 70763);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95806);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 42700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4618);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42701);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 70764);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 95807);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4619);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 18775);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42702);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 70765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t27::n4631(s, out),
		_ => false,
	}
}

/// Words ending -ль.
pub(crate) fn n536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['а', s @ ..] => super::t27::n4632(s, out),
		['б', s @ ..] => {
			super::push(out, 4622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18807);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 42800);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70899);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 95915);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 114387);
													match s {
														[] => true,
														['д', s @ ..] => super::t27::n127504(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42801);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 70900);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 95916);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 114388);
													match s {
														[] => true,
														['р', s @ ..] => super::t27::n127505(s, out),
														_ => false,
													}
												}
												['к', s @ ..] => {
													super::push(out, 114389);
													match s {
														[] => true,
														['а', s @ ..] => super::t27::n127506(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 114390);
													match s {
														[] => true,
														['а', s @ ..] => super::t27::n127507(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42802);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 70901);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 95917);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 42803);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70902);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95918);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 114391);
													match s {
														[] => true,
														['о', s @ ..] => super::t27::n127508(s, out),
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
									super::push(out, 70903);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95919);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114392);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18808);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42804);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 70904);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 95920);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 114393);
													match s {
														[] => true,
														['ф', s @ ..] => super::t27::n127509(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42805);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70905);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95921);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 114394);
													match s {
														[] => true,
														['к', s @ ..] => super::t27::n127510(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 18809);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42806);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 70906);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 95922);
											match s {
												[] => true,
												['а', '-', 'к', 'и', s @ ..] => super::t27::n115607(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t27::n18915(s, out),
				['у', s @ ..] => super::t27::n18916(s, out),
				['ю', s @ ..] => super::t27::n18917(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t27::n4634(s, out),
		['г', s @ ..] => super::t27::n4635(s, out),
		['д', s @ ..] => super::t27::n4636(s, out),
		['е', s @ ..] => super::t27::n4637(s, out),
		['и', s @ ..] => super::t27::n4638(s, out),
		['й', s @ ..] => super::t27::n4639(s, out),
		['к', s @ ..] => super::t27::n4640(s, out),
		['л', s @ ..] => super::t27::n4641(s, out),
		['м', s @ ..] => super::t27::n4642(s, out),
		['н', s @ ..] => super::t27::n4643(s, out),
		['о', s @ ..] => super::t27::n4644(s, out),
		['п', s @ ..] => super::t27::n4645(s, out),
		['р', s @ ..] => super::t27::n4646(s, out),
		['с', s @ ..] => super::t27::n4647(s, out),
		['т', s @ ..] => super::t27::n4648(s, out),
		['у', s @ ..] => super::t27::n4649(s, out),
		['ы', s @ ..] => super::t27::n4650(s, out),
		['э', s @ ..] => super::t27::n4651(s, out),
		['ю', s @ ..] => super::t27::n4652(s, out),
		['я', s @ ..] => super::t27::n4653(s, out),
		_ => false,
	}
}

