use flexion::Path;

/// Words ending -линец.
pub(crate) fn n41991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41575);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69741);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95085);
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					super::push(out, 95086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 69742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 69743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69744);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 69745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95089);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 95090);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113880);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125941);
									s.is_empty()
								}
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
			super::push(out, 69746);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95091);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 113881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'м', s @ ..] => {
			super::push(out, 69747);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95092);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113882);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125943);
									s.is_empty()
								}
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
			super::push(out, 69748);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69749);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 95094);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 113883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'д', s @ ..] => {
			super::push(out, 69750);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95095);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113884);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -минец.
pub(crate) fn n41992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41576);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69751);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 69752);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 95096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 69753);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 69754);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 69755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нинец.
pub(crate) fn n41993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41577);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 95097);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 95098);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 113885);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125946);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133193);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69756);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 95100);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 113887);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 125947);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133194);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 137504);
													match s {
														[] => true,
														['с', s @ ..] => super::t51::n141546(s, out),
														_ => false,
													}
												}
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
							super::push(out, 113888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 69757);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 69758);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95102);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 113889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 69759);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 69760);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95104);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69761);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113891);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 113892);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 125948);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133195);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69762);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95106);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113893);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125949);
									s.is_empty()
								}
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
			super::push(out, 69763);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95107);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 69764);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95108);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пинец.
pub(crate) fn n41994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41578);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69765);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95109);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 69766);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95110);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113897);
							s.is_empty()
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
				['е', s @ ..] => {
					super::push(out, 95111);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113898);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 95112);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113899);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 125951);
									s.is_empty()
								}
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
			super::push(out, 69767);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринец.
pub(crate) fn n41995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41579);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69768);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 95114);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 95115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 69769);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 69770);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 95117);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125952);
									s.is_empty()
								}
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
			super::push(out, 69771);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 95118);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 113902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 69772);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 69773);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 95120);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69774);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95121);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 69775);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 95122);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 113906);
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
				['т', s @ ..] => {
					super::push(out, 95123);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 95124);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113907);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 125953);
									s.is_empty()
								}
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
			super::push(out, 69776);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -синец.
pub(crate) fn n41996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41580);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 69777);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 69778);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 95126);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69779);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 95127);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 69780);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95128);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113910);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125954);
									s.is_empty()
								}
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
			super::push(out, 69781);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 95129);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113911);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 125955);
									s.is_empty()
								}
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
			super::push(out, 69782);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95130);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125956);
									s.is_empty()
								}
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
			super::push(out, 69783);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 95131);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113913);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125957);
									s.is_empty()
								}
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
			super::push(out, 69784);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95132);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинец.
pub(crate) fn n41997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41581);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69785);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 95133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69786);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 95134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 95135);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 95136);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 113916);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 125958);
									match s {
										[] => true,
										['м', '-', s @ ..] => {
											super::push(out, 133196);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137505);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141547(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69787);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 95137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 69788);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 69789);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 95139);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 113919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 95141);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 113920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 95142);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 113921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 69791);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 95143);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113922);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 95144);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 95145);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113924);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125960);
									match s {
										[] => true,
										['а', '-', s @ ..] => {
											super::push(out, 133197);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 137506);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141548(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69792);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 95146);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 113925);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 125961);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133198);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 137507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69793);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95147);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 113926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 95148);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113927);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 125962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 95149);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 113928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 69794);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113929);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 113930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 95150);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113931);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', 'у', s @ ..] => {
			super::push(out, 69795);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95151);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113932);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125965);
									s.is_empty()
								}
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
			super::push(out, 69796);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95152);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113933);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 125966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уинец.
pub(crate) fn n41998(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 69797);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 69798);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 95153);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хинец.
pub(crate) fn n41999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41582);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69799);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95154);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 69800);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 95155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113936);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 125967);
									s.is_empty()
								}
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
			super::push(out, 69801);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95156);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 113937);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 125968);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133199);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137508);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141549(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 95157);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 95158);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 95159);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цинец.
pub(crate) fn n42000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41583);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 69802);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 95160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 69803);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95161);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чинец.
pub(crate) fn n42001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41584);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69804);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 69805);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 95162);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113939);
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
				['н', s @ ..] => {
					super::push(out, 95163);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 113940);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125969);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133200);
											s.is_empty()
										}
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
					super::push(out, 95164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 69806);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 69807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95166);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 69808);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 95167);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 69809);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 95168);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 69810);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 95169);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 113944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шинец.
pub(crate) fn n42002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41585);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 69811);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95170);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 95171);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 95172);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 113945);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 125970);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 133201);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 69812);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 95173);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 113946);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125971);
									s.is_empty()
								}
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
			super::push(out, 69813);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 69814);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95174);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 113947);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 125972);
									s.is_empty()
								}
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
			super::push(out, 69815);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 95175);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 113948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 95176);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 113949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 95177);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щинец.
pub(crate) fn n42003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41586);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 69816);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 95178);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 113950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омар-звонец.
pub(crate) fn n138928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137509);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140109);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ганноверец.
pub(crate) fn n138930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137511);
	s.is_empty()
}

/// The word аннарборец.
pub(crate) fn n138931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137512);
	s.is_empty()
}

/// Words ending -петродворец.
pub(crate) fn n138932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137513);
	s.is_empty()
}

/// Words ending -спирт-сырец.
pub(crate) fn n138933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137514);
	s.is_empty()
}

/// Words ending -нефть-сырец.
pub(crate) fn n138934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137515);
	s.is_empty()
}

/// Words ending -к-бровеносец.
pub(crate) fn n138938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137519);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140110);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141660);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 142561);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 143046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -д-богоносец.
pub(crate) fn n138939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137520);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140111);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142562);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 143047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торпедоносец.
pub(crate) fn n138940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137521);
	s.is_empty()
}

/// Words ending -еноносец.
pub(crate) fn n115265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114053);
	s.is_empty()
}

/// Words ending -иноносец.
pub(crate) fn n115266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114054);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 126025);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роносец.
pub(crate) fn n96351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95327);
	s.is_empty()
}

/// Words ending -тоносец.
pub(crate) fn n96352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95328);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 114055);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 126026);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133229);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 126027);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133230);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137523);
									match s {
										[] => true,
										['р', 'е', 'в', '\u{1}', s @ ..] => {
											super::push(out, 140112);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'л', s @ ..] => {
			super::push(out, 114056);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 126028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 133231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 114057);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 126029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 133232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 114058);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126030);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яносец.
pub(crate) fn n70755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70008);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 95329);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114059);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -росец.
pub(crate) fn n42162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41745);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 70009);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 95330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 70010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 95331);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 114060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 95332);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сосец.
pub(crate) fn n42163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41746);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 70011);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -франкфуртец.
pub(crate) fn n138943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137524);
	s.is_empty()
}

/// Words ending -андерматтец.
pub(crate) fn n138944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137525);
	s.is_empty()
}

/// Words ending -ннектикутец.
pub(crate) fn n138945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137526);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нагоспиталь.
pub(crate) fn n138972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137553);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -когоспиталь.
pub(crate) fn n138973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137554);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140128);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 141667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рогоспиталь.
pub(crate) fn n138974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137555);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140129);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 141668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word элементаль.
pub(crate) fn n138975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137556);
	s.is_empty()
}

/// Words ending -усталь.
pub(crate) fn n71635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70887);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 95904);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 95905);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114382);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 126203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 95906);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 114383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фталь.
pub(crate) fn n43213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42790);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 70889);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 95908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фильцнадель.
pub(crate) fn n138979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137560);
	s.is_empty()
}

/// The word чиппендель.
pub(crate) fn n138980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137561);
	s.is_empty()
}

/// Words ending --модель.
pub(crate) fn n97016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95989);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 114432);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 126232);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 133325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 114433);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 126233);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 114434);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 126234);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 133327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 114435);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126235);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 133328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 114436);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 126236);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омодель.
pub(crate) fn n97020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95993);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 114438);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 126238);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133330);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137562);
									s.is_empty()
								}
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
			super::push(out, 114439);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 126239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', s @ ..] => {
			super::push(out, 114440);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 126240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 133331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмодель.
pub(crate) fn n97021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95994);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 114441);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 126241);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 133332);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 137563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фодель.
pub(crate) fn n71744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70996);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 95995);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдель.
pub(crate) fn n43278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42855);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 70997);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 95996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удель.
pub(crate) fn n43279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42856);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 70998);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 95997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 70999);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 95998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 71000);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 95999);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 114442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядель.
pub(crate) fn n43280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42857);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 71001);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етероаллель.
pub(crate) fn n138983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137564);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 140131);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word полишинель.
pub(crate) fn n138984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137565);
	s.is_empty()
}

/// Words ending -атель.
pub(crate) fn n43351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42928);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 71110);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 96085);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 96086);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 114490);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 126264);
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									super::push(out, 126265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t51::n71859(s, out),
		['г', s @ ..] => {
			super::push(out, 71112);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 96094);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 114534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 114535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 96095);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 96096);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 114537);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 126353);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133433);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137650);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141627(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 133434);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137651);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n141628(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 133435);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 114538);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 126354);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133436);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137652);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 126355);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133437);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137653);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 126356);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 133438);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137654);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 133439);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 133440);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 126357);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133441);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 137655);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141629(s, out),
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
									super::push(out, 126358);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133442);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137656);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 96097);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 114539);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 126359);
									match s {
										[] => true,
										['в', s @ ..] => super::t51::n134813(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t51::n97125(s, out),
				['у', s @ ..] => {
					super::push(out, 96099);
					match s {
						[] => true,
						['р', s @ ..] => super::t51::n115755(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t51::n97127(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t51::n71861(s, out),
		['ж', s @ ..] => super::t51::n71862(s, out),
		['з', s @ ..] => super::t51::n71863(s, out),
		['к', s @ ..] => super::t51::n71864(s, out),
		['л', s @ ..] => super::t51::n71865(s, out),
		['м', s @ ..] => super::t51::n71866(s, out),
		['н', s @ ..] => super::t51::n71867(s, out),
		['п', s @ ..] => super::t51::n71868(s, out),
		['р', s @ ..] => super::t51::n71869(s, out),
		['с', s @ ..] => super::t52::n71870(s, out),
		['т', s @ ..] => super::t52::n71871(s, out),
		['х', s @ ..] => super::t52::n71872(s, out),
		['ц', s @ ..] => super::t52::n71873(s, out),
		['ч', s @ ..] => super::t52::n71874(s, out),
		['ш', s @ ..] => super::t52::n71875(s, out),
		['щ', s @ ..] => super::t52::n71876(s, out),
		_ => false,
	}
}

/// Words ending -итель.
pub(crate) fn n43353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42930);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 71134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 96157);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114607);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 126435);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133501);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137706);
													s.is_empty()
												}
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
							super::push(out, 114608);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 126436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 96158);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114609);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133502);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 137707);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 133503);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 137708);
													match s {
														[] => true,
														['-', s @ ..] => super::t52::n141668(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 96159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 96160);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 114611);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 114612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 126437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 114613);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 126438);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 133504);
											s.is_empty()
										}
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
					super::push(out, 96161);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 114614);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 126439);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 133505);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137709);
													match s {
														[] => true,
														['л', s @ ..] => super::t52::n141669(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 133506);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 137710);
													match s {
														[] => true,
														['л', s @ ..] => super::t52::n141670(s, out),
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
									super::push(out, 126440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t52::n71883(s, out),
		['д', s @ ..] => super::t52::n71884(s, out),
		['ж', s @ ..] => super::t52::n71885(s, out),
		['з', s @ ..] => {
			super::push(out, 71138);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 96181);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 114656);
							match s {
								[] => true,
								['с', s @ ..] => super::t52::n127796(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 114657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t52::n97209(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t52::n71887(s, out),
		['л', s @ ..] => super::t52::n71888(s, out),
		['м', s @ ..] => super::t52::n71889(s, out),
		['н', s @ ..] => super::t52::n71890(s, out),
		['о', s @ ..] => super::t52::n71891(s, out),
		['п', s @ ..] => super::t52::n71892(s, out),
		['р', s @ ..] => super::t52::n71893(s, out),
		['с', s @ ..] => super::t52::n71894(s, out),
		['т', s @ ..] => super::t52::n71895(s, out),
		['ч', s @ ..] => super::t52::n71896(s, out),
		['ш', s @ ..] => super::t52::n71897(s, out),
		['щ', s @ ..] => super::t52::n71898(s, out),
		_ => false,
	}
}

/// Words ending -окартель.
pub(crate) fn n116004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114775);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 126614);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133657);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137824);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ортель.
pub(crate) fn n71912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71164);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 96256);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 114776);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 126615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 96257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стель.
pub(crate) fn n43359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 71165);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 96258);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 114777);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126616);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 133658);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 126617);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 133659);
											s.is_empty()
										}
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
					super::push(out, 96259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 71166);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 96260);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 96261);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 114778);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 126618);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133660);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 71167);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 96262);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 114779);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 126619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 96263);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 114780);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 126620);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 133661);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 137825);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 96264);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 96265);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 114781);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 126621);
									s.is_empty()
								}
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
			super::push(out, 71168);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 96266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттель.
pub(crate) fn n43360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42937);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 71169);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 96267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтель.
pub(crate) fn n43361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42938);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 71170);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 96268);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 114782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 71171);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 96269);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятель.
pub(crate) fn n43362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42939);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 71172);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 96270);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 96271);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 96272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 71173);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 96273);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 96274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 71174);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 96275);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 114784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 71175);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 96276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 71176);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 96277);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 114785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мефистофель.
pub(crate) fn n139250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137826);
	s.is_empty()
}

/// Words ending -глокеншпиль.
pub(crate) fn n139251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137827);
	s.is_empty()
}

/// Words ending -иттельшпиль.
pub(crate) fn n139252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137828);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теродактиль.
pub(crate) fn n139253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137829);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 140308);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мфипростиль.
pub(crate) fn n139254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137830);
	s.is_empty()
}

/// Words ending -иераконполь.
pub(crate) fn n139258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137834);
	s.is_empty()
}

/// Words ending -бизнес-день.
pub(crate) fn n139262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137838);
	s.is_empty()
}

/// Words ending -сковородень.
pub(crate) fn n139263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137839);
	s.is_empty()
}

/// Words ending -белибердень.
pub(crate) fn n139264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137840);
	s.is_empty()
}

/// Words ending -опердень.
pub(crate) fn n116190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114961);
	s.is_empty()
}

/// Words ending -ордень.
pub(crate) fn n72285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71535);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 96573);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 114962);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126707);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 133703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удень.
pub(crate) fn n43693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43270);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 71536);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 71537);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 71538);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 96574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 71539);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 96575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 71540);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 96576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 114963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 71541);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 96577);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдень.
pub(crate) fn n43694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43271);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 71542);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 96578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядень.
pub(crate) fn n43695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43272);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 71543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -микрозелень.
pub(crate) fn n139265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137841);
	s.is_empty()
}

/// Words ending -тырь-камень.
pub(crate) fn n139266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -переступень.
pub(crate) fn n139267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137843);
	s.is_empty()
}

/// Words ending -баха-парень.
pub(crate) fn n139268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137844);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 140313);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141789);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -желтокорень.
pub(crate) fn n139269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137845);
	s.is_empty()
}

/// Words ending -перевертень.
pub(crate) fn n139270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137846);
	s.is_empty()
}

/// Words ending -прихвостень.
pub(crate) fn n139271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137847);
	s.is_empty()
}

/// Words ending -утень.
pub(crate) fn n43750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43327);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 71628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 96667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 71629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 96668);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 115013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 71630);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 96669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пытень.
pub(crate) fn n72381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71631);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 96670);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 115014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютень.
pub(crate) fn n43752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43329);
	match s {
		[] => true,
		['л', '\u{1}', s @ ..] => {
			super::push(out, 71632);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 71633);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 96671);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 115015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 96672);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 115016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ик-секретарь.
pub(crate) fn n139273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137849);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140315);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141791);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 142650);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 143109);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 143337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кс-секретарь.
pub(crate) fn n139274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137850);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 140316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сс-секретарь.
pub(crate) fn n139275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137851);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140317);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тс-секретарь.
pub(crate) fn n139276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137852);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140318);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141793);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -госсекретарь.
pub(crate) fn n139277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137853);
	s.is_empty()
}

/// Words ending -рев-глухарь.
pub(crate) fn n139278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137854);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140319);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141794);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вость.
pub(crate) fn n44148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43724);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72053);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 96957);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 96958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 72054);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 96959);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 115141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 96960);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 115142);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 126794);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 115143);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 126795);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 133744);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 115144);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 115145);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 126796);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 133745);
											s.is_empty()
										}
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
					super::push(out, 96961);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 115146);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 126797);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133746);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 137865);
													match s {
														[] => true,
														['о', s @ ..] => super::t52::n141770(s, out),
														_ => false,
													}
												}
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
							super::push(out, 115147);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126798);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 133747);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137866);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 96962);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 115148);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 126799);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 133748);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 137867);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 96963);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 115149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 96964);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 115150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 96965);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 115151);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 96966);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 115152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 72055);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 96967);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 115153);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 126801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t52::n72808(s, out),
		['о', s @ ..] => super::t52::n72809(s, out),
		['р', s @ ..] => {
			super::push(out, 72058);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 96991);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 115245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 72059);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 96992);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 96993);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 115246);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126907);
									s.is_empty()
								}
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
			super::push(out, 72060);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 96994);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 115247);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126908);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 133864);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72061);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 96995);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 96996);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оропкость.
pub(crate) fn n128238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126915);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 133870);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 137947);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 140367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скость.
pub(crate) fn n72835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72083);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97010);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 115258);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126916);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133871);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137948);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140368);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97011);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 97012);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 97013);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115260);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 126917);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133872);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137949);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140369);
													match s {
														[] => true,
														['и', s @ ..] => super::t52::n143265(s, out),
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
									super::push(out, 133873);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 137950);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140370);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115261);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 115262);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 97014);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 115263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 97015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ткость.
pub(crate) fn n72836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72084);
	match s {
		[] => true,
		['е', 'ч', s @ ..] => {
			super::push(out, 97016);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 97017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкость.
pub(crate) fn n72838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72086);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 97018);
			match s {
				[] => true,
				['я', 'ъ', s @ ..] => {
					super::push(out, 115264);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126919);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133874);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 137951);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лость.
pub(crate) fn n44153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43729);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72087);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 97019);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 115265);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 126920);
									s.is_empty()
								}
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
			super::push(out, 72088);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 72089);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97020);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 115266);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 126921);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 133875);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 137952);
													match s {
														[] => true,
														['л', s @ ..] => super::t52::n141816(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 97021);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 97022);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 115267);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126922);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133876);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 137953);
													match s {
														[] => true,
														['в', s @ ..] => super::t52::n141817(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72090);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 72091);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97023);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 97024);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 115268);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 126923);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 133877);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72092);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 115269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 97026);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115270);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 126924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 133878);
											s.is_empty()
										}
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
					super::push(out, 97027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 115271);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 115272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 72093);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 72094);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 72095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мость.
pub(crate) fn n44154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43730);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72096);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 97029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 72097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97030);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 115273);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => super::t52::n116505(s, out),
						['г', s @ ..] => {
							super::push(out, 115275);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 115276);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 126930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 115277);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 115278);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 115279);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 126931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 115280);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 115281);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 115282);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 115283);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 115284);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 126934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => super::t52::n116516(s, out),
						['ч', s @ ..] => super::t52::n116517(s, out),
						['ш', s @ ..] => super::t52::n116518(s, out),
						['щ', s @ ..] => super::t52::n116519(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t52::n98068(s, out),
				['у', s @ ..] => super::t52::n98069(s, out),
				['я', s @ ..] => super::t52::n98070(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t52::n72850(s, out),
		['о', s @ ..] => super::t52::n72851(s, out),
		['ю', s @ ..] => super::t52::n72852(s, out),
		['я', s @ ..] => super::t52::n72853(s, out),
		_ => false,
	}
}

/// Words ending -ность.
pub(crate) fn n44155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43731);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72102);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97053);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 97054);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 115328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 97055);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 97056);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 115330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 72103);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97057);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 97058);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 115331);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 126967);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 133919);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 137988);
													match s {
														[] => true,
														['н', s @ ..] => super::t52::n141842(s, out),
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
									super::push(out, 126968);
									match s {
										[] => true,
										['ъ', s @ ..] => {
											super::push(out, 133920);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 126969);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 133921);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 126970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 115332);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 126971);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 133922);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137989);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 126972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 115333);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 126973);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 126974);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 133923);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 137990);
													match s {
														[] => true,
														['з', s @ ..] => super::t52::n141843(s, out),
														_ => false,
													}
												}
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
							super::push(out, 115334);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 126975);
									match s {
										[] => true,
										['п', 'с', s @ ..] => {
											match s {
												[] => true,
												['е', 'ж', 'е', 'т', 'а', 'л', 'п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t52::n141844(s, out),
														['е', 'н', '\u{1}', s @ ..] => super::t52::n141845(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['к', 'ж', 'е', 'л', '\u{1}', s @ ..] => super::t52::n141846(s, out),
														['т', s @ ..] => super::t52::n141847(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 97059);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 115335);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 126976);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 133924);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 137991);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97060);
					s.is_empty()
				}
				['е', 'н', s @ ..] => {
					super::push(out, 97061);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							super::push(out, 115336);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 126977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t52::n98100(s, out),
				['о', s @ ..] => super::t52::n98101(s, out),
				['ы', s @ ..] => {
					super::push(out, 97064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t52::n72857(s, out),
		['е', s @ ..] => super::t52::n72858(s, out),
		['ж', s @ ..] => super::t52::n72859(s, out),
		['з', s @ ..] => super::t52::n72860(s, out),
		['и', s @ ..] => super::t52::n72861(s, out),
		['й', s @ ..] => super::t52::n72862(s, out),
		['к', s @ ..] => super::t52::n72863(s, out),
		['м', s @ ..] => super::t52::n72864(s, out),
		['н', s @ ..] => super::t52::n72865(s, out),
		['п', s @ ..] => super::t52::n72866(s, out),
		['р', s @ ..] => super::t52::n72867(s, out),
		['с', s @ ..] => super::t52::n72868(s, out),
		['т', s @ ..] => super::t52::n72869(s, out),
		['ф', s @ ..] => super::t52::n72870(s, out),
		['х', s @ ..] => super::t52::n72871(s, out),
		['ч', s @ ..] => super::t52::n72872(s, out),
		['ш', s @ ..] => super::t52::n72873(s, out),
		['щ', s @ ..] => super::t52::n72874(s, out),
		['ь', s @ ..] => super::t52::n72875(s, out),
		['ю', s @ ..] => super::t52::n72876(s, out),
		['я', s @ ..] => super::t52::n72877(s, out),
		_ => false,
	}
}

/// Words ending -пость.
pub(crate) fn n44156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43732);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72126);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 72127);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 97157);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 115576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 97158);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115577);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 127381);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 134407);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 115578);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 127382);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 134408);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138456);
													match s {
														[] => true,
														['р', s @ ..] => super::t52::n142180(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72128);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 72129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рость.
pub(crate) fn n44157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43733);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72130);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 97159);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 115579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 72131);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 72132);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97160);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 115580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 97161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 72133);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 72134);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 97162);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 115581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127383);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 127384);
									s.is_empty()
								}
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
			super::push(out, 72135);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 97163);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 97164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 72136);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сость.
pub(crate) fn n44158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43734);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 72137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тость.
pub(crate) fn n44159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43735);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72138);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97165);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 97166);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 97167);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127385);
									s.is_empty()
								}
								['у', 'з', s @ ..] => {
									super::push(out, 127386);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134409);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 115583);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127387);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 134410);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138457);
													match s {
														[] => true,
														['р', s @ ..] => super::t52::n142181(s, out),
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
									super::push(out, 127388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 115584);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 115585);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127389);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 134411);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 138458);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t52::n142182(s, out),
														_ => false,
													}
												}
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
							super::push(out, 115586);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 115587);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127391);
									s.is_empty()
								}
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
			super::push(out, 72139);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 97168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 72140);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 97169);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 97170);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 115588);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 115589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127392);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134412);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 127393);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134413);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 115590);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 127394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 97171);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 115591);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127395);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134414);
											match s {
												[] => true,
												['н', 'з', '\u{1}', s @ ..] => {
													super::push(out, 138459);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 97172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 97173);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115592);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 127396);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 134415);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138460);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['к', s @ ..] => {
							super::push(out, 115593);
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							super::push(out, 115594);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 127397);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134416);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['п', 'а', s @ ..] => {
					super::push(out, 97174);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 115595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'ы', s @ ..] => {
					super::push(out, 97175);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 115596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t52::n72895(s, out),
		['у', s @ ..] => {
			super::push(out, 72142);
			match s {
				[] => true,
				['д', s @ ..] => super::t52::n98221(s, out),
				['н', s @ ..] => super::t52::n98222(s, out),
				['р', s @ ..] => {
					super::push(out, 97183);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 115622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t52::n72897(s, out),
		['ю', s @ ..] => {
			super::push(out, 72144);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 97186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t52::n72899(s, out),
		_ => false,
	}
}

/// Words ending -хость.
pub(crate) fn n44160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43736);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 72146);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 72147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ухмистерская.
pub(crate) fn n142201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140756);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 142037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скусомужная.
pub(crate) fn n139950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138515);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 140757);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -беркулезная.
pub(crate) fn n139951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138516);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 140758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ня-гостиная.
pub(crate) fn n139952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138517);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 140759);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142038);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еспартийная.
pub(crate) fn n139953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138518);
	s.is_empty()
}

/// Words ending -рестованная.
pub(crate) fn n139954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138519);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енная.
pub(crate) fn n44371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43947);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 72355);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97359);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 115754);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 127532);
									s.is_empty()
								}
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
			super::push(out, 72356);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 97360);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 115755);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127533);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134516);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72357);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97361);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 115756);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 127534);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 97362);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115757);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 115758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 72358);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97363);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 115759);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 127535);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134517);
											s.is_empty()
										}
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
					super::push(out, 97364);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 115760);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 127536);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134518);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72359);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 97365);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 115761);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127537);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 134519);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138520);
													match s {
														[] => true,
														['в', s @ ..] => super::t52::n142206(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 97366);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 115762);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 127538);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 134520);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138521);
													match s {
														[] => true,
														['р', s @ ..] => super::t52::n142207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 97367);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 115763);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 115764);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 127540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 97368);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 115765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 97369);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 115766);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 127541);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134521);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138522);
													match s {
														[] => true,
														['о', s @ ..] => super::t52::n142208(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 72360);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97370);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 115767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 97371);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 115768);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127542);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 134522);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 138523);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 134523);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 127543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 97372);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115769);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 127544);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134524);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t52::n98416(s, out),
				['ь', s @ ..] => {
					super::push(out, 97374);
					match s {
						[] => true,
						['л', s @ ..] => super::t52::n117019(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t52::n73115(s, out),
		['р', s @ ..] => super::t52::n73116(s, out),
		['т', s @ ..] => super::t52::n73117(s, out),
		['ч', s @ ..] => super::t52::n73118(s, out),
		['ш', s @ ..] => super::t52::n73119(s, out),
		['щ', s @ ..] => super::t52::n73120(s, out),
		_ => false,
	}
}

/// Words ending -благоверная.
pub(crate) fn n139963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138528);
	s.is_empty()
}

/// Words ending -умуляторная.
pub(crate) fn n139964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138529);
	s.is_empty()
}

/// Words ending -а-прачечная.
pub(crate) fn n139965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138530);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140765);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142041);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142776);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 143168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -даточная.
pub(crate) fn n117072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115825);
	match s {
		[] => true,
		['з', 'а', 'р', '-', s @ ..] => {
			super::push(out, 127579);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 134549);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138531);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140766);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142042);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 142777);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лычная.
pub(crate) fn n73178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72424);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 97440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115826);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 127580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячная.
pub(crate) fn n44400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43976);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 72425);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97441);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 115827);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127581);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 134550);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72426);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 97442);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 115828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word дневальная.
pub(crate) fn n139967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138532);
	s.is_empty()
}

/// Words ending -касательная.
pub(crate) fn n139968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138533);
	s.is_empty()
}

/// Words ending -ясновидящая.
pub(crate) fn n139972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138537);
	s.is_empty()
}

/// Words ending -еггогология.
pub(crate) fn n139981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138546);
	s.is_empty()
}

/// Words ending -адология.
pub(crate) fn n98609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97564);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 115910);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127634);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 134582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еология.
pub(crate) fn n98610(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 115911);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 115912);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 115913);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зология.
pub(crate) fn n98611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97565);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 115914);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127637);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134583);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138547);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 140774);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иология.
pub(crate) fn n98612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97566);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 115915);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 134584);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 138548);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 140775);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 142046);
													s.is_empty()
												}
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
							super::push(out, 134585);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 138549);
									s.is_empty()
								}
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
			super::push(out, 115916);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 115917);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 127638);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 134586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кология.
pub(crate) fn n98613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97567);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 115918);
			match s {
				[] => true,
				['с', 'к', 'е', s @ ..] => {
					super::push(out, 127639);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 134587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лология.
pub(crate) fn n98614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97568);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 115919);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 127640);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 134588);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 138550);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140776);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'ф', s @ ..] => {
			super::push(out, 115920);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 115921);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127641);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 134589);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138551);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140777);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 142047);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омология.
pub(crate) fn n98615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97569);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 115922);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 127642);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нология.
pub(crate) fn n98616(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 115923);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 127643);
					s.is_empty()
				}
				['м', 'р', s @ ..] => {
					super::push(out, 127644);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 127645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'с', s @ ..] => {
			super::push(out, 115924);
			s.is_empty()
		}
		['о', 'ф', s @ ..] => {
			super::push(out, 115925);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 115926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -опология.
pub(crate) fn n98617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97570);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 115927);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рология.
pub(crate) fn n98618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97571);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 115928);
			s.is_empty()
		}
		['о', 'е', s @ ..] => {
			super::push(out, 115929);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 115930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -асология.
pub(crate) fn n98619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97572);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 115931);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 127646);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тология.
pub(crate) fn n98620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97573);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115932);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 127647);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 134591);
							match s {
								[] => true,
								['л', 'к', 'о', 'р', s @ ..] => {
									super::push(out, 138552);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 140778);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142048);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t52::n144232(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 127648);
					match s {
						[] => true,
						['о', 'х', s @ ..] => {
							super::push(out, 134592);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 138553);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 140779);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142049);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115933);
			s.is_empty()
		}
		['и', 'ц', s @ ..] => {
			super::push(out, 115934);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 115935);
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['е', 'а', s @ ..] => {
					super::push(out, 127649);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 134593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'е', 'л', s @ ..] => {
					super::push(out, 127650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'м', s @ ..] => {
			super::push(out, 115936);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 127651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134594);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 138554);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 140780);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 115937);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127652);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 134595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 115938);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 127653);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 127654);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 127655);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 115939);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 127656);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134598);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -миксоплодия.
pub(crate) fn n139995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138560);
	s.is_empty()
}

/// Words ending -еатр-студия.
pub(crate) fn n139996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138561);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142050);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -диосинкразия.
pub(crate) fn n142230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140785);
	s.is_empty()
}

/// Words ending -ман-фантазия.
pub(crate) fn n142231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140786);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142051);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ния-фантазия.
pub(crate) fn n142232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140787);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142052);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 142781);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 143170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -псевдопоэзия.
pub(crate) fn n140003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138568);
	s.is_empty()
}

/// Words ending -филия.
pub(crate) fn n44710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44280);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 72759);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 72760);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 97748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 72761);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 72762);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 97750);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116059);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 127715);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 134630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 116060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 127716);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134631);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138571);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t52::n142235(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 97751);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116061);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 127717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 116062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 97752);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 116063);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 127719);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134632);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 138572);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 97753);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116064);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 116065);
							match s {
								[] => true,
								['р', 'е', 'м', 'а', '\u{1}', s @ ..] => {
									super::push(out, 127721);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 116066);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127722);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134633);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 138573);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 97754);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116067);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 127723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134634);
											s.is_empty()
										}
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
					super::push(out, 97755);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116068);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 116069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 97756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116070);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 127724);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134635);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138574);
													match s {
														[] => true,
														['е', s @ ..] => super::t52::n142236(s, out),
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
									super::push(out, 127725);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134636);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 116071);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 127726);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134637);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t52::n117321(s, out),
						['о', s @ ..] => {
							super::push(out, 116073);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 116074);
							match s {
								[] => true,
								['е', s @ ..] => super::t52::n129078(s, out),
								_ => false,
							}
						}
						['я', s @ ..] => super::t52::n117324(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t52::n98810(s, out),
				['р', s @ ..] => super::t52::n98811(s, out),
				['с', s @ ..] => super::t52::n98812(s, out),
				['т', s @ ..] => super::t52::n98813(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гоанатомия.
pub(crate) fn n136060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134655);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фатомия.
pub(crate) fn n98888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97828);
	s.is_empty()
}

/// Words ending -итомия.
pub(crate) fn n73603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72837);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 97829);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 116139);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 127768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктомия.
pub(crate) fn n73604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72838);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 97830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отомия.
pub(crate) fn n73605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72839);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 97831);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 97832);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 97833);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 97834);
			s.is_empty()
		}
		['л', 'у', s @ ..] => {
			super::push(out, 97835);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 116140);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 127769);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 134657);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 138579);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140795);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97836);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 116141);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127770);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 134658);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138580);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97837);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 116142);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 127771);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134659);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138581);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140796);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97838);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 97839);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 116143);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 116144);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 127773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 116145);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стомия.
pub(crate) fn n73606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72840);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 97840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лициногения.
pub(crate) fn n140019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138584);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140798);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -схиигумения.
pub(crate) fn n140020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138585);
	s.is_empty()
}

/// Words ending -астения.
pub(crate) fn n98948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97887);
	s.is_empty()
}

/// Words ending -фения.
pub(crate) fn n44795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44365);
	s.is_empty()
}

/// Words ending -шения.
pub(crate) fn n44796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44366);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 72893);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97888);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116176);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 127788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ежка-молния.
pub(crate) fn n140021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138586);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140799);
			match s {
				[] => true,
				['с', 'а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 142057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рония.
pub(crate) fn n44823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72941);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 72942);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97935);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 116206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 97936);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 116207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 72943);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 97937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 72944);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 97938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 72945);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97939);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 72946);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97940);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 97941);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116210);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 127807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тония.
pub(crate) fn n44825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44395);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72949);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 97943);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 97944);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116211);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 127808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 116212);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127809);
									s.is_empty()
								}
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
			super::push(out, 72950);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97945);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 72951);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 97946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 72952);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 97947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 72953);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116216);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 116217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 97948);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 116218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 97949);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 97950);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 97951);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116221);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 127811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 116222);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127812);
									s.is_empty()
								}
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
			super::push(out, 72954);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97952);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116223);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127813);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 134675);
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['и', s @ ..] => {
					super::push(out, 97953);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 116224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 97954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фония.
pub(crate) fn n44826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44396);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			super::push(out, 72955);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 97955);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 116225);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 127814);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 134676);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 72956);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 97956);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 72957);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 97957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 72958);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 97958);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116227);
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
				['к', s @ ..] => {
					super::push(out, 97959);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116228);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 127815);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 116229);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127816);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134677);
											s.is_empty()
										}
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
					super::push(out, 97960);
					s.is_empty()
				}
				['м', 'о', 'г', s @ ..] => {
					super::push(out, 97961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 97962);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 97963);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 116231);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 127817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мастер-копия.
pub(crate) fn n142246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140801);
	s.is_empty()
}

/// Words ending -роман-утопия.
pub(crate) fn n142247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140802);
	s.is_empty()
}

/// Words ending -александрия.
pub(crate) fn n140030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138595);
	s.is_empty()
}

/// Words ending -аркшейдерия.
pub(crate) fn n140031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138596);
	s.is_empty()
}

/// Words ending -льстремерия.
pub(crate) fn n140032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138597);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птомерия.
pub(crate) fn n117548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116299);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 127858);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134698);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 138598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмерия.
pub(crate) fn n73811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73038);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 98045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юмерия.
pub(crate) fn n73812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73039);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 98046);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 116301);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерия.
pub(crate) fn n44879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44448);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 73040);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -перия.
pub(crate) fn n44880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44449);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 73041);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98047);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рерия.
pub(crate) fn n44881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44450);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 73042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -серия.
pub(crate) fn n44882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44451);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 73043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98048);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116302);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134699);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терия.
pub(crate) fn n44883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44452);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 73044);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 98049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 73045);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98050);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127861);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 127862);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 134700);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138599);
													match s {
														[] => true,
														['р', s @ ..] => super::t52::n142249(s, out),
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
									super::push(out, 127863);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134701);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138600);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', 'и', 'л', s @ ..] => {
											super::push(out, 134702);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138601);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 134703);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 134704);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138602);
													match s {
														[] => true,
														['з', s @ ..] => super::t52::n142250(s, out),
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
									super::push(out, 127864);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134705);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 138603);
													match s {
														[] => true,
														['у', s @ ..] => super::t52::n142251(s, out),
														_ => false,
													}
												}
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
							super::push(out, 116303);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127865);
									s.is_empty()
								}
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
			super::push(out, 73046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 73047);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98052);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 116304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 98053);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 116305);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127866);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 134706);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 73048);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 98054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 73049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 73050);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 98057);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116308);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 116309);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 116310);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 116311);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 116312);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 127867);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134707);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 138604);
													s.is_empty()
												}
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
							super::push(out, 116313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 98058);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 98059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 116314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 98060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 73051);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98061);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 116315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127869);
									s.is_empty()
								}
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
			super::push(out, 73052);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 98062);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116316);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 127870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t52::n73826(s, out),
		_ => false,
	}
}

/// Words ending -ферия.
pub(crate) fn n44884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44453);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 73054);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 98064);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116318);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 127872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херия.
pub(crate) fn n44885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44454);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 73055);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 98065);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 116319);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 127873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церия.
pub(crate) fn n44886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44455);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 73056);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 98066);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116320);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 127874);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 127875);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 134708);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 138605);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ет-оратория.
pub(crate) fn n140041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138606);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142061);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пофория.
pub(crate) fn n99162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98097);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 116337);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 127887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сфория.
pub(crate) fn n73862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73089);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 98098);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 116338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хория.
pub(crate) fn n44908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44477);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 73090);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 98099);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116339);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127888);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134713);
											match s {
												[] => true,
												['и', 'м', '\u{1}', s @ ..] => {
													super::push(out, 138607);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['л', s @ ..] => {
							super::push(out, 116340);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 127889);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 134714);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 116341);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 98100);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 116342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 98101);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 116343);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 127891);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134715);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 138608);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 98102);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 116344);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 127892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 98103);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116345);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 127893);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 134716);
											s.is_empty()
										}
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
					super::push(out, 98104);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 116346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['в', 'а', '\u{1}', s @ ..] => {
							super::push(out, 116347);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 116348);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 127894);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134717);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138609);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t52::n142253(s, out),
														_ => false,
													}
												}
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
							super::push(out, 116349);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьория.
pub(crate) fn n44909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44478);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 73091);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 98105);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 98106);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етрия.
pub(crate) fn n44911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44480);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 73098);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 98116);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 116358);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 116359);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 127897);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134718);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 138610);
													s.is_empty()
												}
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
							super::push(out, 116360);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 127898);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134719);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 138611);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 98117);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 116361);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 127899);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 134720);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t52::n99185(s, out),
				['м', s @ ..] => {
					super::push(out, 98119);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 116367);
							match s {
								[] => true,
								['с', 'а', s @ ..] => {
									super::push(out, 127907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t52::n99187(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бисинистрия.
pub(crate) fn n140077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138642);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140827);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142075);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икрофилярия.
pub(crate) fn n140078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138643);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -турбеллярия.
pub(crate) fn n140079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138644);
	s.is_empty()
}

/// Words ending -ппендикулярия.
pub(crate) fn n140080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138645);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140829);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word полидипсия.
pub(crate) fn n140084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138649);
	s.is_empty()
}

/// Words ending -сеноглоссия.
pub(crate) fn n140085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138650);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -топатия.
pub(crate) fn n99284(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 116466);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127999);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 134795);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 138658);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140831);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 142077);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'а', s @ ..] => {
			super::push(out, 116467);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 128000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хопатия.
pub(crate) fn n99285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98216);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 116468);
			match s {
				[] => true,
				['с', 'п', 'о', s @ ..] => {
					super::push(out, 128001);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134797);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 138659);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140832);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142078);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полстолетия.
pub(crate) fn n140095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138660);
	s.is_empty()
}

/// The word филокартия.
pub(crate) fn n140096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138661);
	s.is_empty()
}

/// Words ending -флудерастия.
pub(crate) fn n140097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138662);
	s.is_empty()
}

/// Words ending -ман-биография.
pub(crate) fn n143530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142079);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142791);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 143175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word мегалография.
pub(crate) fn n143531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142080);
	s.is_empty()
}

/// Words ending -пантомография.
pub(crate) fn n143532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142081);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142792);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 143176);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 143365);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стика-рекомендация.
pub(crate) fn n144246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142793);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 143177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отзыв-рекомендация.
pub(crate) fn n144247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142794);
	s.is_empty()
}

/// Words ending -киборгизация.
pub(crate) fn n142285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140840);
	s.is_empty()
}

/// Words ending -фероидизация.
pub(crate) fn n142286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140841);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ивритизация.
pub(crate) fn n142287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140842);
	s.is_empty()
}

/// Words ending -акт-рекламация.
pub(crate) fn n142288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140843);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 142085);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атье-комбинация.
pub(crate) fn n142289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140844);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142086);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вка-презентация.
pub(crate) fn n142290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140845);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142087);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142797);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 143178);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 143366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дыш-аннотация.
pub(crate) fn n142291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140846);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142088);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142798);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 143179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-трапеция.
pub(crate) fn n140120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138685);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140847);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гада-секция.
pub(crate) fn n140121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138686);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 140848);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142090);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-электростанция.
pub(crate) fn n140122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138687);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140849);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142091);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142800);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 143180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word собаченция.
pub(crate) fn n140123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138688);
	s.is_empty()
}

/// Words ending -старушенция.
pub(crate) fn n140124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138689);
	s.is_empty()
}

/// Words ending -ца-глазунья.
pub(crate) fn n140136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138701);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 140854);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142094);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 142802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ан-да-кунья.
pub(crate) fn n140137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138702);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140855);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етростроевка.
pub(crate) fn n140161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138726);
	s.is_empty()
}

/// Words ending -дивостоковка.
pub(crate) fn n140165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138730);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140867);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142101);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ка-теньковка.
pub(crate) fn n140166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138731);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 140868);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word абзелиловка.
pub(crate) fn n140167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138732);
	s.is_empty()
}

/// Words ending -ыболовка.
pub(crate) fn n100145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99072);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 116961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -воловка.
pub(crate) fn n75296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74518);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 99073);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -головка.
pub(crate) fn n75297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74519);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 116962);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 128285);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 134952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 116963);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 128286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 99074);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 116964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 99075);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 128287);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 134953);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138733);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140869);
													s.is_empty()
												}
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
							super::push(out, 128288);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 134954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 116965);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 128289);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 134955);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138734);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 99076);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 116966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 128290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -золовка.
pub(crate) fn n75298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74520);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 99077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -коловка.
pub(crate) fn n75299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74521);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 99078);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 99079);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -моловка.
pub(crate) fn n75300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74522);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 99080);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 116967);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роловка.
pub(crate) fn n75301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74523);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 99081);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 116968);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 128291);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 134956);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138735);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 99082);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 116969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 99083);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соловка.
pub(crate) fn n75302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74524);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 99084);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 99085);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 116970);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 128292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -толовка.
pub(crate) fn n75303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74525);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 99086);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -холовка.
pub(crate) fn n75304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74526);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 99087);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 116971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 99088);
			match s {
				[] => true,
				['м', '\u{1}', s @ ..] => {
					super::push(out, 116972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рловка.
pub(crate) fn n46647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46205);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 74527);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 99089);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 99090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 74528);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 99091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 74529);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 99092);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -словка.
pub(crate) fn n46648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46206);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 74530);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 99093);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 99094);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 116974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 74531);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 99095);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 116975);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 128293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 99096);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 116976);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 99097);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116977);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 128295);
									s.is_empty()
								}
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
			super::push(out, 74532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тловка.
pub(crate) fn n46649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46207);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 74533);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 99098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уловка.
pub(crate) fn n46650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46208);
	s.is_empty()
}

/// Words ending -ыловка.
pub(crate) fn n46651(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 74534);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 99099);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 116978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 74535);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99100);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 116979);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яловка.
pub(crate) fn n46652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46209);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 74536);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 74537);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99101);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 116980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новозвановка.
pub(crate) fn n140171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138736);
	s.is_empty()
}

/// Words ending -иновка.
pub(crate) fn n46663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46220);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 74563);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 99129);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 117001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 74564);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 117002);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128311);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 134964);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 138737);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 74565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99131);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 99132);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 117003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 99133);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117004);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 99134);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 117005);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128313);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 134965);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 138738);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 74566);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 99135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 74567);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 99136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 74568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 99137);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 99138);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 117006);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128314);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 134966);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138739);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 99139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 74569);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 99140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117007);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128315);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 134967);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 74570);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 74571);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99141);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оновка.
pub(crate) fn n46665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46222);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 74572);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 99142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 74573);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 99143);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 117008);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128316);
									s.is_empty()
								}
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
			super::push(out, 74574);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 99144);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 117009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 74575);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 99145);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117010);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128317);
									s.is_empty()
								}
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
			super::push(out, 74576);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99146);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 117011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 74577);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 99147);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерновка.
pub(crate) fn n75356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74578);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 99148);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 117013);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 128318);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 134968);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138740);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140870);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирновка.
pub(crate) fn n75357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74579);
	s.is_empty()
}

/// Words ending -сновка.
pub(crate) fn n46667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46224);
	s.is_empty()
}

/// Words ending -уновка.
pub(crate) fn n46668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46225);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 74580);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 99149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117014);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 128319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыновка.
pub(crate) fn n46669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46226);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 74581);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99150);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 117015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 74582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яновка.
pub(crate) fn n46670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46227);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 74583);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 99151);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 117016);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 128320);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134969);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word чернобровка.
pub(crate) fn n140176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138741);
	s.is_empty()
}

/// Words ending -лександровка.
pub(crate) fn n140177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138742);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140871);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ногокровка.
pub(crate) fn n129684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128334);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 134977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -покровка.
pub(crate) fn n100245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99171);
	s.is_empty()
}

/// Words ending -токровка.
pub(crate) fn n100246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99172);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 117032);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 128335);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 134978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укровка.
pub(crate) fn n75385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74607);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 99173);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 117033);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 128336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оровка.
pub(crate) fn n46681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46238);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 74608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 74609);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 99175);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 117034);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128337);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 134979);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138743);
													match s {
														[] => true,
														['к', s @ ..] => super::t52::n142317(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 74610);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 99176);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 117035);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128338);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 134980);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 74611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 74612);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 99178);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тровка.
pub(crate) fn n46682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46239);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 74613);
			match s {
				[] => true,
				['м', 'о', 'л', 'и', 'к', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 99179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 74614);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 99180);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117036);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 128339);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 134981);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138744);
													match s {
														[] => true,
														['п', s @ ..] => super::t52::n142318(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уровка.
pub(crate) fn n46683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46240);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 74615);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 99181);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 117037);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женсоветовка.
pub(crate) fn n140180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138745);
	s.is_empty()
}

/// The word брезентовка.
pub(crate) fn n140181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138746);
	s.is_empty()
}

/// Words ending -естовка.
pub(crate) fn n75425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74646);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 99207);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 117056);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -истовка.
pub(crate) fn n75426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74647);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 99208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кстовка.
pub(crate) fn n75427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74648);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 99209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лстовка.
pub(crate) fn n75428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74649);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 99210);
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 117057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остовка.
pub(crate) fn n75429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74650);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 99211);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					super::push(out, 117058);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устовка.
pub(crate) fn n75430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74651);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 99212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыстовка.
pub(crate) fn n75431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74652);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 99213);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 117059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 128352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утовка.
pub(crate) fn n46701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46258);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 74653);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 74654);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 99214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 74655);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 74656);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 99215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 74657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 99216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтовка.
pub(crate) fn n46702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 74658);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 99217);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117062);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 117063);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 117064);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 128353);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 134989);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 138747);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 74659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытовка.
pub(crate) fn n46703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46260);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 74660);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 99218);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 117065);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128354);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 134990);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 138748);
													match s {
														[] => true,
														['о', s @ ..] => super::t52::n142319(s, out),
														_ => false,
													}
												}
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
							super::push(out, 117066);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128355);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 134991);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138749);
													match s {
														[] => true,
														['в', s @ ..] => super::t52::n142320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word нижегородка.
pub(crate) fn n140188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138753);
	s.is_empty()
}

/// Words ending -льхозтехника.
pub(crate) fn n140203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138768);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140885);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -амначальника.
pub(crate) fn n140204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138769);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 140886);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерниопластика.
pub(crate) fn n140206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138771);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 140887);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -социолингвистика.
pub(crate) fn n140207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138772);
	s.is_empty()
}

/// Words ending -вилибристика.
pub(crate) fn n140208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138773);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140888);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 142111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word колористика.
pub(crate) fn n140209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138774);
	s.is_empty()
}

/// Words ending -агностика.
pub(crate) fn n100586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99508);
	match s {
		[] => true,
		['и', 'д', s @ ..] => {
			match s {
				[] => true,
				['-', 'с', s @ ..] => {
					super::push(out, 128437);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 135031);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138775);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140889);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142112);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['и', s @ ..] => {
							super::push(out, 135032);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 135033);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138776);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 140890);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142113);
													match s {
														[] => true,
														['н', s @ ..] => super::t52::n144259(s, out),
														_ => false,
													}
												}
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
							super::push(out, 135034);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138777);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 140891);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 138778);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 138779);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 140892);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142114);
													match s {
														[] => true,
														['л', s @ ..] => super::t52::n144260(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устика.
pub(crate) fn n47055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75016);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 99510);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 75017);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 99511);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ширококолейка.
pub(crate) fn n140219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138784);
	s.is_empty()
}

/// The word слепозмейка.
pub(crate) fn n140220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138785);
	s.is_empty()
}

/// Words ending -бука-копейка.
pub(crate) fn n140221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138786);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 140894);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142116);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гиперборейка.
pub(crate) fn n140222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138787);
	s.is_empty()
}

/// Words ending -лександрийка.
pub(crate) fn n140224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138789);
	s.is_empty()
}

/// Words ending -многосерийка.
pub(crate) fn n140225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138790);
	s.is_empty()
}

/// The word пара-тройка.
pub(crate) fn n140226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138791);
	s.is_empty()
}

/// Words ending -остюм-тройка.
pub(crate) fn n140227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138792);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ом-новостройка.
pub(crate) fn n140228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138793);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 140896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алка-копалка.
pub(crate) fn n140230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138795);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 140898);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -есло-каталка.
pub(crate) fn n140231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138796);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140899);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -анок-качалка.
pub(crate) fn n140232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138797);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140900);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142120);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -есло-качалка.
pub(crate) fn n140233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138798);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140901);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142121);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шадь-качалка.
pub(crate) fn n140234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138799);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140902);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кша-треуголка.
pub(crate) fn n142350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142124);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 142808);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -верховажанка.
pub(crate) fn n140246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138811);
	s.is_empty()
}

/// Words ending -ь-самобранка.
pub(crate) fn n140247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138812);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140908);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142126);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шапка-ушанка.
pub(crate) fn n140248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138813);
	s.is_empty()
}

/// Words ending -испособленка.
pub(crate) fn n140249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138814);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оселенка.
pub(crate) fn n101081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100000);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 117523);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 128589);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 138815);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 138816);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 140910);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142127);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t52::n144263(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иленка.
pub(crate) fn n47659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47194);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 75666);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 100002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 75667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оленка.
pub(crate) fn n47660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47195);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 75668);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100003);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 117524);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128590);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 135105);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 75669);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 100004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пленка.
pub(crate) fn n47661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47196);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 75670);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 100005);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 117526);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 128591);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135106);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138817);
													match s {
														[] => true,
														['с', '\u{1}', s @ ..] => super::t52::n142356(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 100006);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117527);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128592);
									s.is_empty()
								}
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
			super::push(out, 75671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рленка.
pub(crate) fn n47662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47197);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 75672);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 100007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сленка.
pub(crate) fn n47663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47198);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75673);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 100008);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 117529);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128593);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135107);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138818);
													match s {
														[] => true,
														['р', s @ ..] => super::t52::n142357(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 75674);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 100009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 75675);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 100010);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етленка.
pub(crate) fn n76471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75676);
	s.is_empty()
}

/// Words ending -членка.
pub(crate) fn n47665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47200);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 75677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыленка.
pub(crate) fn n47666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47201);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 75678);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100011);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 117531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пол-ягненка.
pub(crate) fn n140254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138819);
	s.is_empty()
}

/// Words ending -полпоросенка.
pub(crate) fn n140255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138820);
	s.is_empty()
}

/// Words ending -балка-стенка.
pub(crate) fn n140256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138821);
	s.is_empty()
}

/// Words ending -благовещенка.
pub(crate) fn n140257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138822);
	s.is_empty()
}

/// Words ending -рытвинка.
pub(crate) fn n101205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100124);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 117603);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -увинка.
pub(crate) fn n47756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47291);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 75809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 100125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хвинка.
pub(crate) fn n47757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47292);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 75810);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 100126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьвинка.
pub(crate) fn n47758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47293);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 75811);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 100127);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явинка.
pub(crate) fn n47759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47294);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 75812);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100128);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 117605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -махачкалинка.
pub(crate) fn n140259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138824);
	s.is_empty()
}

/// Words ending -картофелинка.
pub(crate) fn n140260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138825);
	s.is_empty()
}

/// Words ending -а-шестиклинка.
pub(crate) fn n140261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138826);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140913);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 142129);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 142812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хклинка.
pub(crate) fn n76678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75880);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 100189);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 117630);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 128641);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 135128);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 138827);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 140914);
													match s {
														[] => true,
														['-', s @ ..] => super::t52::n143581(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олинка.
pub(crate) fn n47803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47337);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 75882);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 75883);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 100191);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 117631);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плинка.
pub(crate) fn n47804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47338);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 75884);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 100192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 75885);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 100193);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 117633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлинка.
pub(crate) fn n47805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47339);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 75886);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 100194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 75887);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 100195);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слинка.
pub(crate) fn n47806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75888);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 100196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 75889);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 100197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 100198);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 117635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улинка.
pub(crate) fn n47807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47341);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 75890);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100199);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 117636);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128642);
									s.is_empty()
								}
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
			super::push(out, 75891);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 100200);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 117637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 75892);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 100201);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 117638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлинка.
pub(crate) fn n47808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47342);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 75893);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 100202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылинка.
pub(crate) fn n47809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47343);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 75894);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 75895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рхнепышминка.
pub(crate) fn n140263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138828);
	s.is_empty()
}

/// Words ending -иротинка.
pub(crate) fn n101367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100286);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 117693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тотинка.
pub(crate) fn n76790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75992);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 100287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хотинка.
pub(crate) fn n76791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75993);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 100288);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 117694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьотинка.
pub(crate) fn n76792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75994);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 100289);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 117695);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртинка.
pub(crate) fn n47866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47398);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75995);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 100290);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стинка.
pub(crate) fn n47867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75996);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 100291);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 117696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 75997);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 100292);
					match s {
						[] => true,
						['а', 'п', '\u{1}', s @ ..] => {
							super::push(out, 117697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 75998);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 100293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утинка.
pub(crate) fn n47868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 75999);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 100294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 76000);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 100295);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117698);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 128667);
									s.is_empty()
								}
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
			super::push(out, 76001);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хтинка.
pub(crate) fn n47869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47401);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 76002);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 100296);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штинка.
pub(crate) fn n47870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47402);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 76003);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100297);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 117699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 76004);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 100298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 117700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятинка.
pub(crate) fn n47871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47403);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 76005);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 100299);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 117701);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 128668);
									s.is_empty()
								}
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
			super::push(out, 76006);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 100300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 76007);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 100301);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 117702);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -земляничинка.
pub(crate) fn n140264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138829);
	s.is_empty()
}

/// Words ending -червоточинка.
pub(crate) fn n140265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138830);
	s.is_empty()
}

/// Words ending -умасшедшинка.
pub(crate) fn n140266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138831);
	s.is_empty()
}

/// Words ending -шкаф-колонка.
pub(crate) fn n140267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138832);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 140915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пан-заслонка.
pub(crate) fn n140268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138833);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140916);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142131);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерхнеуслонка.
pub(crate) fn n140269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138834);
	s.is_empty()
}

/// Words ending -вифлеемлянка.
pub(crate) fn n140271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138836);
	s.is_empty()
}

/// Words ending -ица-стремянка.
pub(crate) fn n140272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138837);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 140918);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142132);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -араж-стоянка.
pub(crate) fn n140273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138838);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 140919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ка-революционерка.
pub(crate) fn n140276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138841);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140921);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142134);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142815);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 143182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ол-нью-йорка.
pub(crate) fn n140277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138842);
	s.is_empty()
}

/// Words ending -онсерваторка.
pub(crate) fn n140278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138843);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -респираторка.
pub(crate) fn n140279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138844);
	s.is_empty()
}

/// Words ending -сплуататорка.
pub(crate) fn n140280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138845);
	s.is_empty()
}

/// The word четырехлетка.
pub(crate) fn n142372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140927);
	s.is_empty()
}

/// Words ending -платье-сетка.
pub(crate) fn n140288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138853);
	s.is_empty()
}

/// Words ending -офа-красотка.
pub(crate) fn n140289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138854);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140928);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 142137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -недоброхотка.
pub(crate) fn n140290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138855);
	s.is_empty()
}

/// Words ending -арь-машинистка.
pub(crate) fn n142374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140929);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142138);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142817);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 143183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word антагонистка.
pub(crate) fn n142375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140930);
	s.is_empty()
}

/// Words ending -иллюзионистка.
pub(crate) fn n142376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140931);
	s.is_empty()
}

/// Words ending -ца-акционистка.
pub(crate) fn n142377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140932);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 142139);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142818);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 143184);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вея-мотористка.
pub(crate) fn n142378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140933);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 142140);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word девятихвостка.
pub(crate) fn n140297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138862);
	s.is_empty()
}

/// The word треххвостка.
pub(crate) fn n140298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138863);
	s.is_empty()
}

/// The word четыреххвостка.
pub(crate) fn n140299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138864);
	s.is_empty()
}

/// The word трехперстка.
pub(crate) fn n140300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138865);
	s.is_empty()
}

/// Words ending -очаровашечка.
pub(crate) fn n140304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138869);
	s.is_empty()
}

/// Words ending -мандавошечка.
pub(crate) fn n140305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138870);
	s.is_empty()
}

/// Words ending -пушечка.
pub(crate) fn n77811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76996);
	s.is_empty()
}

/// Words ending -рушечка.
pub(crate) fn n77812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76997);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 101007);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 118087);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 128845);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'а', s @ ..] => {
			super::push(out, 101008);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 118088);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 128846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушечка.
pub(crate) fn n77813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76998);
	s.is_empty()
}

/// Words ending -чушечка.
pub(crate) fn n77814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76999);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 101009);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 118089);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 128847);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышечка.
pub(crate) fn n48912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48412);
	s.is_empty()
}

/// Words ending -юшечка.
pub(crate) fn n48913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48413);
	s.is_empty()
}

/// Words ending -яшечка.
pub(crate) fn n48914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48414);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 77000);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 101010);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 118090);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 128848);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 135216);
											s.is_empty()
										}
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
					super::push(out, 101011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118091);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 128849);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 135217);
											s.is_empty()
										}
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
					super::push(out, 101012);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118092);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128850);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 118093);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йка-алкоголичка.
pub(crate) fn n140307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138872);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140934);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142141);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word рябиновочка.
pub(crate) fn n140312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138877);
	s.is_empty()
}

/// Words ending -зявочка.
pub(crate) fn n77940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77124);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 101111);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 118162);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пиявочка.
pub(crate) fn n102197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101112);
	s.is_empty()
}

/// The word малявочка.
pub(crate) fn n77942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77126);
	s.is_empty()
}

/// The word стерлядочка.
pub(crate) fn n140313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138878);
	s.is_empty()
}

/// Words ending -ровинциалочка.
pub(crate) fn n140314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138879);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 140939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ка-выручалочка.
pub(crate) fn n140315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138880);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 140940);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142145);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олочка.
pub(crate) fn n49023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48521);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 77155);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101134);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 118175);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128897);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 135240);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 138881);
													match s {
														[] => true,
														['а', s @ ..] => super::t52::n142386(s, out),
														_ => false,
													}
												}
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
							super::push(out, 118176);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 128898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 101135);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 118177);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 128899);
									s.is_empty()
								}
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
			super::push(out, 77156);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101136);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 118178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 101137);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118179);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128900);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 118180);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 118181);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 128901);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135242);
											s.is_empty()
										}
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
					super::push(out, 101138);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118182);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 128902);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 135243);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 138882);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 101139);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 118183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 77157);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 101140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 77158);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101141);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 118184);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 118185);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128903);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 101142);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 118186);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 128904);
									s.is_empty()
								}
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
			super::push(out, 77159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101143);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 101144);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 118187);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128905);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 135244);
											s.is_empty()
										}
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
					super::push(out, 101145);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128906);
									s.is_empty()
								}
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
			super::push(out, 77160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -портсменочка.
pub(crate) fn n140318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138883);
	s.is_empty()
}

/// Words ending -иночка.
pub(crate) fn n49040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48538);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77201);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 101183);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 118214);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 128919);
									s.is_empty()
								}
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
			super::push(out, 77202);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 77203);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 101184);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118215);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 128920);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 135249);
											s.is_empty()
										}
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
					super::push(out, 101185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118216);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 118217);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128921);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 135250);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 138884);
													match s {
														[] => true,
														['и', s @ ..] => super::t52::n142387(s, out),
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
									super::push(out, 128922);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 135251);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 138885);
													match s {
														[] => true,
														['в', s @ ..] => super::t52::n142388(s, out),
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
									super::push(out, 128923);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135252);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 138886);
													match s {
														[] => true,
														['о', s @ ..] => super::t52::n142389(s, out),
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
									super::push(out, 128924);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 135253);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138887);
													match s {
														[] => true,
														['м', s @ ..] => super::t52::n142390(s, out),
														_ => false,
													}
												}
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
							super::push(out, 118218);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 128925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 101186);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118219);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 128926);
									s.is_empty()
								}
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
			super::push(out, 77204);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 101187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 77205);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 101188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 77206);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 77207);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101189);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118220);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128927);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135254);
											s.is_empty()
										}
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
					super::push(out, 101190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 77208);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word панночка.
pub(crate) fn n102276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101191);
	s.is_empty()
}

/// Words ending -оночка.
pub(crate) fn n49042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48539);
	match s {
		[] => true,
		['л', 'о', 'б', s @ ..] => {
			super::push(out, 77211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 77212);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 101193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 77213);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 77214);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101195);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118222);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 128928);
									s.is_empty()
								}
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
			super::push(out, 77215);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уночка.
pub(crate) fn n49043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48540);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 77216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыночка.
pub(crate) fn n49044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48541);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 77217);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 101198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 77218);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яночка.
pub(crate) fn n49045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48542);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77219);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 101200);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 118223);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128929);
									s.is_empty()
								}
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
				['б', s @ ..] => {
					super::push(out, 101201);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 101202);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118225);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128930);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135255);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 77220);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 101203);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 118226);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 118227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 101204);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 101205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 77221);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 101206);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 77222);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101207);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 118229);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128931);
									s.is_empty()
								}
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
			super::push(out, 77223);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101208);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 118230);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 128932);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101209);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 101210);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118231);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 128933);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135256);
											s.is_empty()
										}
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
					super::push(out, 101211);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 101212);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 77224);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 101213);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118234);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-сорочка.
pub(crate) fn n140323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138888);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140946);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трочка.
pub(crate) fn n49061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48558);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 77266);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урочка.
pub(crate) fn n49062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48559);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 77267);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 101245);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 118255);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128945);
									s.is_empty()
								}
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
			super::push(out, 77268);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 77269);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 101246);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 118256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128946);
									s.is_empty()
								}
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
			super::push(out, 77270);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 101247);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 77271);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101248);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 101249);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 101250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырочка.
pub(crate) fn n49063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48560);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 77272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юрочка.
pub(crate) fn n49064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48561);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 77273);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 77274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101252);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ярочка.
pub(crate) fn n49065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48562);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 77275);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 77276);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101253);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 118258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 77277);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101254);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мфеточка.
pub(crate) fn n102365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101278);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 118276);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 128957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 135264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иточка.
pub(crate) fn n49078(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77313);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 101279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 77314);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 77315);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 101280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нточка.
pub(crate) fn n49079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48575);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77316);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 101281);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 118279);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128958);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 135265);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 77317);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101282);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 118280);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128959);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135266);
											s.is_empty()
										}
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
					super::push(out, 101283);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118281);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 128960);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 135267);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138889);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 101284);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 101285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оточка.
pub(crate) fn n49080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48576);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 77318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 77319);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101287);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118282);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128961);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101288);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 101289);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 77320);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 101290);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 118285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 77321);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 101291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 77322);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 101292);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 101293);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 77323);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101294);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118288);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128963);
									s.is_empty()
								}
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
			super::push(out, 77324);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 101295);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 118289);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128964);
									s.is_empty()
								}
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
			super::push(out, 77325);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 77326);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101296);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рточка.
pub(crate) fn n49081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48577);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77327);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 77328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сточка.
pub(crate) fn n49082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48578);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77329);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 101297);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 101298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118291);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 77330);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 101299);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 101300);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 101301);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 118293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 77331);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 101302);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118294);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 128966);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 135268);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 138890);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 101303);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 101304);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118295);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 128967);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135269);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 138891);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 77332);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101305);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 101306);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 101307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 77333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тточка.
pub(crate) fn n49083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48579);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 77334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уточка.
pub(crate) fn n49084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48580);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 77335);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 77336);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 101309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 77337);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 101310);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 118296);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128968);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135270);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138892);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 77338);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фточка.
pub(crate) fn n49085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48581);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 77339);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыточка.
pub(crate) fn n49087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48583);
	s.is_empty()
}

/// Words ending -юточка.
pub(crate) fn n49088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48584);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77340);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101311);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яточка.
pub(crate) fn n49089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48585);
	s.is_empty()
}

/// Words ending -енка-монашка.
pub(crate) fn n140331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138896);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140950);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оп-черепашка.
pub(crate) fn n140332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138897);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140951);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142152);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -секретаришка.
pub(crate) fn n140335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138900);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 140952);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ефрейторишка.
pub(crate) fn n140336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138901);
	s.is_empty()
}

/// Words ending -эмигрантишка.
pub(crate) fn n140337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138902);
	s.is_empty()
}

/// Words ending -оммунистишка.
pub(crate) fn n140338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138903);
	s.is_empty()
}

/// Words ending -адовушка.
pub(crate) fn n102619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101532);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 118429);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 129029);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вдовушка.
pub(crate) fn n102620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101533);
	s.is_empty()
}

/// Words ending -ковушка.
pub(crate) fn n78457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77632);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 101534);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 118430);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 129030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 101535);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 118431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 101536);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 118432);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 118433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 101537);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 118434);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 129032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловушка.
pub(crate) fn n78458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77633);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 101538);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 101539);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 118435);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 129033);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 135302);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 138906);
											s.is_empty()
										}
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
					super::push(out, 118436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 129034);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 135303);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 138907);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140955);
													match s {
														[] => true,
														['в', s @ ..] => super::t52::n143605(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 118437);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 129035);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135304);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 138908);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140956);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 101540);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 118438);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 118439);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 118440);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 129036);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135305);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 138909);
											s.is_empty()
										}
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
					super::push(out, 118441);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повушка.
pub(crate) fn n78459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77634);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 101541);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 118442);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129037);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 135306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровушка.
pub(crate) fn n78460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77635);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 101542);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 118443);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 129038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 101543);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 118444);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129039);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 135307);
									s.is_empty()
								}
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
			super::push(out, 101544);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 118445);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совушка.
pub(crate) fn n78461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77636);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 101545);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховушка.
pub(crate) fn n78462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77637);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 101546);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 118446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --раскладушка.
pub(crate) fn n140345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138910);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140957);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142155);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 142825);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143185);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 143369);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пердушка.
pub(crate) fn n102657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101570);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 118466);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 118467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 129047);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 135309);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 138911);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 140958);
											match s {
												[] => true,
												['д', 'о', 'п', '\u{1}', s @ ..] => {
													super::push(out, 142156);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отдушка.
pub(crate) fn n78492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77667);
	s.is_empty()
}

/// Words ending -удушка.
pub(crate) fn n49387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48882);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 77668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядушка.
pub(crate) fn n49388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48883);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77669);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 101571);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 118468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 77670);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 101572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шка-квакушка.
pub(crate) fn n140347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138912);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 140959);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -араж-ракушка.
pub(crate) fn n140348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138913);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 140960);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word сиротинушка.
pub(crate) fn n140349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138914);
	s.is_empty()
}

/// Words ending -холостинушка.
pub(crate) fn n140350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138915);
	s.is_empty()
}

/// Words ending -оронушка.
pub(crate) fn n102717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101630);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 118497);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 118498);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 129062);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнушка.
pub(crate) fn n49430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48925);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77737);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 101631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 77738);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 101632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 77739);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 101633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 77740);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снушка.
pub(crate) fn n49431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48926);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 77741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнушка.
pub(crate) fn n49432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48927);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 77742);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 101636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янушка.
pub(crate) fn n49433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48928);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 77743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аубица-пушка.
pub(crate) fn n140351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138916);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 140961);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ижка-игрушка.
pub(crate) fn n140352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138917);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 140962);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142158);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышка-норушка.
pub(crate) fn n140353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138918);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 140963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еструшка.
pub(crate) fn n102758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101671);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 118516);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 129071);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 135320);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 138919);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140964);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142159);
													match s {
														[] => true,
														['л', s @ ..] => super::t52::n144279(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 118517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урушка.
pub(crate) fn n49451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48946);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 77782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сло-вертушка.
pub(crate) fn n140355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138920);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140965);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142160);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стушка.
pub(crate) fn n49461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77807);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 101695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 101696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 77808);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101697);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118534);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129079);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135323);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 118535);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129080);
									s.is_empty()
								}
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
			super::push(out, 77809);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101698);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 77810);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101699);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 118537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 77811);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 101700);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 118538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 77812);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 101701);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятушка.
pub(crate) fn n49463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48958);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 77814);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101703);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -царь-батюшка.
pub(crate) fn n140358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138923);
	s.is_empty()
}

/// Words ending -тенька.
pub(crate) fn n49656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49147);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78026);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 101865);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 101866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 101867);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 118631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 101868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 78027);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 101869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 78028);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101870);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 101871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 78029);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101872);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 118632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 78030);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 101873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченька.
pub(crate) fn n49658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49149);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78031);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 101874);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118633);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 101875);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118634);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129127);
									s.is_empty()
								}
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
			super::push(out, 78032);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101876);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 101877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 101878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 78033);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101879);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 101880);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 101881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 78034);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 101882);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 118637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 101883);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенька.
pub(crate) fn n49659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49150);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78035);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101884);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 101885);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 118639);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 129128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 101886);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 101887);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118641);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129129);
									s.is_empty()
								}
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
			super::push(out, 78036);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 101888);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 118642);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 129130);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135340);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 78037);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 101889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 78038);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 101890);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 78039);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ещенька.
pub(crate) fn n78867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78040);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 101892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -акула-нянька.
pub(crate) fn n140362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138927);
	s.is_empty()
}

/// Words ending -страхолюдина.
pub(crate) fn n140370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138935);
	s.is_empty()
}

/// Words ending -ина-говядина.
pub(crate) fn n140371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138936);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 140972);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142163);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ол-апельсина.
pub(crate) fn n140374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138939);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 140973);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -примитивщина.
pub(crate) fn n140375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138940);
	s.is_empty()
}

/// Words ending -убъективщина.
pub(crate) fn n140376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138941);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 140974);
			s.is_empty()
		}
		_ => false,
	}
}

