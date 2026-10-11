use flexion::Path;

/// Words ending -етчик.
pub(crate) fn n3063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2904);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4242);
			match s {
				[] => true,
				['т', 'о', 'о', s @ ..] => {
					super::push(out, 5134);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 5613);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4243);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4244);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4245);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5135);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5615);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 5616);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 5865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5136);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5137);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 5617);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 5866);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 5867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5139);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5140);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5141);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5618);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 5619);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5142);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5143);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5621);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 5622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5144);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4247);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4248);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5147);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4249);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 4250);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 4251);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 4252);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пщик.
pub(crate) fn n1644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1585);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2932);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5168);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 5169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2933);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2934);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рщик.
pub(crate) fn n1645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1586);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2936);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2937);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 4283);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5170);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4284);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4285);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4286);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4287);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5172);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4288);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4289);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5174);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5175);
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5630);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5176);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5177);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 5178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 4290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2938);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2939);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьщик.
pub(crate) fn n1646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1587);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					super::push(out, 4291);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5179);
							match s {
								[] => true,
								['м', 'о', s @ ..] => {
									super::push(out, 5632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4292);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 5180);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', 'д', s @ ..] => {
					super::push(out, 4293);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 5181);
							match s {
								[] => true,
								['р', 'п', 'о', 'к', s @ ..] => {
									super::push(out, 5634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ость.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2089);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4578);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3437);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4580);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4581);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 5692);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 5693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'м', s @ ..] => {
							super::push(out, 5295);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5296);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5297);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 5298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4582);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5299);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 5694);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 5695);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5896);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 6073);
													s.is_empty()
												}
												['м', s @ ..] => {
													super::push(out, 6074);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 6075);
													s.is_empty()
												}
												['ч', s @ ..] => {
													super::push(out, 6076);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 5897);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 5898);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 5899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 5300);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5696);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 5900);
											s.is_empty()
										}
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
					super::push(out, 4583);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4586);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3438);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4588);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4589);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5301);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'е', s @ ..] => {
			super::push(out, 3439);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4591);
					s.is_empty()
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5303);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4592);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 4593);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5305);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 5306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4594);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4595);
					s.is_empty()
				}
				['ж', 'е', s @ ..] => {
					super::push(out, 4596);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4597);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4598);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 4599);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4600);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4601);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5307);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4602);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4603);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4604);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4605);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4606);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4607);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5309);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5310);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 5697);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 5698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5311);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5312);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5313);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 5314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3441);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4609);
					s.is_empty()
				}
				['г', 'е', s @ ..] => {
					super::push(out, 4610);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5315);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 5316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5317);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 5699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 5318);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 5319);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5320);
							s.is_empty()
						}
						['с', s @ ..] => super::t2::n5647(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4611);
					match s {
						[] => true,
						['р', s @ ..] => super::t2::n5648(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t2::n4885(s, out),
				['о', s @ ..] => super::t2::n4886(s, out),
				['х', s @ ..] => super::t2::n4887(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t2::n3642(s, out),
		['н', s @ ..] => super::t2::n3643(s, out),
		['п', s @ ..] => super::t2::n3644(s, out),
		['р', s @ ..] => super::t2::n3645(s, out),
		['с', s @ ..] => super::t2::n3646(s, out),
		['т', s @ ..] => super::t2::n3647(s, out),
		['х', s @ ..] => super::t2::n3648(s, out),
		_ => false,
	}
}

/// Words ending -поблескивание.
pub(crate) fn n6629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6246);
	s.is_empty()
}

/// Words ending -сплескивание.
pub(crate) fn n6630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6247);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6256);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юкивание.
pub(crate) fn n5850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5516);
	s.is_empty()
}

/// Words ending -якивание.
pub(crate) fn n5851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5517);
	s.is_empty()
}

/// Words ending -ливание.
pub(crate) fn n5277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4977);
	s.is_empty()
}

/// Words ending -нивание.
pub(crate) fn n5278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4978);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5518);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5831);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ривание.
pub(crate) fn n5279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4979);
	s.is_empty()
}

/// Words ending -хивание.
pub(crate) fn n5280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4980);
	s.is_empty()
}

/// Words ending -чивание.
pub(crate) fn n5281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4981);
	s.is_empty()
}

/// Words ending -шивание.
pub(crate) fn n5282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4982);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5519);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5832);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щивание.
pub(crate) fn n5283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4983);
	s.is_empty()
}

/// Words ending -ование.
pub(crate) fn n4211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3980);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4984);
			match s {
				[] => true,
				['т', 'с', 'р', s @ ..] => {
					super::push(out, 5520);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5834);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 5835);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4985);
			match s {
				[] => true,
				['а', 'р', 'б', 'о', 'о', s @ ..] => {
					super::push(out, 5521);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5837);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6048);
									match s {
										[] => true,
										['л', 'о', s @ ..] => {
											super::push(out, 6185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 5838);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 6049);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 6050);
									s.is_empty()
								}
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
			super::push(out, 4986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -увание.
pub(crate) fn n4212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3981);
	s.is_empty()
}

/// Words ending -ывание.
pub(crate) fn n4213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3982);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4987);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5522);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5839);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5840);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 6051);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 6052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5523);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4988);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5525);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5526);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4989);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 4990);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5528);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4992);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5530);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 6053);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 6054);
									s.is_empty()
								}
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
			super::push(out, 4993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5531);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5532);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5534);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5535);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5536);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5537);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5538);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4994);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5540);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5541);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5542);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5543);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 5544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4996);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5545);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5546);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5547);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5548);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5841);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5549);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5550);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5843);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5844);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5845);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5846);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5847);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5551);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 5552);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 5553);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кетостроение.
pub(crate) fn n6566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6186);
	s.is_empty()
}

/// Words ending -летостроение.
pub(crate) fn n6567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6187);
	s.is_empty()
}

/// Words ending -селость.
pub(crate) fn n5647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5321);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5700);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5701);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рзлость.
pub(crate) fn n5648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5322);
	s.is_empty()
}

/// Words ending -клость.
pub(crate) fn n4885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4612);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5323);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5324);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олость.
pub(crate) fn n4886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4613);
	s.is_empty()
}

/// Words ending -хлость.
pub(crate) fn n4887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4614);
	s.is_empty()
}

/// Words ending -мость.
pub(crate) fn n3642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3442);
	match s {
		[] => true,
		['е', 'а', s @ ..] => {
			super::push(out, 4615);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5326);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 5901);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 6077);
											match s {
												[] => true,
												['л', 'с', 'о', s @ ..] => {
													super::push(out, 6193);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 5902);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 5702);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5903);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 6078);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 6079);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4616);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ность.
pub(crate) fn n3643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3443);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4617);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5703);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5704);
							match s {
								[] => true,
								['о', 'п', 'с', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 6080);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 6194);
													s.is_empty()
												}
												['ж', s @ ..] => {
													super::push(out, 6195);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 6196);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 6197);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 6081);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 6198);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4618);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5327);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5328);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5705);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 5706);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5904);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5707);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5906);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 5907);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 5908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5329);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5708);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5909);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 5910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5330);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5709);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 5710);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 5911);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5331);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5332);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 5333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4620);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5711);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 5712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5713);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 5714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 5334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4621);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5335);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5336);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5715);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 5716);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 5717);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 5718);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 5719);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5720);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5913);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 5914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5337);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5338);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5340);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5341);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5721);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5722);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 5915);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 5916);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 5917);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 5723);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5724);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 5725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5342);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 5343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4623);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 4624);
			match s {
				[] => true,
				['е', s @ ..] => super::t2::n5673(s, out),
				['и', s @ ..] => {
					super::push(out, 5345);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5346);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5347);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t2::n4898(s, out),
		['п', s @ ..] => {
			super::push(out, 4626);
			s.is_empty()
		}
		['р', s @ ..] => super::t2::n4900(s, out),
		['с', s @ ..] => {
			super::push(out, 4628);
			s.is_empty()
		}
		['т', s @ ..] => super::t2::n4902(s, out),
		['ф', s @ ..] => {
			super::push(out, 4630);
			s.is_empty()
		}
		['ч', s @ ..] => super::t2::n4904(s, out),
		['ш', s @ ..] => {
			super::push(out, 4632);
			s.is_empty()
		}
		['щ', s @ ..] => super::t2::n4906(s, out),
		['ь', s @ ..] => super::t2::n4907(s, out),
		['я', s @ ..] => super::t2::n4908(s, out),
		_ => false,
	}
}

/// Words ending -пость.
pub(crate) fn n3644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3444);
	s.is_empty()
}

/// Words ending -рость.
pub(crate) fn n3645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3445);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5372);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'е', s @ ..] => {
			super::push(out, 4637);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5374);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 5375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4638);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5376);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4639);
			s.is_empty()
		}
		['т', 'с', s @ ..] => {
			super::push(out, 4640);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5378);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сость.
pub(crate) fn n3646(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4642);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тость.
pub(crate) fn n3647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3446);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4644);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5380);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5381);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 5786);
							match s {
								[] => true,
								['е', 'л', s @ ..] => {
									super::push(out, 6029);
									s.is_empty()
								}
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
			super::push(out, 4645);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4646);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							super::push(out, 5787);
							s.is_empty()
						}
						['т', 'ы', s @ ..] => {
							super::push(out, 5788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4647);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 5789);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 5790);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6030);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 6180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 5791);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 5792);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5793);
							s.is_empty()
						}
						['с', 'е', s @ ..] => {
							super::push(out, 5794);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 6032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'е', s @ ..] => {
							super::push(out, 5795);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 6033);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 6034);
									s.is_empty()
								}
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
			super::push(out, 4648);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 5796);
							s.is_empty()
						}
						['к', 'р', s @ ..] => {
							super::push(out, 5797);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5798);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 5799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хость.
pub(crate) fn n3648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3447);
	s.is_empty()
}

/// Words ending -емность.
pub(crate) fn n5673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5344);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5726);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5727);
			s.is_empty()
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5918);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5919);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5728);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5729);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5730);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5731);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 5732);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 5733);
			s.is_empty()
		}
		['ъ', s @ ..] => {
			super::push(out, 5734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нность.
pub(crate) fn n4898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5349);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5735);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5921);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 6082);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 6083);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 6084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5736);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 5923);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5924);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5925);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 5926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5737);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5738);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5739);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5740);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5927);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5928);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 5929);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5741);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5742);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5931);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5743);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5933);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 5934);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 5935);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t2::n5679(s, out),
		['и', s @ ..] => {
			super::push(out, 5351);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5352);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5353);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рность.
pub(crate) fn n4900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4627);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5355);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5356);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5762);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6014);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5763);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5764);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 6016);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5765);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 6018);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6174);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 6175);
									s.is_empty()
								}
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
			super::push(out, 5357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тность.
pub(crate) fn n4902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4629);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5358);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5359);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5766);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5767);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5768);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5769);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6019);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 6020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6021);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6022);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5770);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5771);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5772);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5360);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5361);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5362);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					super::push(out, 5774);
					match s {
						[] => true,
						['а', 'в', 'х', s @ ..] => {
							super::push(out, 6024);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6176);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 6177);
									s.is_empty()
								}
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
			super::push(out, 5363);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5775);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5364);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5776);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5777);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5778);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5365);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чность.
pub(crate) fn n4904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4631);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5367);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5780);
					match s {
						[] => true,
						['о', 'р', s @ ..] => {
							super::push(out, 6026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5368);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5369);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5781);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5370);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5782);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6027);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 6178);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 6179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5783);
					s.is_empty()
				}
				['т', 'е', s @ ..] => {
					super::push(out, 5784);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 6028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щность.
pub(crate) fn n4906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4633);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5371);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5785);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьность.
pub(crate) fn n4907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4634);
	s.is_empty()
}

/// Words ending -яность.
pub(crate) fn n4908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4635);
	s.is_empty()
}

/// Words ending -енность.
pub(crate) fn n5679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5350);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5744);
			s.is_empty()
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 5937);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5745);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5746);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5747);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 5939);
					match s {
						[] => true,
						['о', 'р', 'ы', s @ ..] => {
							super::push(out, 6085);
							s.is_empty()
						}
						['у', 'н', 'ы', s @ ..] => {
							super::push(out, 6086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5940);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5748);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6087);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 6088);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 6089);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 6199);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 6200);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 6201);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 6202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'н', s @ ..] => {
					super::push(out, 5942);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5943);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6090);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 6091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5944);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5945);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 6203);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 6204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 6092);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									super::push(out, 6205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5946);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5947);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6093);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 6094);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 6206);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 6248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 6207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 6095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 6208);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 6209);
									s.is_empty()
								}
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
			super::push(out, 5749);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5750);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6096);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6097);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6098);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6210);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 6249);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 6250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6099);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 6100);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 6251);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 6252);
											s.is_empty()
										}
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
					super::push(out, 5950);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6101);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6102);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 6103);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6104);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 6211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t2::n6475(s, out),
						['т', s @ ..] => {
							super::push(out, 6105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5951);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5952);
					s.is_empty()
				}
				['е', s @ ..] => super::t2::n6309(s, out),
				['и', s @ ..] => super::t2::n6310(s, out),
				['к', s @ ..] => super::t2::n6311(s, out),
				['м', s @ ..] => super::t2::n6312(s, out),
				['о', s @ ..] => super::t2::n6313(s, out),
				['п', s @ ..] => super::t2::n6314(s, out),
				['с', s @ ..] => super::t2::n6315(s, out),
				['т', s @ ..] => super::t2::n6316(s, out),
				['ш', s @ ..] => super::t2::n6317(s, out),
				['ы', s @ ..] => super::t2::n6318(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t2::n6100(s, out),
		['н', s @ ..] => super::t2::n6101(s, out),
		['о', s @ ..] => super::t2::n6102(s, out),
		['п', s @ ..] => super::t2::n6103(s, out),
		['р', s @ ..] => super::t2::n6104(s, out),
		['с', s @ ..] => super::t2::n6105(s, out),
		['т', 'е', s @ ..] => super::t2::n6106(s, out),
		['ц', s @ ..] => super::t2::n6107(s, out),
		['ч', s @ ..] => super::t2::n6108(s, out),
		['ш', s @ ..] => super::t2::n6109(s, out),
		['щ', s @ ..] => super::t2::n6110(s, out),
		_ => false,
	}
}

/// Words ending -овленность.
pub(crate) fn n6475(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 6212);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6213);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 6214);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еленность.
pub(crate) fn n6309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5953);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6216);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 6217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6106);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 6107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иленность.
pub(crate) fn n6310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5954);
	s.is_empty()
}

/// Words ending -кленность.
pub(crate) fn n6311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5955);
	s.is_empty()
}

/// Words ending -мленность.
pub(crate) fn n6312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5956);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6108);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6218);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6219);
					s.is_empty()
				}
				['л', 'д', s @ ..] => {
					super::push(out, 6220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оленность.
pub(crate) fn n6313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5957);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6110);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 6111);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6112);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пленность.
pub(crate) fn n6314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5958);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6114);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6115);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6221);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 6222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6223);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6224);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 6253);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сленность.
pub(crate) fn n6315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5959);
	match s {
		[] => true,
		['и', 'к', s @ ..] => {
			super::push(out, 6116);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тленность.
pub(crate) fn n6316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5960);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			super::push(out, 6118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шленность.
pub(crate) fn n6317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5961);
	s.is_empty()
}

/// Words ending -ыленность.
pub(crate) fn n6318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5962);
	s.is_empty()
}

/// Words ending -менность.
pub(crate) fn n6100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5751);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6119);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5963);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5964);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5965);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5966);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5967);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 5968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ненность.
pub(crate) fn n6101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5752);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6121);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6122);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5969);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5970);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5971);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 6124);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6225);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5972);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5973);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6125);
					s.is_empty()
				}
				['я', 'р', s @ ..] => {
					super::push(out, 6126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5974);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5975);
			s.is_empty()
		}
		['м', 'о', s @ ..] => {
			super::push(out, 5976);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5977);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6127);
					s.is_empty()
				}
				['я', 'ы', s @ ..] => {
					super::push(out, 6128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оенность.
pub(crate) fn n6102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5753);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5979);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5980);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5981);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5982);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пенность.
pub(crate) fn n6103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5754);
	s.is_empty()
}

/// Words ending -ренность.
pub(crate) fn n6104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5755);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6129);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6130);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5983);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5984);
			match s {
				[] => true,
				['е', 'н', 'а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 6132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'у', 'м', 'у', s @ ..] => {
			super::push(out, 5985);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5986);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6133);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6134);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'м', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6136);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5987);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5988);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6138);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 6139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5989);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6140);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6141);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6227);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'н', s @ ..] => {
			super::push(out, 5990);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 5991);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5992);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сенность.
pub(crate) fn n6105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5756);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6142);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етенность.
pub(crate) fn n6106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5757);
	s.is_empty()
}

/// Words ending -ценность.
pub(crate) fn n6107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5758);
	s.is_empty()
}

/// Words ending -ченность.
pub(crate) fn n6108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5759);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 5993);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6144);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5994);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5995);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6146);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6147);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6148);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6229);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6231);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 6232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5996);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6151);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6233);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 6234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5997);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6152);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6153);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6235);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 6236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6155);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6237);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 6238);
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
					super::push(out, 6156);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6239);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 6240);
							s.is_empty()
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
				['з', s @ ..] => {
					super::push(out, 6157);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6158);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6159);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6241);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6242);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 6243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'л', s @ ..] => {
			super::push(out, 5998);
			match s {
				[] => true,
				['к', 'ы', s @ ..] => {
					super::push(out, 6160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шенность.
pub(crate) fn n6109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5760);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6000);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6001);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6161);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6002);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6003);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6244);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6245);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 6255);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6258);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 6259);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['д', s @ ..] => {
					super::push(out, 6164);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6165);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6166);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6167);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6168);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щенность.
pub(crate) fn n6110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5761);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6005);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 6006);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6007);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6008);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6009);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6010);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6170);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6171);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6011);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 6012);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 6013);
			match s {
				[] => true,
				['в', 'с', 'о', s @ ..] => {
					super::push(out, 6173);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

