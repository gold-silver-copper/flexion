use flexion::Path;

/// Words ending -олять.
pub(crate) fn n1059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1029);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4041);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17933);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 31614);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'г', 'а', 'л', 'б', 'о', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10394);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10395);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 17936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25052);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25053);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 25054);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31615);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25055);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 25056);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31616);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 4043);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4044);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10397);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10398);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плять.
pub(crate) fn n1060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1030);
	match s {
		[] => true,
		['а', 'р', 'к', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10399);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31617);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36530);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76967(s, out),
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31618);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36531);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76968(s, out),
														_ => false,
													}
												}
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25057);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31619);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 17938);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17939);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31620);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36532);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76969(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 10400);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => super::t15::n12770(s, out),
				['щ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => super::t15::n25375(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t15::n25376(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t15::n25377(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t15::n4368(s, out),
		['у', s @ ..] => super::t15::n4369(s, out),
		_ => false,
	}
}

/// Words ending -урлять.
pub(crate) fn n4370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4047);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 10402);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слять.
pub(crate) fn n1062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1032);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'з', 'в', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'м', 'с', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлять.
pub(crate) fn n1063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1033);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4048);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10403);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17940);
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17941);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17942);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17943);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25058);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 25059);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4049);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улять.
pub(crate) fn n1064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1034);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10406);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10407);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 17945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10408);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17946);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10409);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17947);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10410);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17948);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10411);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 17949);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10412);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 17951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17952);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10413);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17953);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 10414);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17954);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4051);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 10415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17955);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17956);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4052);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10416);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4054);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флять.
pub(crate) fn n1065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1035);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'р', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хлять.
pub(crate) fn n1066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1036);
	match s {
		[] => true,
		['а', 'р', 'а', 'б', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'ч', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4055);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17957);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25062);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4056);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлять.
pub(crate) fn n1067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1037);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4057);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17958);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17959);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17960);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17961);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17962);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 17963);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25064);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 17964);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25065);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 25066);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17965);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25067);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 17966);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25068);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4058);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10421);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4059);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 10422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17967);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 17968);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25069);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31621);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36533);
													match s {
														[] => true,
														['п', s @ ..] => super::t15::n76970(s, out),
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
									super::push(out, 25070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17969);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25072);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 25073);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 36534);
													match s {
														[] => true,
														['н', s @ ..] => super::t15::n76971(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 36535);
													match s {
														[] => true,
														['п', s @ ..] => super::t15::n76972(s, out),
														_ => false,
													}
												}
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
							super::push(out, 17970);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25074);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 31622);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 17971);
							match s {
								[] => true,
								['в', s @ ..] => super::t15::n40097(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылять.
pub(crate) fn n1068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1038);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4060);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10423);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25076);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31623);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36536);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25077);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31624);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4061);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4062);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4063);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 10424);
					match s {
						[] => true,
						['с', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4064);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10425);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 17972);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10426);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25078);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25079);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 25080);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31625);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25081);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -говнять.
pub(crate) fn n12846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10431);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17980);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17981);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 17982);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17983);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25083);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 17984);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровнять.
pub(crate) fn n12847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10432);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17986);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17987);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 17988);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 17989);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 17990);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17991);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17992);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17993);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 17995);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -днять.
pub(crate) fn n1085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1055);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4077);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10433);
					s.is_empty()
				}
				['м', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 10434);
					match s {
						[] => true,
						['с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4078);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4079);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 10435);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 17996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4080);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 10436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17997);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10437);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17998);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 17999);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4081);
			match s {
				[] => true,
				['р', 'т', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енять.
pub(crate) fn n1086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1056);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4082);
			match s {
				[] => true,
				['о', 'р', 'к', 'с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'е', 'т', 'с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4083);
			match s {
				[] => true,
				['е', 'л', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4084);
			match s {
				[] => true,
				['е', 'з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4085);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10439);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10440);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 18001);
							match s {
								[] => true,
								['п', 'с', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 18002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25087);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 10441);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18003);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10443);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10444);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10445);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18004);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25088);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 18005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10446);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10447);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18006);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18007);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 18008);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10449);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 10450);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18009);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4086);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4087);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10452);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10453);
					match s {
						[] => true,
						['т', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ц', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 18010);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4088);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10455);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10456);
					match s {
						[] => true,
						['к', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4089);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4090);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 4091);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жнять.
pub(crate) fn n1087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1057);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', 'в', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10457);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['с', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31626);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36537);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76973(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4092);
			match s {
				[] => true,
				['л', 'с', 'у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -знять.
pub(crate) fn n1088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1058);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4093);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10458);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 10459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4095);
			match s {
				[] => true,
				['р', 'г', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инять.
pub(crate) fn n1089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1059);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4096);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4097);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10461);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ъ', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10462);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 10463);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 10464);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18013);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25089);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18014);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 10465);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10466);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10467);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10469);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 10470);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25090);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31627);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4099);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4100);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 4101);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 10472);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25091);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 31628);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25092);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 31629);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 31630);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 31631);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => super::t15::n66913(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => super::t15::n12913(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнять.
pub(crate) fn n1090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1060);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4102);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мнять.
pub(crate) fn n1091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1061);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онять.
pub(crate) fn n1092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1062);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4103);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10474);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10475);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10476);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10477);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10478);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18017);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 18018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10479);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 18019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 10480);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18020);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 31632);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36538);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t15::n76974(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10481);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 10483);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25094);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31633);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4106);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4107);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18022);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25095);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31634);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 10485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4108);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => super::t15::n12942(s, out),
				['з', s @ ..] => {
					super::push(out, 10486);
					match s {
						[] => true,
						['а', s @ ..] => super::t15::n25602(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t15::n12944(s, out),
				['п', s @ ..] => super::t15::n12945(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t15::n4458(s, out),
		_ => false,
	}
}

/// Words ending -пнять.
pub(crate) fn n1093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1063);
	match s {
		[] => true,
		['у', 'р', 'к', 'у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнять.
pub(crate) fn n1094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1064);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4111);
			match s {
				[] => true,
				['в', 'к', 'с', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -снять.
pub(crate) fn n1095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1065);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4112);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4113);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 10489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4114);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10490);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4115);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10491);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18027);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25097);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18028);
							match s {
								[] => true,
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 10492);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18029);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25098);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31635);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'т', 'ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4116);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 10493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 10494);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 10495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4117);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4118);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнять.
pub(crate) fn n1096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1066);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4119);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10497);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 10498);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18031);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10499);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'м', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унять.
pub(crate) fn n1097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1067);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4120);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4122);
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4123);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 10500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 4124);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнять.
pub(crate) fn n1098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1068);
	match s {
		[] => true,
		['о', 'т', 'у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'т', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word наблошнять.
pub(crate) fn n1099(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ынять.
pub(crate) fn n1100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1069);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4125);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4127);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 10502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18033);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18034);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25099);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18035);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25100);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьнять.
pub(crate) fn n1101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1070);
	match s {
		[] => true,
		['л', 'о', 'в', 'у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 10503);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 18036);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янять.
pub(crate) fn n1102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1071);
	s.is_empty()
}

/// Words ending -мерять.
pub(crate) fn n4497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4134);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10517);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10518);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			super::push(out, 10519);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 18052);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 25106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10520);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25107);
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25108);
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			super::push(out, 10521);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			super::push(out, 10523);
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -терять.
pub(crate) fn n4499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4136);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10524);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10525);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 18056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10526);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10527);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18057);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 18058);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 18059);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 25110);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10530);
			match s {
				[] => true,
				['в', 'е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 18060);
					match s {
						[] => true,
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'д', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word подвечерять.
pub(crate) fn n40240(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -очерять.
pub(crate) fn n13035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10532);
	s.is_empty()
}

/// The word ощерять.
pub(crate) fn n4501(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ирять.
pub(crate) fn n1114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1082);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 4138);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4139);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орять.
pub(crate) fn n1115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1083);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4140);
			match s {
				[] => true,
				['д', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', 'и', 'ж', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['г', s @ ..] => {
									match s {
										[] => true,
										['а', 'л', 'б', 'у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['х', 'у', 'д', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18062);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 25111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'о', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -прять.
pub(crate) fn n1116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1084);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 4141);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 10534);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трять.
pub(crate) fn n1117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1085);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4142);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['а', s @ ..] => {
					super::push(out, 10535);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18063);
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 10536);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 10537);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 10538);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18064);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10539);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18065);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31637);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 10540);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урять.
pub(crate) fn n1118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1086);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 4143);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 10541);
					match s {
						[] => true,
						['ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4144);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'х', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хрять.
pub(crate) fn n1119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1087);
	match s {
		[] => true,
		['и', 'в', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щрять.
pub(crate) fn n1120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1088);
	s.is_empty()
}

/// Words ending -ырять.
pub(crate) fn n1121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1089);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4145);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10542);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 18066);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10543);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25112);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25113);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 31638);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 31639);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 25114);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31640);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 25115);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31641);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 25116);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 31642);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25117);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 31643);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 31644);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 25118);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 31645);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 31646);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 25119);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 31647);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 25120);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 31648);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18067);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18068);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 18069);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25121);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18070);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25122);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18071);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => super::t15::n40267(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t15::n25714(s, out),
						['т', s @ ..] => super::t15::n25715(s, out),
						['ы', s @ ..] => super::t15::n25716(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t15::n4519(s, out),
		['н', s @ ..] => super::t15::n4520(s, out),
		['п', s @ ..] => super::t15::n4521(s, out),
		_ => false,
	}
}

/// Words ending -ярять.
pub(crate) fn n1122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1090);
	s.is_empty()
}

/// Words ending -аться.
pub(crate) fn n1215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1174);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4215);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 10588);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18102);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25139);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t15::n13186(s, out),
				['и', s @ ..] => {
					super::push(out, 10590);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18113);
							match s {
								[] => true,
								['а', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => super::t15::n25803(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t15::n13188(s, out),
				['у', s @ ..] => super::t15::n13189(s, out),
				['ы', s @ ..] => super::t15::n13190(s, out),
				['я', s @ ..] => super::t15::n13191(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t15::n4656(s, out),
		['г', s @ ..] => super::t15::n4657(s, out),
		['д', s @ ..] => super::t15::n4658(s, out),
		['ж', s @ ..] => super::t15::n4659(s, out),
		['з', s @ ..] => super::t15::n4660(s, out),
		['к', s @ ..] => super::t15::n4661(s, out),
		['л', s @ ..] => super::t16::n4662(s, out),
		['м', s @ ..] => super::t16::n4663(s, out),
		['н', s @ ..] => super::t16::n4664(s, out),
		['п', s @ ..] => super::t16::n4665(s, out),
		['р', s @ ..] => super::t16::n4666(s, out),
		['с', s @ ..] => super::t16::n4667(s, out),
		['т', s @ ..] => super::t16::n4668(s, out),
		['х', s @ ..] => super::t16::n4669(s, out),
		['ц', s @ ..] => super::t16::n4670(s, out),
		['ч', s @ ..] => super::t16::n4671(s, out),
		['ш', s @ ..] => super::t16::n4672(s, out),
		['щ', s @ ..] => super::t16::n4673(s, out),
		_ => false,
	}
}

/// Words ending -еться.
pub(crate) fn n1216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1175);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4234);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 4235);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 10796);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18728);
							match s {
								[] => true,
								['о', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4236);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10797);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18729);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26266);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32702);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10798);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18730);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'о', 'р', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4237);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10799);
					s.is_empty()
				}
				['а', s @ ..] => super::t16::n13399(s, out),
				['в', s @ ..] => {
					super::push(out, 10801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t16::n13401(s, out),
				['з', s @ ..] => super::t16::n13402(s, out),
				['и', s @ ..] => super::t16::n13403(s, out),
				['о', s @ ..] => super::t16::n13404(s, out),
				['р', s @ ..] => super::t16::n13405(s, out),
				['у', s @ ..] => super::t16::n13406(s, out),
				['я', s @ ..] => super::t16::n13407(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t16::n4678(s, out),
		['л', s @ ..] => super::t16::n4679(s, out),
		['м', s @ ..] => super::t16::n4680(s, out),
		['н', s @ ..] => super::t16::n4681(s, out),
		['п', s @ ..] => super::t16::n4682(s, out),
		['р', s @ ..] => super::t16::n4683(s, out),
		['с', s @ ..] => super::t16::n4684(s, out),
		['т', s @ ..] => super::t16::n4685(s, out),
		_ => false,
	}
}

/// Words ending -иться.
pub(crate) fn n1218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1177);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4249);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 10850);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26357);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26358);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 26359);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 26360);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32775);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10851);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18809);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18810);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 18811);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t16::n4690(s, out),
		['в', s @ ..] => super::t16::n4691(s, out),
		['г', s @ ..] => {
			super::push(out, 4252);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 10886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26524);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26525);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t16::n4693(s, out),
		['е', s @ ..] => {
			super::push(out, 4254);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 10901);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18965);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26669);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26670);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 26671);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 33071);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 10902);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26672);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 26673);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26674);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10903);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18967);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26675);
									match s {
										[] => true,
										['в', 'е', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10904);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 18968);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t16::n4695(s, out),
		['з', s @ ..] => super::t16::n4696(s, out),
		['л', s @ ..] => super::t16::n4697(s, out),
		['м', s @ ..] => super::t16::n4698(s, out),
		['н', s @ ..] => super::t16::n4699(s, out),
		['о', s @ ..] => super::t16::n4700(s, out),
		['п', s @ ..] => super::t16::n4701(s, out),
		['р', s @ ..] => super::t16::n4702(s, out),
		['с', s @ ..] => super::t16::n4703(s, out),
		['т', s @ ..] => super::t16::n4704(s, out),
		['у', s @ ..] => super::t16::n4705(s, out),
		['ф', s @ ..] => super::t16::n4706(s, out),
		['ч', s @ ..] => super::t16::n4707(s, out),
		['ш', s @ ..] => super::t17::n4708(s, out),
		['щ', s @ ..] => super::t17::n4709(s, out),
		['ь', s @ ..] => super::t17::n4710(s, out),
		_ => false,
	}
}

/// Words ending -лоться.
pub(crate) fn n4711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4271);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28092);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28093);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28094);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28096);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 34277);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28097);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28098);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28099);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 34278);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 19696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28100);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28101);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 28102);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 34279);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ороться.
pub(crate) fn n13705(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 19697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28103);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28104);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 19698);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28105);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 28106);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 28109);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 34280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сться.
pub(crate) fn n1220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4273);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11104);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28110);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28111);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 34281);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11105);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11106);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28112);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28113);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 28114);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 28115);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28116);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 28117);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34282);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11107);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11108);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 19699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11109);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19700);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11110);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11111);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19701);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11112);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19702);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => super::t17::n13716(s, out),
				['ъ', s @ ..] => super::t17::n13717(s, out),
				['ы', s @ ..] => {
					super::push(out, 11116);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t17::n4715(s, out),
		_ => false,
	}
}

/// Words ending -уться.
pub(crate) fn n1221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4276);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4277);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11120);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11121);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 11122);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11123);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11124);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11125);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11126);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11127);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t17::n4719(s, out),
		_ => false,
	}
}

/// Words ending -ыться.
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4280);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11153);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11154);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28266);
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11155);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4281);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4282);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11156);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19810);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19811);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 28267);
									s.is_empty()
								}
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
			super::push(out, 4283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11157);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11159);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 19813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11161);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11162);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 11163);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 19815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19816);
							s.is_empty()
						}
						['а', s @ ..] => super::t17::n27731(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', s @ ..] => super::t17::n27733(s, out),
						['о', s @ ..] => super::t17::n27734(s, out),
						['с', s @ ..] => super::t17::n27735(s, out),
						['т', 'о', s @ ..] => super::t17::n27736(s, out),
						['у', s @ ..] => super::t17::n27737(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t17::n13782(s, out),
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => super::t17::n13784(s, out),
				['ы', s @ ..] => super::t17::n13785(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яться.
pub(crate) fn n1223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4285);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4286);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11166);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19819);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19821);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19822);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28269);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11168);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19823);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19824);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 19825);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19826);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19827);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 28270);
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 19828);
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19829);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19830);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19831);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19833);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19834);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19835);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 11173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19836);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19837);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t17::n4728(s, out),
		['з', s @ ..] => {
			super::push(out, 4288);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4289);
			match s {
				[] => true,
				['л', s @ ..] => super::t17::n13801(s, out),
				['п', s @ ..] => super::t17::n13802(s, out),
				['с', s @ ..] => super::t17::n13803(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t17::n4731(s, out),
		['м', s @ ..] => super::t17::n4732(s, out),
		['н', s @ ..] => super::t17::n4733(s, out),
		['о', s @ ..] => super::t17::n4734(s, out),
		['п', s @ ..] => super::t17::n4735(s, out),
		['р', s @ ..] => super::t17::n4736(s, out),
		['с', s @ ..] => super::t17::n4737(s, out),
		['у', s @ ..] => super::t17::n4738(s, out),
		['ъ', s @ ..] => super::t17::n4739(s, out),
		_ => false,
	}
}

/// Words ending -речься.
pub(crate) fn n4743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4302);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11267);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19952);
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 11268);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11269);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28381);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19955);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34462);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 34463);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 11270);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 19956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19957);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19958);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11272);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19959);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word засечься.
pub(crate) fn n28070(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word насечься.
pub(crate) fn n28071(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word обсечься.
pub(crate) fn n13913(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -дсечься.
pub(crate) fn n13914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11275);
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -есечься.
pub(crate) fn n13915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11276);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 28382);
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word присечься.
pub(crate) fn n13916(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -осечься.
pub(crate) fn n13917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11277);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 19960);
			s.is_empty()
		}
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ссечься.
pub(crate) fn n13918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11278);
	match s {
		[] => true,
		['а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 19961);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отсечься.
pub(crate) fn n13919(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word усечься.
pub(crate) fn n13920(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ысечься.
pub(crate) fn n13921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11279);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19962);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -течься.
pub(crate) fn n4745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4304);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11280);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19963);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word переизобрести.
pub(crate) fn n73767(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оскрести.
pub(crate) fn n14068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11317);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -выскрести.
pub(crate) fn n28149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19992);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -естись.
pub(crate) fn n1359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1280);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11334);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11335);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11336);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11337);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11338);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'о', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11339);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11340);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19995);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19996);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 11342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11343);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4404);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19999);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20000);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 20001);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20002);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20003);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20005);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28401);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11345);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11346);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20006);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11348);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20007);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t17::n4951(s, out),
		['р', s @ ..] => super::t17::n4952(s, out),
		_ => false,
	}
}

/// The word пораспродавать.
pub(crate) fn n81291(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -выдавать.
pub(crate) fn n14260(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20053);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удевать.
pub(crate) fn n5071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4482);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 11453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еевать.
pub(crate) fn n1403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1322);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 4483);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 11454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жевать.
pub(crate) fn n1404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1323);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4484);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4485);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4486);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4487);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11456);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28438);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28439);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28441);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34480);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20070);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20071);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20072);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28442);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 34481);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 20073);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20074);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 28443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 20075);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28444);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11458);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20076);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28445);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4488);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11459);
					match s {
						[] => true,
						['р', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4490);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11460);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4491);
			match s {
				[] => true,
				['е', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 11461);
					match s {
						[] => true,
						['р', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4492);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20078);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28446);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 20079);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4493);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11464);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4494);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4495);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4496);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11467);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4498);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11468);
					match s {
						[] => true,
						['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зевать.
pub(crate) fn n1405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1324);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4499);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4500);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11470);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20080);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20081);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4501);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11472);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -левать.
pub(crate) fn n1406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1325);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4502);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20084);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20085);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20086);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20087);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28448);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20088);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4503);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11476);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11477);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11478);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11481);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20091);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4504);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4505);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11483);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20092);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28449);
									match s {
										[] => true,
										['ш', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 38367);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 38368);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t17::n81292(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20093);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28450);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11484);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11485);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 20095);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28451);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11486);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 20096);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28452);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11487);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['х', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11488);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20097);
							match s {
								[] => true,
								['е', 'т', 'ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34482);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 34483);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => super::t17::n73785(s, out),
												_ => false,
											}
										}
										['е', s @ ..] => super::t17::n61715(s, out),
										['о', s @ ..] => super::t17::n61716(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t17::n14368(s, out),
				['ц', s @ ..] => super::t17::n14369(s, out),
				['ш', s @ ..] => super::t17::n14370(s, out),
				['щ', s @ ..] => super::t17::n14371(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t17::n5095(s, out),
		['к', s @ ..] => super::t17::n5096(s, out),
		['м', s @ ..] => super::t17::n5097(s, out),
		['о', s @ ..] => super::t17::n5098(s, out),
		['п', s @ ..] => super::t17::n5099(s, out),
		['т', s @ ..] => super::t17::n5100(s, out),
		['ф', s @ ..] => super::t17::n5101(s, out),
		['х', s @ ..] => super::t17::n5102(s, out),
		['ы', s @ ..] => super::t17::n5103(s, out),
		_ => false,
	}
}

/// Words ending -мевать.
pub(crate) fn n1407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1326);
	match s {
		[] => true,
		['д', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'н', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4515);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4516);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11526);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20131);
							match s {
								[] => true,
								['р', 'о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20132);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 28469);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11527);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -невать.
pub(crate) fn n1408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1327);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4517);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 11528);
					match s {
						[] => true,
						['а', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4518);
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
						['а', s @ ..] => {
							super::push(out, 20134);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11531);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11532);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20135);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11533);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20136);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11534);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20137);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28470);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20138);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28471);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11535);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20139);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11536);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20140);
							match s {
								[] => true,
								['с', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4520);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11537);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20141);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 28472);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20142);
							match s {
								[] => true,
								['о', 'т', 'с', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11538);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20143);
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20144);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28473);
									match s {
										[] => true,
										['п', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11539);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28474);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28475);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11540);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20145);
							match s {
								[] => true,
								['ц', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'х', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11541);
					match s {
						[] => true,
						['с', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 11542);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4521);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20146);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20147);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 28476);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20148);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28477);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20149);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28478);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4522);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11544);
					match s {
						[] => true,
						['в', 'е', 'р', 'д', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11545);
					match s {
						[] => true,
						['к', s @ ..] => super::t17::n28513(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t17::n5114(s, out),
		['я', s @ ..] => super::t17::n5115(s, out),
		_ => false,
	}
}

/// Words ending -оевать.
pub(crate) fn n1409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1328);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11548);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11549);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11550);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11551);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4526);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11552);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4527);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 11553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -певать.
pub(crate) fn n1410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1329);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4528);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4529);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4530);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4531);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4532);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4533);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4534);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11554);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11556);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20156);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11557);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 20158);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 20159);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 11559);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4536);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4537);
			match s {
				[] => true,
				['п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ревать.
pub(crate) fn n1411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1330);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4538);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20160);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28479);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11561);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20161);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 11562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4539);
			match s {
				[] => true,
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 20162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28480);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28481);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4540);
			match s {
				[] => true,
				['х', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4541);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 20164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11563);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11564);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11565);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20166);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28483);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28484);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34489);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', 'з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 20167);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					super::push(out, 11566);
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					super::push(out, 11567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4542);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4543);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20169);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20170);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4544);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4545);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11571);
					match s {
						[] => true,
						['в', s @ ..] => super::t17::n28571(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t17::n14481(s, out),
				['с', s @ ..] => super::t17::n14482(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t17::n5139(s, out),
		['ы', s @ ..] => super::t17::n5140(s, out),
		_ => false,
	}
}

/// Words ending -севать.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1331);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4548);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4549);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4550);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11577);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4551);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тевать.
pub(crate) fn n1413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1332);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4552);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4553);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'е', 'ч', 'о', 'т', 'с', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4554);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11579);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28485);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11580);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уевать.
pub(crate) fn n1414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1333);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4555);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 11581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 4556);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цевать.
pub(crate) fn n1415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1334);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4557);
			match s {
				[] => true,
				['у', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20177);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 20178);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4558);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11582);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20179);
							match s {
								[] => true,
								['о', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4559);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20180);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20181);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 20182);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11583);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20183);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4560);
			match s {
				[] => true,
				['а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'л', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11584);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20184);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28487);
									match s {
										[] => true,
										['а', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 20185);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 28488);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20186);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28489);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28490);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 34490);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 28491);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 34491);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 28492);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 34492);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 38371);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t17::n81295(s, out),
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
									super::push(out, 28493);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28494);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28495);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 28496);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34493);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28497);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34494);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t17::n14505(s, out),
				['о', s @ ..] => super::t17::n14506(s, out),
				['у', s @ ..] => {
					super::push(out, 11587);
					match s {
						[] => true,
						['п', s @ ..] => super::t17::n28601(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t17::n14508(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t17::n5164(s, out),
		['р', s @ ..] => super::t17::n5165(s, out),
		['с', s @ ..] => super::t17::n5166(s, out),
		['ь', s @ ..] => super::t17::n5167(s, out),
		_ => false,
	}
}

/// Words ending -чевать.
pub(crate) fn n1416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1335);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4566);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11592);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 28511);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['б', s @ ..] => {
					super::push(out, 11593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20198);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20199);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20200);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 28513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4569);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11594);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20201);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11595);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 28514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11596);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20203);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4570);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20204);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20205);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20206);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20207);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20208);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4571);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11598);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 20209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11599);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28516);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28517);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 34507);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 28518);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 34508);
											match s {
												[] => true,
												['р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28519);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34509);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4572);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11600);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20210);
							match s {
								[] => true,
								['т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11601);
					match s {
						[] => true,
						['п', s @ ..] => super::t17::n28639(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4573);
			match s {
				[] => true,
				['с', s @ ..] => super::t17::n14526(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t17::n5176(s, out),
		_ => false,
	}
}

/// Words ending -шевать.
pub(crate) fn n1417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1336);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4575);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11604);
					match s {
						[] => true,
						['а', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4576);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11605);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28522);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28523);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4577);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11606);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20211);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11607);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 20212);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4578);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 11608);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4579);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20213);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20214);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20215);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20216);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20217);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28524);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 20218);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28525);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34510);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 20219);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28526);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20220);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 28527);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 28528);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 28529);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20222);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 20223);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28531);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4580);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11611);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 20224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щевать.
pub(crate) fn n1418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1337);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4581);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4582);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лаивать.
pub(crate) fn n5199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4595);
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 11620);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 11621);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11622);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11623);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -наивать.
pub(crate) fn n5201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4597);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 28532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20228);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28533);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паивать.
pub(crate) fn n5202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4598);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11625);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раивать.
pub(crate) fn n5203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4599);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11626);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28534);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'у', 'р', 'т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таивать.
pub(crate) fn n5204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4600);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11627);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 11628);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11629);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20232);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11630);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20233);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28537);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34511);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 34512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 28538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28539);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20234);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28540);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20235);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20237);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11631);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11632);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11633);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хаивать.
pub(crate) fn n5205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4601);
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бивать.
pub(crate) fn n1427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1346);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4602);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4603);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11634);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20239);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28542);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20240);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28543);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4604);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20241);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4605);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4606);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4607);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4608);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20242);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28545);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4609);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20243);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28546);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 11636);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20244);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11637);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4612);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20246);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28547);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4613);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11638);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20247);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4614);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20248);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28548);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вивать.
pub(crate) fn n1428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1347);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4615);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20249);
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 4616);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4617);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20250);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

