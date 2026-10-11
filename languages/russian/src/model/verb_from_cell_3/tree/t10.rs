use flexion::Path;

/// Words ending -осишься.
pub(crate) fn n8039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7695);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14814);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21903);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21904);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21905);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21906);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21907);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 21908);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21909);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27358);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27359);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27360);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21910);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27361);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21911);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21912);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21913);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14815);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21914);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27364);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27365);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30894);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30895);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 30896);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30897);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32691);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14816);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21915);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21916);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 21917);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 21918);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 21919);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27367);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27368);
							match s {
								[] => true,
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 30898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'в', 'е', s @ ..] => {
							super::push(out, 27369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 21920);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21921);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27371);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21922);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21923);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21924);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 21925);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21926);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21927);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27373);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27374);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27375);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32692);
											s.is_empty()
										}
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
					super::push(out, 21928);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30901);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30902);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 30903);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 32693);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33326);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33327);
													s.is_empty()
												}
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
							super::push(out, 27377);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30904);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32694);
											s.is_empty()
										}
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
					super::push(out, 21929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27378);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27379);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27380);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27381);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30905);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30906);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27382);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 27383);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27384);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30907);
									s.is_empty()
								}
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
			super::push(out, 14818);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 21930);
					match s {
						[] => true,
						['л', 'ы', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30908);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усишься.
pub(crate) fn n8041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7697);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14821);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21932);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21933);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21934);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27389);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21935);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21936);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 21937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14822);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21938);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27392);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 21939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27394);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27395);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27396);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30910);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32695);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27397);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27398);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рысишься.
pub(crate) fn n15638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14824);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27399);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27400);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эсишься.
pub(crate) fn n8043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7699);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14825);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 21943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обвиноватишься.
pub(crate) fn n35324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33328);
	s.is_empty()
}

/// Words ending -матишься.
pub(crate) fn n15645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14831);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21959);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 21960);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30921);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 30922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 21961);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30923);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 30924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 21962);
			match s {
				[] => true,
				['о', 'л', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 27417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мохнатишься.
pub(crate) fn n29069(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 30925);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 30926);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 30927);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -патишься.
pub(crate) fn n15647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14833);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21964);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 27418);
					s.is_empty()
				}
				['н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30928);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 30929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратишься.
pub(crate) fn n15648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14834);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 21965);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 21966);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 21967);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27420);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30930);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 32698);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 30931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 21968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27421);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27422);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30932);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 30933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хатишься.
pub(crate) fn n15649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14835);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 21969);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 21970);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 21971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чатишься.
pub(crate) fn n15650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14836);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21972);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 21973);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гтишься.
pub(crate) fn n8045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7701);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етишься.
pub(crate) fn n8046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7702);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14838);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27425);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27426);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 27427);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27428);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 30935);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32699);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 30936);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32700);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 30937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27429);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27430);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 21975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27431);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27432);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27433);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30939);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30940);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32701);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27434);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30941);
									s.is_empty()
								}
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
			super::push(out, 14839);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 21976);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 21977);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14840);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21978);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 21979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14841);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21980);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 27436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21981);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21982);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 21983);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21984);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14842);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14843);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 21986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27439);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27440);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 14844);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21987);
					match s {
						[] => true,
						['р', 'а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30943);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30944);
									s.is_empty()
								}
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
			super::push(out, 14845);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27442);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 27443);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27444);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 30946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итишься.
pub(crate) fn n8047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7703);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 14846);
			match s {
				[] => true,
				['о', 'л', 'о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27445);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14847);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14848);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21989);
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 27447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14849);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 21990);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 21991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27449);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 14850);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йтишься.
pub(crate) fn n8048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7704);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14851);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтишься.
pub(crate) fn n8049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7705);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14852);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27451);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27452);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27453);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27454);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27455);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30948);
									s.is_empty()
								}
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
			super::push(out, 14853);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтишься.
pub(crate) fn n8050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7706);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14854);
			match s {
				[] => true,
				['р', 'ф', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27456);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27457);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27458);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30949);
									s.is_empty()
								}
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
			super::push(out, 14855);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 21996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14856);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27459);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27460);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27461);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 21997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отишься.
pub(crate) fn n8051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7707);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21998);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27462);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30951);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27463);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30953);
									s.is_empty()
								}
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
			super::push(out, 14858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 21999);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 14859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22000);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 22001);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 22002);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30954);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30955);
									s.is_empty()
								}
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
			super::push(out, 14860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22003);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22004);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27466);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 30956);
									s.is_empty()
								}
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
			super::push(out, 14861);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22005);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30957);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 30958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22006);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27467);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30959);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32702);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 27468);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30960);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30961);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30962);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30963);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30964);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32703);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30965);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30966);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30967);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30968);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32704);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 27470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30969);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 30970);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32705);
											s.is_empty()
										}
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
					super::push(out, 22007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27471);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27472);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14862);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 22008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14863);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22009);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22010);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27475);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30971);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33329);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33330);
													s.is_empty()
												}
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
							super::push(out, 27476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22011);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27477);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30972);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 30973);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 32706);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 30974);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32707);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27478);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30975);
									s.is_empty()
								}
								['о', s @ ..] => super::t11::n32853(s, out),
								['у', s @ ..] => super::t11::n32854(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t11::n29137(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => super::t11::n15678(s, out),
		_ => false,
	}
}

/// Words ending -птишься.
pub(crate) fn n8052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7708);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14865);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27483);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27484);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27485);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртишься.
pub(crate) fn n8053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7709);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14866);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27486);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27487);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27488);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27489);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22015);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27490);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 30981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 22016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27491);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27492);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27493);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30982);
									s.is_empty()
								}
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
			super::push(out, 14867);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27494);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27495);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27496);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стишься.
pub(crate) fn n8054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14868);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22018);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 22019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27498);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27499);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 30984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 27500);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 27501);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27502);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30987);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22020);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 22021);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27504);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27506);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 27507);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27508);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 22023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14869);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22024);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27509);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 27510);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27511);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30991);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32709);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32710);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 33333);
													s.is_empty()
												}
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
							super::push(out, 27512);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27513);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30992);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32711);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 30993);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32712);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32713);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32714);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33334);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 32715);
											s.is_empty()
										}
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
					super::push(out, 22025);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 22026);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27514);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 27515);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27516);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27517);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22027);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27518);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 30995);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 32716);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33335);
													s.is_empty()
												}
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
							super::push(out, 27519);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30996);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 30997);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 30998);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30999);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31000);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31001);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31002);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32717);
											s.is_empty()
										}
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
					super::push(out, 22028);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27520);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31003);
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33336);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33337);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n15684(s, out),
		['к', s @ ..] => {
			super::push(out, 14871);
			match s {
				[] => true,
				['о', s @ ..] => super::t11::n23307(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t11::n15686(s, out),
		['м', s @ ..] => super::t11::n15687(s, out),
		['о', s @ ..] => super::t11::n15688(s, out),
		['р', s @ ..] => super::t11::n15689(s, out),
		['у', s @ ..] => super::t11::n15690(s, out),
		['ы', s @ ..] => super::t11::n15691(s, out),
		['ь', s @ ..] => super::t11::n15692(s, out),
		_ => false,
	}
}

/// Words ending -утишься.
pub(crate) fn n8055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7711);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14879);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22049);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 27556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14880);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22050);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22051);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14881);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22052);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22053);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27558);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 27559);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31025);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											super::push(out, 32724);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 27560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22054);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22055);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22056);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14882);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22058);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14883);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27562);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27563);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 27564);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31026);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 32725);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27565);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27566);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 27567);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31027);
									s.is_empty()
								}
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
			super::push(out, 14884);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 14885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22060);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22061);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22062);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22063);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 14886);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22064);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтишься.
pub(crate) fn n8056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7712);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14887);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14888);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 22066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14889);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чтишься.
pub(crate) fn n8057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7713);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14890);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14891);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытишься.
pub(crate) fn n8058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7714);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 14892);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 22068);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14893);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22069);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14894);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22071);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22072);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22073);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -этишься.
pub(crate) fn n8059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7715);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 14896);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ютишься.
pub(crate) fn n8060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7716);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14897);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14898);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 14899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятишься.
pub(crate) fn n8061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7717);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14900);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27571);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 14901);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22076);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22077);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 22078);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22079);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31028);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31029);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31030);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31031);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31032);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32726);
											s.is_empty()
										}
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
					super::push(out, 22080);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22081);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22082);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22083);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичишься.
pub(crate) fn n8070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7726);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14922);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22114);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22115);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 14924);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22118);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14926);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22120);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 22121);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22122);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27602);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 27603);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22123);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22124);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27604);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22125);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22126);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14927);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22128);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 27606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14928);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22129);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31047);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31048);
									s.is_empty()
								}
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
			super::push(out, 14929);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22130);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14930);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22131);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27608);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мчишься.
pub(crate) fn n8072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7728);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14932);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 14933);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14934);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 14935);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 14936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчишься.
pub(crate) fn n8073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7729);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14937);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22133);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27609);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27610);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', s @ ..] => {
							super::push(out, 27611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27612);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27613);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14938);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22135);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27616);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27617);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27618);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31052);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31053);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31054);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27619);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очишься.
pub(crate) fn n8074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7730);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22137);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22138);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22139);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22140);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22141);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27622);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32727);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32728);
											s.is_empty()
										}
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
					super::push(out, 22142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14940);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 22143);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14941);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22144);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22145);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14942);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22146);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27626);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 27627);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 31056);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32729);
											s.is_empty()
										}
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
					super::push(out, 22147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27628);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22148);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27630);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31057);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31058);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32730);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 31059);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31060);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31061);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32731);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 32732);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31062);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32733);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31063);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31064);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31065);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31066);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31067);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32734);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 27631);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 31068);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 32735);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14943);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22149);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22150);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22151);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22152);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22153);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27633);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 27634);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22155);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27636);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22156);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22157);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22158);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22159);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14944);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27638);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27639);
							match s {
								[] => true,
								['р', s @ ..] => super::t11::n32948(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => super::t11::n29303(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n23443(s, out),
				['п', s @ ..] => super::t11::n23444(s, out),
				['с', s @ ..] => super::t11::n23445(s, out),
				['т', s @ ..] => super::t11::n23446(s, out),
				['у', s @ ..] => super::t11::n23447(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t11::n15760(s, out),
		['т', s @ ..] => super::t11::n15761(s, out),
		_ => false,
	}
}

/// Words ending -рчишься.
pub(crate) fn n8075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7731);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14947);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27658);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14948);
			match s {
				[] => true,
				['п', 'о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27660);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27661);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31089);
									s.is_empty()
								}
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
			super::push(out, 14949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22181);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 22182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27662);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27664);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27665);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27666);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учишься.
pub(crate) fn n8077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7733);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14951);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14952);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22186);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22187);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22188);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27668);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 31092);
									s.is_empty()
								}
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
			super::push(out, 14954);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14955);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14956);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 14957);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22192);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14958);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22195);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22197);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22198);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22199);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22200);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22201);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22202);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22203);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22204);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14961);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22206);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22207);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22208);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22209);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22210);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27671);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14962);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 14963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22211);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22212);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22213);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22214);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14964);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22215);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22216);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 22217);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22218);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22219);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22220);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22221);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27674);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 31093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22222);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14965);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22223);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22224);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 22225);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22226);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22227);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22228);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22229);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22230);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14966);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22231);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22232);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27678);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 14967);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22234);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27680);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычишься.
pub(crate) fn n8078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7734);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 14968);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22235);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 14969);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27681);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 14970);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьчишься.
pub(crate) fn n8079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7735);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14971);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27683);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 27684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14972);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 22238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючишься.
pub(crate) fn n8080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7736);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14973);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22239);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27685);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22240);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27686);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31096);
									s.is_empty()
								}
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
			super::push(out, 14974);
			match s {
				[] => true,
				['д', 'з', s @ ..] => {
					super::push(out, 22241);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'с', s @ ..] => {
					super::push(out, 22242);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14975);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27688);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячишься.
pub(crate) fn n8081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14976);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27690);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27691);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 27692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 14977);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14978);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 14979);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 14980);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14981);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31097);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31098);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31099);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 27694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 27695);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31101);
									s.is_empty()
								}
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
			super::push(out, 14982);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 14983);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -копошишься.
pub(crate) fn n29376(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31108);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 31109);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 31110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рошишься.
pub(crate) fn n15817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15002);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22268);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27711);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 27712);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 27713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22269);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 27714);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31111);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22270);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31112);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 31113);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32745);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 31114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 27716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31116);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 27718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31118);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 31119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22271);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27719);
					match s {
						[] => true,
						['п', 'ы', s @ ..] => {
							super::push(out, 31120);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тошишься.
pub(crate) fn n15818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15003);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22272);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22273);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27721);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22274);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27722);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ршишься.
pub(crate) fn n8088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7744);
	match s {
		[] => true,
		['а', 'б', 'е', 'ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22275);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22276);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22278);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22279);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 22280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27723);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22281);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15005);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22282);
					match s {
						[] => true,
						['е', 'ш', 'о', s @ ..] => {
							super::push(out, 27726);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22283);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ушишься.
pub(crate) fn n8089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7745);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15006);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15007);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22285);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22286);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22287);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27728);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27729);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15008);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22289);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27730);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27731);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27732);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22290);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31125);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31126);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31127);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15009);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15010);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22292);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22293);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22294);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31128);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15011);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22296);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22297);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22298);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22299);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27734);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27735);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31130);
									s.is_empty()
								}
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
			super::push(out, 15012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22301);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22302);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22303);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22304);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22305);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22306);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22307);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22308);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22309);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31131);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 27738);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22310);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22311);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27739);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27740);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22312);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22313);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чшишься.
pub(crate) fn n8090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7746);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 15014);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22314);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышишься.
pub(crate) fn n8091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7747);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15015);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 15016);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27742);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27743);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27744);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31134);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31135);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 27745);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15017);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22316);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 27747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьшишься.
pub(crate) fn n8092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7748);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15018);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22317);
					match s {
						[] => true,
						['м', 'у', 'о', s @ ..] => {
							super::push(out, 27748);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22318);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 27749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эшишься.
pub(crate) fn n8093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7749);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15019);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яшишься.
pub(crate) fn n8094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7750);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 15020);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порщишься.
pub(crate) fn n23633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22342);
	match s {
		[] => true,
		['о', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31149);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31150);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31151);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 31152);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32753);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 32754);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отщишься.
pub(crate) fn n15849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15033);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 22343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ущишься.
pub(crate) fn n8103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7759);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22344);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22345);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22346);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыщишься.
pub(crate) fn n8104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7760);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ющишься.
pub(crate) fn n8105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7761);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15036);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27766);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -навыдергиваешь.
pub(crate) fn n35335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33339);
	s.is_empty()
}

/// Words ending -рыгиваешь.
pub(crate) fn n16004(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22427);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 22428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27805);
					s.is_empty()
				}
				['а', 'н', s @ ..] => {
					super::push(out, 27806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27807);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27808);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27809);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27810);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31165);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 27811);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 31166);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 32759);
									s.is_empty()
								}
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
			super::push(out, 22429);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 22430);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27812);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягиваешь.
pub(crate) fn n8241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7884);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15173);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22431);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15174);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22432);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22433);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 27815);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изживаешь.
pub(crate) fn n16025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15193);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 22457);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иживаешь.
pub(crate) fn n8249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7892);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15194);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15195);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22459);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22460);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 27835);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27836);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22461);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лживаешь.
pub(crate) fn n8250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7893);
	match s {
		[] => true,
		['а', 'д', 'о', 'а', s @ ..] => {
			super::push(out, 15196);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оживаешь.
pub(crate) fn n8251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7894);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15197);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 15198);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15199);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 15200);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 15201);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 15202);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22463);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 22464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рживаешь.
pub(crate) fn n8252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7895);
	s.is_empty()
}

/// Words ending -сживаешь.
pub(crate) fn n8253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7896);
	s.is_empty()
}

/// Words ending -тживаешь.
pub(crate) fn n8254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7897);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15204);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22465);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уживаешь.
pub(crate) fn n8255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7898);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15205);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22466);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15206);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27840);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', s @ ..] => {
							super::push(out, 27841);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 27842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15207);
			s.is_empty()
		}
		['с', 'д', s @ ..] => {
			super::push(out, 15208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22467);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыживаешь.
pub(crate) fn n8256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7899);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15210);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 15211);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15212);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 15213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юживаешь.
pub(crate) fn n8257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7900);
	s.is_empty()
}

/// Words ending -яживаешь.
pub(crate) fn n8258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7901);
	s.is_empty()
}

/// Words ending -повыскакиваешь.
pub(crate) fn n35336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33340);
	s.is_empty()
}

/// Words ending -лкиваешь.
pub(crate) fn n8264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7907);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22480);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27864);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31189);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32769);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'щ', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22482);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22483);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окиваешь.
pub(crate) fn n8266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7909);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 15228);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 22485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15229);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 15230);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22486);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 27867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркиваешь.
pub(crate) fn n8267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7910);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15231);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22488);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27868);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22489);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 22490);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 22491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27869);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 27870);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27871);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 27872);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31192);
									s.is_empty()
								}
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
				['в', s @ ..] => {
					super::push(out, 22492);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27873);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27874);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32770);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 32771);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31193);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 22493);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27875);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 31195);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32772);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', 'ф', s @ ..] => {
			super::push(out, 15232);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22494);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скиваешь.
pub(crate) fn n8268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7911);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15233);
			match s {
				[] => true,
				['т', 'ы', s @ ..] => {
					super::push(out, 22495);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 27877);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31196);
									s.is_empty()
								}
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
			super::push(out, 15234);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 27878);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31197);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32773);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 32774);
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 32775);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 27879);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31199);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32776);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33341);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'т', 'о', s @ ..] => {
					super::push(out, 22496);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27880);
							s.is_empty()
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
				['а', s @ ..] => {
					super::push(out, 22497);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22498);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27882);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 15235);
			match s {
				[] => true,
				['р', 'п', 'в', s @ ..] => {
					super::push(out, 22500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -укиваешь.
pub(crate) fn n8269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7912);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15236);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'с', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22502);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22503);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27883);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 27884);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкиваешь.
pub(crate) fn n8271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7914);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15238);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['з', 'в', s @ ..] => {
					super::push(out, 22504);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22505);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 22506);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'х', s @ ..] => {
			super::push(out, 15239);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15240);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 15241);
			s.is_empty()
		}
		['ц', 'о', s @ ..] => {
			super::push(out, 15242);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22508);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ькиваешь.
pub(crate) fn n8272(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', 'у', 'б', 'о', 'п', s @ ..] => {
			super::push(out, 15243);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'е', 'р', 'т', 'а', s @ ..] => {
			super::push(out, 15244);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юкиваешь.
pub(crate) fn n8273(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15246);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15247);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'ю', 'с', 'и', s @ ..] => {
			super::push(out, 15248);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -якиваешь.
pub(crate) fn n8274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7915);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15250);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27887);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 27888);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27889);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15251);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27891);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27892);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 27893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27894);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 27895);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 27896);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 27897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15252);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 15253);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22513);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оздоравливаешь.
pub(crate) fn n35338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33342);
	s.is_empty()
}

/// Words ending -ыздоравливаешь.
pub(crate) fn n35339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33343);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 33545);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -доливаешь.
pub(crate) fn n16120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15279);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22541);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 22542);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27909);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -роливаешь.
pub(crate) fn n16123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15282);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 22543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -соливаешь.
pub(crate) fn n16124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15283);
	match s {
		[] => true,
		['у', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27910);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27911);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31206);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пливаешь.
pub(crate) fn n8285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7926);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15285);
			match s {
				[] => true,
				['т', 'в', s @ ..] => {
					super::push(out, 22544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15286);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 31207);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32779);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', 'о', s @ ..] => {
									super::push(out, 31208);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32780);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'о', 'о', s @ ..] => {
							super::push(out, 27913);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', 'о', s @ ..] => {
							super::push(out, 27914);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 27915);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', 'о', s @ ..] => {
							super::push(out, 27916);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31212);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', 'о', s @ ..] => {
							super::push(out, 27917);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'о', s @ ..] => {
							super::push(out, 27918);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', 'о', s @ ..] => {
							super::push(out, 27919);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'в', 'о', 'п', s @ ..] => {
							super::push(out, 27920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15287);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 15288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рливаешь.
pub(crate) fn n8286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7927);
	s.is_empty()
}

/// Words ending -сливаешь.
pub(crate) fn n8287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7928);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15289);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 15290);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 15291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тливаешь.
pub(crate) fn n8288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7929);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15292);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уливаешь.
pub(crate) fn n8289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7930);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15294);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 15295);
			match s {
				[] => true,
				['а', 'з', s @ ..] => {
					super::push(out, 22545);
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					super::push(out, 22546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15296);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22547);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', '\u{1}', s @ ..] => {
			super::push(out, 15297);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шливаешь.
pub(crate) fn n8290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7931);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22548);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22549);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27924);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 27925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22550);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 22551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыливаешь.
pub(crate) fn n8291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7932);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15298);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22552);
					match s {
						[] => true,
						['к', 'и', s @ ..] => {
							super::push(out, 27926);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31216);
									s.is_empty()
								}
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
			super::push(out, 15299);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 15300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юливаешь.
pub(crate) fn n8292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7933);
	match s {
		[] => true,
		['д', 'з', 'и', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22553);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 27927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 15301);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 22555);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яливаешь.
pub(crate) fn n8293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7934);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15302);
			s.is_empty()
		}
		['п', 'ы', s @ ..] => {
			super::push(out, 15303);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 22556);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дсматриваешь.
pub(crate) fn n23913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22598);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27952);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 27953);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етриваешь.
pub(crate) fn n16198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15351);
	s.is_empty()
}

/// Words ending -уриваешь.
pub(crate) fn n8317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7958);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15352);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15353);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22600);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 15354);
			match s {
				[] => true,
				['е', 'д', 'д', s @ ..] => {
					super::push(out, 22601);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15355);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'о', s @ ..] => {
			super::push(out, 15356);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22603);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 27957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15357);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22604);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 22605);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27958);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 31231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22606);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27959);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хриваешь.
pub(crate) fn n8318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7959);
	match s {
		[] => true,
		['и', 'в', 'а', s @ ..] => {
			super::push(out, 15358);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22607);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыриваешь.
pub(crate) fn n8319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7960);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15359);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 15360);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22608);
					s.is_empty()
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27960);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 27961);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22609);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15361);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 22610);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 27963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юриваешь.
pub(crate) fn n8320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7961);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22611);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22612);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яриваешь.
pub(crate) fn n8321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7962);
	s.is_empty()
}

/// Words ending -учиваешь.
pub(crate) fn n8338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7978);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15386);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15387);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22661);
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					super::push(out, 22662);
					s.is_empty()
				}
				['с', 'а', s @ ..] => {
					super::push(out, 22663);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15388);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15389);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 15390);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 15391);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22664);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 27998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22665);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27999);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28000);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порасспрашиваешь.
pub(crate) fn n35340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33344);
	s.is_empty()
}

/// Words ending -повыспрашиваешь.
pub(crate) fn n35341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33345);
	s.is_empty()
}

/// Words ending -еторговываешь.
pub(crate) fn n33148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31266);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 32791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иторговываешь.
pub(crate) fn n33149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31267);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 32792);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оторговываешь.
pub(crate) fn n33150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31268);
	s.is_empty()
}

/// Words ending -сторговываешь.
pub(crate) fn n33151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31269);
	s.is_empty()
}

/// Words ending -довываешь.
pub(crate) fn n16315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15458);
	s.is_empty()
}

/// Words ending -зовываешь.
pub(crate) fn n16316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15459);
	s.is_empty()
}

/// Words ending -ковываешь.
pub(crate) fn n16317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15460);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22709);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28026);
					match s {
						[] => true,
						['ш', 'о', s @ ..] => {
							super::push(out, 31270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловываешь.
pub(crate) fn n16318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15461);
	match s {
		[] => true,
		['а', 'ж', s @ ..] => {
			super::push(out, 22710);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28027);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еновываешь.
pub(crate) fn n16319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15462);
	match s {
		[] => true,
		['м', 'а', 'з', s @ ..] => {
			super::push(out, 22711);
			match s {
				[] => true,
				['к', 'э', 'о', 'д', s @ ..] => {
					super::push(out, 28028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повываешь.
pub(crate) fn n16320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15463);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 22712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровываешь.
pub(crate) fn n16321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15464);
	s.is_empty()
}

/// Words ending -совываешь.
pub(crate) fn n16322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15465);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22713);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 22714);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 22715);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28029);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22716);
			match s {
				[] => true,
				['л', 'о', 'п', 'е', s @ ..] => {
					super::push(out, 28030);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 22717);
			match s {
				[] => true,
				['в', 'о', s @ ..] => {
					super::push(out, 28031);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товываешь.
pub(crate) fn n16323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15466);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 22718);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28032);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 31276);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 32794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 31277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 28033);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховываешь.
pub(crate) fn n16324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15467);
	match s {
		[] => true,
		['а', 'р', 'т', 'с', 'д', s @ ..] => {
			super::push(out, 22719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цовываешь.
pub(crate) fn n16325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15468);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 22720);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 31278);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 32795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 31279);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 31280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 22721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -твываешь.
pub(crate) fn n8388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8028);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -истываешь.
pub(crate) fn n16454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15587);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22840);
			match s {
				[] => true,
				['с', 'а', s @ ..] => {
					super::push(out, 28132);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22841);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28133);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остываешь.
pub(crate) fn n16455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15588);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 22842);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22843);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 28135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устываешь.
pub(crate) fn n16457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15590);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 22844);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 28136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыстываешь.
pub(crate) fn n16458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15591);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22845);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 22846);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 31328);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 31329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хтываешь.
pub(crate) fn n8472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8111);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15592);
			s.is_empty()
		}
		['я', 'р', 'к', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22847);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22848);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытываешь.
pub(crate) fn n8473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8112);
	match s {
		[] => true,
		['п', 'с', 'а', s @ ..] => {
			super::push(out, 15593);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22849);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жадничаешь.
pub(crate) fn n24605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23266);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28240);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 28241);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28242);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31348);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 28243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -осадничаешь.
pub(crate) fn n29933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28245);
	s.is_empty()
}

/// Words ending -едничаешь.
pub(crate) fn n17403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16492);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 23269);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 28246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23270);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 23271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28247);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23272);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -здничаешь.
pub(crate) fn n17404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16493);
	s.is_empty()
}

/// Words ending -идничаешь.
pub(crate) fn n17405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16494);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23273);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 23274);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 23275);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31352);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 31353);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 32810);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 32811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', s @ ..] => {
							super::push(out, 31354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдничаешь.
pub(crate) fn n17406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16495);
	match s {
		[] => true,
		['о', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28248);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28249);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одничаешь.
pub(crate) fn n17407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16496);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23276);
			match s {
				[] => true,
				['о', 'х', 'е', 'р', 'г', 'а', s @ ..] => {
					super::push(out, 28251);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'о', s @ ..] => {
					super::push(out, 28252);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 31356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 23277);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28253);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 28254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23278);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 28255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28256);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 28257);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 28258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23280);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28259);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 28260);
					match s {
						[] => true,
						['с', 'а', 'м', 'у', 'с', 'а', s @ ..] => {
							super::push(out, 31358);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28261);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 23281);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28263);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдничаешь.
pub(crate) fn n17408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16497);
	s.is_empty()
}

/// Words ending -удничаешь.
pub(crate) fn n17409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16498);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 23282);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28264);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28265);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23283);
			match s {
				[] => true,
				['т', 'о', 'с', 'о', s @ ..] => {
					super::push(out, 28266);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдничаешь.
pub(crate) fn n17410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16499);
	s.is_empty()
}

/// Words ending -юдничаешь.
pub(crate) fn n17411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16500);
	s.is_empty()
}

/// Words ending -ядничаешь.
pub(crate) fn n17412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16501);
	s.is_empty()
}

/// Words ending -еничаешь.
pub(crate) fn n9608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9182);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16502);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23284);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жничаешь.
pub(crate) fn n9609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9183);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28267);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28268);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28269);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23286);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31363);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31364);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16503);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23288);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23289);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28271);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28272);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', 'е', 'н', 'с', s @ ..] => {
							super::push(out, 28274);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 28275);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31366);
									match s {
										[] => true,
										['е', 'п', 'ы', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33346);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33347);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16504);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16505);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23293);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23294);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23295);
					match s {
						[] => true,
						['о', 'т', 'с', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31367);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31368);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31369);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31370);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16506);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23296);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16507);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 16508);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23298);
					match s {
						[] => true,
						['о', 'р', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31371);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31372);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23299);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23300);
					s.is_empty()
				}
				['т', 'у', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', 'р', s @ ..] => {
							super::push(out, 28279);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28280);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зничаешь.
pub(crate) fn n9610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9184);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16509);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23301);
					match s {
						[] => true,
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31376);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23302);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23303);
					match s {
						[] => true,
						['б', 'о', 'з', 'е', 'б', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31378);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31379);
									s.is_empty()
								}
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
			super::push(out, 16510);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23304);
					match s {
						[] => true,
						['ю', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31380);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23306);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28283);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 23307);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16511);
			match s {
				[] => true,
				['р', 'п', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28285);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16512);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23308);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 23309);
					s.is_empty()
				}
				['и', 'ц', 'а', s @ ..] => {
					super::push(out, 23310);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16513);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23311);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28289);
							match s {
								[] => true,
								['а', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32813);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32814);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 28290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16514);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 23312);
					match s {
						[] => true,
						['л', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31383);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31384);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31385);
									s.is_empty()
								}
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
			super::push(out, 16515);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23313);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23314);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 23315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 16516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йничаешь.
pub(crate) fn n9611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9185);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16517);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 23316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28292);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16518);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23318);
					match s {
						[] => true,
						['о', 'х', s @ ..] => {
							super::push(out, 28294);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23319);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 23320);
					match s {
						[] => true,
						['о', 'к', 'о', s @ ..] => {
							super::push(out, 28295);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31387);
									s.is_empty()
								}
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
			super::push(out, 16519);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23321);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28296);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 16520);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23323);
					match s {
						[] => true,
						['з', 'и', 'п', 'с', 'а', 'р', 'о', s @ ..] => {
							super::push(out, 28298);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23324);
					match s {
						[] => true,
						['о', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31390);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31391);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23325);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31393);
									match s {
										[] => true,
										['л', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33348);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33349);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ю', s @ ..] => {
									super::push(out, 31394);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 32816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мничаешь.
pub(crate) fn n9612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9186);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16521);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16522);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16523);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23326);
					s.is_empty()
				}
				['н', 'о', 'к', 'э', 'о', s @ ..] => {
					super::push(out, 23327);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'к', s @ ..] => {
					super::push(out, 23328);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31395);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31396);
									s.is_empty()
								}
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
			super::push(out, 16524);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23329);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'н', 'ь', 'л', 'о', 'в', 'о', s @ ..] => {
					super::push(out, 23330);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23331);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23332);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31397);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23333);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нничаешь.
pub(crate) fn n9613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9187);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16525);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23334);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28304);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 31399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28305);
							match s {
								[] => true,
								['с', 'о', s @ ..] => {
									super::push(out, 31400);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32817);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16526);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23335);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28306);
							match s {
								[] => true,
								['р', 'к', 'т', 'о', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32818);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32819);
											s.is_empty()
										}
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
					super::push(out, 23336);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23337);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28308);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31401);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 32820);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 31402);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 32821);
											s.is_empty()
										}
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
					super::push(out, 23338);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23339);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23340);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28311);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 31403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 28312);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31404);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28313);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 31405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23341);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31406);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31407);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 31408);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31409);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32822);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31410);
									s.is_empty()
								}
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
			super::push(out, 16527);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16528);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23342);
					match s {
						[] => true,
						['а', 'з', 'з', 'е', 'б', 'а', s @ ..] => {
							super::push(out, 28314);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23343);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23344);
					match s {
						[] => true,
						['и', 'л', '\u{1}', s @ ..] => {
							super::push(out, 28315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16529);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23345);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16530);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 16531);
			match s {
				[] => true,
				['ь', s @ ..] => {
					match s {
						[] => true,
						['з', 'е', 'б', 'о', 'с', s @ ..] => {
							super::push(out, 28317);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рничаешь.
pub(crate) fn n9614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9188);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16532);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23346);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 28319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23347);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28320);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23348);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 28321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 23349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23350);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28322);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31413);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32823);
											s.is_empty()
										}
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
					super::push(out, 23351);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23352);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28323);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 31414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23353);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23354);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28325);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 28326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23355);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28327);
							match s {
								[] => true,
								['ц', 'и', 'л', 'с', s @ ..] => {
									super::push(out, 31415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28328);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 28329);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31416);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32824);
											s.is_empty()
										}
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
					super::push(out, 23356);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23357);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23358);
					match s {
						[] => true,
						['о', 'с', 'о', s @ ..] => {
							super::push(out, 28332);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23359);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28333);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23360);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 23361);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16534);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16535);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23362);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23363);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28336);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28337);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23364);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28338);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31421);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31422);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16536);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 16537);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28340);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28341);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31424);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32825);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16538);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', 'и', 'ф', 'с', s @ ..] => {
							super::push(out, 28342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', 'л', 'и', 'м', 'а', 'ф', 'а', s @ ..] => {
					super::push(out, 23366);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сничаешь.
pub(crate) fn n9615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9189);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16539);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23367);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23368);
					match s {
						[] => true,
						['и', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31426);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31427);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31428);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16540);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23370);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23371);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23372);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23373);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16541);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23375);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16542);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23376);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28348);
							match s {
								[] => true,
								['ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32826);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32827);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32828);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28349);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 16543);
			match s {
				[] => true,
				['а', 'п', 'с', s @ ..] => {
					super::push(out, 23378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тничаешь.
pub(crate) fn n9616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9190);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16544);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23380);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 23381);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23382);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28352);
							match s {
								[] => true,
								['л', 'е', 'д', 'о', s @ ..] => {
									super::push(out, 31431);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32829);
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
								['в', s @ ..] => {
									super::push(out, 31432);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 31433);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 32830);
											s.is_empty()
										}
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
					super::push(out, 23383);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23384);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28354);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 28355);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31434);
									s.is_empty()
								}
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
			super::push(out, 16545);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23385);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28356);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28357);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28358);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23386);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31436);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31437);
									s.is_empty()
								}
								['о', 'д', s @ ..] => {
									super::push(out, 31438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'к', 'е', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28360);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28361);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16546);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 16547);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16548);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23387);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23388);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28363);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23389);
					match s {
						[] => true,
						['ж', 'о', s @ ..] => {
							super::push(out, 28364);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23390);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28365);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31441);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16549);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23391);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23392);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28367);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 28368);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 28369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23393);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23394);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28371);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 31443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 28372);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31444);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33350);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 33351);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33352);
													s.is_empty()
												}
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
							super::push(out, 28373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 23395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16551);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23396);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23397);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23398);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28376);
							match s {
								[] => true,
								['а', 'р', 'о', s @ ..] => {
									super::push(out, 31445);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32831);
											s.is_empty()
										}
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
					super::push(out, 23399);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16552);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23400);
					match s {
						[] => true,
						['о', 'б', s @ ..] => {
							super::push(out, 28378);
							match s {
								[] => true,
								['ю', 'л', 'о', s @ ..] => super::t11::n33333(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t11::n17465(s, out),
		_ => false,
	}
}

/// Words ending -уничаешь.
pub(crate) fn n9617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9191);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16554);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чничаешь.
pub(crate) fn n9618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9192);
	match s {
		[] => true,
		['а', 'м', s @ ..] => {
			super::push(out, 16555);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 23405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16556);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23406);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23407);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16557);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23409);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28382);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23410);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 28384);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31449);
									s.is_empty()
								}
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
			super::push(out, 16558);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28385);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28386);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31450);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23411);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23412);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31451);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31452);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 32833);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шничаешь.
pub(crate) fn n9619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9193);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23413);
					match s {
						[] => true,
						['а', 'ш', 'о', s @ ..] => {
							super::push(out, 28387);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23414);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 23415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28388);
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
				['м', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28389);
							match s {
								[] => true,
								['н', 'с', s @ ..] => {
									super::push(out, 31454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28390);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23416);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16559);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 23418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16560);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23419);
					match s {
						[] => true,
						['н', 'о', s @ ..] => {
							super::push(out, 28391);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31456);
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
						['е', s @ ..] => {
							super::push(out, 28392);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31457);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 31458);
									match s {
										[] => true,
										['и', 'л', 'е', 'в', 'с', s @ ..] => {
											super::push(out, 32834);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 31459);
									match s {
										[] => true,
										['а', 'м', 'с', s @ ..] => {
											super::push(out, 32835);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 31460);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 32836);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31461);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23420);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23421);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28394);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28395);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23422);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23423);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28396);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28397);
							match s {
								[] => true,
								['в', 'д', 'с', s @ ..] => {
									super::push(out, 31465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16562);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23425);
					match s {
						[] => true,
						['а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31466);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31467);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31468);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31469);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 31470);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32837);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16563);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23426);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23427);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щничаешь.
pub(crate) fn n9620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9194);
	match s {
		[] => true,
		['о', 'м', s @ ..] => {
			super::push(out, 16564);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьничаешь.
pub(crate) fn n9621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9195);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23429);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28399);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31471);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32838);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 28400);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28401);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31472);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31473);
									s.is_empty()
								}
								['ц', s @ ..] => {
									super::push(out, 31474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 28402);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28403);
							match s {
								[] => true,
								['и', 'г', 'и', 'р', 'о', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32839);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32840);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28404);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31475);
									match s {
										[] => true,
										['б', 'и', 'л', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33353);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33354);
													s.is_empty()
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 32841);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31476);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28405);
							match s {
								[] => true,
								['н', 'е', 'м', 'и', 'т', 'н', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32843);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 33355);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 32844);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28406);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 28407);
							match s {
								[] => true,
								['а', 'н', 'с', s @ ..] => {
									super::push(out, 31478);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32845);
											s.is_empty()
										}
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
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31479);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31480);
									match s {
										[] => true,
										['е', 'б', 'о', s @ ..] => {
											super::push(out, 32846);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31481);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 32847);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 32848);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 33356);
													s.is_empty()
												}
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
							super::push(out, 28408);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 31482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23430);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28409);
							match s {
								[] => true,
								['а', 'н', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32849);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 33357);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 32850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n24787(s, out),
				['у', s @ ..] => {
					super::push(out, 23432);
					match s {
						[] => true,
						['ж', 'с', s @ ..] => {
							super::push(out, 28418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 28419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -счувствуешь.
pub(crate) fn n30171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28461);
	s.is_empty()
}

/// Words ending -явствуешь.
pub(crate) fn n18099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17154);
	s.is_empty()
}

/// Words ending -гствуешь.
pub(crate) fn n10607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10081);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 17155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дствуешь.
pub(crate) fn n10608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10082);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23631);
					match s {
						[] => true,
						['к', 'и', 'р', 'п', 'о', 'к', 'у', 'р', 'а', s @ ..] => {
							super::push(out, 28463);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23632);
					match s {
						[] => true,
						['о', 'л', 'з', 'о', s @ ..] => {
							super::push(out, 28464);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31508);
									s.is_empty()
								}
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
			super::push(out, 17156);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23633);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23634);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23635);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23636);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28466);
							s.is_empty()
						}
						['с', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31509);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23637);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 31511);
									s.is_empty()
								}
								['с', 'о', s @ ..] => {
									super::push(out, 31512);
									s.is_empty()
								}
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
			super::push(out, 17157);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17158);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23638);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28467);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28468);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 31513);
									match s {
										[] => true,
										['у', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33361);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33362);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23639);
					s.is_empty()
				}
				['р', 'ю', s @ ..] => {
					super::push(out, 23640);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28469);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17159);
			match s {
				[] => true,
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', 'л', 'и', 'м', 'о', s @ ..] => {
							super::push(out, 28470);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31515);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32858);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31516);
									s.is_empty()
								}
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
			super::push(out, 17160);
			match s {
				[] => true,
				['л', 'б', 'о', 'в', s @ ..] => {
					super::push(out, 23642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 17161);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 17162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23643);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 23644);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ествуешь.
pub(crate) fn n10609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10083);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17163);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 17164);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23646);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23647);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23648);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28474);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32859);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32860);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32861);
											s.is_empty()
										}
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
					super::push(out, 23649);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28475);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31517);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32862);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28476);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31518);
									s.is_empty()
								}
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
			super::push(out, 17165);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23650);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28477);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 28478);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 28479);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31519);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31520);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 31521);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 28480);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 28481);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 28482);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23651);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23652);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28483);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 28484);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31523);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 31524);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32864);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 31525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28485);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31526);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31527);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32865);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31528);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32866);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 33363);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 32867);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32868);
											s.is_empty()
										}
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
					super::push(out, 23653);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23654);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28487);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31529);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32869);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 28488);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28489);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31530);
									match s {
										[] => true,
										['р', 'п', 'е', s @ ..] => {
											super::push(out, 32870);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28490);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 31531);
									match s {
										[] => true,
										['б', 'з', 'и', 'е', 'р', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33364);
													match s {
														[] => true,
														['п', s @ ..] => super::t11::n35550(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 33365);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23655);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t11::n18112(s, out),
		['щ', s @ ..] => {
			super::push(out, 17167);
			match s {
				[] => true,
				['и', s @ ..] => super::t11::n25030(s, out),
				['о', s @ ..] => super::t11::n25031(s, out),
				['у', s @ ..] => super::t11::n25032(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иствуешь.
pub(crate) fn n10610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10084);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23667);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 17169);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 17170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йствуешь.
pub(crate) fn n10611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10085);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17171);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23671);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23672);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23673);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28512);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31544);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28513);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['о', 'х', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33372);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 33373);
													match s {
														[] => true,
														['и', s @ ..] => super::t11::n35558(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['я', s @ ..] => {
											super::push(out, 32883);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 31545);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32884);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17172);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28514);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 28515);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28516);
							match s {
								[] => true,
								['б', 'ю', 'л', 'е', 'р', 'п', 'с', s @ ..] => {
									super::push(out, 31547);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 31548);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 31549);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 31550);
									s.is_empty()
								}
								['м', 'и', 'а', 'з', 'в', 'о', s @ ..] => {
									super::push(out, 31551);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32885);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31552);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31553);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31554);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32886);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32887);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33374);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 31556);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 32888);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 33375);
													s.is_empty()
												}
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
							super::push(out, 28517);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 28518);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31557);
									s.is_empty()
								}
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
			super::push(out, 17173);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17174);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23675);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23676);
					match s {
						[] => true,
						['е', 'г', 'о', s @ ..] => {
							super::push(out, 28520);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31558);
									s.is_empty()
								}
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
			super::push(out, 17175);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23677);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28521);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31559);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 31560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 23678);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 17176);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 17177);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23680);
					s.is_empty()
				}
				['з', 'о', 'х', 'а', s @ ..] => {
					super::push(out, 23681);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мствуешь.
pub(crate) fn n10612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10086);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17178);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 23682);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23683);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31561);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31562);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32889);
											s.is_empty()
										}
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
					super::push(out, 23684);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23685);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28526);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 28527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17180);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 17181);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23686);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'н', 'ь', 'л', 'о', 'в', 'о', s @ ..] => {
					super::push(out, 23687);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'е', 'б', 'о', s @ ..] => {
					super::push(out, 23688);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 17182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нствуешь.
pub(crate) fn n10613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10087);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17183);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23690);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							super::push(out, 28531);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 31564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23691);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23692);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28533);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28534);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23693);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28535);
							match s {
								[] => true,
								['ш', 'а', s @ ..] => {
									super::push(out, 31566);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28536);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23694);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28537);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28538);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28539);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32891);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32892);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 32893);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32894);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17184);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23695);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28540);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28541);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23696);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28542);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 23697);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28544);
							match s {
								[] => true,
								['л', 'б', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32895);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 28545);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23698);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23699);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28546);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 28547);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 31571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23700);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23701);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28549);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28550);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31573);
									match s {
										[] => true,
										['в', 'о', 'с', 'у', s @ ..] => {
											super::push(out, 32897);
											s.is_empty()
										}
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
					super::push(out, 23702);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28551);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 28552);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31575);
									s.is_empty()
								}
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
			super::push(out, 17185);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 17186);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 23703);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 23704);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 28553);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31576);
									s.is_empty()
								}
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
			super::push(out, 17187);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23705);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 23706);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23707);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23708);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 31577);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 31578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23709);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28554);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 28555);
							match s {
								[] => true,
								['ь', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32898);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t11::n25078(s, out),
				['р', s @ ..] => super::t11::n25079(s, out),
				['ш', s @ ..] => super::t11::n25080(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t11::n18134(s, out),
		['я', s @ ..] => super::t11::n18135(s, out),
		_ => false,
	}
}

/// Words ending -пствуешь.
pub(crate) fn n10614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10088);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17190);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17191);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17192);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23717);
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
		_ => false,
	}
}

/// Words ending -рствуешь.
pub(crate) fn n10615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10089);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17193);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28563);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28564);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 28565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 23719);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28567);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28568);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 28569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28570);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28571);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23722);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23723);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23724);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28573);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28574);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28575);
							match s {
								[] => true,
								['м', 'а', s @ ..] => {
									super::push(out, 31592);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32901);
											s.is_empty()
										}
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
					super::push(out, 23725);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 23726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28577);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28578);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28579);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28580);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31593);
									s.is_empty()
								}
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
			super::push(out, 17194);
			match s {
				[] => true,
				['о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28581);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23727);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31594);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31595);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31596);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 32902);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17195);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23728);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 23729);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 23730);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 23731);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28585);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28586);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28587);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 31598);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 32903);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 31599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 23732);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 23733);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23734);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28588);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28589);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 31600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23735);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23736);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28591);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28592);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t11::n25107(s, out),
				['п', s @ ..] => super::t11::n25108(s, out),
				['с', s @ ..] => super::t11::n25109(s, out),
				['т', s @ ..] => super::t11::n25110(s, out),
				['ф', s @ ..] => super::t11::n25111(s, out),
				['ч', s @ ..] => super::t11::n25112(s, out),
				['ш', s @ ..] => super::t11::n25113(s, out),
				['ь', s @ ..] => super::t11::n25114(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t11::n18142(s, out),
		['о', s @ ..] => super::t11::n18143(s, out),
		['у', s @ ..] => super::t11::n18144(s, out),
		['ы', s @ ..] => super::t11::n18145(s, out),
		['э', s @ ..] => super::t11::n18146(s, out),
		['я', s @ ..] => super::t11::n18147(s, out),
		_ => false,
	}
}

/// Words ending -тствуешь.
pub(crate) fn n10616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10090);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17202);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23763);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23764);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28625);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 31633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17203);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23766);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28626);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31634);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 33383);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 33384);
													s.is_empty()
												}
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
							super::push(out, 28627);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23767);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23768);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23769);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23770);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28630);
							s.is_empty()
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
				['д', s @ ..] => {
					super::push(out, 23771);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 23772);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23773);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23774);
					match s {
						[] => true,
						['а', 'б', 'и', 'с', 'о', s @ ..] => {
							super::push(out, 28632);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 31636);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17204);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17205);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23777);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28633);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 28634);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 28635);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 31638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23778);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 28636);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 28637);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31640);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 31641);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31642);
									s.is_empty()
								}
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
			super::push(out, 17206);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23779);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23780);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28639);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 28640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23781);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28641);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31643);
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
						['и', s @ ..] => {
							super::push(out, 28642);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 28643);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28644);
							s.is_empty()
						}
						['о', 'р', 'б', 'о', 'д', 'е', s @ ..] => {
							super::push(out, 28645);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31645);
									s.is_empty()
								}
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
			super::push(out, 17207);
			match s {
				[] => true,
				['п', 'а', s @ ..] => {
					super::push(out, 23782);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 28646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23783);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23784);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 17208);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23785);
					match s {
						[] => true,
						['о', 'б', 'ю', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31646);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31647);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32924);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17209);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23786);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 28648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 17210);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23787);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31648);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31649);
									s.is_empty()
								}
								['о', 'г', 'а', 'л', 'б', 'о', s @ ..] => {
									super::push(out, 31650);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 32925);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t11::n25162(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фствуешь.
pub(crate) fn n10617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10091);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17211);
			match s {
				[] => true,
				['ш', 'о', s @ ..] => {
					super::push(out, 23789);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 17212);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17213);
			match s {
				[] => true,
				['с', 'о', 'л', 'и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28650);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьствуешь.
pub(crate) fn n10618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10092);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23791);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28652);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 28653);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31654);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32927);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 31655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31656);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 31657);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 32928);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31658);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32929);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 31659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 28654);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31660);
									match s {
										[] => true,
										['н', 'о', s @ ..] => {
											super::push(out, 32930);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 33385);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23792);
					match s {
						[] => true,
						['т', s @ ..] => super::t11::n30384(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23793);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28656);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => super::t11::n30386(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t11::n25169(s, out),
				['у', s @ ..] => super::t11::n25170(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шардируешь.
pub(crate) fn n25341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23952);
	s.is_empty()
}

/// Words ending -удируешь.
pub(crate) fn n10923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10357);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17438);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 17439);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 17440);
			match s {
				[] => true,
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28741);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдируешь.
pub(crate) fn n10924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10358);
	s.is_empty()
}

/// Words ending -ализируешь.
pub(crate) fn n25367(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', 'ц', 'е', s @ ..] => {
			super::push(out, 28757);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31701);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28758);
			match s {
				[] => true,
				['а', 'о', s @ ..] => {
					super::push(out, 31702);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', 'и', 'с', 'о', s @ ..] => {
					super::push(out, 31703);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'г', s @ ..] => {
					super::push(out, 31704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 28759);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31706);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'и', 'п', s @ ..] => {
			super::push(out, 28760);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31707);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллизируешь.
pub(crate) fn n25369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23976);
	match s {
		[] => true,
		['а', 'т', 'с', 'и', 'р', 'к', 'ы', s @ ..] => {
			super::push(out, 28763);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олизируешь.
pub(crate) fn n25370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23977);
	s.is_empty()
}

/// Words ending -мизируешь.
pub(crate) fn n18445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17463);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23978);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28764);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23979);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28765);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31711);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 32959);
									s.is_empty()
								}
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
			super::push(out, 23980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -низируешь.
pub(crate) fn n18446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17464);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23981);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28766);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 23982);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23983);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23984);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28767);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 28768);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28770);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28771);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28772);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28773);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32960);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 32961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 31715);
							match s {
								[] => true,
								['н', 'и', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 33412);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 33413);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 31716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ризируешь.
pub(crate) fn n18447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17465);
	match s {
		[] => true,
		['а', 'т', 'е', s @ ..] => {
			super::push(out, 23985);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', s @ ..] => {
			super::push(out, 23986);
			match s {
				[] => true,
				['р', 'е', 'т', 'а', s @ ..] => {
					super::push(out, 28775);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизируешь.
pub(crate) fn n18448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17466);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23987);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28776);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28777);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 31719);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 32962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28778);
					match s {
						[] => true,
						['к', 'о', 'р', s @ ..] => {
							super::push(out, 31720);
							match s {
								[] => true,
								['ю', 'б', 'а', s @ ..] => {
									super::push(out, 32963);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 33414);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23988);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28779);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28780);
					s.is_empty()
				}
				['н', 'г', 'а', 'м', 'а', s @ ..] => {
					super::push(out, 28781);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 31721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31722);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31723);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32964);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 33415);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23989);
			match s {
				[] => true,
				['л', 'о', 'п', 'а', s @ ..] => {
					super::push(out, 28782);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28783);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 31725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23990);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28784);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28785);
					s.is_empty()
				}
				['н', 'п', 'и', 'г', 'а', s @ ..] => {
					super::push(out, 28786);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31726);
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
				['а', 'д', 'н', 'а', 'т', 'с', 'а', s @ ..] => {
					super::push(out, 28787);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 31727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 23991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28789);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28790);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзируешь.
pub(crate) fn n10939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10373);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17467);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23992);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28792);
							s.is_empty()
						}
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31729);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31730);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 31731);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 28793);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31732);
									s.is_empty()
								}
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
			super::push(out, 17468);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23993);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31733);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32965);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 31734);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32966);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озируешь.
pub(crate) fn n10940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10374);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 17469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23994);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23995);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17470);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 17471);
			match s {
				[] => true,
				['г', 'о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28794);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17472);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23997);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17473);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юзируешь.
pub(crate) fn n10941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10375);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 17474);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олируешь.
pub(crate) fn n10963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10397);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 17519);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17520);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 24024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17521);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24025);
					match s {
						[] => true,
						['т', 'о', 'р', 'п', 'а', s @ ..] => {
							super::push(out, 28821);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 31744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24026);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17522);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17523);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24027);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24028);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28823);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28824);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28825);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 31745);
									match s {
										[] => true,
										['с', 'к', 'э', 'о', s @ ..] => {
											super::push(out, 32967);
											s.is_empty()
										}
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
					super::push(out, 24029);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28826);
							match s {
								[] => true,
								['т', 'н', 'и', 'о', s @ ..] => {
									super::push(out, 31746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24030);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 24032);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28829);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17524);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24033);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24034);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28830);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32968);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 32969);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 33416);
													s.is_empty()
												}
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
							super::push(out, 28831);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31747);
									s.is_empty()
								}
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
			super::push(out, 17525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улируешь.
pub(crate) fn n10966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10400);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17528);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24037);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24038);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31748);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31749);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 32970);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 31750);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 31751);
									s.is_empty()
								}
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
			super::push(out, 17530);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17531);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24040);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'т', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28834);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28835);
							match s {
								[] => true,
								['е', 'ж', 'а', s @ ..] => {
									super::push(out, 31753);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 32971);
											s.is_empty()
										}
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
					super::push(out, 24041);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24042);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 24043);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28837);
							match s {
								[] => true,
								['а', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32972);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32973);
											s.is_empty()
										}
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
					super::push(out, 24044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17532);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28838);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31754);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31755);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32974);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'о', 'ф', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28839);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'к', 'к', 'а', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28841);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 24045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17533);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17534);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24046);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24047);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24048);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17535);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 24049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17536);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17537);
			match s {
				[] => true,
				['и', 'п', 'а', 'к', 'е', s @ ..] => {
					super::push(out, 24051);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флируешь.
pub(crate) fn n10967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10401);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 17538);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17539);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24053);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31756);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28844);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28845);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реформируешь.
pub(crate) fn n33673(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 32976);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 33417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32977);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 33418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нформируешь.
pub(crate) fn n30609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28863);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 31764);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32978);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оформируешь.
pub(crate) fn n30610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28864);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 31765);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 31766);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32980);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сформируешь.
pub(crate) fn n30611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28865);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31767);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 31768);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32981);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ифмируешь.
pub(crate) fn n18536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17546);
	s.is_empty()
}

/// Words ending -юмируешь.
pub(crate) fn n10978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10412);
	s.is_empty()
}

/// Words ending -роаккомпанируешь.
pub(crate) fn n35419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33419);
	s.is_empty()
}

/// Words ending -инируешь.
pub(crate) fn n10982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10416);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17562);
			match s {
				[] => true,
				['м', 'о', 'к', 'с', s @ ..] => {
					super::push(out, 24083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 17563);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24084);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 17564);
			match s {
				[] => true,
				['р', 'о', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28887);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 28888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 17565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24085);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28889);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 28890);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 28891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24088);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24089);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28892);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31777);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 31778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17567);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 17568);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 17569);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24090);
					match s {
						[] => true,
						['а', 'в', 'о', s @ ..] => {
							super::push(out, 28894);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 31779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 24091);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 28895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онируешь.
pub(crate) fn n10985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10419);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17570);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 17571);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 17572);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 17573);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24094);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31780);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 31781);
									s.is_empty()
								}
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
			super::push(out, 17574);
			match s {
				[] => true,
				['ц', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28896);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28897);
							match s {
								[] => true,
								['з', 'о', 'п', 'е', s @ ..] => {
									super::push(out, 31782);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 32984);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31783);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31784);
									match s {
										[] => true,
										['л', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 33420);
													match s {
														[] => true,
														['с', s @ ..] => super::t11::n35575(s, out),
														_ => false,
													}
												}
												['л', s @ ..] => {
													super::push(out, 33421);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 31785);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 32985);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 33422);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 32986);
											match s {
												[] => true,
												['ф', 'а', s @ ..] => {
													super::push(out, 33423);
													match s {
														[] => true,
														['з', s @ ..] => super::t11::n35576(s, out),
														_ => false,
													}
												}
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
							super::push(out, 28898);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 28899);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31786);
									s.is_empty()
								}
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
			super::push(out, 17575);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 17576);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17577);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24096);
					s.is_empty()
				}
				['к', 'т', s @ ..] => {
					super::push(out, 24097);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17578);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17579);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28901);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32987);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 32988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 28902);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28903);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 32989);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 32990);
											s.is_empty()
										}
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
					super::push(out, 24099);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 28904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24100);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24101);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 24102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17580);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28907);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28908);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28909);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 28910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24104);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 24106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24107);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28912);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28913);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31787);
									s.is_empty()
								}
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
			super::push(out, 17582);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24108);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28914);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 28915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24109);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31788);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 31789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24110);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 24111);
					s.is_empty()
				}
				['о', s @ ..] => super::t11::n25530(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t11::n18573(s, out),
		['ш', s @ ..] => super::t11::n18574(s, out),
		_ => false,
	}
}

/// Words ending -онспирируешь.
pub(crate) fn n18610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17616);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28949);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 28950);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28951);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орируешь.
pub(crate) fn n11008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10441);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17617);
			s.is_empty()
		}
		['к', 'е', 'д', 'а', s @ ..] => {
			super::push(out, 17618);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17619);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 17620);
			match s {
				[] => true,
				['г', 'и', 'о', s @ ..] => {
					super::push(out, 24139);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17621);
			s.is_empty()
		}
		_ => false,
	}
}

