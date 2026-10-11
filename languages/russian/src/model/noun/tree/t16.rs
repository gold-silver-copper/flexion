use flexion::Path;

/// Words ending -кми.
pub(crate) fn n1955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1949);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9593);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мми.
pub(crate) fn n1956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1950);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9594);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9595);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28552);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9596);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28554);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9597);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9598);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28557);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оми.
pub(crate) fn n1957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1951);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9599);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 9600);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 9601);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28560);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55937);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83943);
									s.is_empty()
								}
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
			super::push(out, 9602);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 9603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28561);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9604);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рми.
pub(crate) fn n1958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1952);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9605);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 28563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9606);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28564);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55939);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сми.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1953);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9607);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28565);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55940);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83945);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106267);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121463);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132119(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9608);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28566);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55941);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уми.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1954);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9609);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28567);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55942);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 83947);
									s.is_empty()
								}
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
			super::push(out, 9610);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28568);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9611);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28569);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 9612);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28570);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хми.
pub(crate) fn n1961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1955);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9613);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28571);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шми.
pub(crate) fn n1962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1956);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9614);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28572);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ями.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1957);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28573);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -на-кубани.
pub(crate) fn n122738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121464);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 130763);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 136202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бастумани.
pub(crate) fn n122739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121465);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аджрапани.
pub(crate) fn n122740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121466);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 130765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рани.
pub(crate) fn n9651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28585);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 55959);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 83957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 55960);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83958);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 106274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 55961);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 83959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 55962);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83960);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 106275);
									s.is_empty()
								}
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
			super::push(out, 28586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 55963);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сани.
pub(crate) fn n9652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9625);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28587);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 55964);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 83962);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тани.
pub(crate) fn n9653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9626);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 28588);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 55965);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 83963);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 106277);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 121467);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 130766);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 28589);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 55966);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83964);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 106278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фани.
pub(crate) fn n9654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9627);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28590);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 55967);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 83965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хани.
pub(crate) fn n9655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9628);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 28591);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 55968);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 83966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28592);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 55969);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вни.
pub(crate) fn n1965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9629);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28593);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9630);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 28594);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55972);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83968);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106279);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9631);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 28595);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55973);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83969);
									s.is_empty()
								}
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
			super::push(out, 9632);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28596);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гни.
pub(crate) fn n1966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1960);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9633);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дни.
pub(crate) fn n1967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1961);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9634);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28597);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9635);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28598);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83970);
									s.is_empty()
								}
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
			super::push(out, 9636);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28599);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28600);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9637);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28601);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ени.
pub(crate) fn n1968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1962);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9638);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28602);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83971);
									s.is_empty()
								}
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
			super::push(out, 9639);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28603);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 55980);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83972);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106280);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121468);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9640);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28604);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 55981);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жни.
pub(crate) fn n1969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1963);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9641);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28605);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зни.
pub(crate) fn n1970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1964);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9642);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28606);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55983);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83974);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106281);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121469);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9643);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 28607);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55984);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83975);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106282);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9644);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9645);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ини.
pub(crate) fn n1971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1965);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9646);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 9647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9648);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28611);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55986);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83976);
									s.is_empty()
								}
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
			super::push(out, 9649);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28612);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28613);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55988);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 83977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28614);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55989);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28615);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55990);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83979);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106283);
											s.is_empty()
										}
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
					super::push(out, 28616);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 55991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28617);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55992);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83980);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106284);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121470);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28618);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55993);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83981);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106285);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121471);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132124(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28619);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 55994);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83982);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106286);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9651);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28620);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83983);
									s.is_empty()
								}
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
			super::push(out, 9652);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28621);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9653);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28622);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 55997);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83984);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 106287);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 55998);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83985);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106288);
											s.is_empty()
										}
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
					super::push(out, 28623);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55999);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83986);
									s.is_empty()
								}
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
			super::push(out, 9654);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28624);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 9655);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28625);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56001);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83987);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106289);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 121473);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кни.
pub(crate) fn n1972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1966);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9656);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лни.
pub(crate) fn n1973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1967);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нни.
pub(crate) fn n1974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1968);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9658);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28627);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56002);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 83988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 28628);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9659);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28629);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56004);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 56005);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83989);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106290);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9660);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28631);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28632);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9661);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 28633);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9662);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28635);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -они.
pub(crate) fn n1975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1969);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9663);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28637);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56007);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83990);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106291);
											s.is_empty()
										}
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
					super::push(out, 28638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9664);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28639);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 9665);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9666);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28641);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56010);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83991);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106292);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9667);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28642);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56011);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83992);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106293);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121474);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132125(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9668);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28643);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83993);
									s.is_empty()
								}
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
			super::push(out, 9669);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9670);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28644);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 56013);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83994);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106294);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 56014);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 83995);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106295);
											s.is_empty()
										}
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
					super::push(out, 28645);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56015);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83996);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106296);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28646);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 9672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рни.
pub(crate) fn n1976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1970);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9673);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9674);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9675);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28648);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сни.
pub(crate) fn n1977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1971);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9676);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28649);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56017);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83997);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106297);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121475);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тни.
pub(crate) fn n1978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1972);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9677);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9678);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28651);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9679);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28652);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56019);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83998);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уни.
pub(crate) fn n1979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1973);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9680);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28653);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9681);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28655);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56022);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 83999);
									s.is_empty()
								}
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
			super::push(out, 9682);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28656);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56023);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28657);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56024);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84001);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106299);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чни.
pub(crate) fn n1980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1974);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 9684);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28658);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56025);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шни.
pub(crate) fn n1981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1975);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9685);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 28659);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьни.
pub(crate) fn n1982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1976);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9686);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28660);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56026);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 84003);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106300);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121476);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юни.
pub(crate) fn n1983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1977);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9687);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яни.
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1978);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9689);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28663);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 56027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -миссисипи.
pub(crate) fn n122751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121477);
	s.is_empty()
}

/// Words ending -эйчтитипи.
pub(crate) fn n122752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121478);
	s.is_empty()
}

/// Words ending -ну-яагупи.
pub(crate) fn n122753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121479);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 130769);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -традивари.
pub(crate) fn n122754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121480);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 130770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еванагари.
pub(crate) fn n122755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121481);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 130771);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бокари.
pub(crate) fn n56624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 84019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скари.
pub(crate) fn n28894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28700);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 56052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 56053);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 84021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мари.
pub(crate) fn n9750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9723);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28701);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 28702);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56054);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 56055);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 84022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 28703);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56056);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 84023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нари.
pub(crate) fn n9751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9724);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28704);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 56057);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 84024);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106309);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 121482);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пари.
pub(crate) fn n9752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28705);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 28706);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56058);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 84025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рари.
pub(crate) fn n9753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9726);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 28707);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 56059);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 84026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сари.
pub(crate) fn n9754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9727);
	s.is_empty()
}

/// Words ending -тари.
pub(crate) fn n9755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9728);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28708);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 56060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 56061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 28709);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 56062);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 84028);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 106310);
									s.is_empty()
								}
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
			super::push(out, 28710);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 56063);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 84029);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 106311);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 121483);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130772);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n137614(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фари.
pub(crate) fn n9756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9729);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28711);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 56064);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 84030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 56065);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 84031);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106312);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 121484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шари.
pub(crate) fn n9757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9730);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 28712);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56066);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бри.
pub(crate) fn n2001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1995);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9731);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9732);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9733);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28715);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56068);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гри.
pub(crate) fn n2002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1996);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9734);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28716);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56069);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84033);
									s.is_empty()
								}
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
			super::push(out, 9735);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 28717);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56070);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84034);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106313);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дри.
pub(crate) fn n2003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1997);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9736);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9737);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ери.
pub(crate) fn n2004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1998);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9738);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56071);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28721);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56072);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84036);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106314);
											s.is_empty()
										}
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
					super::push(out, 28722);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56073);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84037);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106315);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 56074);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 84038);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 106316);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28723);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56075);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84039);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106317);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9740);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28724);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28725);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56076);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84040);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106318);
											s.is_empty()
										}
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
					super::push(out, 28726);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 9741);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9743);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28729);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56078);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84041);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 106319);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 121485);
													match s {
														[] => true,
														['б', s @ ..] => super::t34::n132130(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9744);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9745);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28731);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56079);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84042);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106320);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9746);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28732);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56080);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84043);
									s.is_empty()
								}
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
			super::push(out, 9747);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 9748);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 28734);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зри.
pub(crate) fn n2005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1999);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9749);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28735);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56081);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84044);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 106321);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ири.
pub(crate) fn n2006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2000);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9750);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28736);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28737);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56083);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84045);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 106322);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 121486);
													match s {
														[] => true,
														['г', s @ ..] => super::t34::n132131(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9751);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28738);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56084);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84046);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106323);
											s.is_empty()
										}
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
					super::push(out, 28739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56085);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84047);
									s.is_empty()
								}
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
			super::push(out, 9752);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 28740);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56086);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28741);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56087);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84049);
									s.is_empty()
								}
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
			super::push(out, 9753);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28742);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56088);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84050);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106324);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9754);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 28743);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56089);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84051);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106325);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9755);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28744);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56090);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мри.
pub(crate) fn n2007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9756);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 9757);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нри.
pub(crate) fn n2008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2002);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9758);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9759);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28747);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56091);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84053);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106326);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121487);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28748);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56092);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 84054);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106327);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ори.
pub(crate) fn n2009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2003);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9760);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9761);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28750);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56094);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 84055);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106328);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9762);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 28751);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84056);
									s.is_empty()
								}
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
			super::push(out, 9763);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28752);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56096);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84057);
									s.is_empty()
								}
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
			super::push(out, 9764);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28753);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56097);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84058);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106329);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9765);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28754);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56098);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84059);
									s.is_empty()
								}
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
			super::push(out, 9766);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28755);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9767);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 9768);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28756);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56100);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84060);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28757);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56101);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84061);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106330);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121488);
													match s {
														[] => true,
														['х', s @ ..] => super::t34::n132132(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9769);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9770);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 9771);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28758);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56102);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28759);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56103);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84063);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106331);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 56104);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84064);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106332);
											s.is_empty()
										}
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
					super::push(out, 28760);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56105);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84065);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106333);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121489);
													s.is_empty()
												}
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
							super::push(out, 56106);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84066);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106334);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9772);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28761);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56107);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84067);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106335);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 121490);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -при.
pub(crate) fn n2010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2004);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9773);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28762);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56108);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84068);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106336);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9774);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28763);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28764);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9775);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 28765);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рри.
pub(crate) fn n2011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2005);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9776);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28766);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9777);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 28769);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28770);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56112);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84069);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106337);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 121491);
													match s {
														[] => true,
														['-', s @ ..] => super::t34::n132133(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28771);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28772);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 28773);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56113);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84070);
									s.is_empty()
								}
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
			super::push(out, 9778);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28774);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28775);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56115);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106338);
											s.is_empty()
										}
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
					super::push(out, 28776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -три.
pub(crate) fn n2012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2006);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9779);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28777);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56116);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84072);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106339);
											s.is_empty()
										}
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
					super::push(out, 28778);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56117);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 84073);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106340);
											s.is_empty()
										}
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
					super::push(out, 28779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9780);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28780);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56118);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 84074);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106341);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121492);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132134(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 106342);
											s.is_empty()
										}
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
					super::push(out, 28781);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 56119);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84075);
									s.is_empty()
								}
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
			super::push(out, 9781);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28782);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84076);
									s.is_empty()
								}
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
			super::push(out, 9782);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28783);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56121);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84077);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106343);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121493);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132135(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ури.
pub(crate) fn n2013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2007);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9783);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28784);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28786);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 56123);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84078);
									s.is_empty()
								}
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
			super::push(out, 9784);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28787);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56124);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84079);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106344);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9785);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 28788);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56125);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84080);
									s.is_empty()
								}
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
			super::push(out, 9786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28789);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 28790);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9787);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28791);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 56127);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84081);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106345);
											s.is_empty()
										}
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
					super::push(out, 28792);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56128);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84082);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106346);
											s.is_empty()
										}
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
					super::push(out, 28793);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56129);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84083);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106347);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9788);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28794);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56130);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84084);
									s.is_empty()
								}
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
			super::push(out, 9789);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28795);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56131);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84085);
									s.is_empty()
								}
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
			super::push(out, 9790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28796);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фри.
pub(crate) fn n2014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2008);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9791);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28797);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56133);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 84086);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 106348);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121494);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9792);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28798);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 56134);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84087);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 106349);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9793);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28799);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хри.
pub(crate) fn n2015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2009);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9794);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 28800);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыри.
pub(crate) fn n2016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2010);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 9795);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28801);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эри.
pub(crate) fn n2017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2011);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 9796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юри.
pub(crate) fn n2018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2012);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9797);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 9798);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28802);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56136);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 28803);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56137);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84090);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106350);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9799);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 9800);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28804);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56138);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84091);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106351);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 121495);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9801);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яри.
pub(crate) fn n2019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2013);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 9802);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56139);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ультра-си.
pub(crate) fn n122770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121496);
	s.is_empty()
}

/// Words ending -вакидзаси.
pub(crate) fn n122771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121497);
	s.is_empty()
}

/// Words ending -ет-такси.
pub(crate) fn n107489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106356);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 121498);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 130779);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 136206);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 139386);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 141228);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икси.
pub(crate) fn n9844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9817);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 28822);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -окси.
pub(crate) fn n9845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9818);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 28823);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 56153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лси.
pub(crate) fn n2026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2020);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9819);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мси.
pub(crate) fn n2027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2021);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 9821);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28825);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нси.
pub(crate) fn n2028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2022);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28826);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9823);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9824);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28829);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 56154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оси.
pub(crate) fn n2029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2023);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9825);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9826);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28831);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пси.
pub(crate) fn n2030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2024);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9827);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9828);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рси.
pub(crate) fn n2031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2025);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9829);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 28834);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9830);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28835);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56158);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 28836);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56159);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 84104);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 106357);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 121499);
													match s {
														[] => true,
														['н', s @ ..] => super::t34::n132137(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сси.
pub(crate) fn n2032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2026);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9831);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28838);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28839);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56161);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84105);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106358);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121500);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132138(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28840);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9832);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28841);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 28842);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28843);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56164);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84106);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106359);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9833);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28844);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тси.
pub(crate) fn n2033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2027);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9834);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28845);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56166);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84107);
									s.is_empty()
								}
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
			super::push(out, 9835);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уси.
pub(crate) fn n2034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2028);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 9836);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28847);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56167);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьси.
pub(crate) fn n2035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2029);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9838);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 28848);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 56168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юси.
pub(crate) fn n2036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2030);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яси.
pub(crate) fn n2037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2031);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 9840);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28849);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56169);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инциннати.
pub(crate) fn n122775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121501);
	s.is_empty()
}

/// Words ending -афтерпати.
pub(crate) fn n122776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121502);
	s.is_empty()
}

/// Words ending -гуджарати.
pub(crate) fn n122777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121503);
	s.is_empty()
}

/// Words ending -ити.
pub(crate) fn n2042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2036);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9856);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28871);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 9857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28873);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28874);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56184);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84118);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106366);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121504);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28875);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56185);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 28876);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 56186);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84120);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106367);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9859);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28877);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 28878);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 56188);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84122);
									s.is_empty()
								}
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
			super::push(out, 9860);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28879);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9861);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28880);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56190);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84123);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106368);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121505);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28881);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56191);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84124);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106369);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121506);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28882);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56192);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84125);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106370);
											s.is_empty()
										}
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
					super::push(out, 28883);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 56193);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84126);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106371);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9862);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 28884);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56194);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84127);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106372);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121508);
													match s {
														[] => true,
														['й', s @ ..] => super::t34::n132139(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28885);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56195);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84128);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106373);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121509);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9863);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28886);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28887);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56196);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84129);
									s.is_empty()
								}
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
			super::push(out, 9864);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 28888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56197);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84130);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106374);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -айти.
pub(crate) fn n9892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9865);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28889);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кти.
pub(crate) fn n2044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2038);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9866);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 28890);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28891);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лти.
pub(crate) fn n2045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2039);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 9867);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28892);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мти.
pub(crate) fn n2046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2040);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 9868);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28893);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56200);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нти.
pub(crate) fn n2047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2041);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9869);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 28894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28895);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9870);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9871);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9872);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 28897);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оти.
pub(crate) fn n2048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2042);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 9873);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28898);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9874);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28899);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28900);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рти.
pub(crate) fn n2049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2043);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9876);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28901);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28902);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28903);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9877);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28904);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56207);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84132);
									s.is_empty()
								}
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
			super::push(out, 9878);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28905);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28906);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56208);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84133);
									s.is_empty()
								}
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
			super::push(out, 9879);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28907);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56209);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84134);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сти.
pub(crate) fn n2050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2044);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9880);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28908);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56210);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84135);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106376);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121510);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132140(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28909);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56212);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84136);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106377);
											s.is_empty()
										}
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
					super::push(out, 28910);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56213);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84137);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106378);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9881);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28911);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56214);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84138);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106379);
											s.is_empty()
										}
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
					super::push(out, 28912);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28913);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56216);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 28914);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56217);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84140);
									s.is_empty()
								}
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
			super::push(out, 9882);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28915);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56218);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84141);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106380);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121511);
													match s {
														[] => true,
														['е', s @ ..] => super::t34::n132141(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 106381);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121512);
													match s {
														[] => true,
														['и', s @ ..] => super::t34::n132142(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28917);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56220);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84142);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106382);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121513);
													s.is_empty()
												}
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
							super::push(out, 56221);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84143);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106383);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121514);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28918);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56222);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84144);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106384);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t34::n29113(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t34::n9910(s, out),
		_ => false,
	}
}

/// Words ending -тти.
pub(crate) fn n2051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2045);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9884);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56228);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84150);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106391);
											s.is_empty()
										}
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
					super::push(out, 28922);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56229);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84151);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106392);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9885);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9886);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9887);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28925);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9888);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28926);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9889);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28928);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28929);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28930);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9890);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 28932);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56231);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84152);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106393);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ути.
pub(crate) fn n2052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2046);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9891);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 28933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9892);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28934);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 56232);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84153);
									s.is_empty()
								}
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
			super::push(out, 9893);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9894);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28936);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56233);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84154);
									s.is_empty()
								}
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
			super::push(out, 9895);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28937);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56234);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фти.
pub(crate) fn n2053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2047);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 9896);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28938);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9897);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28939);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хти.
pub(crate) fn n2054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2048);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9898);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28940);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьти.
pub(crate) fn n2055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2049);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9899);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28941);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56236);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84156);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яти.
pub(crate) fn n2056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2050);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9900);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28942);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -велопрофи.
pub(crate) fn n122795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121521);
	s.is_empty()
}

/// Words ending -бахуврихи.
pub(crate) fn n122796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121522);
	s.is_empty()
}

/// Words ending -л-осьмухи.
pub(crate) fn n122797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121523);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130789);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 136213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -папарацци.
pub(crate) fn n122798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121524);
	s.is_empty()
}

/// Words ending -магдагачи.
pub(crate) fn n122799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121525);
	s.is_empty()
}

/// Words ending -климовичи.
pub(crate) fn n122800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121526);
	s.is_empty()
}

/// Words ending -арановичи.
pub(crate) fn n122801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121527);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 130790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вапчичи.
pub(crate) fn n85071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84179);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 106411);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 121528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мчи.
pub(crate) fn n2081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2075);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9939);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28983);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 28984);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчи.
pub(crate) fn n2082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2076);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9940);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28985);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 56272);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очи.
pub(crate) fn n2083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2077);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9941);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28986);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56273);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84181);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106412);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9942);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28987);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9943);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рчи.
pub(crate) fn n2084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2078);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9944);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 28988);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -счи.
pub(crate) fn n2085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2079);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9945);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28989);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56275);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тчи.
pub(crate) fn n2086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2080);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 9946);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28990);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56276);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84183);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106413);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учи.
pub(crate) fn n2087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2081);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9947);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 28991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56277);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84184);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 106414);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121529);
													match s {
														[] => true,
														['а', s @ ..] => super::t34::n132148(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28992);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9948);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28993);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9949);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28994);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9950);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28995);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28996);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эччи.
pub(crate) fn n9978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9951);
	s.is_empty()
}

/// Words ending -ячи.
pub(crate) fn n2089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2083);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 9952);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 28997);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56283);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84185);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106415);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121530);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --товарищи.
pub(crate) fn n122805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121531);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 130792);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 136214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сухоовощи.
pub(crate) fn n122806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121532);
	s.is_empty()
}

/// Words ending -лагай.
pub(crate) fn n29239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29045);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 56314);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сагай.
pub(crate) fn n29240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29046);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 56315);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тагай.
pub(crate) fn n29241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29047);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 56316);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 84204);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -егай.
pub(crate) fn n10025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9998);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 29048);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 56317);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 84205);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лгай.
pub(crate) fn n10026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9999);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 29049);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 56318);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 84206);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огай.
pub(crate) fn n10027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10000);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 29050);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 56319);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угай.
pub(crate) fn n10028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10001);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 29051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 56320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 29052);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56321);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 84208);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 106425);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121533);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 130793);
													match s {
														[] => true,
														['ы', s @ ..] => super::t34::n137626(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фгай.
pub(crate) fn n10029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10002);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29054);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 56322);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дай.
pub(crate) fn n2109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2103);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10003);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 29055);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10004);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 29056);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10005);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29058);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10006);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29059);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10007);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29060);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56328);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84209);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106426);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10008);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56329);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84210);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106427);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121534);
													match s {
														[] => true,
														['д', s @ ..] => super::t34::n132151(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жай.
pub(crate) fn n2110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2104);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 10009);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29063);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зай.
pub(crate) fn n2111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2105);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10010);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10011);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кай.
pub(crate) fn n2112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2106);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10012);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29066);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29068);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10015);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29069);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56334);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84211);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106428);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121535);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132152(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10016);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29070);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лай.
pub(crate) fn n2113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2107);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10017);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29071);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 29072);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56336);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 84212);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106429);
											s.is_empty()
										}
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
					super::push(out, 29073);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56337);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84213);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106430);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10018);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29074);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10019);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29075);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56339);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84214);
									s.is_empty()
								}
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
			super::push(out, 10020);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29076);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56340);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84215);
									s.is_empty()
								}
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
			super::push(out, 10021);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29077);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10022);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29079);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10023);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29080);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29081);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -май.
pub(crate) fn n2114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2108);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10024);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29082);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56345);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29083);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 29084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10025);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29085);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10026);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29086);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56347);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84217);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106431);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10027);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -най.
pub(crate) fn n2115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2109);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10028);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29088);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56349);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84218);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106432);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10029);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29089);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56350);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84219);
									s.is_empty()
								}
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
			super::push(out, 10030);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29090);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10031);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29092);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10032);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29093);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 29094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пай.
pub(crate) fn n2116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2110);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10033);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10034);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29095);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56354);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84220);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106433);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10035);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29096);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10036);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29098);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56357);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 84221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29099);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56358);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84222);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106434);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рай.
pub(crate) fn n2117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10038);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29100);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56359);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 56360);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84223);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 106435);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121536);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132153(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10039);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29101);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29102);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56362);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84225);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106436);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121537);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10040);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29103);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56363);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84226);
									s.is_empty()
								}
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
			super::push(out, 10041);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29104);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 29105);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 56364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10042);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29106);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29107);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56365);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 29108);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 56366);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84228);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106437);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121538);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10043);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29109);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56367);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84229);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 106438);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сай.
pub(crate) fn n2118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2112);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10044);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10045);
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

/// Words ending -тай.
pub(crate) fn n2119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2113);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10046);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 29112);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56368);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84230);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106439);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10047);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29113);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56369);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 29114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29116);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 29117);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56372);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84232);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106440);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121539);
													s.is_empty()
												}
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
							super::push(out, 56373);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84233);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106441);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 121540);
													match s {
														[] => true,
														['а', s @ ..] => super::t34::n132154(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10048);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29118);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 56374);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84234);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106442);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10049);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29119);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56375);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84235);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106443);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121541);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ю', s @ ..] => {
									super::push(out, 84236);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106444);
											s.is_empty()
										}
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
					super::push(out, 29120);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56376);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10050);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29121);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29122);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56378);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 84237);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 106445);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121542);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132155(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29123);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56379);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84238);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106446);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10051);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29124);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29125);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10052);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29126);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10053);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29127);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56383);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84239);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106447);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 121543);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10054);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29128);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56384);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фай.
pub(crate) fn n2120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2114);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10055);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 29129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10056);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29130);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10057);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хай.
pub(crate) fn n2121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10058);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29132);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10059);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10060);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10061);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10062);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29136);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56388);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84241);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106448);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цай.
pub(crate) fn n2122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2116);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10063);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10064);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29137);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84242);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 84243);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106449);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121544);
													match s {
														[] => true,
														['а', s @ ..] => super::t34::n132156(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чай.
pub(crate) fn n2123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2117);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10065);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 10066);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29138);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56389);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84244);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106450);
											s.is_empty()
										}
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
					super::push(out, 29139);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56390);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84245);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106451);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121545);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10067);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56391);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84246);
									s.is_empty()
								}
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
			super::push(out, 10068);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29141);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56392);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29142);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56393);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106452);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10069);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29143);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10070);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 29145);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шай.
pub(crate) fn n2124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2118);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10071);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херви-бей.
pub(crate) fn n122820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121546);
	s.is_empty()
}

/// Words ending -ч-воробей.
pub(crate) fn n122821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121547);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 130800);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 136220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овей.
pub(crate) fn n10115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10088);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 29165);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 29166);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56410);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 84258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 29167);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 56411);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арвей.
pub(crate) fn n29362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29168);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 56412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 84259);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ервей.
pub(crate) fn n29363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29169);
	s.is_empty()
}

/// Words ending -орвей.
pub(crate) fn n29364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29170);
	s.is_empty()
}

/// Words ending -свей.
pub(crate) fn n10117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10090);
	s.is_empty()
}

/// Words ending -твей.
pub(crate) fn n10118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10091);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29171);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -швей.
pub(crate) fn n10119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10092);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 29172);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 56414);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84260);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106457);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гей.
pub(crate) fn n2127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2121);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10093);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10095);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 29174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10096);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29175);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56415);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84261);
									s.is_empty()
								}
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
			super::push(out, 10097);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29176);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10098);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29177);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дей.
pub(crate) fn n2128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2122);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10099);
			s.is_empty()
		}
		['а', 'р', s @ ..] => {
			super::push(out, 10100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29178);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 56418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84262);
									s.is_empty()
								}
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
			super::push(out, 10101);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29179);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56419);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29180);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29181);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56421);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29182);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56422);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 56423);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84265);
									s.is_empty()
								}
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
			super::push(out, 10102);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 10103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жей.
pub(crate) fn n2129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2123);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10104);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29183);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зей.
pub(crate) fn n2130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2124);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10105);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10106);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29185);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56424);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84266);
									s.is_empty()
								}
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
			super::push(out, 10107);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 29186);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10108);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29187);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56426);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84267);
									s.is_empty()
								}
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
			super::push(out, 10109);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29188);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56427);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84268);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106458);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121549);
													match s {
														[] => true,
														['н', s @ ..] => super::t34::n132158(s, out),
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
									super::push(out, 84269);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106459);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 121550);
													match s {
														[] => true,
														['р', s @ ..] => super::t34::n132159(s, out),
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
									super::push(out, 84270);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106460);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121551);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 106461);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121552);
													match s {
														[] => true,
														['п', s @ ..] => super::t34::n132160(s, out),
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
									super::push(out, 84271);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106462);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121553);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 84272);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106463);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121554);
													match s {
														[] => true,
														['у', s @ ..] => super::t34::n132161(s, out),
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
									super::push(out, 84273);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106464);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121555);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132162(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56428);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84274);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 106465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кей.
pub(crate) fn n2131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2125);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10110);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29189);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29190);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56429);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84275);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106466);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['н', s @ ..] => {
					super::push(out, 29191);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56430);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10111);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10112);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29194);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56431);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 84277);
									s.is_empty()
								}
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
			super::push(out, 10113);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 29195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10114);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29196);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56432);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84278);
									s.is_empty()
								}
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
			super::push(out, 10115);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лей.
pub(crate) fn n2132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2126);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10116);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29198);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 29199);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56433);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29200);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56434);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84280);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106467);
											s.is_empty()
										}
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
					super::push(out, 29201);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56435);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84281);
									s.is_empty()
								}
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
			super::push(out, 10117);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29202);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56436);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84282);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106468);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121556);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10118);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10119);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29203);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56437);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29205);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56439);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84284);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106469);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10120);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29206);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 56440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29207);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56441);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84285);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106470);
											s.is_empty()
										}
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
					super::push(out, 29208);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56442);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106471);
											s.is_empty()
										}
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
					super::push(out, 29209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29210);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29211);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56443);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84287);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106472);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121557);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29212);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56444);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84288);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106473);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121558);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10122);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29213);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56445);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84289);
									s.is_empty()
								}
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
			super::push(out, 10124);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29214);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56446);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 84290);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106474);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121559);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132163(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29215);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84291);
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									super::push(out, 84292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29216);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56448);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84293);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106475);
											s.is_empty()
										}
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
					super::push(out, 29217);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56449);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84294);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106476);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10125);
			match s {
				[] => true,
				['-', s @ ..] => super::t34::n29412(s, out),
				['а', s @ ..] => super::t34::n29413(s, out),
				['с', s @ ..] => super::t34::n29414(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t34::n10154(s, out),
		['с', s @ ..] => super::t34::n10155(s, out),
		['у', s @ ..] => super::t34::n10156(s, out),
		['э', s @ ..] => super::t34::n10157(s, out),
		_ => false,
	}
}

/// Words ending -мей.
pub(crate) fn n2133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2127);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10130);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 10131);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29224);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10132);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 10133);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29225);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'б', s @ ..] => {
			super::push(out, 10135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ней.
pub(crate) fn n2134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10136);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29227);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56457);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84299);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106478);
											s.is_empty()
										}
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
					super::push(out, 29228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29229);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56459);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84300);
									s.is_empty()
								}
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
			super::push(out, 10137);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29230);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56460);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84301);
									s.is_empty()
								}
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
			super::push(out, 10138);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29231);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29232);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10139);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10140);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29234);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29235);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10141);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29236);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29237);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29238);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56467);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 56468);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84302);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106479);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121561);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10143);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29239);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10144);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пей.
pub(crate) fn n2135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2129);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10145);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10146);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29241);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56471);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84303);
									s.is_empty()
								}
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
			super::push(out, 10147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29242);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 56472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29243);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10148);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29244);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рей.
pub(crate) fn n2136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2130);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10149);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 10150);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29245);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 29246);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 10151);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56474);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', '\u{1}', s @ ..] => {
					super::push(out, 29249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10152);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29250);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 56475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29251);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10154);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29253);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56477);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84305);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 56478);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84306);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106481);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10155);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29254);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10156);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29255);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 29256);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 56480);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84307);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106482);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121562);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132165(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56481);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 56482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29258);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56483);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29259);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 29260);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56484);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84309);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106483);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10157);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29261);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 29262);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56485);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29263);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 29264);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56486);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 84311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 29265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10158);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56488);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 56489);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84312);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106484);
											s.is_empty()
										}
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
					super::push(out, 29267);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29268);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29269);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56491);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84313);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106485);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121563);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10159);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29271);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10160);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 10161);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29273);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10162);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10163);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29275);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сей.
pub(crate) fn n2137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2131);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10164);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29276);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56495);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84314);
									s.is_empty()
								}
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
			super::push(out, 10165);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29277);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56496);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10166);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10167);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56497);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84316);
									s.is_empty()
								}
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
			super::push(out, 10168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тей.
pub(crate) fn n2138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2132);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10169);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10171);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10172);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29284);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фей.
pub(crate) fn n2139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2133);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10173);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29285);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 29286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10174);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56501);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56502);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84317);
									s.is_empty()
								}
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
			super::push(out, 10175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56503);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 56504);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84318);
									s.is_empty()
								}
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
			super::push(out, 10176);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29289);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10177);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29291);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56507);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29292);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56508);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29293);
					match s {
						[] => true,
						['е', '\u{1}', s @ ..] => {
							super::push(out, 56509);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56510);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29294);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56512);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84322);
									s.is_empty()
								}
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
			super::push(out, 10178);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56513);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 56514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10179);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29296);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10180);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29297);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10181);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29298);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56517);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хей.
pub(crate) fn n2140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2134);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10182);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29299);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29300);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10183);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29301);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10184);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29302);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56520);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84324);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56521);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84325);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106487);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цей.
pub(crate) fn n2141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2135);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10185);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29303);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56522);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84326);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106488);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121564);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132166(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чей.
pub(crate) fn n2142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2136);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10186);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10187);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29305);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10188);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шей.
pub(crate) fn n2143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2137);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 10189);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щей.
pub(crate) fn n2144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2138);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 10190);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нурбий.
pub(crate) fn n57117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56543);
	s.is_empty()
}

/// Words ending -эрбий.
pub(crate) fn n29516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29322);
	s.is_empty()
}

/// Words ending -сбий.
pub(crate) fn n10228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10200);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 29323);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 56544);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 84340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тбий.
pub(crate) fn n10229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10201);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29324);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 56545);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 84341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убий.
pub(crate) fn n10230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10202);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29325);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 56546);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 84342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шбий.
pub(crate) fn n10231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10203);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29326);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 56547);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьбий.
pub(crate) fn n10232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10204);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29327);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вий.
pub(crate) fn n2146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2140);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10205);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29328);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84344);
									s.is_empty()
								}
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
			super::push(out, 10206);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56550);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 56551);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84345);
									s.is_empty()
								}
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
			super::push(out, 10207);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29330);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29331);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56553);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 84346);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106495);
											s.is_empty()
										}
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
					super::push(out, 29332);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10208);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 10209);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гий.
pub(crate) fn n2147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10210);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10211);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10212);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29335);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10213);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29336);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29337);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56559);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 84348);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106496);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121565);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132167(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56560);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84349);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106497);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 84350);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 106498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 56561);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84351);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106499);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10214);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29339);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29340);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56563);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84352);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106500);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 56564);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дий.
pub(crate) fn n2148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10215);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29341);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56565);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84354);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106501);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121566);
													match s {
														[] => true,
														['у', s @ ..] => super::t34::n132168(s, out),
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
									super::push(out, 84355);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106502);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121567);
													match s {
														[] => true,
														['у', s @ ..] => super::t34::n132169(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56566);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56567);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84356);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106503);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 56568);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29343);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56569);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84358);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106504);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 56570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29344);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56571);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 56572);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84361);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106505);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121568);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10216);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10217);
			match s {
				[] => true,
				['м', '\u{1}', s @ ..] => {
					super::push(out, 29347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10218);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29348);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56574);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 56575);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56576);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84362);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106506);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121569);
													match s {
														[] => true,
														['м', s @ ..] => super::t34::n132170(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29349);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56578);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84363);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106507);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121570);
													match s {
														[] => true,
														['б', s @ ..] => super::t34::n132171(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10219);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10220);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29350);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29351);
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56579);
							match s {
								[] => true,
								['п', 'е', s @ ..] => {
									super::push(out, 84364);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 56580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29352);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56581);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84365);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106509);
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t34::n122845(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t34::n29549(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t34::n10249(s, out),
		['э', s @ ..] => super::t34::n10250(s, out),
		_ => false,
	}
}

/// Words ending -жий.
pub(crate) fn n2149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2143);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 10223);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29357);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10224);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29358);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56587);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84368);
									s.is_empty()
								}
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
			super::push(out, 10225);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 29359);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зий.
pub(crate) fn n2150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2144);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10227);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 29362);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56591);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84369);
									s.is_empty()
								}
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
			super::push(out, 10228);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 29363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10229);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10230);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29365);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56592);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84370);
									s.is_empty()
								}
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
			super::push(out, 10231);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кий.
pub(crate) fn n2151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2145);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10232);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29367);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84371);
									s.is_empty()
								}
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
			super::push(out, 10233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29368);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56594);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84372);
									s.is_empty()
								}
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
			super::push(out, 10234);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29369);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56595);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84373);
									s.is_empty()
								}
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
			super::push(out, 10235);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29370);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 29371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56596);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 56597);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29372);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56598);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29373);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56599);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84376);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106511);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121572);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10236);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29374);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56600);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84377);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106512);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121573);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132172(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29375);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56601);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84378);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106513);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10237);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29376);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56602);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84379);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t34::n10266(s, out),
		['ц', s @ ..] => {
			super::push(out, 10239);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29396);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56652);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84488);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106618);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 56653);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 84489);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106619);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121656);
													match s {
														[] => true,
														['-', s @ ..] => super::t34::n132230(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56654);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84490);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106620);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121657);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 84491);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106621);
											s.is_empty()
										}
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
					super::push(out, 29397);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56655);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84492);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 106622);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121658);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 84493);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106623);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121659);
													match s {
														[] => true,
														['м', s @ ..] => super::t34::n132231(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t34::n29594(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t34::n10268(s, out),
		_ => false,
	}
}

/// Words ending -лий.
pub(crate) fn n2152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2146);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10241);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 29402);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56660);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84500);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106633);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121668);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29403);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 56661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29405);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29406);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 29407);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56664);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 56665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 29408);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56666);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84502);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106634);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10242);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10243);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29410);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 29411);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56669);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84503);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106635);
											s.is_empty()
										}
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
					super::push(out, 29412);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56670);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29413);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29414);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56672);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84505);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106636);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121669);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29415);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56673);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84506);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106637);
											s.is_empty()
										}
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
					super::push(out, 29416);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56674);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84507);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106638);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 56675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29417);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 29418);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10244);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29419);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29420);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56679);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84508);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106639);
											s.is_empty()
										}
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
					super::push(out, 29421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10245);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29422);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56680);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84509);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106640);
											s.is_empty()
										}
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
					super::push(out, 29423);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56681);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84510);
									s.is_empty()
								}
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
			super::push(out, 10246);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29424);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56682);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 56683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10247);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29426);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 29427);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56684);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84511);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106641);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t34::n29624(s, out),
				['ф', s @ ..] => super::t34::n29625(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t34::n10276(s, out),
		['ю', s @ ..] => super::t34::n10277(s, out),
		_ => false,
	}
}

/// Words ending -мий.
pub(crate) fn n2153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2147);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10250);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29433);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10252);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29434);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56690);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56691);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29435);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84517);
									s.is_empty()
								}
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
			super::push(out, 10253);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10254);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 29436);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84518);
									s.is_empty()
								}
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
			super::push(out, 10255);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29437);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29438);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 29439);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56696);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84519);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106643);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10256);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29440);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56697);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 56698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10257);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10258);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10259);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29443);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56699);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ний.
pub(crate) fn n2154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2148);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10260);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29444);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56700);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29445);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29446);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56702);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84523);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106644);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10261);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29447);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10262);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29448);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10263);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29449);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56705);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84524);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106645);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121671);
													match s {
														[] => true,
														['н', s @ ..] => super::t34::n132235(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10264);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56706);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 56707);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84525);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106646);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56708);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84526);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106647);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121672);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29451);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29452);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56710);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29453);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56711);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29454);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 29455);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56712);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84529);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106648);
											s.is_empty()
										}
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
					super::push(out, 29456);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56713);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84530);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 29457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56714);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 56715);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84531);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106649);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10265);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29458);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56716);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84532);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106650);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10266);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29459);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56717);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 56718);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29460);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56720);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84533);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106651);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 56721);
							match s {
								[] => true,
								['л', 'а', 'р', s @ ..] => {
									super::push(out, 84534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29461);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10267);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29462);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56723);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 84535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29463);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10268);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29464);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t34::n10297(s, out),
		['о', s @ ..] => super::t34::n10298(s, out),
		['р', s @ ..] => super::t34::n10299(s, out),
		['т', s @ ..] => super::t34::n10300(s, out),
		['у', s @ ..] => super::t34::n10301(s, out),
		['ф', s @ ..] => super::t34::n10302(s, out),
		['ш', s @ ..] => super::t34::n10303(s, out),
		_ => false,
	}
}

/// Words ending -пий.
pub(crate) fn n2155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2149);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10276);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29486);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56746);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84551);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106665);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10277);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29487);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56747);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 29488);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56748);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 29489);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84554);
									s.is_empty()
								}
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
			super::push(out, 10278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29490);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29491);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 29492);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 29493);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56751);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84555);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106666);
											s.is_empty()
										}
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
						['в', s @ ..] => {
							super::push(out, 56752);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84556);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56753);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84557);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106667);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10280);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29494);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84558);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 84559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29495);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 56755);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84560);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106668);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121679);
													match s {
														[] => true,
														['в', s @ ..] => super::t34::n132238(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29496);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56756);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84561);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106669);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10281);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29497);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56757);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84562);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106670);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рий.
pub(crate) fn n2156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2150);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29498);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 29499);
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', 'х', s @ ..] => {
							super::push(out, 56758);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84563);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106671);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', 'и', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84564);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106672);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121680);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 84565);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106673);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121681);
													s.is_empty()
												}
												_ => false,
											}
										}
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
							super::push(out, 56759);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 56760);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 56761);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 56762);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84566);
									match s {
										[] => true,
										['б', s @ ..] => super::t34::n107808(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t34::n29699(s, out),
				['и', s @ ..] => super::t34::n29700(s, out),
				['к', s @ ..] => super::t34::n29701(s, out),
				['л', s @ ..] => {
					super::push(out, 29501);
					s.is_empty()
				}
				['м', s @ ..] => super::t34::n29703(s, out),
				['н', s @ ..] => super::t34::n29704(s, out),
				['о', s @ ..] => super::t34::n29705(s, out),
				['р', s @ ..] => super::t34::n29706(s, out),
				['с', s @ ..] => super::t34::n29707(s, out),
				['т', s @ ..] => super::t34::n29708(s, out),
				['у', s @ ..] => super::t34::n29709(s, out),
				['ф', s @ ..] => super::t34::n29710(s, out),
				['х', s @ ..] => super::t34::n29711(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t34::n10311(s, out),
		['д', s @ ..] => super::t34::n10312(s, out),
		['е', s @ ..] => super::t34::n10313(s, out),
		['и', s @ ..] => super::t34::n10314(s, out),
		['о', s @ ..] => super::t34::n10315(s, out),
		['т', s @ ..] => super::t34::n10316(s, out),
		['у', s @ ..] => super::t34::n10317(s, out),
		['ф', s @ ..] => super::t34::n10318(s, out),
		['ю', s @ ..] => super::t34::n10319(s, out),
		['я', s @ ..] => super::t34::n10320(s, out),
		_ => false,
	}
}

/// Words ending -сий.
pub(crate) fn n2157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2151);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10291);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29544);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56839);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 56840);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84654);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106738);
											s.is_empty()
										}
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
					super::push(out, 29545);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10292);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29546);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10293);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29548);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56844);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84655);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106739);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121717);
													s.is_empty()
												}
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
							super::push(out, 56845);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84656);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106740);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10294);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29549);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56846);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84657);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106741);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10295);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29550);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56847);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84658);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 56848);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84659);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106742);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 56849);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29551);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29552);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10297);
			match s {
				[] => true,
				['л', 'е', 'ц', '\u{1}', s @ ..] => {
					super::push(out, 29553);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тий.
pub(crate) fn n2158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2152);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10298);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29554);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29555);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56852);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29556);
					s.is_empty()
				}
				['р', 'к', s @ ..] => {
					super::push(out, 29557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10299);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29558);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56853);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 84662);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106743);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10300);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29559);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56854);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84663);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106744);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10301);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29560);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29561);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 56855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 29562);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56856);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84664);
									s.is_empty()
								}
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
			super::push(out, 10302);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29563);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56857);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84665);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106745);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 121718);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10303);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29564);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56858);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84666);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106746);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121719);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29565);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56859);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 56860);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84667);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106747);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121720);
													match s {
														[] => true,
														['у', s @ ..] => super::t34::n132262(s, out),
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
									super::push(out, 84668);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106748);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121721);
													match s {
														[] => true,
														['у', s @ ..] => super::t34::n132263(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10304);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29567);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10305);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 56862);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84669);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106749);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10306);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29569);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 56863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10307);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29570);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фий.
pub(crate) fn n2159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2153);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10308);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10309);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29571);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10310);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29572);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10311);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56867);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 56868);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29573);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10312);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29574);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56870);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84671);
									s.is_empty()
								}
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
			super::push(out, 10313);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29575);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хий.
pub(crate) fn n2160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2154);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10314);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29576);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56871);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84672);
									s.is_empty()
								}
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
			super::push(out, 10315);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29577);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 29578);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56872);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84673);
									s.is_empty()
								}
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
			super::push(out, 10316);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29579);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10317);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29580);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56874);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84674);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106750);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121722);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ций.
pub(crate) fn n2161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2155);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10318);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 29581);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56875);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84675);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106751);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121723);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10319);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56876);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 56877);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84676);
									s.is_empty()
								}
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
			super::push(out, 10320);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29583);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 29584);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56878);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84677);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106752);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10321);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56879);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 84678);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56880);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84679);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106753);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121724);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132264(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29585);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56881);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84680);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106754);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 121725);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29586);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56882);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29587);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10322);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 10323);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29588);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

