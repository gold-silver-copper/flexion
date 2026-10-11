use flexion::Path;

/// Words ending -сливать.
pub(crate) fn n5301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11765);
			match s {
				[] => true,
				['м', s @ ..] => {
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
		['у', s @ ..] => {
			super::push(out, 11766);
			match s {
				[] => true,
				['м', s @ ..] => {
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
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11767);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
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
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тливать.
pub(crate) fn n5302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4672);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11768);
			match s {
				[] => true,
				['в', 'с', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11769);
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

/// Words ending -уливать.
pub(crate) fn n5303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4673);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11770);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11771);
			match s {
				[] => true,
				['р', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11772);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20402);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28698);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20403);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
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
						['п', s @ ..] => {
							super::push(out, 28699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28700);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20404);
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
		['ж', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11773);
			match s {
				[] => true,
				['р', 'у', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20405);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28701);
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
		['п', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20406);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['т', 'у', 'с', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', 'и', 'м', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word проолифливать.
pub(crate) fn n5304(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -хливать.
pub(crate) fn n5305(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'х', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'р', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шливать.
pub(crate) fn n5306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4674);
	match s {
		[] => true,
		['а', 'к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20407);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28702);
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
				['о', s @ ..] => {
					super::push(out, 20408);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28703);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 28704);
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
					super::push(out, 20409);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'п', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыливать.
pub(crate) fn n5307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4675);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20411);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20412);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 34590);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 38421);
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
								['о', 'р', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11775);
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
				['з', s @ ..] => {
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11776);
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
						['р', 'п', '\u{1}', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
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
		['р', 'к', 'с', s @ ..] => {
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

/// Words ending -юливать.
pub(crate) fn n5308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4676);
	match s {
		[] => true,
		['д', 'з', 'и', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20413);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28705);
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
					super::push(out, 20414);
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
		['ъ', s @ ..] => {
			super::push(out, 11777);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20415);
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

/// Words ending -яливать.
pub(crate) fn n5309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4677);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11778);
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
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
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
				['ы', s @ ..] => {
					super::push(out, 20416);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 34591);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 38422);
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
		_ => false,
	}
}

/// Words ending -ганивать.
pub(crate) fn n14914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11781);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20419);
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20420);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
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
		['ы', 'ц', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -канивать.
pub(crate) fn n14915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11782);
	match s {
		[] => true,
		['а', 'т', 'с', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'ч', s @ ..] => {
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'а', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -манивать.
pub(crate) fn n14917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11783);
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'к', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
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
						['с', 'а', 'б', 'о', '\u{1}', s @ ..] => {
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
		['у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранивать.
pub(crate) fn n14918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11784);
	match s {
		[] => true,
		['а', 'т', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 20421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28711);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
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
		['г', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
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
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'т', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 20422);
			match s {
				[] => true,
				['б', s @ ..] => {
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
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -мещанивать.
pub(crate) fn n14919(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -внивать.
pub(crate) fn n5311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4679);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28712);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34597);
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

/// Words ending -гнивать.
pub(crate) fn n5312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4680);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11785);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20424);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20425);
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
			super::push(out, 11786);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20426);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 28713);
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
					super::push(out, 20427);
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
			super::push(out, 11787);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20428);
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
			super::push(out, 11788);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20429);
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
			super::push(out, 11789);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 20432);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20433);
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
			super::push(out, 11790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11791);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11792);
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

/// The word осаднивать.
pub(crate) fn n5313(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -енивать.
pub(crate) fn n5314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4681);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', 'щ', s @ ..] => {
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
				['у', 'б', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 11793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20434);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20435);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28714);
							match s {
								[] => true,
								['в', 'о', 'с', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 28715);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 34598);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20436);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28716);
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
				['с', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'к', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', 'т', 'у', 'б', 'з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11795);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
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

/// The word выпоражнивать.
pub(crate) fn n5315(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -знивать.
pub(crate) fn n5316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4682);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инивать.
pub(crate) fn n5317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4683);
	match s {
		[] => true,
		['в', 'о', 'л', 'о', 'п', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
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
		['ж', 'у', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'е', 'б', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11796);
			match s {
				[] => true,
				['а', 'т', 'ф', 'а', 'н', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20437);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28717);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 34599);
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
				['к', s @ ..] => {
					super::push(out, 20438);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'и', 'р', 'б', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['т', 'о', 'к', 'с', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онивать.
pub(crate) fn n5318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4684);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11797);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20439);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28718);
							match s {
								[] => true,
								['р', 'т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'е', 'р', s @ ..] => {
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'а', s @ ..] => {
			match s {
				[] => true,
				['з', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'д', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'о', 'к', 'е', 'д', 'о', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', 'и', 'л', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'у', 'с', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['о', 'в', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'а', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'а', 'ф', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word вычернивать.
pub(crate) fn n5319(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -снивать.
pub(crate) fn n5320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4685);
	match s {
		[] => true,
		['е', 'л', 'п', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'л', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 20440);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28719);
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

/// Words ending -унивать.
pub(crate) fn n5321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4686);
	match s {
		[] => true,
		['б', 'а', 'т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'р', 'а', 'г', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'т', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20441);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20442);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// The word опрозрачнивать.
pub(crate) fn n5322(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -шнивать.
pub(crate) fn n5323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4687);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['м', 'о', 'д', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28721);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ынивать.
pub(crate) fn n5324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4688);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11798);
			match s {
				[] => true,
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11799);
			match s {
				[] => true,
				['с', 'у', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнивать.
pub(crate) fn n5325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4689);
	match s {
		[] => true,
		['л', 'с', s @ ..] => {
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
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'и', '\u{1}', s @ ..] => {
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

/// Words ending -янивать.
pub(crate) fn n5326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4690);
	match s {
		[] => true,
		['в', 'а', 'л', 'с', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', 'у', 'р', s @ ..] => {
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'т', 'с', 'е', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оваривать.
pub(crate) fn n29538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20445);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34600);
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
				['з', s @ ..] => {
					super::push(out, 34601);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40742);
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
							super::push(out, 38423);
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
							super::push(out, 38424);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', s @ ..] => {
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
		['т', s @ ..] => {
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

/// Words ending -даривать.
pub(crate) fn n14989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11802);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20446);
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
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 20447);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28722);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34602);
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
				['с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жаривать.
pub(crate) fn n14990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11803);
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
		['в', s @ ..] => {
			super::push(out, 20448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28723);
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
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -маривать.
pub(crate) fn n14992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11804);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'ы', 'в', '\u{1}', s @ ..] => {
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
		['о', 'к', 'ы', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -паривать.
pub(crate) fn n14993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11805);
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
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
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
		['ш', s @ ..] => {
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
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -саривать.
pub(crate) fn n14994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11806);
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
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -таривать.
pub(crate) fn n14995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11807);
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
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', 'м', s @ ..] => {
			match s {
				[] => true,
				['з', 'и', '\u{1}', s @ ..] => {
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

/// Words ending -уаривать.
pub(crate) fn n14996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11808);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 20449);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28724);
					match s {
						[] => true,
						['с', 'к', 'и', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -харивать.
pub(crate) fn n14997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11809);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20450);
			match s {
				[] => true,
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
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -шаривать.
pub(crate) fn n14998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11810);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20451);
			s.is_empty()
		}
		['а', 'н', '\u{1}', s @ ..] => {
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
		['с', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бривать.
pub(crate) fn n5344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4697);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11811);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20452);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 20453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 11812);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20454);
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
		['е', 'р', 'е', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11813);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['о', s @ ..] => {
			super::push(out, 11814);
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
						['а', s @ ..] => {
							super::push(out, 28726);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 28727);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28728);
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
					super::push(out, 20456);
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
			super::push(out, 11815);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 11816);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20457);
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
			super::push(out, 11817);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20458);
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
						['з', 'и', '\u{1}', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 11818);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гривать.
pub(crate) fn n5345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4698);
	match s {
		[] => true,
		['а', 'б', s @ ..] => {
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
				['о', '\u{1}', s @ ..] => {
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

/// Words ending -дривать.
pub(crate) fn n5346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4699);
	match s {
		[] => true,
		['а', 'б', 'з', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'п', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -еривать.
pub(crate) fn n5347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4700);
	match s {
		[] => true,
		['м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20460);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['ш', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['щ', 'с', 'а', 'р', '\u{1}', s @ ..] => {
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
				['к', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
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
		['х', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ч', 'е', 'в', 'д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['щ', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word заподозривать.
pub(crate) fn n5348(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -иривать.
pub(crate) fn n5349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4701);
	match s {
		[] => true,
		['д', 'н', 'у', 'м', s @ ..] => {
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', 'р', 'т', s @ ..] => {
					match s {
						[] => true,
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'о', 'х', 'и', 'т', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оривать.
pub(crate) fn n5350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4702);
	match s {
		[] => true,
		['б', 'и', 'р', 'п', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 11819);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['г', s @ ..] => {
			super::push(out, 11820);
			match s {
				[] => true,
				['е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11821);
			match s {
				[] => true,
				['а', 'з', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', 'п', 'ш', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28729);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 34603);
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
							super::push(out, 28730);
							match s {
								[] => true,
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
		['к', s @ ..] => {
			super::push(out, 11822);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 20461);
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
		['п', s @ ..] => {
			super::push(out, 11823);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20462);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28731);
							match s {
								[] => true,
								['с', 'а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 34604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28732);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28733);
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
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20464);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 34605);
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 34606);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 34607);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38425);
											s.is_empty()
										}
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
					super::push(out, 20465);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20466);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20467);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28734);
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
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11824);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11825);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20468);
					match s {
						[] => true,
						['о', 'ф', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тривать.
pub(crate) fn n5351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4703);
	match s {
		[] => true,
		['а', 'м', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20469);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20470);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28735);
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
				['д', s @ ..] => {
					super::push(out, 20471);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28736);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 34608);
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
							super::push(out, 28737);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20472);
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
						['п', s @ ..] => {
							super::push(out, 28738);
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
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20473);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28739);
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
		['е', s @ ..] => {
			super::push(out, 11826);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'х', 'ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', 'о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'в', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
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

/// Words ending -уривать.
pub(crate) fn n5352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4704);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11827);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11828);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20474);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28740);
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
		['ж', s @ ..] => {
			super::push(out, 11829);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28741);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 34609);
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
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
		['з', 'а', 'л', 'г', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20475);
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
					super::push(out, 20476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28742);
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
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
				['ш', s @ ..] => {
					match s {
						[] => true,
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
		['м', s @ ..] => {
			super::push(out, 11831);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20477);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20478);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28743);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 34610);
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
				['х', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 11832);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20479);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => super::t25::n62453(s, out),
								['у', s @ ..] => super::t25::n62454(s, out),
								_ => false,
							}
						}
						['р', 'е', 'т', 'и', 'л', s @ ..] => super::t25::n47134(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t25::n29701(s, out),
				['н', s @ ..] => super::t25::n29702(s, out),
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ь', 'л', 'у', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', 'и', 'м', 'б', 'о', '\u{1}', s @ ..] => super::t25::n15044(s, out),
		['щ', s @ ..] => super::t25::n15045(s, out),
		_ => false,
	}
}

/// Words ending -хривать.
pub(crate) fn n5353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4705);
	match s {
		[] => true,
		['и', 'в', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20482);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28746);
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
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыривать.
pub(crate) fn n5354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4706);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11834);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
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
						_ => false,
					}
				}
				['ш', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
								['с', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['з', 'у', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11835);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20483);
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
				['ш', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 28747);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 34615);
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 28748);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 34616);
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
					super::push(out, 20484);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11836);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20485);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28750);
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
		['ф', 'у', 'ф', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юривать.
pub(crate) fn n5355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4707);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20486);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28751);
							match s {
								[] => true,
								['п', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20487);
					match s {
						[] => true,
						['е', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'п', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'б', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яривать.
pub(crate) fn n5356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4708);
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
		['о', 'б', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', 'х', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -вихивать.
pub(crate) fn n15064(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пихивать.
pub(crate) fn n15065(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20498);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28759);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 34620);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38426);
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чихивать.
pub(crate) fn n15066(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -охивать.
pub(crate) fn n5361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4712);
	match s {
		[] => true,
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 11840);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 20499);
					match s {
						[] => true,
						['с', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рхивать.
pub(crate) fn n5362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4713);
	match s {
		[] => true,
		['а', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 20500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
					super::push(out, 20501);
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20503);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ухивать.
pub(crate) fn n5363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4714);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20504);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28762);
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11841);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28763);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 28764);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34621);
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

/// Words ending -ыхивать.
pub(crate) fn n5364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4715);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11842);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20505);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 28765);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'о', 'р', 'г', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20507);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28766);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 28767);
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
		['п', s @ ..] => {
			super::push(out, 11843);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20508);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20509);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20510);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28768);
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юхивать.
pub(crate) fn n5365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4716);
	match s {
		[] => true,
		['н', s @ ..] => {
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20511);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34622);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яхивать.
pub(crate) fn n5366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4717);
	match s {
		[] => true,
		['р', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20512);
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
					super::push(out, 20513);
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
					super::push(out, 20514);
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
				['ы', s @ ..] => {
					super::push(out, 20515);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 34623);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 38427);
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
		_ => false,
	}
}

/// Words ending -ачивать.
pub(crate) fn n5367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4718);
	match s {
		[] => true,
		['б', 'о', 'с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
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
				['и', 'ф', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'н', 'а', 'м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11844);
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
						['л', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28770);
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11845);
			match s {
				[] => true,
				['е', 'щ', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', 'о', 'в', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20516);
					match s {
						[] => true,
						['б', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
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
									super::push(out, 34624);
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
								['с', s @ ..] => super::t25::n62507(s, out),
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => super::t25::n62509(s, out),
								['ы', 'в', '\u{1}', s @ ..] => super::t25::n62510(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => super::t25::n47248(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t25::n29821(s, out),
				['х', 'о', 'х', 'с', 'в', '\u{1}', s @ ..] => super::t25::n29822(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t25::n15085(s, out),
		['н', s @ ..] => super::t25::n15086(s, out),
		['п', s @ ..] => super::t25::n15087(s, out),
		['р', s @ ..] => super::t25::n15088(s, out),
		['с', s @ ..] => super::t25::n15089(s, out),
		['т', s @ ..] => super::t25::n15090(s, out),
		['х', 'о', 'а', 'н', '\u{1}', s @ ..] => super::t25::n15091(s, out),
		_ => false,
	}
}

/// Words ending -ичивать.
pub(crate) fn n5369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4720);
	match s {
		[] => true,
		['в', 'е', 'ш', 'ь', 'л', 'о', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', 'о', 'л', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ж', 'у', 'м', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'о', 'л', 'о', 'в', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11853);
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
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
						_ => false,
					}
				}
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', 'о', 'м', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'а', 'к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20526);
					match s {
						[] => true,
						['р', 'г', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20527);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 34632);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 38430);
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
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
				['н', s @ ..] => {
					super::push(out, 20528);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28782);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 34633);
									match s {
										[] => true,
										['о', 'м', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11855);
			match s {
				[] => true,
				['р', 'и', 'к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20529);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лчивать.
pub(crate) fn n5370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4721);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20530);
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 20531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28784);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 34634);
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
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34635);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 34636);
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
							super::push(out, 28785);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 28787);
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
		['о', s @ ..] => {
			super::push(out, 11856);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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

/// Words ending -нчивать.
pub(crate) fn n5371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4722);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11857);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 28788);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 38431);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 40746);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n85601(s, out),
														_ => false,
													}
												}
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'ф', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11858);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28789);
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			match s {
				[] => true,
				['л', 'к', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['о', 'д', '\u{1}', s @ ..] => {
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

/// Words ending -очивать.
pub(crate) fn n5372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4723);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11859);
			match s {
				[] => true,
				['а', 'р', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'с', 'о', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11860);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 20532);
					match s {
						[] => true,
						['р', 'о', 'п', 'у', s @ ..] => {
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
		['к', 'о', 'л', 'к', 'с', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11861);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
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
				['п', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11862);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20533);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20534);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28790);
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
					super::push(out, 20535);
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
			super::push(out, 11863);
			match s {
				[] => true,
				['к', 'н', 'а', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11864);
			match s {
				[] => true,
				['е', 'п', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'у', 'в', 'д', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20536);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 28791);
							match s {
								[] => true,
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20537);
					match s {
						[] => true,
						['д', 'е', 'р', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 20538);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28792);
							match s {
								[] => true,
								['у', 'з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 11866);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20539);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28793);
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
		_ => false,
	}
}

/// Words ending -акапчивать.
pub(crate) fn n29917(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word подкапчивать.
pub(crate) fn n29918(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перекапчивать.
pub(crate) fn n29919(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -окапчивать.
pub(crate) fn n29920(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word искапчивать.
pub(crate) fn n29921(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выкапчивать.
pub(crate) fn n29922(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рчивать.
pub(crate) fn n5374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4725);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
								['п', '\u{1}', s @ ..] => {
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
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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
		['о', 'к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учивать.
pub(crate) fn n5375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4726);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 11867);
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
				['е', 'ч', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'ч', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', 'з', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', 'а', 'л', 'б', 'о', '\u{1}', s @ ..] => {
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
		['г', 'р', 'у', 'с', 'а', 'н', '\u{1}', s @ ..] => {
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 11868);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20540);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 28794);
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28795);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 34637);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'б', 'а', 'к', s @ ..] => {
			match s {
				[] => true,
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 11869);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', 'а', 'б', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
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
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11870);
			match s {
				[] => true,
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
										['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['с', s @ ..] => super::t25::n15139(s, out),
		['т', s @ ..] => super::t25::n15140(s, out),
		['ш', s @ ..] => super::t25::n15141(s, out),
		['щ', s @ ..] => {
			match s {
				[] => true,
				['и', 'р', 'п', '\u{1}', s @ ..] => super::t25::n29973(s, out),
				['у', '\u{1}', s @ ..] => super::t25::n29974(s, out),
				_ => false,
			}
		}
		['ы', 'в', s @ ..] => super::t25::n15143(s, out),
		_ => false,
	}
}

/// Words ending -кавычивать.
pub(crate) fn n5376(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word выняньчивать.
pub(crate) fn n5377(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ючивать.
pub(crate) fn n5378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4727);
	match s {
		[] => true,
		['р', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', 'в', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20546);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28800);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 34639);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
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

/// Words ending -ячивать.
pub(crate) fn n5379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4728);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['к', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', 'о', 'х', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'к', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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
					super::push(out, 20547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28801);
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
		['р', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'х', s @ ..] => {
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
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

/// Words ending -окашивать.
pub(crate) fn n30007(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word раскашивать.
pub(crate) fn n47481(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word искашивать.
pub(crate) fn n47482(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word откашивать.
pub(crate) fn n30009(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word укашивать.
pub(crate) fn n30010(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выкашивать.
pub(crate) fn n30011(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лашивать.
pub(crate) fn n15158(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нашивать.
pub(crate) fn n15159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11876);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20549);
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
			super::push(out, 20550);
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
				['л', 'г', 'о', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
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

/// The word обезопашивать.
pub(crate) fn n15160(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рашивать.
pub(crate) fn n15161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11877);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 20551);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
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
				['х', s @ ..] => {
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 20552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28802);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28803);
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						['с', 'а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38432);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40747);
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
							super::push(out, 34640);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40748);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42161);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n87974(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
				['ы', s @ ..] => {
					super::push(out, 28805);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38433);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40749);
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
		['т', 'о', 'п', 'ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бшивать.
pub(crate) fn n5382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4731);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11878);
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

/// Words ending -вшивать.
pub(crate) fn n5383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4732);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дшивать.
pub(crate) fn n5384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4733);
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11879);
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

/// Words ending -ешивать.
pub(crate) fn n5385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4734);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11880);
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
						['н', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
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
						['з', 'и', '\u{1}', s @ ..] => {
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
			match s {
				[] => true,
				['е', 'т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
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
				['в', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11881);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20553);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 28806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ишивать.
pub(crate) fn n5386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4735);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11882);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20554);
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

/// Words ending -ошивать.
pub(crate) fn n5387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4736);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11883);
			match s {
				[] => true,
				['е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11884);
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
		['к', s @ ..] => {
			super::push(out, 11885);
			match s {
				[] => true,
				['о', 'к', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20555);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20556);
					match s {
						[] => true,
						['л', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20557);
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
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20558);
					match s {
						[] => true,
						['ъ', 'з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20559);
					match s {
						[] => true,
						['г', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20560);
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

/// Words ending -вершивать.
pub(crate) fn n5388(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сшивать.
pub(crate) fn n5389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4737);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11886);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11887);
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

/// Words ending -тшивать.
pub(crate) fn n5390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4738);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11888);
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

/// Words ending -ушивать.
pub(crate) fn n5391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4739);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11889);
			s.is_empty()
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20561);
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11891);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20562);
					match s {
						[] => true,
						['ш', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28807);
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 28808);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 34641);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 38434);
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
							super::push(out, 28809);
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
		['о', 'л', 'г', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11892);
			match s {
				[] => true,
				['т', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
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
			super::push(out, 11893);
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
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
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
		['т', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ышивать.
pub(crate) fn n5392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4740);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11894);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11895);
			match s {
				[] => true,
				['с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word втемяшивать.
pub(crate) fn n5393(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -таращивать.
pub(crate) fn n47575(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 34642);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40750);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42162);
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

/// Words ending -зращивать.
pub(crate) fn n30134(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -оращивать.
pub(crate) fn n30136(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -тращивать.
pub(crate) fn n30138(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', 'о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word выращивать.
pub(crate) fn n30139(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ещивать.
pub(crate) fn n5395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4742);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20567);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28812);
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
		['л', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11902);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 34643);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38435);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ощивать.
pub(crate) fn n5396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4743);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 11903);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20568);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
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
		['п', s @ ..] => {
			super::push(out, 11904);
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рщивать.
pub(crate) fn n5397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4744);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11905);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20569);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11906);
			match s {
				[] => true,
				['м', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'о', 'т', 'с', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ущивать.
pub(crate) fn n5398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4745);
	match s {
		[] => true,
		['л', s @ ..] => {
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
				['о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ющивать.
pub(crate) fn n5399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4746);
	match s {
		[] => true,
		['л', 'п', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -твовать.
pub(crate) fn n5407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4754);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 11915);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20575);
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
		['р', s @ ..] => {
			super::push(out, 11916);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20576);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34648);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 34649);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 38436);
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
									super::push(out, 34650);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 38437);
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
		['с', s @ ..] => super::t25::n15223(s, out),
		_ => false,
	}
}

/// The word забрезговать.
pub(crate) fn n81722(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word побрезговать.
pub(crate) fn n81723(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -енговать.
pub(crate) fn n15232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11926);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20598);
			match s {
				[] => true,
				['е', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34841);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34842);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38661);
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
			super::push(out, 20599);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28913);
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

/// Words ending -инговать.
pub(crate) fn n15233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11927);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 20600);
			match s {
				[] => true,
				['е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 20601);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28914);
					match s {
						[] => true,
						['и', 'р', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 28915);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 34843);
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
		['м', s @ ..] => {
			super::push(out, 20602);
			match s {
				[] => true,
				['м', 'и', 'р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ю', 'ь', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 20603);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28916);
					match s {
						[] => true,
						['х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 28917);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 34844);
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
			super::push(out, 20604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28918);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 34845);
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
		['р', 'р', 'а', 'п', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 20605);
			match s {
				[] => true,
				['р', 'и', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 20606);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28919);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 34846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38662);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40954);
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
									super::push(out, 38663);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40955);
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
				['й', s @ ..] => {
					super::push(out, 28920);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34847);
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
				['н', s @ ..] => {
					super::push(out, 28921);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34848);
							match s {
								[] => true,
								['р', 'п', 'м', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34849);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стоговать.
pub(crate) fn n15234(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20607);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 20608);
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

/// Words ending -рговать.
pub(crate) fn n5413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4760);
	match s {
		[] => true,
		['о', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20609);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20610);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 28923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 28924);
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
					super::push(out, 20611);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20612);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20613);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20614);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 28925);
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
					super::push(out, 20615);
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
					super::push(out, 20616);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20618);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 28926);
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
		['у', s @ ..] => {
			super::push(out, 11928);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20619);
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

/// Words ending -уговать.
pub(crate) fn n5414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4761);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11929);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20620);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 28927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20621);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20622);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20623);
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

/// Words ending -юговать.
pub(crate) fn n5415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4762);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11931);
			match s {
				[] => true,
				['е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отстрадовать.
pub(crate) fn n81727(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word раздосадовать.
pub(crate) fn n81728(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word подосадовать.
pub(crate) fn n81729(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оведовать.
pub(crate) fn n30229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20633);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34855);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34856);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34857);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 38667);
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
					super::push(out, 34858);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 38668);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 40956);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 42279);
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
		_ => false,
	}
}

/// Words ending -ледовать.
pub(crate) fn n15244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11936);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28933);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 28934);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38669);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 40957);
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
									super::push(out, 38670);
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
					super::push(out, 28935);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 34859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 38671);
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42280);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 42281);
													match s {
														[] => true,
														['н', '\u{1}', s @ ..] => super::t25::n88143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 40958);
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
					super::push(out, 28936);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34860);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 38672);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									super::push(out, 38673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28937);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 34861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 34862);
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
						['р', s @ ..] => {
							super::push(out, 34863);
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
					super::push(out, 28938);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34864);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 40959);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 42282);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n88144(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 40960);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 42283);
													match s {
														[] => true,
														['е', 'п', '\u{1}', s @ ..] => super::t25::n88145(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 40961);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 42284);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n88146(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34865);
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

/// Words ending -редовать.
pub(crate) fn n15245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11937);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 20634);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 28939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34866);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 38675);
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
			super::push(out, 20635);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28940);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -седовать.
pub(crate) fn n15246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11938);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 20636);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34867);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 34868);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 38676);
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
							super::push(out, 34869);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38677);
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
								['с', s @ ..] => {
									super::push(out, 38678);
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
							super::push(out, 34870);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 38679);
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

/// Words ending -аждовать.
pub(crate) fn n15247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11939);
	match s {
		[] => true,
		['р', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28941);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 28942);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28943);
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

/// Words ending -здовать.
pub(crate) fn n5419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4766);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11940);
			match s {
				[] => true,
				['н', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20637);
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
			super::push(out, 11941);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28944);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28945);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 28946);
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

/// Words ending -идовать.
pub(crate) fn n5420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4767);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11942);
			match s {
				[] => true,
				['а', 'з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28947);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28948);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34871);
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

/// Words ending -лдовать.
pub(crate) fn n5421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4768);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11943);
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
							super::push(out, 28951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 34872);
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
							super::push(out, 28952);
							match s {
								[] => true,
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндовать.
pub(crate) fn n5422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4769);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11944);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20639);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 28953);
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
				['м', s @ ..] => {
					super::push(out, 20640);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34873);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34874);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 38680);
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
									super::push(out, 34875);
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
									super::push(out, 34876);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 34877);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 38681);
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
			super::push(out, 11945);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20641);
					match s {
						[] => true,
						['о', 'к', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34878);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34879);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 34880);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 34881);
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
				['р', s @ ..] => {
					super::push(out, 20642);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34882);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34883);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 34884);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 34885);
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
				['т', s @ ..] => {
					super::push(out, 20643);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одовать.
pub(crate) fn n5423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4770);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 11946);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20644);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20645);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34886);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34887);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 38682);
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
									super::push(out, 34888);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 38683);
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
									super::push(out, 34889);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 38684);
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
			super::push(out, 11947);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20646);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11948);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20647);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28955);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 28956);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 34890);
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
		['х', s @ ..] => {
			super::push(out, 11949);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20649);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34891);
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20650);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34892);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 34893);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 34894);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 38685);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдовать.
pub(crate) fn n5424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4771);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11950);
			match s {
				[] => true,
				['б', 'м', 'о', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11951);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20651);
					match s {
						[] => true,
						['о', 'л', 'и', 'м', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11952);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28957);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28958);
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
		['о', s @ ..] => {
			super::push(out, 11953);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28959);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28960);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 28961);
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
		_ => false,
	}
}

/// Words ending -удовать.
pub(crate) fn n5425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4772);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11954);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28962);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 28963);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 38686);
											match s {
												[] => true,
												['р', 'е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 38687);
											match s {
												[] => true,
												['д', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n85904(s, out),
														['е', s @ ..] => super::t25::n85905(s, out),
														_ => false,
													}
												}
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
							super::push(out, 28964);
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

/// Words ending -ядовать.
pub(crate) fn n5426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4773);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11955);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28965);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28966);
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
				['о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 28967);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 28968);
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
		_ => false,
	}
}

/// Words ending -переобразовать.
pub(crate) fn n81756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40962);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word преобразовать.
pub(crate) fn n81757(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -изовать.
pub(crate) fn n5431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4778);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11962);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20656);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28971);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 11963);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20657);
					match s {
						[] => true,
						['ф', 'о', 'р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11964);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 28972);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 34896);
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
					super::push(out, 20659);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 28973);
							match s {
								[] => true,
								['ф', 'и', 'ц', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 11965);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20660);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', 'и', 'м', 'р', 'е', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ф', 'р', 'о', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 11966);
			match s {
				[] => true,
				['п', 'о', 'р', 'в', 'е', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t25::n15275(s, out),
		['м', s @ ..] => {
			super::push(out, 11968);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20667);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 28993);
							match s {
								[] => true,
								['и', 'г', 'е', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20668);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28994);
							match s {
								[] => true,
								['ф', 'и', 'н', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20669);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 28995);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 38715);
											match s {
												[] => true,
												['г', 'л', 'а', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20670);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 28996);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 34927);
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
						['в', s @ ..] => {
							super::push(out, 28997);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 34928);
									match s {
										[] => true,
										['л', 'а', 'г', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 28998);
							match s {
								[] => true,
								['р', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 38716);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => super::t25::n74610(s, out),
										['з', s @ ..] => {
											super::push(out, 38718);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 40984);
													match s {
														[] => true,
														['д', '\u{1}', s @ ..] => super::t25::n85935(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => super::t25::n74612(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t25::n47839(s, out),
						['к', s @ ..] => super::t25::n47840(s, out),
						['м', s @ ..] => super::t25::n47841(s, out),
						['р', s @ ..] => super::t25::n47842(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t25::n30287(s, out),
				['о', s @ ..] => super::t25::n30288(s, out),
				['р', s @ ..] => super::t25::n30289(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t25::n15278(s, out),
		['р', s @ ..] => super::t25::n15279(s, out),
		['т', s @ ..] => super::t25::n15280(s, out),
		['ф', s @ ..] => super::t25::n15281(s, out),
		['х', s @ ..] => super::t25::n15282(s, out),
		_ => false,
	}
}

/// The word переиспользовать.
pub(crate) fn n81829(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -доиспользовать.
pub(crate) fn n81830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40996);
	match s {
		[] => true,
		['е', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юзовать.
pub(crate) fn n5436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4783);
	match s {
		[] => true,
		['л', 'ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20686);
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

/// The word помараковать.
pub(crate) fn n81831(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -атаковать.
pub(crate) fn n30326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20699);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бковать.
pub(crate) fn n5439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4786);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 11988);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11989);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20700);
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

/// Words ending -вковать.
pub(crate) fn n5440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4787);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11990);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дковать.
pub(crate) fn n5441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4788);
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11991);
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

/// Words ending -ековать.
pub(crate) fn n5442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4789);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20701);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20703);
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
		['р', s @ ..] => {
			super::push(out, 11993);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20704);
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
		['ц', s @ ..] => {
			super::push(out, 11994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иковать.
pub(crate) fn n5443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4790);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20706);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20707);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29051);
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
					super::push(out, 20708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29052);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 29053);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 38740);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 40997);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n85947(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 38741);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 38742);
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
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20709);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29054);
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
					super::push(out, 20710);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 29055);
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
					super::push(out, 20711);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29056);
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
				['т', s @ ..] => {
					super::push(out, 20712);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29057);
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
			super::push(out, 11997);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20713);
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
		['н', s @ ..] => {
			super::push(out, 11998);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20714);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34966);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34967);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 34968);
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
		['п', s @ ..] => {
			super::push(out, 11999);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29058);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29059);
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
		['р', s @ ..] => {
			super::push(out, 12000);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20716);
					match s {
						[] => true,
						['а', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34969);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34970);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 34971);
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
					super::push(out, 20717);
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
			super::push(out, 12001);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20718);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 34972);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40998);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 40999);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n85948(s, out),
														_ => false,
													}
												}
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
							super::push(out, 29061);
							match s {
								[] => true,
								['к', s @ ..] => super::t25::n63199(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t25::n30349(s, out),
				_ => false,
			}
		}
		['ф', s @ ..] => super::t25::n15312(s, out),
		['ч', s @ ..] => {
			super::push(out, 12003);
			match s {
				[] => true,
				['м', s @ ..] => super::t25::n30351(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t25::n15314(s, out),
		['щ', s @ ..] => super::t25::n15315(s, out),
		_ => false,
	}
}

/// The word хикковать.
pub(crate) fn n5444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4791);
	s.is_empty()
}

/// Words ending -лковать.
pub(crate) fn n5445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4792);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12006);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20723);
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
			super::push(out, 12007);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29063);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29064);
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
		['о', s @ ..] => {
			super::push(out, 12008);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29065);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29066);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 34975);
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
							super::push(out, 29067);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 29068);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 29069);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29070);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 34976);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 38748);
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
									super::push(out, 34977);
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
							super::push(out, 29071);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34978);
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
							super::push(out, 29072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34979);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 34980);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 34981);
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
							super::push(out, 29073);
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

/// Words ending -нковать.
pub(crate) fn n5446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4793);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12009);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20727);
					match s {
						[] => true,
						['ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20728);
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
			super::push(out, 12010);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29074);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'е', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29075);
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 12011);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20729);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29076);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 29077);
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
				['ц', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29078);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29079);
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
				['ш', s @ ..] => {
					super::push(out, 20731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29080);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29081);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
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
		['ы', s @ ..] => {
			super::push(out, 12012);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20732);
					match s {
						[] => true,
						['а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оковать.
pub(crate) fn n5447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4794);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12013);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 12014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 12015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12016);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20733);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 29082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34983);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 34984);
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
				['п', s @ ..] => {
					super::push(out, 20734);
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
			super::push(out, 12017);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20735);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20736);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29083);
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
					super::push(out, 20737);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29084);
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

/// Words ending -пковать.
pub(crate) fn n5448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4795);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12018);
			match s {
				[] => true,
				['ц', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рковать.
pub(crate) fn n5449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4796);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12019);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29085);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 29086);
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
		['и', s @ ..] => {
			super::push(out, 12020);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20740);
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
		['о', s @ ..] => {
			super::push(out, 12021);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29087);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29088);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 34985);
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
							super::push(out, 29089);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 34987);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 38749);
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
		['у', s @ ..] => {
			super::push(out, 12022);
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

/// Words ending -сковать.
pub(crate) fn n5450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4797);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12023);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12024);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20741);
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
			super::push(out, 12025);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20742);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29090);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29091);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 34988);
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
				['р', s @ ..] => {
					super::push(out, 20744);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20745);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 29092);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 38750);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 41001);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n85950(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12026);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20746);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29093);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29094);
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

/// Words ending -тковать.
pub(crate) fn n5451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4798);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12027);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20747);
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
			super::push(out, 12028);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12029);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20748);
					match s {
						[] => true,
						['в', 'з', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34989);
									s.is_empty()
								}
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
		['у', s @ ..] => {
			super::push(out, 12030);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29095);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29096);
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

/// Words ending -уковать.
pub(crate) fn n5452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4799);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 12031);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20750);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20751);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 29098);
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
					super::push(out, 20752);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29099);
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
					super::push(out, 20753);
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
					super::push(out, 20754);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29100);
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
					super::push(out, 20755);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29101);
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
			super::push(out, 12032);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29102);
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
		_ => false,
	}
}

/// Words ending -чковать.
pub(crate) fn n5453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4800);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12033);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29103);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29104);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 34990);
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
							super::push(out, 29105);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 34991);
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
			super::push(out, 12034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 12035);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20758);
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
		_ => false,
	}
}

/// Words ending -шковать.
pub(crate) fn n5454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4801);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12036);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12037);
			match s {
				[] => true,
				['е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 12038);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20759);
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

/// Words ending -ыковать.
pub(crate) fn n5455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4802);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 12039);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 12040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12041);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 20760);
					match s {
						[] => true,
						['а', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12042);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29106);
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
						['о', 'с', '\u{1}', s @ ..] => {
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
				['ш', s @ ..] => {
					super::push(out, 20762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьковать.
pub(crate) fn n5456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4803);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12043);
			match s {
				[] => true,
				['и', 'п', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 12044);
			match s {
				[] => true,
				['и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юковать.
pub(crate) fn n5457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4804);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 12045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20763);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
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

/// Words ending -яковать.
pub(crate) fn n5458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4805);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 12046);
			match s {
				[] => true,
				['с', 'о', 'л', 'о', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word преизобиловать.
pub(crate) fn n81845(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ремиловать.
pub(crate) fn n48038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29114);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омиловать.
pub(crate) fn n30441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20783);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 29115);
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

/// Words ending -силовать.
pub(crate) fn n15367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12057);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20784);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34993);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 34994);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 38752);
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

/// Words ending -кловать.
pub(crate) fn n5463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4810);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 12058);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20785);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 34995);
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
			super::push(out, 12059);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 20786);
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

/// Words ending -оловать.
pub(crate) fn n5464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4811);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 12060);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20787);
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
		_ => false,
	}
}

/// Words ending -словать.
pub(crate) fn n5465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4812);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12061);
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

/// Words ending -тловать.
pub(crate) fn n5466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4813);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 12062);
			match s {
				[] => true,
				['в', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уловать.
pub(crate) fn n5467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4814);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 12063);
			match s {
				[] => true,
				['и', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word заштормовать.
pub(crate) fn n81847(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перешельмовать.
pub(crate) fn n63272(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ошельмовать.
pub(crate) fn n48064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29130);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эмовать.
pub(crate) fn n5475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4822);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12073);
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

/// Words ending -реассигновать.
pub(crate) fn n81848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41002);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -азассигновать.
pub(crate) fn n81849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41003);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трассигновать.
pub(crate) fn n81850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41004);
	match s {
		[] => true,
		['н', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -праздновать.
pub(crate) fn n30478(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29139);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 29140);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35003);
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
		['о', s @ ..] => {
			super::push(out, 29141);
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
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 29142);
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

/// Words ending -лудновать.
pub(crate) fn n30479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20807);
	match s {
		[] => true,
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еновать.
pub(crate) fn n5480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4827);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 12081);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', 'к', 'э', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35004);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 35005);
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
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 29143);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 38757);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 41005);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t25::n85954(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 38758);
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29144);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29145);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29146);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 29147);
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
				['ы', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зновать.
pub(crate) fn n5481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4828);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 12082);
			match s {
				[] => true,
				['л', 'о', 'б', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29148);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 35006);
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

/// Words ending -иновать.
pub(crate) fn n5482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4829);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20808);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20809);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20810);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29149);
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
				['ю', s @ ..] => {
					super::push(out, 20811);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 29150);
							match s {
								[] => true,
								['л', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 12085);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29151);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29152);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 12086);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20812);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20813);
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
			super::push(out, 12087);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20814);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29153);
							match s {
								[] => true,
								['х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йновать.
pub(crate) fn n5483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4830);
	match s {
		[] => true,
		['а', 'м', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лновать.
pub(crate) fn n5484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4831);
	match s {
		[] => true,
		['о', 'в', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20815);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20816);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29154);
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
					super::push(out, 20817);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20818);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оновать.
pub(crate) fn n5485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4832);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 12088);
			match s {
				[] => true,
				['м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29155);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 12089);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29156);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35007);
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
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 29157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35008);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 35009);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 38759);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рновать.
pub(crate) fn n5486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4833);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12090);
			match s {
				[] => true,
				['т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12091);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29158);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 29159);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 29160);
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
			super::push(out, 12092);
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

/// Words ending -сновать.
pub(crate) fn n5487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4834);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12093);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 12094);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20819);
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
			super::push(out, 12095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20820);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 20821);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29161);
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
					super::push(out, 20822);
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

/// The word выштамповать.
pub(crate) fn n81854(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -амокалибровать.
pub(crate) fn n81855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41006);
	match s {
		[] => true,
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word прокалибровать.
pub(crate) fn n81856(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word несдобровать.
pub(crate) fn n81857(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -удровать.
pub(crate) fn n15435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12122);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 20845);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29171);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29172);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35021);
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
					super::push(out, 29173);
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

/// Words ending -еровать.
pub(crate) fn n5500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4847);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12123);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20846);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 20847);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20848);
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
			super::push(out, 12124);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 20849);
					match s {
						[] => true,
						['е', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12125);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20850);
					match s {
						[] => true,
						['р', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 12126);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20851);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 29174);
							match s {
								[] => true,
								['к', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 12127);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20852);
					match s {
						[] => true,
						['р', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 35022);
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
		['и', s @ ..] => {
			super::push(out, 12128);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 29175);
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
		['к', s @ ..] => {
			super::push(out, 12129);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20853);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 35023);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 38765);
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
				['к', 'у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20854);
					match s {
						[] => true,
						['е', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 29176);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 35024);
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
						['у', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 12130);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 35025);
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
					super::push(out, 20856);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 29177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 35026);
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
		['м', s @ ..] => {
			super::push(out, 12131);
			match s {
				[] => true,
				['о', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 29178);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20857);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 35027);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 35028);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 35029);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 35030);
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
		['н', s @ ..] => {
			super::push(out, 12132);
			match s {
				[] => true,
				['а', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29179);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 35031);
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
					super::push(out, 20858);
					match s {
						[] => true,
						['е', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12133);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20859);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 35032);
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
			super::push(out, 12134);
			match s {
				[] => true,
				['и', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'ф', s @ ..] => super::t25::n30564(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t25::n15448(s, out),
		['ч', s @ ..] => super::t25::n15449(s, out),
		_ => false,
	}
}

