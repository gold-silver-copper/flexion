use flexion::Path;

/// Words ending -гивать.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1348);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4618);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11639);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20251);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
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
						['д', s @ ..] => {
							super::push(out, 20252);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28550);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20253);
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
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
										['с', 'и', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20254);
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
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20255);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28551);
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
			super::push(out, 4619);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20256);
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11641);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
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
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'и', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['з', s @ ..] => super::t17::n5234(s, out),
		['и', s @ ..] => {
			super::push(out, 4621);
			match s {
				[] => true,
				['м', s @ ..] => super::t17::n14644(s, out),
				['с', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => super::t17::n28775(s, out),
						['е', s @ ..] => super::t17::n28776(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t17::n5236(s, out),
		['р', s @ ..] => super::t17::n5237(s, out),
		['у', s @ ..] => super::t17::n5238(s, out),
		['ы', s @ ..] => super::t17::n5239(s, out),
		['я', s @ ..] => super::t17::n5240(s, out),
		_ => false,
	}
}

/// Words ending -еивать.
pub(crate) fn n1430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1349);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4627);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11653);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20274);
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
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'к', s @ ..] => {
			match s {
				[] => true,
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
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'б', 'о', '\u{1}', s @ ..] => {
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
		['м', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'о', 'р', 'в', 'е', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ф', 'о', 'р', 'т', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -живать.
pub(crate) fn n1431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4628);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4629);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20275);
							s.is_empty()
						}
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => super::t17::n28841(s, out),
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
				['г', s @ ..] => super::t17::n14687(s, out),
				['з', s @ ..] => {
					super::push(out, 11656);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t17::n28853(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t17::n14689(s, out),
				['м', s @ ..] => super::t17::n14690(s, out),
				['н', s @ ..] => super::t17::n14691(s, out),
				['р', s @ ..] => super::t17::n14692(s, out),
				['с', s @ ..] => super::t17::n14693(s, out),
				['х', s @ ..] => super::t17::n14694(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t17::n5250(s, out),
		['в', s @ ..] => super::t17::n5251(s, out),
		['д', s @ ..] => super::t17::n5252(s, out),
		['е', s @ ..] => super::t17::n5253(s, out),
		['ж', 'а', 'в', s @ ..] => super::t17::n5254(s, out),
		['з', s @ ..] => super::t17::n5255(s, out),
		['и', s @ ..] => super::t17::n5256(s, out),
		['л', s @ ..] => super::t17::n5257(s, out),
		['о', s @ ..] => super::t17::n5258(s, out),
		['р', s @ ..] => super::t17::n5259(s, out),
		['с', s @ ..] => super::t17::n5260(s, out),
		['т', s @ ..] => super::t17::n5261(s, out),
		['у', s @ ..] => super::t17::n5262(s, out),
		['ы', s @ ..] => super::t17::n5263(s, out),
		['ю', s @ ..] => super::t17::n5264(s, out),
		['я', s @ ..] => super::t17::n5265(s, out),
		_ => false,
	}
}

/// Words ending -зивать.
pub(crate) fn n1432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['а', 'у', 'ж', 'р', 'у', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'л', 'е', 'ж', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4645);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11693);
					match s {
						[] => true,
						['е', 'р', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кивать.
pub(crate) fn n1433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4646);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4647);
			match s {
				[] => true,
				['в', 'к', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20323);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28613);
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
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11695);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20325);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28614);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 34543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 28615);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 28616);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 34544);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 38395);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t17::n81315(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 34545);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 38396);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t17::n81316(s, out),
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
									super::push(out, 28617);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 34546);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 28618);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 34547);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 28619);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28620);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 28621);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 34548);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 28622);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28623);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34549);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 38397);
													match s {
														[] => true,
														['п', s @ ..] => super::t17::n81317(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20326);
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
				['л', s @ ..] => super::t17::n14750(s, out),
				['м', s @ ..] => {
					super::push(out, 11697);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
				['т', s @ ..] => super::t17::n14752(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t17::n5271(s, out),
		['е', 'м', 'у', 'к', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => super::t17::n5273(s, out),
		['л', s @ ..] => super::t17::n5274(s, out),
		['м', s @ ..] => super::t17::n5275(s, out),
		['о', s @ ..] => super::t17::n5276(s, out),
		['р', s @ ..] => super::t17::n5277(s, out),
		['с', s @ ..] => super::t17::n5278(s, out),
		['у', s @ ..] => super::t17::n5279(s, out),
		['ч', s @ ..] => super::t17::n5280(s, out),
		['ш', 'е', 'м', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t17::n5281(s, out),
		['ы', s @ ..] => super::t17::n5282(s, out),
		['ь', s @ ..] => super::t17::n5283(s, out),
		['ю', s @ ..] => super::t17::n5284(s, out),
		['я', s @ ..] => super::t17::n5285(s, out),
		_ => false,
	}
}

/// Words ending -ливать.
pub(crate) fn n1434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1353);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4659);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4660);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20364);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20365);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20366);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20367);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28674);
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
							super::push(out, 20368);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28675);
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
							super::push(out, 20369);
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
				['в', s @ ..] => super::t17::n14808(s, out),
				['д', 'н', 'а', s @ ..] => {
					match s {
						[] => true,
						['к', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', s @ ..] => {
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
						_ => false,
					}
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20374);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t17::n14812(s, out),
				['м', s @ ..] => super::t17::n14813(s, out),
				['н', s @ ..] => super::t17::n14814(s, out),
				['п', s @ ..] => super::t17::n14815(s, out),
				['р', 'т', s @ ..] => super::t17::n14816(s, out),
				['с', s @ ..] => super::t17::n14817(s, out),
				['т', s @ ..] => super::t17::n14818(s, out),
				['ч', s @ ..] => super::t17::n14819(s, out),
				['ш', s @ ..] => super::t17::n14820(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t17::n5288(s, out),
		['в', s @ ..] => super::t17::n5289(s, out),
		['г', 'у', s @ ..] => super::t17::n5290(s, out),
		['д', s @ ..] => super::t17::n5291(s, out),
		['е', s @ ..] => super::t17::n5292(s, out),
		['з', s @ ..] => super::t17::n5293(s, out),
		['и', s @ ..] => super::t17::n5294(s, out),
		['к', s @ ..] => super::t17::n5295(s, out),
		['л', 'о', 'р', 'т', 'а', 'з', '\u{1}', s @ ..] => super::t17::n5296(s, out),
		['м', s @ ..] => super::t17::n5297(s, out),
		['о', s @ ..] => super::t17::n5298(s, out),
		['п', s @ ..] => super::t17::n5299(s, out),
		['р', s @ ..] => super::t17::n5300(s, out),
		['с', s @ ..] => super::t18::n5301(s, out),
		['т', s @ ..] => super::t18::n5302(s, out),
		['у', s @ ..] => super::t18::n5303(s, out),
		['ф', 'и', 'л', 'о', 'о', 'р', 'п', '\u{1}', s @ ..] => super::t18::n5304(s, out),
		['х', s @ ..] => super::t18::n5305(s, out),
		['ш', s @ ..] => super::t18::n5306(s, out),
		['ы', s @ ..] => super::t18::n5307(s, out),
		['ю', s @ ..] => super::t18::n5308(s, out),
		['я', s @ ..] => super::t18::n5309(s, out),
		_ => false,
	}
}

/// Words ending -нивать.
pub(crate) fn n1435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4678);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11779);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['р', 'а', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											match s {
												[] => true,
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['т', 'о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', 'т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 20417);
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11780);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28706);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28707);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 34592);
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
									super::push(out, 28708);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28709);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 34593);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 34594);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 34595);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 34596);
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'о', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20418);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28710);
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
				['г', s @ ..] => super::t18::n14914(s, out),
				['к', s @ ..] => super::t18::n14915(s, out),
				['л', 'к', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => super::t18::n14917(s, out),
				['р', s @ ..] => super::t18::n14918(s, out),
				['щ', 'е', 'м', s @ ..] => super::t18::n14919(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t18::n5311(s, out),
		['г', s @ ..] => super::t18::n5312(s, out),
		['д', 'а', 'с', 'о', '\u{1}', s @ ..] => super::t18::n5313(s, out),
		['е', s @ ..] => super::t18::n5314(s, out),
		['ж', 'а', 'р', 'о', 'п', 'ы', 'в', '\u{1}', s @ ..] => super::t18::n5315(s, out),
		['з', s @ ..] => super::t18::n5316(s, out),
		['и', s @ ..] => super::t18::n5317(s, out),
		['о', s @ ..] => super::t18::n5318(s, out),
		['р', 'е', 'ч', 'ы', 'в', '\u{1}', s @ ..] => super::t18::n5319(s, out),
		['с', s @ ..] => super::t18::n5320(s, out),
		['у', s @ ..] => super::t18::n5321(s, out),
		['ч', 'а', 'р', 'з', 'о', 'р', 'п', 'о', '\u{1}', s @ ..] => super::t18::n5322(s, out),
		['ш', s @ ..] => super::t18::n5323(s, out),
		['ы', s @ ..] => super::t18::n5324(s, out),
		['ю', s @ ..] => super::t18::n5325(s, out),
		['я', s @ ..] => super::t18::n5326(s, out),
		_ => false,
	}
}

/// Words ending -оивать.
pub(crate) fn n1436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['в', s @ ..] => {
			match s {
				[] => true,
				['д', 'у', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['к', 'о', 'п', 'с', s @ ..] => {
			match s {
				[] => true,
				['е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'т', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
		['т', 'с', 'о', 'д', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пивать.
pub(crate) fn n1437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4691);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4692);
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
			super::push(out, 4693);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11800);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20443);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4694);
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
			super::push(out, 4695);
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

/// Words ending -ривать.
pub(crate) fn n1438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1357);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4696);
			match s {
				[] => true,
				['б', 'а', 'т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11801);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20444);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => super::t18::n29538(s, out),
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
				['г', 'е', 'ч', 'о', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => super::t18::n14989(s, out),
				['ж', s @ ..] => super::t18::n14990(s, out),
				['з', 'о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => super::t18::n14992(s, out),
				['п', s @ ..] => super::t18::n14993(s, out),
				['с', s @ ..] => super::t18::n14994(s, out),
				['т', s @ ..] => super::t18::n14995(s, out),
				['у', s @ ..] => super::t18::n14996(s, out),
				['х', s @ ..] => super::t18::n14997(s, out),
				['ш', s @ ..] => super::t18::n14998(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t18::n5344(s, out),
		['г', s @ ..] => super::t18::n5345(s, out),
		['д', s @ ..] => super::t18::n5346(s, out),
		['е', s @ ..] => super::t18::n5347(s, out),
		['з', 'о', 'д', 'о', 'п', 'а', 'з', '\u{1}', s @ ..] => super::t18::n5348(s, out),
		['и', s @ ..] => super::t18::n5349(s, out),
		['о', s @ ..] => super::t18::n5350(s, out),
		['т', s @ ..] => super::t18::n5351(s, out),
		['у', s @ ..] => super::t18::n5352(s, out),
		['х', s @ ..] => super::t18::n5353(s, out),
		['ы', s @ ..] => super::t18::n5354(s, out),
		['ю', s @ ..] => super::t18::n5355(s, out),
		['я', s @ ..] => super::t18::n5356(s, out),
		_ => false,
	}
}

/// Words ending -хивать.
pub(crate) fn n1440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1359);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4709);
			match s {
				[] => true,
				['б', 'а', 'б', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11837);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20488);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20489);
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
							super::push(out, 20490);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28752);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											super::push(out, 34617);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', '\u{1}', s @ ..] => {
									super::push(out, 28753);
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
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 28754);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20491);
							match s {
								[] => true,
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
							super::push(out, 20492);
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
				['п', s @ ..] => {
					super::push(out, 11838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20493);
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
							super::push(out, 20494);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28755);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34618);
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
							super::push(out, 20495);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28756);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 20496);
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 28757);
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
									super::push(out, 28758);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
							super::push(out, 20497);
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11839);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
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
		['е', s @ ..] => {
			super::push(out, 4710);
			match s {
				[] => true,
				['д', 'з', 'б', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'б', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4711);
			match s {
				[] => true,
				['в', s @ ..] => super::t18::n15064(s, out),
				['п', s @ ..] => super::t18::n15065(s, out),
				['ч', s @ ..] => super::t18::n15066(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t18::n5361(s, out),
		['р', s @ ..] => super::t18::n5362(s, out),
		['у', s @ ..] => super::t18::n5363(s, out),
		['ы', s @ ..] => super::t18::n5364(s, out),
		['ю', s @ ..] => super::t18::n5365(s, out),
		['я', s @ ..] => super::t18::n5366(s, out),
		_ => false,
	}
}

/// Words ending -чивать.
pub(crate) fn n1441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1360);
	match s {
		[] => true,
		['а', s @ ..] => super::t18::n5367(s, out),
		['е', s @ ..] => {
			super::push(out, 4719);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11850);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20523);
							match s {
								[] => true,
								['л', 'е', 'ч', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
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
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'б', 'о', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 20524);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 28777);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 28778);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28779);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34629);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 34630);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 28780);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 34631);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 28781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 20525);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				['л', s @ ..] => {
					super::push(out, 11851);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', s @ ..] => {
									match s {
										[] => true,
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'б', 'о', '\u{1}', s @ ..] => {
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
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['д', 'е', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
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
								['р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['р', 'у', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11852);
					match s {
						[] => true,
						['е', 'р', 'д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t18::n5369(s, out),
		['л', s @ ..] => super::t18::n5370(s, out),
		['н', s @ ..] => super::t18::n5371(s, out),
		['о', s @ ..] => super::t18::n5372(s, out),
		['п', s @ ..] => {
			super::push(out, 4724);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => super::t18::n29917(s, out),
						['д', 'о', 'п', '\u{1}', s @ ..] => super::t18::n29918(s, out),
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t18::n29919(s, out),
						['о', s @ ..] => super::t18::n29920(s, out),
						['с', 'и', '\u{1}', s @ ..] => super::t18::n29921(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t18::n29922(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t18::n5374(s, out),
		['у', s @ ..] => super::t18::n5375(s, out),
		['ы', 'в', 'а', 'к', s @ ..] => super::t18::n5376(s, out),
		['ь', 'н', 'я', 'н', 'ы', 'в', '\u{1}', s @ ..] => super::t18::n5377(s, out),
		['ю', s @ ..] => super::t18::n5378(s, out),
		['я', s @ ..] => super::t18::n5379(s, out),
		_ => false,
	}
}

/// Words ending -шивать.
pub(crate) fn n1442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1361);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4729);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4730);
			match s {
				[] => true,
				['б', 'е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11873);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11874);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 20548);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['о', s @ ..] => super::t18::n30007(s, out),
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => super::t18::n47481(s, out),
								['и', '\u{1}', s @ ..] => super::t18::n47482(s, out),
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => super::t18::n30009(s, out),
						['у', '\u{1}', s @ ..] => super::t18::n30010(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t18::n30011(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t18::n15158(s, out),
				['н', s @ ..] => super::t18::n15159(s, out),
				['п', 'о', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => super::t18::n15160(s, out),
				['р', s @ ..] => super::t18::n15161(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t18::n5382(s, out),
		['в', s @ ..] => super::t18::n5383(s, out),
		['д', s @ ..] => super::t18::n5384(s, out),
		['е', s @ ..] => super::t18::n5385(s, out),
		['и', s @ ..] => super::t18::n5386(s, out),
		['о', s @ ..] => super::t18::n5387(s, out),
		['р', 'е', 'в', s @ ..] => super::t18::n5388(s, out),
		['с', s @ ..] => super::t18::n5389(s, out),
		['т', s @ ..] => super::t18::n5390(s, out),
		['у', s @ ..] => super::t18::n5391(s, out),
		['ы', s @ ..] => super::t18::n5392(s, out),
		['я', 'м', 'е', 'т', 'в', '\u{1}', s @ ..] => super::t18::n5393(s, out),
		_ => false,
	}
}

/// Words ending -щивать.
pub(crate) fn n1443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11896);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20563);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 28810);
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
				['г', s @ ..] => {
					super::push(out, 11897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20564);
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20565);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28811);
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11898);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
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
								['у', '\u{1}', s @ ..] => {
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
				['м', s @ ..] => {
					super::push(out, 11899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['л', 'х', 'а', 'з', '\u{1}', s @ ..] => {
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
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'с', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['р', s @ ..] => {
					super::push(out, 11900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20566);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => super::t18::n47575(s, out),
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => super::t18::n30134(s, out),
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => super::t18::n30136(s, out),
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => super::t18::n30138(s, out),
						['ы', 'в', '\u{1}', s @ ..] => super::t18::n30139(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t18::n5395(s, out),
		['о', s @ ..] => super::t18::n5396(s, out),
		['р', s @ ..] => super::t18::n5397(s, out),
		['у', s @ ..] => super::t18::n5398(s, out),
		['ю', s @ ..] => super::t18::n5399(s, out),
		_ => false,
	}
}

/// The word вытребовать.
pub(crate) fn n74152(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вытрамбовать.
pub(crate) fn n74153(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word попробовать.
pub(crate) fn n74154(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -спробовать.
pub(crate) fn n47606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28829);
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

/// The word отпробовать.
pub(crate) fn n47607(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рбовать.
pub(crate) fn n5403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4750);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11911);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28830);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28831);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбовать.
pub(crate) fn n5404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4751);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20573);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20574);
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

/// Words ending -вовать.
pub(crate) fn n1445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1364);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4752);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11913);
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
			super::push(out, 4753);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11914);
					match s {
						[] => true,
						['т', 'с', 'е', 'ш', 'е', 'т', 'у', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t18::n5407(s, out),
		['х', s @ ..] => {
			super::push(out, 4755);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11918);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20592);
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
		_ => false,
	}
}

/// Words ending -говать.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1365);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 4756);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11919);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20593);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34835);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 34836);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 38659);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81722(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 34837);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 38660);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81723(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11920);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28903);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 28904);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 28905);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34838);
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
									super::push(out, 28906);
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
		['и', s @ ..] => {
			super::push(out, 4757);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11921);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11922);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 20595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28907);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28908);
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
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11923);
					match s {
						[] => true,
						['т', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28909);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 28910);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 34839);
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
									super::push(out, 28911);
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
				['т', s @ ..] => {
					super::push(out, 11924);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20596);
							match s {
								[] => true,
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4758);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11925);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 28912);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34840);
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
				['е', s @ ..] => super::t18::n15232(s, out),
				['и', s @ ..] => super::t18::n15233(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4759);
			match s {
				[] => true,
				['т', 'с', s @ ..] => super::t18::n15234(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t18::n5413(s, out),
		['у', s @ ..] => super::t18::n5414(s, out),
		['ю', s @ ..] => super::t18::n5415(s, out),
		_ => false,
	}
}

/// Words ending -довать.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1366);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4763);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20624);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20625);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 28928);
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
							super::push(out, 20626);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 28929);
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
							super::push(out, 20627);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 28930);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34850);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 34851);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 38664);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81727(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 11933);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20628);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34852);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 34853);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 38665);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t18::n81728(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 34854);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 38666);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81729(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4764);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20629);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20630);
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
							super::push(out, 20631);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 28931);
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
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11935);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20632);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 28932);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t18::n30229(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t18::n15244(s, out),
				['р', s @ ..] => super::t18::n15245(s, out),
				['с', s @ ..] => super::t18::n15246(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4765);
			match s {
				[] => true,
				['а', s @ ..] => super::t18::n15247(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t18::n5419(s, out),
		['и', s @ ..] => super::t18::n5420(s, out),
		['л', s @ ..] => super::t18::n5421(s, out),
		['н', s @ ..] => super::t18::n5422(s, out),
		['о', s @ ..] => super::t18::n5423(s, out),
		['р', s @ ..] => super::t18::n5424(s, out),
		['у', s @ ..] => super::t18::n5425(s, out),
		['я', s @ ..] => super::t18::n5426(s, out),
		_ => false,
	}
}

/// Words ending -жовать.
pub(crate) fn n1448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1367);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 4774);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11956);
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
		_ => false,
	}
}

/// Words ending -зовать.
pub(crate) fn n1449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1368);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4775);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20652);
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11958);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 28969);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 38688);
													match s {
														[] => true,
														['п', s @ ..] => super::t18::n81756(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 38689);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81757(s, out),
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
									super::push(out, 28970);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4776);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11959);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20653);
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
			super::push(out, 4777);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 11960);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20654);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 34895);
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
				['р', s @ ..] => {
					super::push(out, 11961);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 20655);
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
		['и', s @ ..] => super::t18::n5431(s, out),
		['м', s @ ..] => {
			super::push(out, 4779);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11975);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29032);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29033);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 34959);
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
		['н', s @ ..] => {
			super::push(out, 4780);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11976);
					match s {
						[] => true,
						['р', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4781);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20684);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20685);
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
		['ь', s @ ..] => {
			super::push(out, 4782);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11978);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29034);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 34960);
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
									super::push(out, 29035);
									match s {
										[] => true,
										['и', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 38737);
													match s {
														[] => true,
														['р', 'е', 'п', '\u{1}', s @ ..] => super::t18::n81829(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 38738);
													match s {
														[] => true,
														['д', s @ ..] => super::t18::n81830(s, out),
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
									super::push(out, 29036);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34961);
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
		['ю', s @ ..] => super::t18::n5436(s, out),
		_ => false,
	}
}

/// Words ending -ковать.
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1369);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4784);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4785);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11979);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 20687);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29037);
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
					super::push(out, 11980);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 20688);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29038);
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
				['з', s @ ..] => {
					super::push(out, 11981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20689);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20690);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29039);
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
				['л', s @ ..] => {
					super::push(out, 11982);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 20691);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29040);
									s.is_empty()
								}
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
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11983);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29041);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 29042);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29043);
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
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20692);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 29044);
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
					super::push(out, 11984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11985);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20693);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20694);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20695);
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
					super::push(out, 11986);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20696);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 29045);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34962);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 34963);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 38739);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81831(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29046);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29047);
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29048);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 34964);
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
							super::push(out, 20698);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29049);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 34965);
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
					super::push(out, 11987);
					match s {
						[] => true,
						['а', s @ ..] => super::t18::n30326(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t18::n5439(s, out),
		['в', s @ ..] => super::t18::n5440(s, out),
		['д', s @ ..] => super::t18::n5441(s, out),
		['е', s @ ..] => super::t18::n5442(s, out),
		['и', s @ ..] => super::t18::n5443(s, out),
		['к', 'и', 'х', '\u{1}', s @ ..] => super::t18::n5444(s, out),
		['л', s @ ..] => super::t18::n5445(s, out),
		['н', s @ ..] => super::t18::n5446(s, out),
		['о', s @ ..] => super::t18::n5447(s, out),
		['п', s @ ..] => super::t18::n5448(s, out),
		['р', s @ ..] => super::t18::n5449(s, out),
		['с', s @ ..] => super::t18::n5450(s, out),
		['т', s @ ..] => super::t18::n5451(s, out),
		['у', s @ ..] => super::t18::n5452(s, out),
		['ч', s @ ..] => super::t18::n5453(s, out),
		['ш', s @ ..] => super::t18::n5454(s, out),
		['ы', s @ ..] => super::t18::n5455(s, out),
		['ь', s @ ..] => super::t18::n5456(s, out),
		['ю', s @ ..] => super::t18::n5457(s, out),
		['я', s @ ..] => super::t18::n5458(s, out),
		_ => false,
	}
}

/// Words ending -ловать.
pub(crate) fn n1451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1370);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4806);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20764);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20765);
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
							super::push(out, 20766);
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
						['с', s @ ..] => {
							super::push(out, 20767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12048);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20768);
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
				['ж', s @ ..] => {
					super::push(out, 12049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20769);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20770);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29109);
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
							super::push(out, 20771);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29110);
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
							super::push(out, 20772);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29111);
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
							super::push(out, 20773);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29112);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 12050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12051);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20775);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29113);
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
			super::push(out, 4807);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12052);
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
			super::push(out, 4808);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 12054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20777);
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
						['о', s @ ..] => {
							super::push(out, 20778);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20779);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4809);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12055);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20780);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 34992);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 38751);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t18::n81845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20781);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20782);
							match s {
								[] => true,
								['р', s @ ..] => super::t18::n48038(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t18::n30441(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t18::n15367(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t18::n5463(s, out),
		['о', s @ ..] => super::t18::n5464(s, out),
		['с', s @ ..] => super::t18::n5465(s, out),
		['т', s @ ..] => super::t18::n5466(s, out),
		['у', s @ ..] => super::t18::n5467(s, out),
		_ => false,
	}
}

/// Words ending -мовать.
pub(crate) fn n1452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4815);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12064);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20788);
							match s {
								[] => true,
								['т', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 4816);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20789);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20790);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20791);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29116);
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
		['р', s @ ..] => {
			super::push(out, 4817);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12066);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 20792);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29117);
									match s {
										[] => true,
										['о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 12067);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20793);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 29118);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34996);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 34997);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 38753);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81847(s, out),
														_ => false,
													}
												}
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
							super::push(out, 20794);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29119);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29120);
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
									super::push(out, 29121);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 29123);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 34998);
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
					super::push(out, 12068);
					match s {
						[] => true,
						['т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 29124);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 34999);
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
			super::push(out, 4818);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12069);
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
		['у', s @ ..] => {
			super::push(out, 4819);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 12070);
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
		['ф', s @ ..] => {
			super::push(out, 4820);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12071);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29125);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 35000);
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
									super::push(out, 29126);
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
		['ь', s @ ..] => {
			super::push(out, 4821);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12072);
					match s {
						[] => true,
						['е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29127);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 29128);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 29129);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => super::t18::n63272(s, out),
										_ => false,
									}
								}
								['о', s @ ..] => super::t18::n48064(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => super::t18::n5475(s, out),
		_ => false,
	}
}

/// Words ending -новать.
pub(crate) fn n1453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4823);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12074);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20795);
							match s {
								[] => true,
								['ч', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12075);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20796);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29131);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29132);
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
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20797);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20798);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20799);
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
					super::push(out, 12077);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20800);
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
		['в', s @ ..] => {
			super::push(out, 4824);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20801);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20802);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20803);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29133);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29134);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 35001);
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
						['и', s @ ..] => {
							super::push(out, 20804);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20805);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29135);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29136);
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
							super::push(out, 20806);
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
		['г', s @ ..] => {
			super::push(out, 4825);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12078);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29137);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 35002);
											match s {
												[] => true,
												['т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 29138);
									match s {
										[] => true,
										['а', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 38754);
													match s {
														[] => true,
														['р', s @ ..] => super::t18::n81848(s, out),
														_ => false,
													}
												}
												['з', s @ ..] => {
													super::push(out, 38755);
													match s {
														[] => true,
														['а', s @ ..] => super::t18::n81849(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 38756);
													match s {
														[] => true,
														['т', s @ ..] => super::t18::n81850(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4826);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12079);
					match s {
						[] => true,
						['а', 'р', 'п', s @ ..] => super::t18::n30478(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12080);
					match s {
						[] => true,
						['л', s @ ..] => super::t18::n30479(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t18::n5480(s, out),
		['з', s @ ..] => super::t18::n5481(s, out),
		['и', s @ ..] => super::t18::n5482(s, out),
		['й', s @ ..] => super::t18::n5483(s, out),
		['л', s @ ..] => super::t18::n5484(s, out),
		['о', s @ ..] => super::t18::n5485(s, out),
		['р', s @ ..] => super::t18::n5486(s, out),
		['с', s @ ..] => super::t18::n5487(s, out),
		_ => false,
	}
}

/// Words ending -повать.
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4835);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12096);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20823);
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
					super::push(out, 12097);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20824);
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
		['и', s @ ..] => {
			super::push(out, 4836);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12098);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 12099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12100);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20825);
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
					super::push(out, 12101);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 12103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20826);
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
		['й', s @ ..] => {
			super::push(out, 4837);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12104);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29162);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 35010);
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
		['м', s @ ..] => {
			super::push(out, 4838);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12105);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20827);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29163);
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
							super::push(out, 20828);
							match s {
								[] => true,
								['ш', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35011);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 35012);
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
										['о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 35013);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 38760);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81854(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4839);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12106);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29164);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 29165);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 20829);
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
			super::push(out, 4840);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12107);
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
		['у', s @ ..] => {
			super::push(out, 4841);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12108);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12109);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20830);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29166);
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
		['э', s @ ..] => {
			super::push(out, 4842);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровать.
pub(crate) fn n1455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1374);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4843);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12111);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12112);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20831);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 20833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20834);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20835);
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
				['ш', s @ ..] => {
					super::push(out, 12114);
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
			super::push(out, 4844);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12115);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 20836);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 20837);
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
					super::push(out, 12116);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20838);
							match s {
								[] => true,
								['а', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35014);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 35015);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 38761);
													match s {
														[] => true,
														['а', s @ ..] => super::t18::n81855(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 38762);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t18::n81856(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 35016);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													super::push(out, 38763);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12117);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20839);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35017);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 35018);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 38764);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t18::n81857(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4845);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12118);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20840);
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
			super::push(out, 4846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12119);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20841);
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
				['е', s @ ..] => {
					super::push(out, 12120);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12121);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20843);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 29168);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 35019);
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
								['м', s @ ..] => {
									super::push(out, 29169);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 35020);
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
						['и', s @ ..] => {
							super::push(out, 20844);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 29170);
									match s {
										[] => true,
										['и', 'ц', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t18::n15435(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t18::n5500(s, out),
		['и', s @ ..] => super::t19::n5501(s, out),
		['о', s @ ..] => super::t19::n5502(s, out),
		['т', s @ ..] => super::t19::n5503(s, out),
		['у', s @ ..] => super::t19::n5504(s, out),
		['ф', s @ ..] => super::t19::n5505(s, out),
		['ю', s @ ..] => super::t19::n5506(s, out),
		_ => false,
	}
}

/// Words ending -совать.
pub(crate) fn n1456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1375);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4854);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4855);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12176);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 21069);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
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
										['с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 12177);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21071);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 21072);
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
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12179);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 21073);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 21074);
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
					super::push(out, 12180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21075);
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
				['ф', s @ ..] => {
					super::push(out, 12181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21076);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21077);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21078);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4857);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12182);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21079);
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
			super::push(out, 4858);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12183);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21080);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 29740);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 29741);
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
					super::push(out, 12184);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21081);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35813);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 35814);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 35815);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 21082);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29742);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35816);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => super::t19::n48811(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t19::n5512(s, out),
		['к', s @ ..] => super::t19::n5513(s, out),
		['о', s @ ..] => super::t19::n5514(s, out),
		['п', s @ ..] => super::t19::n5515(s, out),
		['р', s @ ..] => super::t19::n5516(s, out),
		['с', s @ ..] => super::t19::n5517(s, out),
		['у', s @ ..] => super::t19::n5518(s, out),
		['ю', s @ ..] => super::t19::n5519(s, out),
		_ => false,
	}
}

/// Words ending -товать.
pub(crate) fn n1457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4867);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12198);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29767);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29768);
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
					super::push(out, 12199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21109);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21110);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 21111);
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
				['п', s @ ..] => {
					super::push(out, 12200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21112);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21113);
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
				['р', s @ ..] => {
					super::push(out, 12201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21114);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 21115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21116);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29769);
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
							super::push(out, 21117);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29770);
									s.is_empty()
								}
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
			super::push(out, 4868);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12202);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21118);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29771);
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
				['в', s @ ..] => {
					super::push(out, 12203);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21119);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 29772);
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
							super::push(out, 21120);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35841);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 35842);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 39565);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83052(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 35843);
											match s {
												[] => true,
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 35844);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													super::push(out, 39566);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 35845);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 35846);
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
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21121);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21122);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 29773);
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
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12205);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21123);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35847);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 35848);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 35849);
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21124);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21125);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29774);
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
		['и', s @ ..] => super::t19::n5522(s, out),
		['к', s @ ..] => super::t19::n5523(s, out),
		['л', s @ ..] => super::t19::n5524(s, out),
		['н', s @ ..] => super::t19::n5525(s, out),
		['о', s @ ..] => super::t19::n5526(s, out),
		['п', s @ ..] => super::t19::n5527(s, out),
		['р', s @ ..] => super::t19::n5528(s, out),
		['с', s @ ..] => super::t19::n5529(s, out),
		['т', s @ ..] => super::t19::n5530(s, out),
		['у', s @ ..] => super::t19::n5531(s, out),
		['ф', s @ ..] => super::t19::n5532(s, out),
		['х', s @ ..] => super::t19::n5533(s, out),
		['ч', s @ ..] => super::t19::n5534(s, out),
		['ы', s @ ..] => super::t19::n5535(s, out),
		['ь', s @ ..] => super::t19::n5536(s, out),
		['э', s @ ..] => super::t19::n5537(s, out),
		['ю', s @ ..] => super::t19::n5538(s, out),
		['я', s @ ..] => super::t19::n5539(s, out),
		_ => false,
	}
}

/// Words ending -фовать.
pub(crate) fn n1458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1377);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4887);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12253);
					match s {
						[] => true,
						['т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29864);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29865);
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
			super::push(out, 4888);
			match s {
				[] => true,
				['л', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21196);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21197);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 21198);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21199);
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
		['и', s @ ..] => {
			super::push(out, 4889);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12254);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29866);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
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
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29867);
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 29868);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 35900);
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
				['р', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 4890);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12255);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29869);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29870);
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
				['е', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 21200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29871);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29872);
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
						['л', s @ ..] => {
							super::push(out, 21201);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 29873);
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
							super::push(out, 21202);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35901);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 35902);
											match s {
												[] => true,
												['з', s @ ..] => super::t19::n76135(s, out),
												_ => false,
											}
										}
										['и', s @ ..] => super::t19::n64563(s, out),
										['о', s @ ..] => super::t19::n64564(s, out),
										['с', s @ ..] => super::t19::n64565(s, out),
										['т', s @ ..] => super::t19::n64566(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t19::n5544(s, out),
		['р', s @ ..] => super::t19::n5545(s, out),
		_ => false,
	}
}

/// Words ending -ховать.
pub(crate) fn n1459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1378);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4893);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4894);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12260);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29874);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29875);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 29876);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['ш', s @ ..] => {
					super::push(out, 12261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4895);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 12262);
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
		['и', s @ ..] => {
			super::push(out, 4896);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12263);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29877);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 29878);
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
				['р', s @ ..] => {
					super::push(out, 12264);
					match s {
						[] => true,
						['т', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29879);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29880);
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
				['с', s @ ..] => {
					super::push(out, 12265);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21207);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29881);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 35907);
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
		['л', s @ ..] => {
			super::push(out, 4897);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12266);
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
		['о', s @ ..] => {
			super::push(out, 4898);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12267);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29882);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 29883);
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
			super::push(out, 4899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 4900);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12269);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовать.
pub(crate) fn n1460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1379);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4901);
			match s {
				[] => true,
				['р', 'п', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4902);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21208);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21209);
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
						_ => false,
					}
				}
				['я', 'л', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21210);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21211);
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
		['п', 'у', 'л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12270);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21212);
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
			super::push(out, 4903);
			match s {
				[] => true,
				['а', 'в', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4904);
			match s {
				[] => true,
				['л', 'а', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word схлебывать.
pub(crate) fn n64576(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word расхлебывать.
pub(crate) fn n64577(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отхлебывать.
pub(crate) fn n49042(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ухлебывать.
pub(crate) fn n49043(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выхлебывать.
pub(crate) fn n49044(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оебывать.
pub(crate) fn n15629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12278);
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

/// Words ending -ребывать.
pub(crate) fn n15630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12279);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 21215);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 29887);
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
		['к', s @ ..] => {
			super::push(out, 21216);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29888);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
		['п', s @ ..] => {
			super::push(out, 21217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29889);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35908);
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

/// The word выстебывать.
pub(crate) fn n15631(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -уебывать.
pub(crate) fn n15632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12280);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 21218);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъебывать.
pub(crate) fn n15633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12281);
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 21219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29890);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35909);
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 21220);
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
		['с', s @ ..] => {
			super::push(out, 21221);
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

/// The word выебывать.
pub(crate) fn n15634(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -збывать.
pub(crate) fn n5584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4914);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12282);
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

/// Words ending -ибывать.
pub(crate) fn n5585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4915);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12283);
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

/// Words ending -лбывать.
pub(crate) fn n5586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4916);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12284);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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

/// Words ending -обывать.
pub(crate) fn n5587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4917);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 12285);
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
		['п', s @ ..] => {
			super::push(out, 12286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12287);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21222);
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

/// Words ending -сбывать.
pub(crate) fn n5588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4918);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тбывать.
pub(crate) fn n5589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4919);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12288);
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

/// Words ending -убывать.
pub(crate) fn n5590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4920);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12289);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21223);
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

/// Words ending -ыбывать.
pub(crate) fn n5591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4921);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12290);
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

/// Words ending -ябывать.
pub(crate) fn n5592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4922);
	match s {
		[] => true,
		['л', 'х', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12291);
			match s {
				[] => true,
				['а', 'к', 'о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вывать.
pub(crate) fn n1471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4923);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4924);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12294);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4925);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 12295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['е', s @ ..] => {
							super::push(out, 21226);
							match s {
								[] => true,
								['в', 'с', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 29891);
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
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => super::t19::n64590(s, out),
										['т', s @ ..] => super::t19::n64591(s, out),
										['ы', 'в', '\u{1}', s @ ..] => super::t19::n64592(s, out),
										_ => false,
									}
								}
								['р', s @ ..] => super::t19::n49064(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => super::t19::n31036(s, out),
						['о', s @ ..] => super::t19::n31037(s, out),
						['с', '\u{1}', s @ ..] => super::t19::n31038(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t19::n31039(s, out),
						['я', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => super::t19::n31040(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t19::n15653(s, out),
				['л', s @ ..] => super::t19::n15654(s, out),
				['н', 'и', 'л', 'з', 'а', 'р', '\u{1}', s @ ..] => super::t19::n15655(s, out),
				['о', 'в', 'о', s @ ..] => super::t19::n15656(s, out),
				['р', s @ ..] => super::t19::n15657(s, out),
				['ч', s @ ..] => super::t19::n15658(s, out),
				['ш', s @ ..] => super::t19::n15659(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t19::n5596(s, out),
		['и', s @ ..] => super::t19::n5597(s, out),
		['о', s @ ..] => super::t19::n5598(s, out),
		['т', s @ ..] => super::t19::n5599(s, out),
		_ => false,
	}
}

/// The word ошмурыгывать.
pub(crate) fn n1472(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -дывать.
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1390);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4930);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12318);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 21250);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29911);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 29912);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35930);
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 21251);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 35931);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39608);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83133(s, out),
														_ => false,
													}
												}
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				['л', s @ ..] => {
					super::push(out, 12319);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
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
								['ы', 'в', '\u{1}', s @ ..] => {
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
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t19::n76311(s, out),
												['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t19::n76312(s, out),
												['о', 'р', 'п', '\u{1}', s @ ..] => super::t19::n76313(s, out),
												_ => false,
											}
										}
										['н', '\u{1}', s @ ..] => super::t19::n64834(s, out),
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => super::t19::n49287(s, out),
								['в', '\u{1}', s @ ..] => super::t19::n49288(s, out),
								['д', 'о', 'п', '\u{1}', s @ ..] => super::t19::n49289(s, out),
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t19::n49290(s, out),
								['и', 'р', 'п', '\u{1}', s @ ..] => super::t19::n49291(s, out),
								['о', s @ ..] => super::t19::n49292(s, out),
								['с', s @ ..] => super::t19::n49293(s, out),
								['т', 'о', '\u{1}', s @ ..] => super::t19::n49294(s, out),
								['у', s @ ..] => super::t19::n49295(s, out),
								['ы', 'в', '\u{1}', s @ ..] => super::t19::n49296(s, out),
								_ => false,
							}
						}
						['о', 'г', s @ ..] => super::t19::n31153(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t19::n15682(s, out),
				['р', s @ ..] => super::t19::n15683(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t19::n5601(s, out),
		['з', s @ ..] => super::t19::n5602(s, out),
		['и', s @ ..] => super::t19::n5603(s, out),
		['р', s @ ..] => super::t19::n5604(s, out),
		['у', 'г', 'о', 'п', '\u{1}', s @ ..] => super::t19::n5605(s, out),
		['ы', s @ ..] => super::t19::n5606(s, out),
		['я', s @ ..] => super::t19::n5607(s, out),
		_ => false,
	}
}

/// Words ending -зывать.
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1391);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4937);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12332);
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
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 21272);
							s.is_empty()
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
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 21273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 29928);
									match s {
										[] => true,
										['е', 'р', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 39611);
													match s {
														[] => true,
														['п', s @ ..] => super::t19::n83138(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
								['с', s @ ..] => {
									super::push(out, 29929);
									match s {
										[] => true,
										['а', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 39612);
													match s {
														[] => true,
														['н', s @ ..] => super::t19::n83139(s, out),
														_ => false,
													}
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
						['у', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12333);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21274);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'и', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 29930);
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
				['н', s @ ..] => super::t19::n15697(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4938);
			match s {
				[] => true,
				['о', s @ ..] => super::t19::n15698(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t19::n5611(s, out),
		['д', s @ ..] => super::t19::n5612(s, out),
		['е', s @ ..] => super::t19::n5613(s, out),
		['и', s @ ..] => super::t19::n5614(s, out),
		['л', s @ ..] => super::t19::n5615(s, out),
		['о', s @ ..] => super::t19::n5616(s, out),
		['р', s @ ..] => super::t19::n5617(s, out),
		['с', s @ ..] => super::t19::n5618(s, out),
		['т', s @ ..] => super::t19::n5619(s, out),
		['ы', s @ ..] => super::t19::n5620(s, out),
		['ь', s @ ..] => super::t19::n5621(s, out),
		['я', s @ ..] => super::t19::n5622(s, out),
		_ => false,
	}
}

/// Words ending -лывать.
pub(crate) fn n1475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1392);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4951);
			match s {
				[] => true,
				['ж', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21290);
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
						['в', s @ ..] => {
							super::push(out, 21291);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29938);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 35943);
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
									super::push(out, 29939);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 35944);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21292);
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
							super::push(out, 21293);
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
							super::push(out, 21294);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 35945);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39614);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83141(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12348);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12349);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4952);
			match s {
				[] => true,
				['е', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t19::n5625(s, out),
		['п', s @ ..] => super::t19::n5626(s, out),
		['с', s @ ..] => {
			super::push(out, 4955);
			match s {
				[] => true,
				['о', s @ ..] => super::t19::n15733(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мывать.
pub(crate) fn n1476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1393);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4956);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21304);
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
						['б', s @ ..] => {
							super::push(out, 21305);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 35949);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39616);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83143(s, out),
														_ => false,
													}
												}
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
							super::push(out, 21306);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 21307);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 35950);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39617);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83144(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12362);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29943);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 35951);
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
									super::push(out, 29944);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 35952);
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
								['и', s @ ..] => {
									super::push(out, 29945);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29946);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4957);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12363);
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
			super::push(out, 4958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12364);
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
			super::push(out, 4960);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12365);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21309);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
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
						['е', s @ ..] => {
							super::push(out, 21310);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29947);
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
		['з', s @ ..] => super::t19::n5634(s, out),
		['и', s @ ..] => {
			super::push(out, 4962);
			match s {
				[] => true,
				['р', s @ ..] => super::t19::n15744(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t19::n5636(s, out),
		['с', s @ ..] => super::t19::n5637(s, out),
		['т', s @ ..] => super::t19::n5638(s, out),
		['у', s @ ..] => super::t19::n5639(s, out),
		['ы', s @ ..] => super::t19::n5640(s, out),
		_ => false,
	}
}

/// Words ending -нывать.
pub(crate) fn n1477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1394);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4968);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12375);
					match s {
						[] => true,
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12376);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12377);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 29952);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29953);
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4969);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12378);
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
			super::push(out, 4970);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12379);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
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
			super::push(out, 4971);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4972);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 4973);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12381);
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
			super::push(out, 4974);
			match s {
				[] => true,
				['а', 'к', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4975);
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
		_ => false,
	}
}

/// Words ending -пывать.
pub(crate) fn n1478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1395);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4976);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21322);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 35958);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 39620);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n83149(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', s @ ..] => {
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
							super::push(out, 21323);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 35959);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 39621);
													match s {
														[] => true,
														['о', 'п', '\u{1}', s @ ..] => super::t19::n83150(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
						_ => false,
					}
				}
				['л', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12383);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21324);
							match s {
								[] => true,
								['ц', s @ ..] => {
									match s {
										[] => true,
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
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
						['к', s @ ..] => super::t19::n31400(s, out),
						['х', s @ ..] => super::t19::n31401(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t19::n15769(s, out),
				['х', 'а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => super::t19::n31404(s, out),
						['н', '\u{1}', s @ ..] => super::t19::n31405(s, out),
						_ => false,
					}
				}
				['ц', s @ ..] => super::t19::n15771(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t19::n5650(s, out),
		['и', s @ ..] => super::t19::n5651(s, out),
		['о', s @ ..] => super::t19::n5652(s, out),
		['р', s @ ..] => super::t19::n5653(s, out),
		['у', s @ ..] => super::t19::n5654(s, out),
		['ю', 'л', 'х', s @ ..] => super::t19::n5655(s, out),
		['я', s @ ..] => super::t19::n5656(s, out),
		_ => false,
	}
}

/// Words ending -рывать.
pub(crate) fn n1479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1396);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4983);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12387);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 21335);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 29966);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21336);
							match s {
								[] => true,
								['д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['в', s @ ..] => {
					super::push(out, 12388);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 29967);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 29968);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 29969);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 35964);
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
									super::push(out, 29970);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 29971);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 35965);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 35966);
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
				['г', s @ ..] => {
					super::push(out, 12389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21338);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29972);
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
					super::push(out, 12391);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21339);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4984);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21340);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 29973);
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
			super::push(out, 4985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t19::n5661(s, out),
		['д', s @ ..] => super::t19::n5662(s, out),
		['е', s @ ..] => super::t19::n5663(s, out),
		['з', s @ ..] => super::t19::n5664(s, out),
		['и', s @ ..] => super::t19::n5665(s, out),
		['к', s @ ..] => super::t19::n5666(s, out),
		['о', s @ ..] => super::t19::n5667(s, out),
		['с', s @ ..] => super::t19::n5668(s, out),
		['т', s @ ..] => super::t19::n5669(s, out),
		['у', s @ ..] => {
			super::push(out, 4995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t19::n15825(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t19::n5671(s, out),
		_ => false,
	}
}

